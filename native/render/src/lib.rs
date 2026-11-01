//! Sprite renderer for the word clock, shared by every native target.
//!
//! Everything expensive (backgrounds, letters with glow, dots, particles) is rasterised into [`Sprite`]s.
//! Each frame then only produces a fixed-length list of [`Instance`]s: which sprite, where, how opaque,
//! how big. A GPU compositor (Core Animation on the Mac) or a small software blitter (ESP32) draws that.
//! All animation timing lives here, so every platform moves the same way. Themes come from the web CSS
//! (`native/data/themes.json`), languages from the web locales (`native/data/locales.json`).

pub mod css;
pub mod paint;
pub mod raster;
pub mod special;
pub mod theme;

use std::collections::HashMap;

use qlock_core::{Frame, Locale};
use tiny_skia::Pixmap;

use css::{Shadow, Units};
use paint::{FloatCanvas, Picture};
use raster::{Coverage, Face, brighten, circle_coverage, fill_circle, fill_rect, paint_coverage, rounded_rect_mask_xy};
use theme::{Color, Font, FxKind, Motion, Theme};

/// Inter as a variable font, the web app's default face.
pub const INTER: &[u8] = include_bytes!("../../assets/fonts/InterVariable.ttf");
pub const AMATIC: &[u8] = include_bytes!("../../assets/fonts/AmaticSC-Regular.ttf");
pub const AMATIC_BOLD: &[u8] = include_bytes!("../../assets/fonts/AmaticSC-Bold.ttf");

pub fn font_bytes(font: Font, weight: f32) -> &'static [u8] {
    match font {
        Font::Inter => INTER,
        Font::Amatic if weight >= 600.0 => AMATIC_BOLD,
        Font::Amatic => AMATIC,
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Options {
    pub intro: bool,
    pub dots: bool,
    pub stretch: bool,
    /// Letter effect (key E in the web app).
    pub effect: bool,
    /// Focus pull when the time changes (key U).
    pub transition: bool,
    /// Background particles (key B).
    pub ambient: bool,
    /// Seconds frame around the face (key R).
    pub progress: bool,
    /// Every word lit, for comparing themes.
    pub all: bool,
}

/// Premultiplied RGBA pixels.
pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

/// Where a sprite goes this frame. Positions are the sprite's centre in pixels, y down.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Instance {
    pub sprite: u32,
    /// 0 = screen, 1 = clipped to the clock face.
    pub group: u32,
    pub x: f32,
    pub y: f32,
    pub opacity: f32,
    pub scale: f32,
    /// Extra vertical scale on top of `scale` (bat wings, confetti flips).
    pub scale_y: f32,
    /// Radians, clockwise.
    pub rotation: f32,
}

impl Instance {
    pub fn at(sprite: u32, group: u32, x: f32, y: f32, opacity: f32) -> Instance {
        Instance { sprite, group, x, y, opacity, scale: 1.0, scale_y: 1.0, rotation: 0.0 }
    }
}

#[derive(Clone)]
pub(crate) struct Layout {
    pub face: [f32; 4],
    pub radius: (f32, f32),
    pub pad: f32,
    pub cell_w: f32,
    pub cell_h: f32,
    pub font_px: f32,
    pub dot_size: f32,
    pub view: (f32, f32),
}

impl Layout {
    fn new(w: f32, h: f32, locale: &Locale, theme: &Theme, stretch: bool) -> Layout {
        let face = if stretch {
            [0.0, 0.0, w, h]
        } else {
            let s = w.min(h) * theme.clock_size;
            [(w - s) / 2.0, (h - s) / 2.0, s, s]
        };
        let pad = face[2].min(face[3]) * theme.pad;
        let cell_w = (face[2] - 2.0 * pad) / locale.cols as f32;
        let cell_h = (face[3] - 2.0 * pad) / locale.rows as f32;
        Layout {
            face,
            radius: if stretch { (0.0, 0.0) } else { (theme.face_radius * face[2], theme.face_radius * face[3]) },
            pad,
            cell_w,
            cell_h,
            font_px: (cell_h * theme.char_scale).min(cell_w * 0.9),
            dot_size: pad * theme.dot_size,
            view: (w, h),
        }
    }

    pub fn units(&self, em: f32) -> Units {
        Units { em, view_min: self.view.0.min(self.view.1), view_w: self.view.0, view_h: self.view.1, face_w: self.face[2], face_h: self.face[3] }
    }

    fn cell_centre(&self, cols: usize, cell: usize) -> (f32, f32) {
        let (row, col) = (cell / cols, cell % cols);
        (self.face[0] + self.pad + (col as f32 + 0.5) * self.cell_w, self.face[1] + self.pad + (row as f32 + 0.5) * self.cell_h)
    }

    fn dot_centre(&self, corner: usize) -> (f32, f32) {
        let [x, y, w, h] = self.face;
        let half = self.pad / 2.0;
        match corner {
            0 => (x + half, y + half),
            1 => (x + w - half, y + half),
            2 => (x + w - half, y + h - half),
            _ => (x + half, y + h - half),
        }
    }
}

