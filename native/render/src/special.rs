//! What the web themes do with extra CSS selectors and components rather than custom properties:
//! face shadows, Halloween's spider web, candles, spiders and bat, Chaos Burst's turning rays,
//! the retro "groove", soap bubbles. Positions and timings follow `src/styles.css`.

use tiny_skia::{Mask, Pixmap, PixmapPaint, Transform};

use crate::css::{Shadow, Units};
use crate::paint::{FloatCanvas, Picture};
use crate::raster::{Coverage, fill_circle, paint_coverage, rounded_rect_path};
use crate::theme::{Color, Theme};
use crate::{EASE_IN_OUT, Engine, Instance, LINEAR, Layout, bezier, keyframes};

#[derive(Default)]
pub struct Special {
    /// Retro: the face decoration as its own sprite, swelling from `origin`.
    pub groove: Option<(u32, (f32, f32))>,
    /// Chaos Burst: the ray disc (drawn at half size) and its scale.
    pub rays: Option<(u32, f32)>,
    pub spooky: Option<Spooky>,
    /// The sea: wave bands, each two tiles wide so they can drift seamlessly; top band first.
    pub waves: Vec<(u32, f32)>,  // (sprite, band height without the extra water)
}

pub struct Spooky {
    /// Two leg poses per spider size, crossfaded.
    drop: [u32; 2],
    walk: [u32; 2],
    thread: u32,
    bat: u32,
}

/// CSS box-shadow outside the face.
pub(crate) fn face_shadow(bg: &mut FloatCanvas, l: &Layout, s: &Shadow, u: &Units) {
    let base = l.face[2].min(l.face[3]);
    let (dx, dy, blur, spread) = (s.x.px(u, base), s.y.px(u, base), s.blur.px(u, base), s.spread.px(u, base));
    let [x, y, w, h] = l.face;
    // Soft shadows are computed at a quarter of the resolution and scaled up; hard ones in full.
    let step = if blur > 8.0 { 4.0 } else { 1.0 };
    let rect = [(x + dx - spread) / step, (y + dy - spread) / step, (w + 2.0 * spread) / step, (h + 2.0 * spread) / step];
    let (cw, ch) = ((bg.w as f32 / step).ceil() as usize, (bg.h as f32 / step).ceil() as usize);
    let Some(path) = rounded_rect_path(rect, (l.radius.0 + spread) / step, (l.radius.1 + spread) / step) else { return };
    let Some(mut mask) = Mask::new(cw as u32, ch as u32) else { return };
    mask.fill_path(&path, tiny_skia::FillRule::Winding, true, Transform::identity());
    let cov = Coverage { w: cw, h: ch, data: mask.data().iter().map(|&v| v as f32 / 255.0).collect() };
    let cov = if blur > 0.5 { cov.blurred(blur / 2.0 / step) } else { cov };
    bg.paint_coverage_scaled(&cov.data, cw, ch, step, s.color.or(Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }));
}

/// Decoration painted once into the background.
pub(crate) fn static_decoration(bg: &mut FloatCanvas, theme: &Theme, l: &Layout, face_mask: Option<&Mask>, u: &Units) {
    if !theme.extras.spooky {
        return;
    }
    // Halloween: spider web in the top-left corner, faded out by a radial mask (CSS .clock::before).
    let [x, y, w, h] = l.face;
    let reach = w.hypot(h) * 0.38;
    let Some(mut mask) = Mask::new(bg.w as u32, bg.h as u32) else { return };
    {
        let data = mask.data_mut();
        for py in y.max(0.0) as usize..((y + h) as usize).min(bg.h) {
            for px in x.max(0.0) as usize..((x + w) as usize).min(bg.w) {
                let d = (px as f32 - x).hypot(py as f32 - y);
                let k = (1.0 - d / reach).clamp(0.0, 1.0);
                let i = py * bg.w + px;
                let face = face_mask.map_or(1.0, |m| m.data()[i] as f32 / 255.0);
                data[i] = (k * face * 255.0) as u8;
            }
        }
    }
    bg.paint_css(
        "repeating-conic-gradient(from 90deg at 0 0, rgb(220 200 255 / 0.18) 0 0.35deg, transparent 0.35deg 15deg), \
         repeating-radial-gradient(circle at 0 0, transparent 0 5.6%, rgb(220 200 255 / 0.14) 5.6% 5.9%)",
        l.face,
        Some(&mask),
        u,
    );
}

