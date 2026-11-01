//! Paints CSS backgrounds (gradients, SVG and bitmap images) in floating point, then quantises once with
//! dithering, so large gentle gradients don't band.

use std::sync::Arc;

use tiny_skia::{FilterQuality, Mask, Pixmap, PixmapPaint, Transform};

use crate::css::{self, Image, Layer, RadialSize, Size, Units};
use crate::raster::dither;
use crate::theme::Color;

/// Theme bitmaps referenced from the CSS, embedded so every target has them.
fn asset(url: &str) -> Option<&'static [u8]> {
    let name = url.rsplit('/').next()?;
    Some(match name {
        "pattern.jpg" => include_bytes!("../../assets/themes/formrausch-pattern.png"),
        "cross.png" => include_bytes!("../../assets/themes/formrausch-cross.png"),
        "bg.png" => include_bytes!("../../assets/themes/metal-bg.png"),
        _ => return None,
    })
}

/// A decoded image: an SVG tree (any size) or a bitmap.
#[derive(Clone)]
pub enum Picture {
    Svg { tree: Arc<resvg::usvg::Tree>, stretch: bool },
    Bitmap(Arc<Pixmap>),
}

impl Picture {
    pub fn load(url: &str) -> Option<Picture> {
        if let Some(data) = url.strip_prefix("data:image/svg+xml,") {
            let text = percent_encoding::percent_decode_str(data).decode_utf8().ok()?;
            let tree = resvg::usvg::Tree::from_str(&text, &resvg::usvg::Options::default()).ok()?;
            return Some(Picture::Svg { tree: Arc::new(tree), stretch: text.contains("preserveAspectRatio='none'") || text.contains("preserveAspectRatio=\"none\"") });
        }
        Pixmap::decode_png(asset(url)?).ok().map(|p| Picture::Bitmap(Arc::new(p)))
    }

    /// Intrinsic size in CSS px (SVG: its viewBox).
    pub fn size(&self) -> (f32, f32) {
        match self {
            Picture::Svg { tree, .. } => (tree.size().width(), tree.size().height()),
            Picture::Bitmap(p) => (p.width() as f32, p.height() as f32),
        }
    }

    /// Renders into a new pixmap of `w`×`h`.
    pub fn render(&self, w: u32, h: u32) -> Option<Pixmap> {
        let mut out = Pixmap::new(w.max(1), h.max(1))?;
        let (iw, ih) = self.size();
        match self {
            Picture::Svg { tree, stretch } => {
                let (sx, sy) = (w as f32 / iw, h as f32 / ih);
                let t = if *stretch {
                    Transform::from_scale(sx, sy)
                } else {
                    let s = sx.min(sy);
                    Transform::from_row(s, 0.0, 0.0, s, (w as f32 - iw * s) / 2.0, (h as f32 - ih * s) / 2.0)
                };
                resvg::render(tree, t, &mut out.as_mut());
            }
            Picture::Bitmap(p) => {
                let paint = PixmapPaint { quality: FilterQuality::Bicubic, ..Default::default() };
                out.draw_pixmap(0, 0, p.as_ref().as_ref(), &paint, Transform::from_scale(w as f32 / iw, h as f32 / ih), None);
            }
        }
        Some(out)
    }
}

/// Premultiplied float canvas.
pub struct FloatCanvas {
    pub w: usize,
    pub h: usize,
    data: Vec<[f32; 4]>,
}

impl FloatCanvas {
    pub fn new(w: u32, h: u32) -> FloatCanvas {
        FloatCanvas { w: w as usize, h: h as usize, data: vec![[0.0; 4]; (w * h) as usize] }
    }

    #[inline]
    fn over(&mut self, i: usize, src: [f32; 4], coverage: f32) {
        let a = src[3] * coverage;
        if a <= 0.0 {
            return;
        }
        let d = &mut self.data[i];
        let inv = 1.0 - a;
        d[0] = src[0] * coverage + d[0] * inv;
        d[1] = src[1] * coverage + d[1] * inv;
        d[2] = src[2] * coverage + d[2] * inv;
        d[3] = a + d[3] * inv;
    }