/// CSS `cubic-bezier(x1, y1, x2, y2)` at progress `t`.
pub fn bezier(c: [f32; 4], t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    let curve = |a: f32, b: f32, s: f32| 3.0 * a * s * (1.0 - s).powi(2) + 3.0 * b * s * s * (1.0 - s) + s * s * s;
    let (mut lo, mut hi) = (0.0f32, 1.0f32);
    for _ in 0..24 {
        let mid = (lo + hi) / 2.0;
        if curve(c[0], c[2], mid) < t {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    curve(c[1], c[3], (lo + hi) / 2.0)
}

pub const EASE: [f32; 4] = [0.25, 0.1, 0.25, 1.0];
pub const EASE_IN_OUT: [f32; 4] = [0.42, 0.0, 0.58, 1.0];
pub const EASE_OUT: [f32; 4] = [0.0, 0.0, 0.58, 1.0];
pub const LINEAR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const FOCUS_EASE: [f32; 4] = [0.2, 0.7, 0.2, 1.0];

/// Evaluates CSS keyframes (offset, value) at `phase` 0–1, the timing function applied per segment.
pub fn keyframes(frames: &[(f32, f32)], phase: f32, ease: [f32; 4]) -> f32 {
    let p = phase.clamp(0.0, 1.0);
    for pair in frames.windows(2) {
        let ((a, va), (b, vb)) = (pair[0], pair[1]);
        if p <= b {
            let k = if b > a { (p - a) / (b - a) } else { 1.0 };
            return va + (vb - va) * bezier(ease, k);
        }
    }
    frames.last().map_or(0.0, |f| f.1)
}

#[derive(Clone, Copy)]
struct CellState {
    on: bool,
    /// When the current fade starts (seconds) and the level it starts from.
    since: f64,
    from: f32,
    order: usize,
}

struct LetterPixmaps {
    off: Pixmap,
    on: Pixmap,
    focus: Pixmap,
    tints: Vec<Pixmap>,
}

#[derive(Clone)]
struct LetterSprites {
    off: u32,
    on: u32,
    focus: u32,
    /// Shimmer: the letter filled with colours along the sweep, evenly spaced over [`SHIMMER_SPAN`].
    tints: Vec<u32>,
}

/// One clock on one screen.
pub struct Engine {
    locale: Locale,
    theme: Theme,
    options: Options,
    layout: Layout,
    sprites: Vec<Sprite>,
    /// The page around the face (not in full-screen mode).
    page: Option<u32>,
    /// The face with its shadow; `face_origin` is the sprite's top left on screen.
    background: u32,
    face_origin: (f32, f32),
    /// Per distinct letter, and per cell the index into `letters`.
    letters: Vec<LetterSprites>,
    cell_letter: Vec<usize>,
    /// Chaos Night: on-sprites per (letter, hue bucket), made when first needed.
    hued: HashMap<(usize, u32), u32>,
    on_face: Face,
    dots_off: u32,
    dots_on: [u32; 4],
    progress: Vec<(u32, f32, f32)>,
    particles: Vec<u32>,
    caption: Option<u32>,
    /// Today's (month, day) for date words, set by the host.
    date: Option<(u32, u32)>,
    special: special::Special,
    state: Vec<CellState>,
    dots_lit: u8,
    dots_since: f64,
    current: Option<Frame>,
    started: Option<f64>,
    instances: Vec<Instance>,
}

/// The web app waits this long before the first words light up.
const START_DELAY: f64 = 0.6;
/// Focus pull of the transition effect (CSS `tx-focus`).
const FOCUS: f64 = 1.6;
const PARTICLES: usize = 14;
const SHIMMER_TINTS: usize = 9;
/// Pieces of the seconds frame: ten per second.
const PROGRESS_PIECES: usize = 600;

impl Engine {
    /// `width`/`height` in pixels. `caption` is the date line, already formatted for the locale.
    pub fn new(locale: Locale, theme: Theme, options: Options, width: u32, height: u32, caption: Option<&str>) -> Engine {
        let layout = Layout::new(width as f32, height as f32, &locale, &theme, options.stretch);
        let on_face = Face::new(font_bytes(theme.font, theme.weight_on), theme.weight_on).expect("font");
        let mut engine = Engine {
            state: vec![CellState { on: false, since: f64::MIN, from: 0.0, order: 0 }; locale.cell_count()],
            special: special::Special::default(),
            locale,
            theme,
            options,
            layout,
            sprites: Vec::new(),
            page: None,
            background: 0,
            face_origin: (0.0, 0.0),
            letters: Vec::new(),
            cell_letter: Vec::new(),
            hued: HashMap::new(),
            on_face,
            dots_off: 0,
            dots_on: [0; 4],
            progress: Vec::new(),
            particles: Vec::new(),
            caption: None,
            date: None,
            dots_lit: 0,
            dots_since: f64::MIN,
            current: None,
            started: None,
            instances: Vec::new(),
        };
        engine.build(width, height, caption);
        engine
    }

    pub fn sprites(&self) -> &[Sprite] {
        &self.sprites
    }

    /// Frees the pixels of all sprites so far once the host has copied them (sizes stay).
    pub fn release_pixels(&mut self) {
        for s in &mut self.sprites {
            s.rgba = Vec::new();
        }
    }

    /// The clock face in pixels, for clipping group 1.
    pub fn face(&self) -> [f32; 4] {
        self.layout.face
    }

    pub fn face_radius(&self) -> f32 {
        self.layout.radius.0
    }

    pub(crate) fn push(&mut self, pixmap: Pixmap) -> u32 {
        self.sprites.push(Sprite { width: pixmap.width(), height: pixmap.height(), rgba: pixmap.take() });
        (self.sprites.len() - 1) as u32
    }

    pub(crate) fn units(&self) -> Units {
        self.layout.units(self.layout.font_px)
    }

    fn build(&mut self, width: u32, height: u32, caption: Option<&str>) {
        let theme = self.theme.clone();
        let u = self.units();
        let l = &self.layout.clone();

        // The page and the clock face are separate sprites, so the face can spring in on its own (CSS
        // clock-in). In full-screen mode the face covers the page, which is then left out.
        if !self.options.stretch {
            let mut page = FloatCanvas::new(width, height);
            page.paint_css(&theme.page_bg, [0.0, 0.0, width as f32, height as f32], None, &u);
            let page = page.into_pixmap();
            self.page = Some(self.push(page));
        }
        // Face with its shadow, decoration and extras, on a canvas just big enough for the shadow.
        let reach = if self.options.stretch {
            0.0
        } else {
            let base = l.face[2].min(l.face[3]);
            theme.face_shadow.iter().map(|s| s.x.px(&u, base).abs().max(s.y.px(&u, base).abs()) + s.blur.px(&u, base) + s.spread.px(&u, base).max(0.0)).fold(0.0, f32::max)
        };
        let m = reach.ceil();
        let mut fl = l.clone();
        fl.face = [m, m, l.face[2], l.face[3]];
        let (cw, ch) = ((l.face[2] + 2.0 * m).ceil() as u32, (l.face[3] + 2.0 * m).ceil() as u32);
        let mut bg = FloatCanvas::new(cw, ch);
        if !self.options.stretch {
            for shadow in theme.face_shadow.iter().rev() {
                special::face_shadow(&mut bg, &fl, shadow, &u);
            }
        }
        let mask = rounded_rect_mask_xy(cw, ch, fl.face, fl.radius.0, fl.radius.1);
        bg.paint_css(&theme.face_bg, fl.face, mask.as_ref(), &u);
        let deco_moves = theme.extras.groove && self.options.ambient;
        if theme.face_deco != "none" && !self.options.ambient {
            bg.paint_css(&theme.face_deco, fl.face, mask.as_ref(), &u);
        }
        special::static_decoration(&mut bg, &theme, &fl, mask.as_ref(), &u);
        let bg = bg.into_pixmap();
        self.background = self.push(bg);
        self.face_origin = (l.face[0] - m, l.face[1] - m);
        if deco_moves {
            self.special.groove = special::groove_sprite(self, width, height);
        }
        if theme.extras.waves && self.options.ambient {
            self.special.waves = special::wave_sprites(self);
        }

        // Letters: one set of sprites per distinct letter, drawn in parallel.
        let mut chars: Vec<String> = Vec::new();
        for c in 0..self.locale.cell_count() {
            let ch = self.display_char(c);
            let i = chars.iter().position(|x| *x == ch).unwrap_or_else(|| {
                chars.push(ch);
                chars.len() - 1
            });
            self.cell_letter.push(i);
        }
        let drawn: Vec<LetterPixmaps> = {
            let this = &*self;
            std::thread::scope(|scope| {
                let jobs: Vec<_> = chars.iter().map(|ch| scope.spawn(move || this.letter_pixmaps(ch))).collect();
                jobs.into_iter().map(|j| j.join().expect("letter")).collect()
            })
        };
        for d in drawn {
            let set = LetterSprites { off: self.push(d.off), on: self.push(d.on), focus: self.push(d.focus), tints: d.tints.into_iter().map(|p| self.push(p)).collect() };
            self.letters.push(set);
        }

        self.build_dots();
        // Seconds only make sense when the dots give the minute.
        if self.options.progress && self.options.dots {
            self.build_progress();
        }
        if self.options.ambient {
            self.build_particles();
        }
        if self.theme.extras.rays {
            self.special.rays = special::rays_sprite(self);
        }
        if self.theme.extras.spooky {
            special::build_spooky(self);
        }

        // Date line, spaced out like the CSS caption.
        if let Some(text) = caption {
            let px = self.layout.pad * 0.14;
            let (cov, _, _, _) = self.on_face.text(&text.to_uppercase(), px, px * 0.32);
            if cov.w > 0 {
                let mut p = Pixmap::new(cov.w as u32 + 4, cov.h as u32 + 4).unwrap();
                paint_coverage(&mut p, &cov, 2, 2, theme.caption_color);
                self.caption = Some(self.push(p));
            }
        }
    }

    fn display_char(&self, cell: usize) -> String {
        let ch = &self.locale.chars[cell];
        if self.theme.lowercase { ch.to_lowercase() } else { ch.clone() }
    }

    /// Sprite canvas size for one letter: the cell plus room for glow and movement.
    fn letter_canvas(&self) -> (u32, u32) {
        let em = self.layout.font_px;
        let u = self.units();
        let reach = self
            .theme
            .char_glow
            .iter()
            .map(|s| s.blur.px(&u, em) * 1.5 + s.x.px(&u, em).abs().max(s.y.px(&u, em).abs()))
            .fold(em * 0.2, f32::max);
        ((self.layout.cell_w + 2.0 * reach).ceil() as u32 + 2, (self.layout.cell_h + 2.0 * reach).ceil() as u32 + 2)
    }

    /// Draws one letter into a new sprite: background block, shadows/glow, outline, fill, underline.
    fn draw_letter(&self, face: &Face, ch: &str, fill: Color, lit: bool, glow: bool) -> Pixmap {
        let t = &self.theme;
        let u = self.units();
        let em = self.layout.font_px;
        let (sw, sh) = self.letter_canvas();
        let mut p = Pixmap::new(sw, sh).unwrap();
        let (cx, cy) = (sw as f32 / 2.0, sh as f32 / 2.0);
        let (cell_w, cell_h) = (self.layout.cell_w, self.layout.cell_h);
        if lit {
            if let Some(bg) = t.char_on_bg {
                // Opaque, so neighbouring blocks don't show seams where they overlap.
                let face_dark = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
                let mix = |a: f32, b: f32| a * bg.a + b * (1.0 - bg.a);
                let solid = Color { r: mix(bg.r, face_dark.r), g: mix(bg.g, face_dark.g), b: mix(bg.b, face_dark.b), a: 1.0 };
                fill_rect(&mut p, [cx - cell_w / 2.0 - 0.5, cy - cell_h / 2.0 - 0.5, cell_w + 1.0, cell_h + 1.0], solid);
            }
        }
        let (mut cov, ox, oy, adv) = face.text(ch, em, 0.0);
        let (asc, desc) = face.metrics(em);
        let pen_x = cx - adv / 2.0;
        let baseline = cy + (asc + desc) / 2.0;
        let mut gx = (pen_x + ox).round() as i32;
        let mut gy = (baseline + oy).round() as i32;
        if lit && t.underline {
            // Text decoration: a bar under the glyph, part of the same shape so glow and colour apply.
            let (off, thick) = face.underline(em);
            let pad = ((thick + off) * 2.0).ceil() as usize;
            let mut big = cov.padded(pad);
            gx -= pad as i32;
            gy -= pad as i32;
            let line_y = (baseline + off) as i32 - gy;
            let (x0, x1) = ((pen_x as i32 - gx).max(0) as usize, ((pen_x + adv) as i32 - gx).max(0) as usize);
            for y in line_y.max(0) as usize..((line_y as f32 + thick).ceil().max(0.0) as usize).min(big.h) {
                for x in x0..x1.min(big.w) {
                    big.data[y * big.w + x] = 1.0;
                }
            }
            cov = big;
        }
        let stroke = t.extras.letter_stroke.filter(|_| lit);
        if glow {
            for s in t.char_glow.iter().rev() {
                let sigma = s.blur.px(&u, em) / 2.0;
                let pad = (sigma * 3.0).ceil() as usize + 2;
                let base = cov.padded(pad);
                let shape = match stroke {
                    Some((w, _)) => base.dilated(w * em / 2.0),
                    None => base,
                };
                let blurred = shape.blurred(sigma);
                let (dx, dy) = (s.x.px(&u, em).round() as i32, s.y.px(&u, em).round() as i32);
                paint_coverage(&mut p, &blurred, gx - pad as i32 + dx, gy - pad as i32 + dy, Theme::shadow_color(s, fill));
            }
        }
        if let Some((w, ink)) = stroke {
            // -webkit-text-stroke with paint-order stroke fill: half the stroke shows outside the glyph.
            let r = (w * em / 2.0).max(1.0);
            let pad = r.ceil() as usize + 1;
            let outline = cov.padded(pad).dilated(r);
            paint_coverage(&mut p, &outline, gx - pad as i32, gy - pad as i32, ink);
        }
        paint_coverage(&mut p, &cov, gx, gy, fill);
        p
    }

    fn letter_pixmaps(&self, ch: &str) -> LetterPixmaps {
        let t = &self.theme;
        let off_face = Face::new(font_bytes(t.font, t.weight_off), t.weight_off).expect("font");
        let on_face = Face::new(font_bytes(t.font, t.weight_on), t.weight_on).expect("font");
        let off = self.draw_letter(&off_face, ch, t.char_off, false, false);
        let on = self.draw_letter(&on_face, ch, t.char_on, true, true);
        // Transition: the letter starts blurred and bright and pulls into focus (CSS tx-focus).
        let mut focus_src = on.clone();
        brighten(&mut focus_src, 1.6);
        let focus = special::blur_pixmap(&focus_src, 0.12 * self.layout.font_px / 2.0);
        let tints = if self.options.effect && t.fx.kind == FxKind::Shimmer && !t.fx.gradient.is_empty() {
            // The shimmer replaces colour and glow with the sweeping gradient.
            (0..SHIMMER_TINTS)
                .map(|k| {
                    let p = SHIMMER_SPAN.0 + (SHIMMER_SPAN.1 - SHIMMER_SPAN.0) * k as f32 / (SHIMMER_TINTS - 1) as f32;
                    self.draw_letter(&on_face, ch, gradient_color(&t.fx.gradient, p), true, false)
                })
                .collect()
        } else {
            Vec::new()
        };
        LetterPixmaps { off, on, focus, tints }
    }

    /// Chaos Night: lit letter coloured by its place in the sentence.
    fn hued_sprite(&mut self, cell: usize, order: usize) -> u32 {
        let Some((start, step)) = self.theme.extras.letter_hues else { return self.letters[self.cell_letter[cell]].on };
        let bucket = ((start + order as f32 * step).rem_euclid(360.0) / 10.0).round() as u32 % 36;
        let key = (self.cell_letter[cell], bucket);
        if let Some(&id) = self.hued.get(&key) {
            return id;
        }
        let color = css::hsl(bucket as f32 * 10.0, 0.95, 0.62, 1.0);
        let ch = self.display_char(cell);
        let face = Face::new(font_bytes(self.theme.font, self.theme.weight_on), self.theme.weight_on).unwrap();
        let sprite = self.draw_letter(&face, &ch, color, true, true);
        let id = self.push(sprite);
        self.hued.insert(key, id);
        id
    }

    fn build_dots(&mut self) {
        let t = self.theme.clone();
        let u = self.units();
        let dot = self.layout.dot_size;
        let view_min = self.layout.view.0.min(self.layout.view.1);
        let reach = t.dot_glow.iter().map(|g| g.blur.px(&u, dot) * 1.5).fold(0.0, f32::max)
            + t.extras.dot_ring.map_or(0.0, |(w, _)| w * view_min);
        let size = (dot + 2.0 * reach).ceil() as u32 + 4;
        let c = size as f32 / 2.0;
        let image = t.extras.dot_image.and_then(|name| Picture::load(&format!("./{name}")));
        let make = |color: Color, glow: &[Shadow], lit: bool| {
            let mut p = Pixmap::new(size, size).unwrap();
            if let Some(img) = &image {
                // Formrausch: the cross bitmap, visible only when lit.
                if lit {
                    if let Some(tile) = img.render(dot.round() as u32, dot.round() as u32) {
                        let at = (c - dot / 2.0).round() as i32;
                        p.draw_pixmap(at, at, tile.as_ref(), &tiny_skia::PixmapPaint::default(), tiny_skia::Transform::identity(), None);
                    }
                }
                return p;
            }
            for s in glow.iter().rev() {
                let sigma = s.blur.px(&u, dot) / 2.0;
                let cov = circle_coverage(size as usize, c, c, dot / 2.0 + s.spread.px(&u, dot)).blurred(sigma);
                paint_coverage(&mut p, &cov, 0, 0, Theme::shadow_color(s, color));
            }
            if let Some((w, ink)) = t.extras.dot_ring {
                fill_circle(&mut p, c, c, dot / 2.0 + w * view_min, ink);
            }
            if let Some(border) = t.extras.dot_border {
                // 1px border inside the dot's box, then the fill inside it.
                fill_circle(&mut p, c, c, dot / 2.0, border);
                fill_circle_clear(&mut p, c, c, dot / 2.0 - 1.0);
                fill_circle(&mut p, c, c, dot / 2.0 - 1.0, color);
                return p;
            }
            fill_circle(&mut p, c, c, dot / 2.0, color);
            p
        };
        let off = make(t.dot_off, &[], false);
        self.dots_off = self.push(off);
        for i in 0..4 {
            let on = make(t.dot_colors[i], &t.dot_glow, true);
            self.dots_on[i] = self.push(on);
        }
    }

    /// Seconds frame: the line through the corner dots, cut into [`PROGRESS_PIECES`] pieces (a few per second,
    /// so the line can grow smoothly instead of jumping).
    ///
    /// The whole line is drawn once and every pixel is then given to exactly one piece (by its position
    /// along the line), so the pieces add up to the unbroken line: no seams or darker dots where they meet.
    fn build_progress(&mut self) {
        let [fx, fy, fw, fh] = self.layout.face;
        let half = self.layout.pad / 2.0;
        // Through the dot centres, like the web's SVG (inset by half the padding).
        let (x0, y0, w, h) = (fx + half, fy + half, fw - 2.0 * half, fh - 2.0 * half);
        let perimeter = 2.0 * (w + h);
        // 1 CSS px; the host renders at the backing scale, so two device pixels.
        let width = self.theme.progress_width * 2.0;
        let m = width + 2.0;
        let (ox, oy) = ((x0 - m).floor(), (y0 - m).floor());
        let (bw, bh) = ((w + 2.0 * m).ceil() as u32 + 2, (h + 2.0 * m).ceil() as u32 + 2);
        let Some(mut line) = Pixmap::new(bw, bh) else { return };
        if let Some(rect) = tiny_skia::Rect::from_xywh(x0 - ox, y0 - oy, w, h) {
            let path = tiny_skia::PathBuilder::from_rect(rect);
            let mut paint = tiny_skia::Paint::default();
            paint.set_color(self.theme.progress_color.to_skia());
            paint.anti_alias = true;
            let stroke = tiny_skia::Stroke { width, line_join: tiny_skia::LineJoin::Round, ..Default::default() };
            line.stroke_path(&path, &paint, &stroke, tiny_skia::Transform::identity(), None);
        }

        // Position along the line (from the top centre, clockwise) of a pixel, by its nearest edge.
        let along = |px: f32, py: f32| -> f32 {
            let (top, right, bottom, left) = ((py - y0).abs(), (px - x0 - w).abs(), (py - y0 - h).abs(), (px - x0).abs());
            let near = top.min(right).min(bottom).min(left);
            let d = if near == top {
                px - (x0 + w / 2.0)
            } else if near == right {
                w / 2.0 + (py - y0)
            } else if near == bottom {
                w / 2.0 + h + (x0 + w - px)
            } else {
                w / 2.0 + h + w + (y0 + h - py)
            };
            d.rem_euclid(perimeter)
        };
        let fade = w * 0.25;
        let start_x = x0 + w / 2.0;
        let data = line.data();
        let mut pieces: Vec<Vec<(u32, u32, [u8; 4])>> = vec![Vec::new(); PROGRESS_PIECES];
        for py in 0..bh {
            for px in 0..bw {
                let i = ((py * bw + px) * 4) as usize;
                if data[i + 3] == 0 {
                    continue;
                }
                let (ax, ay) = (ox + px as f32 + 0.5, oy + py as f32 + 0.5);
                let d = along(ax, ay);
                let piece = ((d / perimeter * PROGRESS_PIECES as f32) as usize).min(PROGRESS_PIECES - 1);
                let mut c = [data[i], data[i + 1], data[i + 2], data[i + 3]];
                // The start fades in over the first quarter of the top edge (the web's SVG mask).
                if (ay - y0).abs() < m && ax >= start_x - 1.0 && ax < start_x + fade {
                    let k = ((ax - start_x) / fade).clamp(0.0, 1.0);
                    c = c.map(|v| (v as f32 * k) as u8);
                }
                pieces[piece].push((px, py, c));
            }
        }
        for piece in pieces {
            if piece.is_empty() {
                let id = self.push(Pixmap::new(1, 1).unwrap());
                self.progress.push((id, 0.0, 0.0));
                continue;
            }
            let (minx, miny) = piece.iter().fold((u32::MAX, u32::MAX), |a, p| (a.0.min(p.0), a.1.min(p.1)));
            let (maxx, maxy) = piece.iter().fold((0, 0), |a, p| (a.0.max(p.0), a.1.max(p.1)));
            let (pw, ph) = (maxx - minx + 1, maxy - miny + 1);
            let mut p = Pixmap::new(pw, ph).unwrap();
            let out = p.data_mut();
            for (x, y, c) in piece {
                let i = (((y - miny) * pw + (x - minx)) * 4) as usize;
                out[i..i + 4].copy_from_slice(&c);
            }
            let id = self.push(p);
            // Integer origin, so the piece lands on the same pixels it was cut from.
            self.progress.push((id, ox + minx as f32 + pw as f32 / 2.0, oy + miny as f32 + ph as f32 / 2.0));
        }
    }

    fn build_particles(&mut self) {
        let Some(amb) = self.theme.ambient.clone() else { return };
        let face_min = self.layout.face[2].min(self.layout.face[3]);
        // Drawn for the biggest particle (size factor 1.3), scaled down per instance.
        let w = amb.size * face_min * 1.3;
        let h = w / amb.ratio;
        let glow = self.theme.extras.particle_glow * face_min * 1.3;
        let size_w = (w + 4.0 * glow).ceil() as u32 + 2;
        let size_h = (h + 4.0 * glow).ceil() as u32 + 2;
        let shape = amb.shape.as_ref().and_then(|u| Picture::load(u));
        for color in amb.colors {
            let mut p = Pixmap::new(size_w, size_h).unwrap();
            let (cx, cy) = (size_w as f32 / 2.0, size_h as f32 / 2.0);
            if glow > 0.0 {
                let side = size_w.max(size_h) as usize;
                let cov = circle_coverage(side, side as f32 / 2.0, side as f32 / 2.0, w / 2.0).blurred(glow / 2.0);
                paint_coverage(&mut p, &cov, (cx - side as f32 / 2.0) as i32, (cy - side as f32 / 2.0) as i32, color);
            }
            if let Some(shape) = &shape {
                // CSS mask-image: the SVG's alpha cuts the colour.
                if let Some(mask) = shape.render(w.round() as u32, h.round() as u32) {
                    let cov = Coverage { w: mask.width() as usize, h: mask.height() as usize, data: mask.data().chunks_exact(4).map(|px| px[3] as f32 / 255.0).collect() };
                    paint_coverage(&mut p, &cov, (cx - w / 2.0).round() as i32, (cy - h / 2.0).round() as i32, color);
                }
            } else if self.theme.extras.bubbles {
                special::bubble(&mut p, cx, cy, w / 2.0, color, face_min);
            } else if amb.round {
                fill_circle(&mut p, cx, cy, w / 2.0, color);
            } else {
                fill_rect(&mut p, [cx - w / 2.0, cy - h / 2.0, w, h], color);
            }
            let id = self.push(p);
            self.particles.push(id);
        }
    }

    fn level(&self, s: &CellState, t: f64) -> f32 {
        let fade = self.theme.fade as f64;
        let p = ((t - s.since) / fade).clamp(0.0, 1.0) as f32;
        let target = if s.on { 1.0 } else { 0.0 };
        s.from + (target - s.from) * bezier(EASE, p)
    }

    fn set_frame(&mut self, frame: Frame, t: f64) {
        let stagger = self.theme.stagger as f64;
        for c in 0..self.state.len() {
            let order = frame.cells.iter().position(|&x| x as usize == c);
            let level = self.level(&self.state[c], t);
            let s = &mut self.state[c];
            match (s.on, order) {
                (false, Some(o)) => *s = CellState { on: true, since: t + o as f64 * stagger, from: level, order: o },
                (true, None) => *s = CellState { on: false, since: t, from: level, order: s.order },
                (true, Some(o)) => s.order = o,
                (false, None) => {}
            }
        }
        if frame.dots != self.dots_lit {
            self.dots_lit = frame.dots;
            self.dots_since = t;
        }
        self.current = Some(frame);
    }

    /// Today's date for birthday words and the like.
    pub fn set_date(&mut self, month: u32, day: u32) {
        self.date = Some((month, day));
    }

    fn all_frame(&self) -> Frame {
        let mut cells: Vec<u16> = Vec::new();
        for word in &self.locale.words {
            for &c in word {
                if !cells.contains(&c) {
                    cells.push(c);
                }
            }
        }
        Frame { cells, dots: 4 }
    }

    /// Instances for local time `hour:minute:second` at animation time `t` (seconds, monotonic).
    /// The list has the same length every frame; sprite ids may change.
    pub fn update(&mut self, hour: u32, minute: u32, second: f32, t: f64) -> &[Instance] {
        let started = *self.started.get_or_insert(t);
        if t - started >= START_DELAY {
            let frame = if self.options.all { self.all_frame() } else { self.locale.frame_on(hour, minute, self.options.intro, self.date) };
            if self.current.as_ref() != Some(&frame) {
                self.set_frame(frame, t);
            }
        }
        let since_start = t - started;

        let mut out = std::mem::take(&mut self.instances);
        out.clear();
        let (vw, vh) = self.layout.view;
        // Always an entry for the page, so the list keeps its length; invisible in full-screen mode.
        out.push(Instance::at(self.page.unwrap_or(self.background), 0, vw / 2.0, vh / 2.0, if self.page.is_some() { 1.0 } else { 0.0 }));
        let clock_from = out.len();
        let bg = &self.sprites[self.background as usize];
        out.push(Instance::at(self.background, 0, self.face_origin.0 + bg.width as f32 / 2.0, self.face_origin.1 + bg.height as f32 / 2.0, 1.0));

        special::before_particles(self, &mut out, t);
        self.push_particles(&mut out, t);
        self.push_progress(&mut out, second);

        if let Some(caption) = self.caption {
            let l = &self.layout;
            let opacity = self.theme.caption_opacity * bezier(EASE, (since_start / 0.9) as f32);
            out.push(Instance::at(caption, 0, l.face[0] + l.face[2] / 2.0, l.face[1] + l.pad * 0.7, opacity));
        }

        self.push_dots(&mut out, t);
        self.push_letters(&mut out, t);
        self.clock_in(&mut out[clock_from..], since_start);
        special::after_letters(self, &mut out, t, since_start);

        self.instances = out;
        &self.instances
    }

    /// The clock appearing at the start (CSS `clock-in`: fade in from 94 %; TickTack's `ticktack-in`:
    /// springs from a squashed quarter size). Scales every clock instance about the face centre.
    fn clock_in(&self, list: &mut [Instance], since: f64) {
        let [fx, fy, fw, fh] = self.layout.face;
        let (cx, cy) = (fx + fw / 2.0, fy + fh / 2.0);
        let (sx, sy, opacity) = if self.theme.extras.spring_in {
            let k = bezier([0.34, 1.8, 0.5, 1.0], (since / 1.6) as f32);
            (0.25 + 0.75 * k, 0.5 + 0.5 * k, 1.0)
        } else {
            let k = bezier([0.16, 1.0, 0.3, 1.0], (since / 1.4) as f32);
            (0.94 + 0.06 * k, 0.94 + 0.06 * k, k)
        };
        if (sx - 1.0).abs() < 1e-4 && (sy - 1.0).abs() < 1e-4 && opacity >= 1.0 {
            return;
        }
        for inst in list {
            inst.x = cx + (inst.x - cx) * sx;
            inst.y = cy + (inst.y - cy) * sy;
            inst.scale *= sx;
            inst.scale_y *= sy / sx;
            inst.opacity *= opacity;
        }
    }

    fn push_particles(&self, out: &mut Vec<Instance>, t: f64) {
        let Some(amb) = self.theme.ambient.as_ref().filter(|_| self.options.ambient) else { return };
        let [fx, fy, fw, fh] = self.layout.face;
        let face_min = fw.min(fh);
        for i in 0..PARTICLES {
            let x = ((i * 37 + 11) % 100) as f32 / 100.0;
            let delay = -((i as f32 * 1.73) % PARTICLES as f32) as f64;
            let speed = 0.75 + ((i * 7) % 5) as f32 * 0.12;
            let size = 0.7 + ((i * 3) % 4) as f32 * 0.2;
            let rot0 = ((i * 53) % 360) as f32;
            let wander = ((i * 5) % 7) as f32 - 3.0;
            let duration = (amb.duration * speed) as f64;
            // f64 on purpose: hosts pass uptime-based seconds (~10^5–10^6), where f32 only resolves
            // ~1/32 s and the particles would move in steps.
            let p = (((t - delay) / duration).rem_euclid(1.0)) as f32;
            let (cw, ch) = (fw / 100.0, fh / 100.0);
            let lin = |frames: &[(f32, f32)]| keyframes(frames, p, LINEAR);
            let (dx, dy, scale, rot, opacity, flip) = match amb.motion {
                Motion::Rise => (
                    lin(&[(0.0, 0.0), (0.5, wander * 1.5 * cw), (1.0, -wander * cw)]),
                    lin(&[(0.0, 0.0), (0.5, -55.0 * ch), (1.0, -115.0 * ch)]),
                    lin(&[(0.0, 0.6), (0.5, 1.0), (1.0, 0.8)]),
                    0.0,
                    lin(&[(0.0, 0.0), (0.15, 1.0), (0.85, 1.0), (1.0, 0.0)]) * amb.opacity,
                    1.0,
                ),
                Motion::Fall => (
                    lin(&[(0.0, 0.0), (0.25, wander * 2.0 * cw), (0.5, -wander * 1.5 * cw), (0.75, wander * 2.0 * cw), (1.0, 0.0)]),
                    lin(&[(0.0, 0.0), (0.25, 30.0 * ch), (0.5, 60.0 * ch), (0.75, 90.0 * ch), (1.0, 118.0 * ch)]),
                    1.0,
                    rot0 + lin(&[(0.0, 0.0), (0.25, 80.0), (0.5, 170.0), (0.75, 260.0), (1.0, 360.0)]),
                    amb.opacity,
                    1.0,
                ),
                Motion::Sparkle => (
                    lin(&[(0.0, 0.0), (0.3, wander * cw), (0.6, -wander * cw), (0.8, 0.0), (1.0, wander * cw)]),
                    lin(&[(0.0, 0.0), (0.3, -30.0 * ch), (0.6, -60.0 * ch), (0.8, -80.0 * ch), (1.0, -115.0 * ch)]),
                    lin(&[(0.0, 0.0), (0.3, 1.0), (0.6, 0.3), (0.8, 1.0), (1.0, 0.0)]),
                    lin(&[(0.0, 0.0), (0.3, 90.0), (0.6, 180.0), (0.8, 270.0), (1.0, 360.0)]),
                    amb.opacity,
                    1.0,
                ),
                Motion::Rain => (0.0, 140.0 * ch * p, 1.0, 0.0, amb.opacity, 1.0),
                Motion::Confetti => (
                    lin(&[(0.0, 0.0), (1.0, wander * 3.0 * cw)]),
                    lin(&[(0.0, 0.0), (1.0, 118.0 * ch)]),
                    1.0,
                    rot0 + 540.0 * p,
                    amb.opacity,
                    // rotateX(720deg): the paper flips, seen as a vertical squash.
                    (p * 720.0).to_radians().cos(),
                ),
            };
            let w = amb.size * face_min * size;
            let h = w / amb.ratio;
            let sprite = self.particles[i % self.particles.len()];
            out.push(Instance {
                sprite,
                group: 1,
                x: fx + x * fw + w / 2.0 + dx,
                y: fy + amb.top * fh + h / 2.0 + dy,
                opacity,
                scale: scale * size / 1.3,
                scale_y: flip,
                rotation: rot.to_radians(),
            });
        }
    }

    fn push_progress(&self, out: &mut Vec<Instance>, second: f32) {
        // Like the web's stroke-dashoffset with a 400 ms ease-out transition: at second s the line grows
        // from s−1 to s sixtieths; at :00 it empties at once.
        let whole = second.floor() as usize % 60;
        let grown = if whole == 0 { 0.0 } else { (whole - 1) as f32 + bezier(EASE_OUT, (second - second.floor()) / 0.4) };
        let per = (PROGRESS_PIECES / 60) as f32;
        for (i, &(id, x, y)) in self.progress.iter().enumerate() {
            // Each small piece fades in as the front passes it, so the end of the line moves smoothly.
            let opacity = (grown * per - i as f32).clamp(0.0, 1.0);
            out.push(Instance::at(id, 0, x, y, opacity));
        }
    }

    fn push_dots(&self, out: &mut Vec<Instance>, t: f64) {
        let fade_p = ((t - self.dots_since) / self.theme.fade as f64) as f32;
        for corner in 0..4 {
            let (x, y) = self.layout.dot_centre(corner);
            let visible = if self.options.dots && !self.theme.extras.no_dots { 1.0 } else { 0.0 };
            let lit = (corner as u8) < self.dots_lit;
            let newest = corner as u8 + 1 == self.dots_lit;
            let on = if lit { if newest { bezier(EASE, fade_p) } else { 1.0 } } else { 0.0 };
            let pop = if self.options.transition && lit && newest {
                1.0 + 0.7 * (1.0 - bezier(FOCUS_EASE, ((t - self.dots_since) / 0.9) as f32))
            } else {
                1.0
            };
            let candle = special::candle(self, corner, t);
            out.push(Instance::at(self.dots_off, 0, x, y, visible * (1.0 - on)));
            out.push(Instance { scale: pop, ..Instance::at(self.dots_on[corner], 0, x, y, visible * on * candle) });
        }
    }

    fn push_letters(&mut self, out: &mut Vec<Instance>, t: f64) {
        let cols = self.locale.cols;
        let em = self.layout.font_px;
        let fx = self.theme.fx.clone();
        for c in 0..self.state.len() {
            let s = self.state[c];
            let level = self.level(&s, t);
            let (x, y) = self.layout.cell_centre(cols, c);
            let set = self.letters[self.cell_letter[c]].clone();
            let on_sprite = if s.on && self.theme.extras.letter_hues.is_some() { self.hued_sprite(c, s.order) } else { set.on };

            // Idle effect, running since the letter lit up, shifted by its place in the sentence.
            let (mut dy, mut rot, mut scale, mut fx_opacity) = (0.0f32, 0.0f32, 1.0f32, 1.0f32);
            let mut shimmer: Option<(u32, u32, f32)> = None;
            if self.options.effect && s.on {
                let phase = (((t - s.since + s.order as f64 * fx.step as f64) / fx.duration as f64).rem_euclid(1.0)) as f32;
                let k = |frames: &[(f32, f32)]| keyframes(frames, phase, fx.ease);
                match fx.kind {
                    FxKind::Breathe => fx_opacity = k(&[(0.0, 1.0), (0.5, 0.6), (1.0, 1.0)]),
                    FxKind::Wave => dy = k(&[(0.0, 0.0), (0.5, -0.12), (1.0, 0.0)]) * em,
                    FxKind::Wobble => rot = k(&[(0.0, 0.0), (0.25, -7.0), (0.75, 7.0), (1.0, 0.0)]),
                    FxKind::Bob => {
                        dy = k(&[(0.0, 0.0), (0.5, -0.08), (1.0, 0.0)]) * em;
                        rot = k(&[(0.0, 0.0), (0.5, -4.0), (1.0, 0.0)]);
                    }
                    FxKind::Sway => {
                        dy = k(&[(0.0, 0.0), (0.5, -0.05), (1.0, 0.0)]) * em;
                        rot = k(&[(0.0, 0.0), (0.5, 4.0), (1.0, 0.0)]);
                    }
                    FxKind::Flicker => {
                        fx_opacity = k(&[(0.0, 1.0), (0.2, 0.72), (0.23, 0.95), (0.5, 0.82), (0.7, 1.0), (0.72, 0.65), (0.75, 1.0), (1.0, 1.0)])
                    }
                    FxKind::Pop => {
                        scale = k(&[(0.0, 1.0), (0.25, 1.18), (0.4, 0.96), (0.55, 1.0), (1.0, 1.0)]);
                        rot = k(&[(0.0, 0.0), (0.25, -5.0), (0.4, 2.0), (0.55, 0.0), (1.0, 0.0)]);
                    }
                    FxKind::Shimmer if !set.tints.is_empty() => {
                        // background-position 100 % → 0 over a 400 % wide gradient: the letter's centre sees
                        // gradient position 0.75 · (1 − phase) + 0.125.
                        let gp = 0.75 * (1.0 - phase) + 0.125;
                        let n = (set.tints.len() - 1) as f32;
                        let i = ((gp - SHIMMER_SPAN.0) / (SHIMMER_SPAN.1 - SHIMMER_SPAN.0) * n).clamp(0.0, n);
                        let lo = i.floor() as usize;
                        let hi = (lo + 1).min(set.tints.len() - 1);
                        shimmer = Some((set.tints[lo], set.tints[hi], i - lo as f32));
                    }
                    FxKind::Shimmer => {}
                }
            }
            let focus = if self.options.transition && s.on { 1.0 - bezier(FOCUS_EASE, ((t - s.since) / FOCUS) as f32) } else { 0.0 };
            let place = |sprite: u32, opacity: f32| Instance { sprite, group: 0, x, y: y + dy, opacity, scale, scale_y: 1.0, rotation: rot.to_radians() };
            out.push(place(set.off, (1.0 - level) * if s.on { fx_opacity } else { 1.0 }));
            let lit = level * fx_opacity * (1.0 - focus);
            match shimmer {
                Some((a, b, k)) => {
                    out.push(place(a, lit));
                    out.push(place(b, lit * k));
                }
                None => {
                    out.push(place(on_sprite, lit));
                    out.push(place(on_sprite, 0.0));
                }
            }
            out.push(place(set.focus, level * fx_opacity * focus));
        }
    }

    /// Software compositor: draws instances into a pixmap. Used for previews and tests, and the shape of
    /// what the ESP32 blitter does.
    pub fn compose(&self, instances: &[Instance], width: u32, height: u32) -> Pixmap {
        let mut target = Pixmap::new(width, height).unwrap();
        let clip = rounded_rect_mask_xy(width, height, self.layout.face, self.layout.radius.0, self.layout.radius.1);
        for inst in instances {
            if inst.opacity <= 0.001 {
                continue;
            }
            let s = &self.sprites[inst.sprite as usize];
            if s.rgba.is_empty() {
                continue;
            }
            let Some(src) = tiny_skia::PixmapRef::from_bytes(&s.rgba, s.width, s.height) else { continue };
            let paint = tiny_skia::PixmapPaint { opacity: inst.opacity.min(1.0), quality: tiny_skia::FilterQuality::Bilinear, ..Default::default() };
            let transform = tiny_skia::Transform::from_translate(inst.x, inst.y)
                .pre_rotate(inst.rotation.to_degrees())
                .pre_scale(inst.scale, inst.scale * inst.scale_y)
                .pre_translate(-(s.width as f32) / 2.0, -(s.height as f32) / 2.0);
            let mask = if inst.group == 1 { clip.as_ref() } else { None };
            target.draw_pixmap(0, 0, src, &paint, transform, mask);
        }
        target
    }

    pub(crate) fn layout(&self) -> &Layout {
        &self.layout
    }

    pub(crate) fn theme(&self) -> &Theme {
        &self.theme
    }

    pub(crate) fn options(&self) -> &Options {
        &self.options
    }

    pub(crate) fn special(&self) -> &special::Special {
        &self.special
    }

    pub(crate) fn special_mut(&mut self) -> &mut special::Special {
        &mut self.special
    }
}

/// Part of the shimmer gradient that differs from the base colour (gradient positions).
const SHIMMER_SPAN: (f32, f32) = (0.3, 0.7);

fn gradient_color(stops: &[(Color, f32)], p: f32) -> Color {
    let c = css::sample(stops, p, false);
    if c[3] <= 0.0 {
        return Color::TRANSPARENT;
    }
    Color { r: c[0] / c[3], g: c[1] / c[3], b: c[2] / c[3], a: c[3] }
}

fn fill_circle_clear(p: &mut Pixmap, cx: f32, cy: f32, r: f32) {
    if let Some(path) = tiny_skia::PathBuilder::from_circle(cx, cy, r) {
        let mut paint = tiny_skia::Paint::default();
        paint.set_color(tiny_skia::Color::BLACK);
        paint.blend_mode = tiny_skia::BlendMode::Clear;
        paint.anti_alias = true;
        p.fill_path(&path, &paint, tiny_skia::FillRule::Winding, tiny_skia::Transform::identity(), None);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use theme::ThemeFile;

    pub(crate) fn engine(theme: &str, options: Options) -> Engine {
        let lib: qlock_core::Library = serde_json::from_str(include_str!("../../data/locales.json")).unwrap();
        let file: ThemeFile = serde_json::from_str(include_str!("../../data/themes.json")).unwrap();
        let theme = Theme::from_export(theme, &file.themes[theme]);
        Engine::new(lib.locale("de").unwrap().clone(), theme, options, 400, 300, Some("So · 1. November"))
    }

    #[test]
    fn instance_list_is_stable() {
        let options = Options { dots: true, ambient: true, transition: true, effect: true, progress: true, ..Options::default() };
        for theme in ["sunset", "halloween", "chaos-burst", "retro", "formrausch", "chaos-night", "weihnachten", "silvester", "ostern", "karneval"] {
            let mut e = engine(theme, options);
            let a = e.update(15, 15, 1.0, 0.0).len();
            let b = e.update(15, 15, 5.0, 5.0).len();
            let c = e.update(15, 16, 9.0, 9.0).len();
            assert_eq!((a, a), (b, c), "{theme}");
        }
    }

    #[test]
    fn letters_light_up_after_the_start_delay() {
        let mut e = engine("formrausch", Options::default());
        e.update(15, 15, 0.0, 0.0);
        let lit = |e: &Engine| e.state.iter().filter(|s| s.on).count();
        assert_eq!(lit(&e), 0);
        e.update(15, 15, 0.0, 10.0);
        assert_eq!(lit(&e), "VIERTELNACHDREI".chars().count());
    }

    #[test]
    fn particles_move_every_frame_at_large_host_times() {
        let base = 345_600.0;
        let options = Options { ambient: true, ..Options::default() };
        let mut e = engine("lagoon", options);
        let particle = |e: &mut Engine, t: f64| e.update(15, 15, 0.0, t).iter().find(|i| i.group == 1).copied().unwrap();
        let mut last = particle(&mut e, base);
        for k in 1..120 {
            let now = particle(&mut e, base + k as f64 / 60.0);
            assert!(now.y != last.y || now.opacity == 0.0, "particle stood still at frame {k}");
            assert!((now.y - last.y).abs() < 20.0, "particle jumped at frame {k}");
            last = now;
        }
    }

    #[test]
    fn curves() {
        assert!(bezier(EASE, 0.0).abs() < 1e-4);
        assert!((bezier(EASE, 1.0) - 1.0).abs() < 1e-4);
        assert!((bezier(EASE_IN_OUT, 0.5) - 0.5).abs() < 1e-3);
        assert!((keyframes(&[(0.0, 1.0), (0.5, 0.6), (1.0, 1.0)], 0.5, LINEAR) - 0.6).abs() < 1e-5);
    }
}