/// Retro: the decoration on its own layer, so it can swell (CSS deco-groove).
pub fn groove_sprite(e: &mut Engine, width: u32, height: u32) -> Option<(u32, (f32, f32))> {
    let theme = e.theme().clone();
    let l = e.layout();
    let [x, y, w, h] = l.face;
    let u = e.units();
    let mut canvas = FloatCanvas::new(w.ceil() as u32, h.ceil() as u32);
    canvas.paint_css(&theme.face_deco, [0.0, 0.0, w, h], None, &u);
    let origin = if theme.id == "retro-stripes" { (x + w, y + h) } else { (x + w / 2.0, y + h) };
    let _ = (width, height);
    let id = e.push(canvas.into_pixmap());
    Some((id, origin))
}

/// The sea: each wave layer of the decoration as a strip two faces wide (it tiles), drawn once.
pub fn wave_sprites(e: &mut Engine) -> Vec<(u32, f32)> {
    let theme = e.theme().clone();
    let [_, _, fw, fh] = e.layout().face;
    let u = e.units();
    let mut out = Vec::new();
    for layer in crate::css::parse_background(&theme.face_deco).0 {
        let crate::css::Image::Url(url) = &layer.image else { continue };
        let Some(picture) = Picture::load(url) else { continue };
        let th = match layer.size {
            crate::css::Size::Explicit(_, Some(h)) => h.px(&u, fh),
            _ => fh * 0.16,
        };
        let (w, h) = (fw.round() as u32, th.round().max(1.0) as u32);
        let Some(tile) = picture.render(w, h) else { continue };
        // Extra water below the band: the swell lifts it, and the face edge must stay covered.
        let below = (fh * 0.02).ceil() as u32 + 2;
        let mut strip = Pixmap::new(w * 2, h + below).unwrap();
        for k in 0..2 {
            strip.draw_pixmap((k * w) as i32, 0, tile.as_ref(), &PixmapPaint::default(), Transform::identity(), None);
        }
        let row = (w * 2 * 4) as usize;
        let data = strip.data_mut();
        let last = data[(h as usize - 1) * row..h as usize * row].to_vec();
        for y in h..h + below {
            data[y as usize * row..(y as usize + 1) * row].copy_from_slice(&last);
        }
        let id = e.push(strip);
        out.push((id, h as f32));
    }
    out
}

/// Chaos Burst: a disc of light rays covering the face when turned, drawn at half resolution.
pub fn rays_sprite(e: &mut Engine) -> Option<(u32, f32)> {
    let l = e.layout();
    let side = (l.face[2].hypot(l.face[3]) / 2.0).ceil();
    let u = e.units();
    let mut canvas = FloatCanvas::new(side as u32, side as u32);
    canvas.paint_css(
        "repeating-conic-gradient(from 0deg at 50% 50%, rgb(255 255 255 / 0.16) 0 7deg, transparent 7deg 15deg)",
        [0.0, 0.0, side, side],
        None,
        &u,
    );
    let id = e.push(canvas.into_pixmap());
    Some((id, 2.0))
}

/// Soap bubble: thin ring in the particle colour plus a highlight (CSS for Susanna's themes).
pub fn bubble(p: &mut Pixmap, cx: f32, cy: f32, r: f32, color: Color, face_min: f32) {
    let ring = (0.0025 * face_min).max(2.0);
    fill_circle(p, cx, cy, r, color);
    let mut hole = tiny_skia::Paint::default();
    hole.blend_mode = tiny_skia::BlendMode::Clear;
    hole.anti_alias = true;
    if let Some(path) = tiny_skia::PathBuilder::from_circle(cx, cy, (r - ring).max(0.0)) {
        p.fill_path(&path, &hole, tiny_skia::FillRule::Winding, Transform::identity(), None);
    }
    // radial-gradient(circle at 35% 30%, white 0.9 0 12%, transparent 30%) of the particle box.
    let (hx, hy) = (cx - r + 0.7 * r, cy - r + 0.6 * r);
    let far = (2.0 * r - 0.7 * r).hypot(2.0 * r - 0.6 * r);
    let side = p.width().max(p.height()) as usize;
    let mut cov = Coverage::new(side, side);
    for yy in 0..side {
        for xx in 0..side {
            let d = (xx as f32 + 0.5 - hx).hypot(yy as f32 + 0.5 - hy) / far;
            let inside = (xx as f32 + 0.5 - cx).hypot(yy as f32 + 0.5 - cy) <= r;
            cov.data[yy * side + xx] = if !inside { 0.0 } else if d <= 0.12 { 0.9 } else if d < 0.3 { 0.9 * (0.3 - d) / 0.18 } else { 0.0 };
        }
    }
    paint_coverage(p, &cov, 0, 0, Color::WHITE);
}