    /// Runs `src(x, y)` for every pixel of the box in parallel row bands and blends the result through
    /// the mask. Full-screen gradients at 5K are tens of millions of pixels per layer.
    fn shade(&mut self, b: (usize, usize, usize, usize), mask: Option<&Mask>, src: impl Fn(usize, usize) -> Option<[f32; 4]> + Sync) {
        let (x0, y0, x1, y1) = b;
        if x1 <= x0 || y1 <= y0 {
            return;
        }
        let w = self.w;
        let threads = std::thread::available_parallelism().map_or(4, |n| n.get()).min(16);
        let rows = &mut self.data[y0 * w..y1 * w];
        let band = (y1 - y0).div_ceil(threads).max(1);
        let mask = mask.map(|m| m.data());
        std::thread::scope(|scope| {
            for (k, chunk) in rows.chunks_mut(band * w).enumerate() {
                let src = &src;
                scope.spawn(move || {
                    for (r, row) in chunk.chunks_mut(w).enumerate() {
                        let y = y0 + k * band + r;
                        for x in x0..x1 {
                            let Some(c) = src(x, y) else { continue };
                            let cov = mask.map_or(1.0, |m| m[y * w + x] as f32 / 255.0);
                            let a = c[3] * cov;
                            if a <= 0.0 {
                                continue;
                            }
                            let d = &mut row[x];
                            let inv = 1.0 - a;
                            d[0] = c[0] * cov + d[0] * inv;
                            d[1] = c[1] * cov + d[1] * inv;
                            d[2] = c[2] * cov + d[2] * inv;
                            d[3] = a + d[3] * inv;
                        }
                    }
                });
            }
        });
    }

    fn bounds(&self, area: [f32; 4]) -> (usize, usize, usize, usize) {
        let [x, y, w, h] = area;
        (
            x.floor().max(0.0) as usize,
            y.floor().max(0.0) as usize,
            ((x + w).ceil().max(0.0) as usize).min(self.w),
            ((y + h).ceil().max(0.0) as usize).min(self.h),
        )
    }

    /// Paints a CSS `background` value into `area` ([x, y, w, h] in px), through an optional mask.
    pub fn paint_css(&mut self, value: &str, area: [f32; 4], mask: Option<&Mask>, u: &Units) {
        let (layers, color) = css::parse_background(value);
        if let Some(c) = color {
            let pre = [c.r * c.a, c.g * c.a, c.b * c.a, c.a];
            let b = self.bounds(area);
            self.shade(b, mask, |_, _| Some(pre));
        }
        for layer in layers.iter().rev() {
            self.paint_layer(layer, area, mask, u);
        }
    }

