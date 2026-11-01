//! Drawing primitives on top of tiny-skia: CSS-like fills, SVG paths, glyphs, glows.

use ab_glyph::{Font, FontVec, PxScale, ScaleFont, VariableFont};
use tiny_skia::{
    FillRule, GradientStop, LinearGradient, Mask, Paint, PathBuilder, Pixmap, Point, RadialGradient, Rect, Shader,
    SpreadMode, Transform,
};

use crate::theme::{Color, Fill};

/// Single-channel coverage, 0–1.
pub struct Coverage {
    pub w: usize,
    pub h: usize,
    pub data: Vec<f32>,
}

impl Coverage {
    pub fn new(w: usize, h: usize) -> Coverage {
        Coverage { w, h, data: vec![0.0; w * h] }
    }

    /// Gaussian blur, approximated by three box blurs.
    pub fn blurred(&self, sigma: f32) -> Coverage {
        if sigma < 0.3 {
            return Coverage { w: self.w, h: self.h, data: self.data.clone() };
        }
        // Box sizes for three passes (Kovesi), as radii.
        let n = 3.0;
        let ideal = (12.0 * sigma * sigma / n + 1.0).sqrt();
        let mut wl = ideal.floor() as i32;
        if wl % 2 == 0 {
            wl -= 1;
        }
        let wu = wl + 2;
        let m = ((12.0 * sigma * sigma - n * (wl * wl) as f32 - 4.0 * n * wl as f32 - 3.0 * n) / (-4.0 * wl as f32 - 4.0)).round() as i32;
        let mut out = Coverage { w: self.w, h: self.h, data: self.data.clone() };
        for pass in 0..3 {
            let radius = (if pass < m { wl } else { wu } - 1) / 2;
            out.box_blur(radius.max(0) as usize);
        }
        out
    }

    fn box_blur(&mut self, r: usize) {
        if r == 0 {
            return;
        }
        let (w, h) = (self.w, self.h);
        let mut tmp = vec![0.0f32; w * h];
        let norm = 1.0 / (2 * r + 1) as f32;
        for y in 0..h {
            let row = &self.data[y * w..(y + 1) * w];
            let mut acc: f32 = row.iter().take(r + 1).sum();
            for x in 0..w {
                tmp[y * w + x] = acc * norm;
                if x + r + 1 < w {
                    acc += row[x + r + 1];
                }
                if x >= r {
                    acc -= row[x - r];
                }
            }
        }
        for x in 0..w {
            let mut acc: f32 = (0..=r.min(h - 1)).map(|y| tmp[y * w + x]).sum();
            for y in 0..h {
                self.data[y * w + x] = acc * norm;
                if y + r + 1 < h {
                    acc += tmp[(y + r + 1) * w + x];
                }
                if y >= r {
                    acc -= tmp[(y - r) * w + x];
                }
            }
        }
    }
}

/// Paints `color` through `coverage` onto the pixmap at (x, y), source-over, premultiplied.
pub fn paint_coverage(pixmap: &mut Pixmap, coverage: &Coverage, x: i32, y: i32, color: Color) {
    let (pw, ph) = (pixmap.width() as i32, pixmap.height() as i32);
    let data = pixmap.data_mut();
    for cy in 0..coverage.h as i32 {
        let py = y + cy;
        if py < 0 || py >= ph {
            continue;
        }
        for cx in 0..coverage.w as i32 {
            let px = x + cx;
            if px < 0 || px >= pw {
                continue;
            }
            let a = (coverage.data[(cy as usize) * coverage.w + cx as usize] * color.a).clamp(0.0, 1.0);
            if a <= 0.0 {
                continue;
            }
            let i = ((py * pw + px) * 4) as usize;
            let inv = 1.0 - a;
            let src = [color.r * a, color.g * a, color.b * a, a];
            for k in 0..4 {
                let dst = data[i + k] as f32 / 255.0;
                data[i + k] = ((src[k] + dst * inv) * 255.0 + 0.5).clamp(0.0, 255.0) as u8;
            }
        }
    }
}