pub fn blur_pixmap(src: &Pixmap, sigma: f32) -> Pixmap {
    let (w, h) = (src.width() as usize, src.height() as usize);
    let mut out = Pixmap::new(w as u32, h as u32).unwrap();
    let data = src.data();
    let mut channels: Vec<Coverage> = (0..4).map(|_| Coverage::new(w, h)).collect();
    for i in 0..w * h {
        for (k, ch) in channels.iter_mut().enumerate() {
            ch.data[i] = data[i * 4 + k] as f32 / 255.0;
        }
    }
    let blurred: Vec<Coverage> = channels.iter().map(|c| c.blurred(sigma)).collect();
    let dst = out.data_mut();
    for i in 0..w * h {
        let a = blurred[3].data[i].clamp(0.0, 1.0);
        dst[i * 4 + 3] = (a * 255.0 + 0.5) as u8;
        for k in 0..3 {
            dst[i * 4 + k] = (blurred[k].data[i].min(a) * 255.0 + 0.5) as u8;
        }
    }
    out
}

/// Halloween candles: lit dots flicker (CSS `candle`, 2.4 s alternate, staggered per corner).
pub fn candle(e: &Engine, corner: usize, t: f64) -> f32 {
    if !e.theme().extras.spooky {
        return 1.0;
    }
    let delay = [0.0, 0.7, 1.3, 1.9][corner];
    let cycle = ((t + delay) / 2.4).rem_euclid(2.0) as f32;
    let phase = if cycle < 1.0 { cycle } else { 2.0 - cycle };
    keyframes(&[(0.0, 1.0), (0.3, 0.75), (0.55, 0.95), (0.8, 0.7), (1.0, 1.0)], phase, EASE_IN_OUT)
}

/// Layers between background and particles: Chaos Burst rays, retro groove.
pub fn before_particles(e: &Engine, out: &mut Vec<Instance>, t: f64) {
    let [x, y, w, h] = e.layout().face;
    if let Some((id, scale)) = e.special().rays {
        let turn = ((t / 90.0).rem_euclid(1.0) * std::f64::consts::TAU) as f32;
        out.push(Instance { scale, rotation: turn, ..Instance::at(id, 1, x + w / 2.0, y + h / 2.0, 1.0) });
    }
    // The sea (CSS waves-drift 48 s, waves-swell 7 s): bands drift in opposite directions, all bob together.
    let waves = &e.special().waves;
    let drift = ((t / 48.0).rem_euclid(1.0)) as f32 * w;
    let swell = keyframes(&[(0.0, 0.0), (0.5, -0.006 * h), (1.0, 0.0)], ((t / 7.0).rem_euclid(1.0)) as f32, EASE_IN_OUT);
    for (k, &(id, th)) in waves.iter().enumerate().rev() {
        // CSS lists the top band first; the first moves left, the second right.
        let left = if k % 2 == 0 { x - drift } else { x - w + drift };
        // The band's top sits where CSS puts it (tile at the bottom); the extra water hangs below the face.
        let sprite_h = e.sprites()[id as usize].height as f32;
        let top = y + h - th;
        out.push(Instance::at(id, 1, left + w, top + sprite_h / 2.0 + swell, 1.0));
    }
    if let Some((id, (ox, oy))) = e.special().groove {
        let phase = ((t / 5.0).rem_euclid(1.0)) as f32;
        let s = keyframes(&[(0.0, 1.0), (0.5, 1.12), (1.0, 1.0)], phase, EASE_IN_OUT);
        // Scale about the origin: the centre moves away from it.
        let (cx, cy) = (x + w / 2.0, y + h / 2.0);
        out.push(Instance { scale: s, ..Instance::at(id, 1, ox + (cx - ox) * s, oy + (cy - oy) * s, 1.0) });
    }
}

const SPIDER: &str = "<svg xmlns='http://www.w3.org/2000/svg' viewBox='-20 -20 40 40'>\
<g fill='none' stroke='rgb(225,205,255)' stroke-opacity='0.75' stroke-width='1.1' stroke-linecap='round' stroke-linejoin='round'>\
<g transform='{A}'><path d='M-3 -2 L-10 -9 L-13 -15'/><path d='M-4 3 L-12 2 L-17 6'/><path d='M3 0 L11 -3 L16 -1'/><path d='M3 6 L10 11 L12 17'/></g>\
<g transform='{B}'><path d='M-3 0 L-11 -3 L-16 -1'/><path d='M-3 6 L-10 11 L-12 17'/><path d='M3 -2 L10 -9 L13 -15'/><path d='M4 3 L12 2 L17 6'/></g></g>\
<g fill='#120a1a' stroke='rgb(255,160,60)' stroke-opacity='0.6' stroke-width='0.7'><ellipse cx='0' cy='5' rx='5.5' ry='7'/><circle cx='0' cy='-4.5' r='3.6'/></g>\
<g fill='#ff8a1c'><circle cx='-1.4' cy='-5.4' r='0.9'/><circle cx='1.4' cy='-5.4' r='0.9'/></g></svg>";

