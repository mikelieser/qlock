//! Drawing primitives on top of tiny-skia: coverage masks, blur, glyphs.

use ab_glyph::{Font, FontVec, PxScale, ScaleFont, VariableFont};
use tiny_skia::{FillRule, Mask, Paint, PathBuilder, Pixmap, Rect, Transform};

use crate::theme::Color;

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

    /// A copy with `pad` empty pixels on every side.
    pub fn padded(&self, pad: usize) -> Coverage {
        let mut out = Coverage::new(self.w + 2 * pad, self.h + 2 * pad);
        for y in 0..self.h {
            out.data[(y + pad) * out.w + pad..(y + pad) * out.w + pad + self.w].copy_from_slice(&self.data[y * self.w..(y + 1) * self.w]);
        }
        out
    }

    /// Grows the shape by `r` pixels (round brush), for outlines.
    pub fn dilated(&self, r: f32) -> Coverage {
        let ri = r.ceil() as i32;
        let mut out = Coverage::new(self.w, self.h);
        let offsets: Vec<(i32, i32, f32)> = (-ri..=ri)
            .flat_map(|dy| (-ri..=ri).map(move |dx| (dx, dy)))
            .filter_map(|(dx, dy)| {
                let d = ((dx * dx + dy * dy) as f32).sqrt();
                (d <= r + 0.5).then(|| (dx, dy, (r + 0.5 - d).min(1.0)))
            })
            .collect();
        for y in 0..self.h as i32 {
            for x in 0..self.w as i32 {
                let mut m: f32 = 0.0;
                for &(dx, dy, w) in &offsets {
                    let (sx, sy) = (x + dx, y + dy);
                    if sx >= 0 && sy >= 0 && (sx as usize) < self.w && (sy as usize) < self.h {
                        m = m.max(self.data[sy as usize * self.w + sx as usize] * w);
                    }
                }
                out.data[y as usize * self.w + x as usize] = m;
            }
        }
        out
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
            // Dither: soft glows span few 8-bit steps and would show rings otherwise.
            let d = dither(i as u32);
            let out_a = ((src[3] + data[i + 3] as f32 / 255.0 * inv) * 255.0 + 0.5 + d).clamp(0.0, 255.0);
            for k in 0..3 {
                let dst = data[i + k] as f32 / 255.0;
                data[i + k] = ((src[k] + dst * inv) * 255.0 + 0.5 + d).clamp(0.0, out_a) as u8;
            }
            data[i + 3] = out_a as u8;
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
    rounded_rect_mask_xy(w, h, rect, radius, radius)
}

/// Rounded rectangle with elliptical corners (CSS `border-radius: 3%` is 3 % of width and of height).
pub fn rounded_rect_mask_xy(w: u32, h: u32, rect: [f32; 4], rx: f32, ry: f32) -> Option<Mask> {
    let path = rounded_rect_path(rect, rx, ry)?;
    let mut mask = Mask::new(w, h)?;
    mask.fill_path(&path, FillRule::Winding, true, Transform::identity());
    Some(mask)
}

pub fn rounded_rect_path(rect: [f32; 4], rx: f32, ry: f32) -> Option<tiny_skia::Path> {
    let [x, y, rw, rh] = rect;
    let (rx, ry) = (rx.min(rw / 2.0).max(0.0), ry.min(rh / 2.0).max(0.0));
    let mut pb = PathBuilder::new();
    if rx <= 0.0 || ry <= 0.0 {
        pb.push_rect(Rect::from_xywh(x, y, rw, rh)?);
        return pb.finish();
    }
    let (kx, ky) = (0.552_284_8 * rx, 0.552_284_8 * ry);
    pb.move_to(x + rx, y);
    pb.line_to(x + rw - rx, y);
    pb.cubic_to(x + rw - rx + kx, y, x + rw, y + ry - ky, x + rw, y + ry);
    pb.line_to(x + rw, y + rh - ry);
    pb.cubic_to(x + rw, y + rh - ry + ky, x + rw - rx + kx, y + rh, x + rw - rx, y + rh);
    pb.line_to(x + rx, y + rh);
    pb.cubic_to(x + rx - kx, y + rh, x, y + rh - ry + ky, x, y + rh - ry);
    pb.line_to(x, y + ry);
    pb.cubic_to(x, y + ry - ky, x + rx - kx, y, x + rx, y);
    pb.close();
    pb.finish()
}

/// A font at one weight: a variable font with its `wght` axis set, or a static cut.
pub struct Face {
    font: FontVec,
    /// For letters the main font lacks: Japanese (a subset of Noto Sans JP with just the clock's kanji).
    fallback: Option<FontVec>,
}

const FALLBACK: &[u8] = include_bytes!("../../assets/fonts/NotoSansJP-subset.ttf");

impl Face {
    pub fn new(bytes: &[u8], weight: f32) -> Option<Face> {
        let mut font = FontVec::try_from_vec(bytes.to_vec()).ok()?;
        font.set_variation(b"wght", weight);
        let fallback = FontVec::try_from_vec(FALLBACK.to_vec()).ok().map(|mut f| {
            f.set_variation(b"wght", weight);
            f
        });
        Some(Face { font, fallback })
    }

    /// Underline offset below the baseline and thickness, at `px`.
    pub fn underline(&self, px: f32) -> (f32, f32) {
        (0.12 * px, (0.07 * px).max(1.0))
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
            if id.0 == 0 {
                // Not in the main font: draw it from the fallback at the same size.
                if let Some(fb) = self.fallback.as_ref().filter(|f| f.glyph_id(ch).0 != 0) {
                    let fscale = PxScale::from(px * fb.height_unscaled() / fb.units_per_em().unwrap_or(1000.0));
                    let fid = fb.glyph_id(ch);
                    let glyph = fid.with_scale_and_position(fscale, ab_glyph::point(pen, 0.0));
                    pen += fb.as_scaled(fscale).h_advance(fid) + tracking;
                    prev = None;
                    if let Some(o) = fb.outline_glyph(glyph) {
                        outlined.push(o);
                    }
                    continue;
                }
            }
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

/// Triangular-distributed offset in (-1, 1) from the pixel index: two cheap hashes added.
pub fn dither(i: u32) -> f32 {
    let hash = |seed: u32| {
        let mut v = i.wrapping_mul(0x9E37_79B9) ^ seed;
        v ^= v >> 16;
        v = v.wrapping_mul(0x85EB_CA6B);
        v ^= v >> 13;
        v = v.wrapping_mul(0xC2B2_AE35);
        v ^= v >> 16;
        v as f32 / u32::MAX as f32
    };
    hash(0x68E3_1DA4) + hash(0xB529_7A4D) - 1.0
}