    fn paint_layer(&mut self, layer: &Layer, area: [f32; 4], mask: Option<&Mask>, u: &Units) {
        let [ax, ay, aw, ah] = area;
        let picture = match &layer.image {
            Image::Url(url) => Picture::load(url),
            _ => None,
        };
        if matches!(layer.image, Image::Url(_)) && picture.is_none() {
            return;
        }
        // Tile size: images keep their ratio for `auto`, gradients default to the whole area.
        let ratio = picture.as_ref().map(|p| p.size()).map(|(w, h)| w / h);
        let natural = picture.as_ref().map(|p| p.size());
        let (tw, th) = match (layer.size, ratio) {
            (Size::Cover, Some(r)) => if aw / ah > r { (aw, aw / r) } else { (ah * r, ah) },
            (Size::Contain, Some(r)) => if aw / ah > r { (ah * r, ah) } else { (aw, aw / r) },
            (Size::Explicit(w, h), r) => {
                let w = w.map(|l| l.px(u, aw));
                let h = h.map(|l| l.px(u, ah));
                match (w, h, r) {
                    (Some(w), Some(h), _) => (w, h),
                    (Some(w), None, Some(r)) => (w, w / r),
                    (None, Some(h), Some(r)) => (h * r, h),
                    (Some(w), None, None) => (w, ah),
                    (None, Some(h), None) => (aw, h),
                    _ => (aw, ah),
                }
            }
            (_, Some(_)) => natural.unwrap(),
            _ => (aw, ah),
        };
        let tx = ax + layer.pos.0.px(u, aw - tw);
        let ty = ay + layer.pos.1.px(u, ah - th);
        let b = self.bounds(area);

        if let Some(picture) = picture {
            let Some(tile) = picture.render(tw.round().max(1.0) as u32, th.round().max(1.0) as u32) else { return };
            let (tw_i, th_i) = (tile.width() as i64, tile.height() as i64);
            let data = tile.data();
            let (repeat_x, repeat_y) = (layer.repeat, layer.repeat_y);
            self.shade(b, mask, |x, y| {
                let (mut lx, mut ly) = (x as i64 - tx.round() as i64, y as i64 - ty.round() as i64);
                if repeat_x {
                    lx = lx.rem_euclid(tw_i);
                }
                if repeat_y {
                    ly = ly.rem_euclid(th_i);
                }
                if lx < 0 || ly < 0 || lx >= tw_i || ly >= th_i {
                    return None;
                }
                let k = ((ly * tw_i + lx) * 4) as usize;
                Some([data[k] as f32 / 255.0, data[k + 1] as f32 / 255.0, data[k + 2] as f32 / 255.0, data[k + 3] as f32 / 255.0])
            });
            return;
        }

        // Gradients, evaluated per pixel within the tile.
        let repeat = layer.repeat;
        let inside = move |x: f32, y: f32| repeat || (x >= tx && y >= ty && x < tx + tw && y < ty + th);
        match &layer.image {
            Image::Linear { angle, stops, repeating } => {
                let a = angle.to_radians();
                let (dx, dy) = (a.sin(), -a.cos());
                let len = (tw * a.sin()).abs() + (th * a.cos()).abs();
                let stops = css::resolve_stops(stops, len, u);
                let (cx, cy) = (tx + tw / 2.0, ty + th / 2.0);
                let rep = *repeating;
                self.shade(b, mask, |x, y| {
                    let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
                    inside(px, py).then(|| css::sample(&stops, ((px - cx) * dx + (py - cy) * dy) / len + 0.5, rep))
                });
            }
            Image::Radial { circle, size, at, stops, repeating } => {
                let (cx, cy) = (tx + at.0.px(u, tw), ty + at.1.px(u, th));
                let (dl, dr, dt, db) = (cx - tx, tx + tw - cx, cy - ty, ty + th - cy);
                let (near_x, far_x, near_y, far_y) = (dl.min(dr), dl.max(dr), dt.min(db), dt.max(db));
                let (rx, ry) = match (size, circle) {
                    (RadialSize::Circle(len), _) => {
                        let r = len.px(u, tw);
                        (r, r)
                    }
                    (RadialSize::Ellipse(a, b), _) => (a.px(u, tw), b.px(u, th)),
                    (RadialSize::ClosestSide, true) => (near_x.min(near_y), near_x.min(near_y)),
                    (RadialSize::ClosestSide, false) => (near_x, near_y),
                    (RadialSize::FarthestSide, true) => (far_x.max(far_y), far_x.max(far_y)),
                    (RadialSize::FarthestSide, false) => (far_x, far_y),
                    (RadialSize::ClosestCorner, true) => {
                        let r = near_x.hypot(near_y);
                        (r, r)
                    }
                    (RadialSize::ClosestCorner, false) => (near_x * std::f32::consts::SQRT_2, near_y * std::f32::consts::SQRT_2),
                    (RadialSize::FarthestCorner, true) => {
                        let r = far_x.hypot(far_y);
                        (r, r)
                    }
                    (RadialSize::FarthestCorner, false) => (far_x * std::f32::consts::SQRT_2, far_y * std::f32::consts::SQRT_2),
                };
                let (rx, ry) = (rx.max(1e-3), ry.max(1e-3));
                let stops = css::resolve_stops(stops, rx, u);
                let rep = *repeating;
                self.shade(b, mask, |x, y| {
                    let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
                    inside(px, py).then(|| css::sample(&stops, ((px - cx) / rx).hypot((py - cy) / ry), rep))
                });
            }
            Image::Conic { from, at, stops, repeating } => {
                let (cx, cy) = (tx + at.0.px(u, tw), ty + at.1.px(u, th));
                let stops = css::resolve_stops(stops, 360.0, u);
                let (rep, from) = (*repeating, *from);
                self.shade(b, mask, |x, y| {
                    let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
                    // CSS: 0deg points up, clockwise.
                    let deg = ((px - cx).atan2(-(py - cy)).to_degrees() - from).rem_euclid(360.0);
                    inside(px, py).then(|| css::sample(&stops, deg / 360.0, rep))
                });
            }
            Image::Url(_) => unreachable!(),
        }
    }