const BAT: &str = "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 50'><path fill='#06030a' d='M50 20 C47 12 41 10 36 13 C31 5 19 3 5 9 C12 13 14 19 12 25 C18 21 25 23 29 29 C33 25 39 25 43 31 C45 27 47 26 50 27 C53 26 55 27 57 31 C61 25 67 25 71 29 C75 23 82 21 88 25 C86 19 88 13 95 9 C81 3 69 5 64 13 C59 10 53 12 50 20 Z M46 14 L47 8 L49 13 Z M54 14 L53 8 L51 13 Z'/></svg>";

fn svg_sprite(svg: &str, w: u32, h: u32) -> Option<Pixmap> {
    let data = format!("data:image/svg+xml,{}", percent_encoding::utf8_percent_encode(svg, percent_encoding::NON_ALPHANUMERIC));
    Picture::load(&data)?.render(w, h)
}

pub fn build_spooky(e: &mut Engine) {
    let (vw, vh) = e.layout().view;
    let vmin = vw.min(vh) / 100.0;
    let legs = |bent_a: bool| {
        let bent = "rotate(7) scale(0.96)";
        SPIDER.replace("{A}", if bent_a { bent } else { "" }).replace("{B}", if bent_a { "" } else { bent })
    };
    let mut spider = |size: f32| -> [u32; 2] {
        let px = (size * vmin).round() as u32;
        [true, false].map(|a| {
            let sprite = svg_sprite(&legs(a), px, px).unwrap_or_else(|| Pixmap::new(1, 1).unwrap());
            e.push(sprite)
        })
    };
    let drop = spider(7.0);
    let walk = spider(6.0);

    let mut thread = Pixmap::new(2, vh.ceil() as u32 * 2).unwrap();
    thread.fill(Color { r: 225.0 / 255.0, g: 205.0 / 255.0, b: 1.0, a: 0.4 }.to_skia());
    let thread = e.push(thread);

    // Bat with a soft warm halo (drop-shadow 0.6vmin).
    let (bw, bh) = ((9.0 * vmin).round() as u32, (4.5 * vmin).round() as u32);
    let pad = (1.5 * vmin).ceil() as u32;
    let mut bat = Pixmap::new(bw + 2 * pad, bh + 2 * pad).unwrap();
    if let Some(body) = svg_sprite(BAT, bw, bh) {
        let cov = Coverage { w: body.width() as usize, h: body.height() as usize, data: body.data().chunks_exact(4).map(|p| p[3] as f32 / 255.0).collect() };
        let halo = cov.padded(pad as usize).blurred(0.3 * vmin);
        paint_coverage(&mut bat, &halo, 0, 0, Color { r: 1.0, g: 200.0 / 255.0, b: 120.0 / 255.0, a: 0.25 });
        bat.draw_pixmap(pad as i32, pad as i32, body.as_ref(), &PixmapPaint::default(), Transform::identity(), None);
    }
    let bat = e.push(bat);

    e.special_mut().spooky = Some(Spooky { drop, walk, thread, bat });
}