/// Raises the brightness of every pixel, like CSS `filter: brightness(f)`.
pub fn brighten(pixmap: &mut Pixmap, factor: f32) {
    for px in pixmap.data_mut().chunks_exact_mut(4) {
        let a = px[3];
        for c in &mut px[..3] {
            *c = ((*c as f32 * factor).min(a as f32)) as u8;
        }
    }
}

pub fn fill_rect(pixmap: &mut Pixmap, rect: [f32; 4], color: Color) {
    if let Some(r) = Rect::from_xywh(rect[0], rect[1], rect[2], rect[3]) {
        let mut paint = Paint::default();
        paint.set_color(color.to_skia());
        pixmap.fill_rect(r, &paint, Transform::identity(), None);
    }
}

pub fn fill_circle(pixmap: &mut Pixmap, cx: f32, cy: f32, r: f32, color: Color) {
    if let Some(path) = PathBuilder::from_circle(cx, cy, r) {
        let mut paint = Paint::default();
        paint.set_color(color.to_skia());
        paint.anti_alias = true;
        pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
    }
}

/// Coverage of a filled circle, for glows.
pub fn circle_coverage(size: usize, cx: f32, cy: f32, r: f32) -> Coverage {
    let mut pixmap = Pixmap::new(size as u32, size as u32).unwrap();
    fill_circle(&mut pixmap, cx, cy, r, Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 });
    let data = pixmap.data().chunks_exact(4).map(|p| p[3] as f32 / 255.0).collect();
    Coverage { w: size, h: size, data }
}

pub fn rounded_rect_mask(w: u32, h: u32, rect: [f32; 4], radius: f32) -> Option<Mask> {
    let mut mask = Mask::new(w, h)?;
    let [x, y, rw, rh] = rect;
    let r = radius.min(rw / 2.0).min(rh / 2.0);
    let mut pb = PathBuilder::new();
    if r <= 0.0 {
        pb.push_rect(Rect::from_xywh(x, y, rw, rh)?);
    } else {
        // Corners as quarter circles via cubic Béziers.
        let k = 0.552_284_8 * r;
        pb.move_to(x + r, y);
        pb.line_to(x + rw - r, y);
        pb.cubic_to(x + rw - r + k, y, x + rw, y + r - k, x + rw, y + r);
        pb.line_to(x + rw, y + rh - r);
        pb.cubic_to(x + rw, y + rh - r + k, x + rw - r + k, y + rh, x + rw - r, y + rh);
        pb.line_to(x + r, y + rh);
        pb.cubic_to(x + r - k, y + rh, x, y + rh - r + k, x, y + rh - r);
        pb.line_to(x, y + r);
        pb.cubic_to(x, y + r - k, x + r - k, y, x + r, y);
        pb.close();
    }
    let path = pb.finish()?;
    mask.fill_path(&path, FillRule::Winding, true, Transform::identity());
    Some(mask)
}

fn stops(list: &[(Color, f32)]) -> Vec<GradientStop> {
    list.iter().map(|(c, p)| GradientStop::new(*p, c.to_skia())).collect()
}