    /// Paints a soft or hard shape (coverage 0–1 over the whole canvas) in one colour.
    pub fn paint_coverage(&mut self, coverage: &[f32], color: Color) {
        let pre = [color.r * color.a, color.g * color.a, color.b * color.a, color.a];
        for (i, &c) in coverage.iter().enumerate() {
            if c > 0.0 {
                self.over(i, pre, c);
            }
        }
    }

    /// Premultiplied 8 bit with triangular dither of ±1 step.
    pub fn into_pixmap(self) -> Pixmap {
        let mut out = Pixmap::new(self.w as u32, self.h as u32).unwrap();
        let threads = std::thread::available_parallelism().map_or(4, |n| n.get()).min(16);
        let per = self.data.len().div_ceil(threads).max(1);
        std::thread::scope(|scope| {
            for (k, (dst, src)) in out.data_mut().chunks_mut(per * 4).zip(self.data.chunks(per)).enumerate() {
                scope.spawn(move || {
                    for (j, px) in src.iter().enumerate() {
                        let d = dither((k * per + j) as u32);
                        let a = (px[3] * 255.0 + 0.5).clamp(0.0, 255.0);
                        dst[j * 4 + 3] = a as u8;
                        for c in 0..3 {
                            dst[j * 4 + c] = (px[c] * 255.0 + 0.5 + d).clamp(0.0, a) as u8;
                        }
                    }
                });
            }
        });
        out
    }

    /// Paints `color` through a coverage map given at 1/`step` resolution, bilinearly upsampled.
    /// For very soft shapes (shadows), where full resolution only costs time.
    pub fn paint_coverage_scaled(&mut self, cov: &[f32], cw: usize, ch: usize, step: f32, color: Color) {
        let pre = [color.r * color.a, color.g * color.a, color.b * color.a, color.a];
        let b = (0, 0, self.w, self.h);
        self.shade(b, None, |x, y| {
            let (fx, fy) = ((x as f32 + 0.5) / step - 0.5, (y as f32 + 0.5) / step - 0.5);
            let (x0, y0) = (fx.floor().clamp(0.0, (cw - 1) as f32) as usize, fy.floor().clamp(0.0, (ch - 1) as f32) as usize);
            let (x1, y1) = ((x0 + 1).min(cw - 1), (y0 + 1).min(ch - 1));
            let (tx, ty) = ((fx - x0 as f32).clamp(0.0, 1.0), (fy - y0 as f32).clamp(0.0, 1.0));
            let at = |x: usize, y: usize| cov[y * cw + x];
            let v = (at(x0, y0) * (1.0 - tx) + at(x1, y0) * tx) * (1.0 - ty) + (at(x0, y1) * (1.0 - tx) + at(x1, y1) * tx) * ty;
            (v > 0.0).then(|| [pre[0] * v, pre[1] * v, pre[2] * v, pre[3] * v])
        });
    }
}