/// Halloween extras on top of everything: the dangling and the walking spider, the bat.
pub fn after_letters(e: &Engine, out: &mut Vec<Instance>, _t: f64, since: f64) {
    let Some(s) = e.special().spooky.as_ref() else { return };
    let l = e.layout();
    let (vw, vh) = l.view;
    let vmin = vw.min(vh) / 100.0;
    let stretch = e.options().stretch;
    let clock = e.theme().clock_size;
    let sprite_h = |id: u32| e.sprites()[id as usize].height as f32;

    // Legs: two poses crossfaded (CSS legs, alternate).
    let legs = |period: f64| -> f32 {
        let c = (since / period).rem_euclid(2.0) as f32;
        let p = if c < 1.0 { c } else { 2.0 - c };
        bezier(EASE_IN_OUT, p)
    };
    let local = |delay: f64, duration: f64| -> f32 { if since < delay { 0.0 } else { (((since - delay) / duration).rem_euclid(1.0)) as f32 } };

    // Dangling spider: drops into the web corner and back up (drop, 19 s from 3 s).
    {
        let x = if stretch { 9.0 * vmin } else { vw / 2.0 - 50.0 * vmin * clock + 9.0 * vmin };
        let drop = if stretch { 24.0 * vmin } else { vh / 2.0 - 50.0 * vmin * clock + 22.0 * vmin };
        let p = local(3.0, 19.0);
        let ease = [0.45, 0.0, 0.3, 1.0];
        let top = -12.0 * vmin;
        let ty = keyframes(&[(0.0, top), (0.22, drop), (0.27, drop - 3.0 * vmin), (0.32, drop), (0.58, drop), (0.8, top), (1.0, top)], p, ease);
        let rot = keyframes(&[(0.0, 0.0), (0.32, 0.0), (0.45, 12.0), (0.58, 0.0), (1.0, 0.0)], p, ease).to_radians();
        let size = 7.0 * vmin;
        let (cx, cy) = (x, ty + size / 2.0);
        // Thread hangs from the spider's centre straight up, swinging with it.
        let th = sprite_h(s.thread);
        let (sx, sy) = (cx + rot.sin() * th / 2.0, cy - rot.cos() * th / 2.0);
        out.push(Instance { rotation: rot, ..Instance::at(s.thread, 0, sx, sy, 1.0) });
        let k = legs(0.9);
        out.push(Instance { rotation: rot, ..Instance::at(s.drop[0], 0, cx, cy, 1.0 - k) });
        out.push(Instance { rotation: rot, ..Instance::at(s.drop[1], 0, cx, cy, k) });
    }

    // Walking spider: in from the right, across the face, out over the top (walk, 42 s from 8 s).
    {
        let p = local(8.0, 42.0);
        let (w, h) = (vw / 100.0, vh / 100.0);
        let kx = [(0.0, 110.0), (0.12, 68.0), (0.15, 68.0), (0.19, 68.0), (0.28, 52.0), (0.33, 52.0), (0.36, 52.0), (0.48, 50.0), (0.51, 50.0), (0.62, 60.0), (1.0, 60.0)];
        let ky = [(0.0, 80.0), (0.12, 80.0), (0.15, 80.0), (0.19, 80.0), (0.28, 58.0), (0.33, 58.0), (0.36, 58.0), (0.48, 30.0), (0.51, 30.0), (0.62, -10.0), (1.0, -10.0)];
        let kr = [(0.0, -90.0), (0.12, -90.0), (0.15, -40.0), (0.19, -40.0), (0.28, -40.0), (0.33, -75.0), (0.36, -5.0), (0.48, -5.0), (0.51, 20.0), (0.62, 20.0), (1.0, 20.0)];
        let half = 3.0 * vmin;
        let (x, y) = (keyframes(&kx, p, LINEAR) * w + half, keyframes(&ky, p, LINEAR) * h + half);
        let rot = keyframes(&kr, p, LINEAR).to_radians();
        let k = legs(0.16);
        out.push(Instance { rotation: rot, ..Instance::at(s.walk[0], 0, x, y, 1.0 - k) });
        out.push(Instance { rotation: rot, ..Instance::at(s.walk[1], 0, x, y, k) });
    }

    // Bat past the moon (bat-fly, 27 s from 5 s), flapping (bat-flap 180 ms alternate).
    {
        let p = local(5.0, 27.0);
        let (w, h) = (vw / 100.0, vh / 100.0);
        let kx = [(0.0, -15.0), (0.08, 25.0), (0.15, 60.0), (0.22, 84.0), (0.3, 115.0), (1.0, 115.0)];
        let ky = [(0.0, 26.0), (0.08, 14.0), (0.15, 20.0), (0.22, 9.0), (0.3, 4.0), (1.0, 4.0)];
        let ks = [(0.0, 0.8), (0.08, 0.9), (0.15, 1.0), (0.22, 1.1), (0.3, 1.2), (1.0, 1.2)];
        let (bw, bh) = (9.0 * vmin, 4.5 * vmin);
        let scale = keyframes(&ks, p, LINEAR);
        let flap = legs(0.18);
        let squash = 1.0 - 0.65 * flap;
        let x = keyframes(&kx, p, LINEAR) * w + bw / 2.0;
        let y = keyframes(&ky, p, LINEAR) * h + bh / 2.0 + 0.15 * bh * flap * scale;
        out.push(Instance { scale, scale_y: squash, ..Instance::at(s.bat, 0, x, y, 1.0) });
    }
}