/// Paints one CSS-like background layer into `area` = [x, y, w, h].
pub fn paint_fill(pixmap: &mut Pixmap, fill: &Fill, area: [f32; 4], mask: Option<&Mask>) {
    let [x, y, w, h] = area;
    let Some(rect) = Rect::from_xywh(x, y, w, h) else { return };
    let mut paint = Paint { anti_alias: true, ..Paint::default() };
    match fill {
        Fill::Solid(color) => paint.set_color(color.to_skia()),
        Fill::Linear { angle, stops: list } => {
            // CSS gradient line: through the centre, long enough that the corners hit 0 % and 100 %.
            let a = angle.to_radians();
            let (dx, dy) = (a.sin(), -a.cos());
            let len = (w * a.sin()).abs() + (h * a.cos()).abs();
            let (cx, cy) = (x + w / 2.0, y + h / 2.0);
            let start = Point::from_xy(cx - dx * len / 2.0, cy - dy * len / 2.0);
            let end = Point::from_xy(cx + dx * len / 2.0, cy + dy * len / 2.0);
            match LinearGradient::new(start, end, stops(list), SpreadMode::Pad, Transform::identity()) {
                Some(shader) => paint.shader = shader,
                None => return,
            }
        }
        Fill::Radial { x: fx, y: fy, ellipse, stops: list } => {
            let (cx, cy) = (x + fx * w, y + fy * h);
            let far_x = (cx - x).max(x + w - cx);
            let far_y = (cy - y).max(y + h - cy);
            // farthest-corner: a circle through the corner; an ellipse keeps the side ratio.
            let (rx, ry) = if *ellipse {
                (far_x * std::f32::consts::SQRT_2, far_y * std::f32::consts::SQRT_2)
            } else {
                let r = (far_x * far_x + far_y * far_y).sqrt();
                (r, r)
            };
            let transform = Transform::from_row(rx, 0.0, 0.0, ry, cx, cy);
            let centre = Point::from_xy(0.0, 0.0);
            match RadialGradient::new(centre, 0.0, centre, 1.0, stops(list), SpreadMode::Pad, transform) {
                Some(shader) => paint.shader = shader,
                None => return,
            }
        }
        Fill::Path { d, view, fill, area: rel } => {
            let Some(path) = parse_svg_path(d) else { return };
            let bx = x + rel[0] * w;
            let by = y + rel[1] * h;
            let sx = rel[2] * w / view[0];
            let sy = rel[3] * h / view[1];
            paint.set_color(fill.to_skia());
            pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::from_row(sx, 0.0, 0.0, sy, bx, by), mask);
            return;
        }
    }
    if matches!(paint.shader, Shader::SolidColor(_)) && !matches!(fill, Fill::Solid(_)) {
        // Single-stop gradients degrade to a solid colour; still fine to paint.
    }
    pixmap.fill_rect(rect, &paint, Transform::identity(), mask);
}

/// Minimal SVG path parser for the shapes the themes use: absolute M L H V Q T C Z.
pub fn parse_svg_path(d: &str) -> Option<tiny_skia::Path> {
    let mut tokens: Vec<String> = Vec::new();
    let mut num = String::new();
    for ch in d.chars() {
        if ch.is_ascii_alphabetic() {
            if !num.is_empty() {
                tokens.push(std::mem::take(&mut num));
            }
            tokens.push(ch.to_string());
        } else if ch == ' ' || ch == ',' {
            if !num.is_empty() {
                tokens.push(std::mem::take(&mut num));
            }
        } else if ch == '-' && !num.is_empty() {
            tokens.push(std::mem::take(&mut num));
            num.push(ch);
        } else {
            num.push(ch);
        }
    }
    if !num.is_empty() {
        tokens.push(num);
    }

    let mut pb = PathBuilder::new();
    let (mut cur, mut last_ctrl) = ((0.0f32, 0.0f32), None::<(f32, f32)>);
    let mut i = 0;
    let mut cmd = 'M';
    let next = |i: &mut usize| -> Option<f32> {
        let v = tokens.get(*i)?.parse().ok();
        *i += 1;
        v
    };
    while i < tokens.len() {
        if let Some(c) = tokens[i].chars().next().filter(|c| c.is_ascii_alphabetic()) {
            cmd = c;
            i += 1;
        }
        match cmd {
            'M' => {
                cur = (next(&mut i)?, next(&mut i)?);
                pb.move_to(cur.0, cur.1);
                cmd = 'L';
                last_ctrl = None;
            }
            'L' => {
                cur = (next(&mut i)?, next(&mut i)?);
                pb.line_to(cur.0, cur.1);
                last_ctrl = None;
            }
            'H' => {
                cur.0 = next(&mut i)?;
                pb.line_to(cur.0, cur.1);
                last_ctrl = None;
            }
            'V' => {
                cur.1 = next(&mut i)?;
                pb.line_to(cur.0, cur.1);
                last_ctrl = None;
            }
            'Q' => {
                let c = (next(&mut i)?, next(&mut i)?);
                cur = (next(&mut i)?, next(&mut i)?);
                pb.quad_to(c.0, c.1, cur.0, cur.1);
                last_ctrl = Some(c);
            }
            'T' => {
                let c = match last_ctrl {
                    Some((lx, ly)) => (2.0 * cur.0 - lx, 2.0 * cur.1 - ly),
                    None => cur,
                };
                cur = (next(&mut i)?, next(&mut i)?);
                pb.quad_to(c.0, c.1, cur.0, cur.1);
                last_ctrl = Some(c);
            }
            'C' => {
                let c1 = (next(&mut i)?, next(&mut i)?);
                let c2 = (next(&mut i)?, next(&mut i)?);
                cur = (next(&mut i)?, next(&mut i)?);
                pb.cubic_to(c1.0, c1.1, c2.0, c2.1, cur.0, cur.1);
                last_ctrl = None;
            }
            'Z' | 'z' => {
                pb.close();
                last_ctrl = None;
            }
            _ => return None,
        }
    }
    pb.finish()
}

/// A variable font fixed at one weight.
pub struct Face {
    font: FontVec,
}

impl Face {
    pub fn new(bytes: &[u8], weight: f32) -> Option<Face> {
        let mut font = FontVec::try_from_vec(bytes.to_vec()).ok()?;
        font.set_variation(b"wght", weight);
        Some(Face { font })
    }

    /// Coverage of `text` at `px` (CSS font-size), with `tracking` extra space after each letter in px.
    /// Returns the coverage and the offset from the pen origin (left edge, baseline) to its top left.
    pub fn text(&self, text: &str, px: f32, tracking: f32) -> (Coverage, f32, f32, f32) {
        let scale = PxScale::from(px * self.font.height_unscaled() / self.font.units_per_em().unwrap_or(1000.0));
        let scaled = self.font.as_scaled(scale);
        let mut pen = 0.0f32;
        let mut outlined = Vec::new();
        let mut prev = None;
        for ch in text.chars() {
            let id = self.font.glyph_id(ch);
            if let Some(p) = prev {
                pen += scaled.kern(p, id);
            }
            let glyph = id.with_scale_and_position(scale, ab_glyph::point(pen, 0.0));
            pen += scaled.h_advance(id) + tracking;
            prev = Some(id);
            if let Some(o) = self.font.outline_glyph(glyph) {
                outlined.push(o);
            }
        }
        let advance = pen - if text.is_empty() { 0.0 } else { tracking };
        let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        for o in &outlined {
            let b = o.px_bounds();
            x0 = x0.min(b.min.x);
            y0 = y0.min(b.min.y);
            x1 = x1.max(b.max.x);
            y1 = y1.max(b.max.y);
        }
        if outlined.is_empty() {
            return (Coverage::new(0, 0), 0.0, 0.0, advance);
        }
        let (x0, y0) = (x0.floor(), y0.floor());
        let mut cov = Coverage::new((x1.ceil() - x0) as usize + 1, (y1.ceil() - y0) as usize + 1);
        for o in &outlined {
            let b = o.px_bounds();
            let (ox, oy) = ((b.min.x - x0) as i32, (b.min.y - y0) as i32);
            o.draw(|gx, gy, c| {
                let (x, y) = (ox + gx as i32, oy + gy as i32);
                if x >= 0 && y >= 0 && (x as usize) < cov.w && (y as usize) < cov.h {
                    let v = &mut cov.data[y as usize * cov.w + x as usize];
                    *v = (*v + c).min(1.0);
                }
            });
        }
        (cov, x0, y0, advance)
    }

    /// Ascent and descent at `px`, for CSS-like vertical centring (line-height 1).
    pub fn metrics(&self, px: f32) -> (f32, f32) {
        let em = self.font.units_per_em().unwrap_or(1000.0);
        (self.font.ascent_unscaled() / em * px, self.font.descent_unscaled() / em * px)
    }
}
