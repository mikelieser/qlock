//! Sprite renderer for the word clock, shared by every native target.
//!
//! Everything that is expensive (letters with glow, backgrounds, dots, particles) is rasterised once into
//! [`Sprite`]s. Each frame then only produces a fixed list of [`Instance`]s: which sprite, where, how
//! opaque, how big. A GPU compositor (Core Animation on the Mac) or a small software blitter (ESP32) can
//! draw that at full frame rate, and all animation timing lives here, identical on every platform.

pub mod raster;
pub mod theme;

use qlock_core::{Frame, Locale};
use tiny_skia::Pixmap;

use raster::{Coverage, Face, FloatCanvas, brighten, circle_coverage, fill_circle, fill_rect, paint_coverage, rounded_rect_mask};
use theme::{Color, DotShape, Motion, Theme};

/// Inter as a variable font, the web app's default face.
pub const INTER: &[u8] = include_bytes!("../../assets/fonts/InterVariable.ttf");

#[derive(Debug, Clone, Copy, Default)]
pub struct Options {
    pub intro: bool,
    pub dots: bool,
    pub stretch: bool,
    /// Letter effect (breathing).
    pub effect: bool,
    /// Focus pull when the time changes.
    pub transition: bool,
    /// Background particles.
    pub ambient: bool,
}

/// Premultiplied RGBA pixels.
pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

impl Sprite {
    fn from_pixmap(p: Pixmap) -> Sprite {
        Sprite { width: p.width(), height: p.height(), rgba: p.take() }
    }
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
    pub rotation: f32,
}

struct Layout {
    face: [f32; 4],
    radius: f32,
    pad: f32,
    cell_w: f32,
    cell_h: f32,
    font_px: f32,
    dot_size: f32,
    view_min: f32,
}

impl Layout {
    fn new(w: f32, h: f32, locale: &Locale, theme: &Theme, stretch: bool) -> Layout {
        let face = if stretch {
            [0.0, 0.0, w, h]
        } else {
            let s = w.min(h) * 0.8;
            [(w - s) / 2.0, (h - s) / 2.0, s, s]
        };
        let pad = face[2].min(face[3]) * 0.14;
        let cell_w = (face[2] - 2.0 * pad) / locale.cols as f32;
        let cell_h = (face[3] - 2.0 * pad) / locale.rows as f32;
        Layout {
            face,
            radius: if stretch { 0.0 } else { theme.face.radius * face[2] },
            pad,
            cell_w,
            cell_h,
            font_px: (cell_h * theme.chars.scale).min(cell_w * 0.9),
            dot_size: pad * theme.dots.size,
            view_min: w.min(h),
        }
    }

    fn cell_centre(&self, cols: usize, cell: usize) -> (f32, f32) {
        let (row, col) = (cell / cols, cell % cols);
        (
            self.face[0] + self.pad + (col as f32 + 0.5) * self.cell_w,
            self.face[1] + self.pad + (row as f32 + 0.5) * self.cell_h,
        )
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
fn bezier(x1: f32, y1: f32, x2: f32, y2: f32, t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    let curve = |a: f32, b: f32, s: f32| 3.0 * a * s * (1.0 - s).powi(2) + 3.0 * b * s * s * (1.0 - s) + s * s * s;
    // Solve x(s) = t by bisection; plenty precise for animation.
    let (mut lo, mut hi) = (0.0f32, 1.0f32);
    for _ in 0..24 {
        let mid = (lo + hi) / 2.0;
        if curve(x1, x2, mid) < t {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    curve(y1, y2, (lo + hi) / 2.0)
}

fn ease(t: f32) -> f32 {
    bezier(0.25, 0.1, 0.25, 1.0, t)
}

fn ease_in_out(t: f32) -> f32 {
    bezier(0.42, 0.0, 0.58, 1.0, t)
}

/// Per-cell animation state.
#[derive(Clone, Copy)]
struct CellState {
    on: bool,
    /// When the current fade starts (seconds), and the level it starts from.
    since: f64,
    from: f32,
    order: usize,
}

struct CellSprites {
    off: u32,
    on: u32,
    focus: u32,
}

/// One clock on one screen.
pub struct Engine {
    locale: Locale,
    theme: Theme,
    options: Options,
    layout: Layout,
    sprites: Vec<Sprite>,
    background: u32,
    cells: Vec<CellSprites>,
    dots_off: u32,
    dots_on: [u32; 4],
    dots_offset: f32,
    particles: Vec<u32>,
    caption: Option<u32>,
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

impl Engine {
    /// `width`/`height` in pixels. `caption` is the date line, already formatted for the locale.
    pub fn new(locale: Locale, theme: Theme, options: Options, width: u32, height: u32, caption: Option<&str>) -> Engine {
        let layout = Layout::new(width as f32, height as f32, &locale, &theme, options.stretch);
        let mut engine = Engine {
            state: vec![CellState { on: false, since: f64::MIN, from: 0.0, order: 0 }; locale.cell_count()],
            locale,
            theme,
            options,
            layout,
            sprites: Vec::new(),
            background: 0,
            cells: Vec::new(),
            dots_off: 0,
            dots_on: [0; 4],
            dots_offset: 0.0,
            particles: Vec::new(),
            caption: None,
            dots_lit: 0,
            dots_since: f64::MIN,
            current: None,
            started: None,
            instances: Vec::new(),
        };
        engine.build_sprites(width, height, caption);
        engine
    }

    pub fn sprites(&self) -> &[Sprite] {
        &self.sprites
    }

    /// Frees the sprite pixels once the host has copied them (sizes stay); `compose` draws nothing after.
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
        self.layout.radius
    }

    fn push(&mut self, pixmap: Pixmap) -> u32 {
        self.sprites.push(Sprite::from_pixmap(pixmap));
        (self.sprites.len() - 1) as u32
    }

    fn build_sprites(&mut self, width: u32, height: u32, caption: Option<&str>) {
        let theme = self.theme.clone();
        let l = &self.layout;

        // Page and face, including the face's decoration.
        let mut bg = FloatCanvas::new(width, height);
        bg.paint(&theme.page.fill, [0.0, 0.0, width as f32, height as f32], None);
        let mask = rounded_rect_mask(width, height, l.face, l.radius);
        for layer in &theme.face.layers {
            bg.paint(layer, l.face, mask.as_ref());
        }
        let bg = bg.into_pixmap();
        self.background = self.push(bg);

        // Letters: one sprite per distinct letter and state.
        let off_face = Face::new(INTER, theme.chars.weight_off).expect("font");
        let on_face = Face::new(INTER, theme.chars.weight_on).expect("font");
        let font_px = self.layout.font_px;
        let (cell_w, cell_h) = (self.layout.cell_w, self.layout.cell_h);
        let glow_reach = theme.chars.glow.iter().map(|g| g.blur * font_px * 1.5).fold(0.0, f32::max);
        let margin = glow_reach.max(font_px * 0.12 * 1.5).ceil();
        let sw = (cell_w + 2.0 * margin).ceil() as u32 + 1;
        let sh = (cell_h + 2.0 * margin).ceil() as u32 + 1;
        let mut by_char: Vec<(String, CellSprites)> = Vec::new();
        for c in 0..self.locale.cell_count() {
            let ch = self.locale.chars[c].clone();
            if let Some((_, s)) = by_char.iter().find(|(k, _)| *k == ch) {
                self.cells.push(CellSprites { off: s.off, on: s.on, focus: s.focus });
                continue;
            }
            let draw = |face: &Face, color: Color, background: Option<Color>, glow: bool| {
                let mut p = Pixmap::new(sw, sh).unwrap();
                let (cx, cy) = (sw as f32 / 2.0, sh as f32 / 2.0);
                if let Some(bgc) = background {
                    // Slightly larger than the cell so neighbouring blocks merge without seams.
                    fill_rect(&mut p, [cx - cell_w / 2.0 - 0.5, cy - cell_h / 2.0 - 0.5, cell_w + 1.0, cell_h + 1.0], bgc);
                }
                let (cov, ox, oy, adv) = face.text(&ch, font_px, 0.0);
                let (asc, desc) = face.metrics(font_px);
                let pen_x = cx - adv / 2.0;
                let baseline = cy + (asc + desc) / 2.0;
                let (gx, gy) = ((pen_x + ox).round() as i32, (baseline + oy).round() as i32);
                if glow {
                    for shadow in &theme.chars.glow {
                        let sigma = shadow.blur * font_px / 2.0;
                        let pad = (sigma * 3.0).ceil() as usize;
                        let mut big = Coverage::new(cov.w + 2 * pad, cov.h + 2 * pad);
                        for y in 0..cov.h {
                            for x in 0..cov.w {
                                big.data[(y + pad) * big.w + x + pad] = cov.data[y * cov.w + x];
                            }
                        }
                        let blurred = big.blurred(sigma);
                        paint_coverage(&mut p, &blurred, gx - pad as i32, gy - pad as i32, shadow.color.unwrap_or(color));
                    }
                }
                paint_coverage(&mut p, &cov, gx, gy, color);
                p
            };
            let off = draw(&off_face, theme.chars.off, None, false);
            let on = draw(&on_face, theme.chars.on, theme.chars.on_background, true);
            // Transition: the letter starts blurred and bright and pulls into focus.
            let mut focus_src = on.clone();
            brighten(&mut focus_src, 1.6);
            let focus = blur_pixmap(&focus_src, 0.12 * font_px / 2.0);
            let s = CellSprites { off: self.push(off), on: self.push(on), focus: self.push(focus) };
            self.cells.push(CellSprites { off: s.off, on: s.on, focus: s.focus });
            by_char.push((ch, s));
        }

        // Corner dots.
        let dot = self.layout.dot_size;
        let view_min = self.layout.view_min;
        let glow_reach = theme.dots.glow.iter().map(|g| g.blur * view_min * 1.5).fold(0.0, f32::max);
        let size = (dot + 2.0 * glow_reach).ceil() as u32 + 2;
        self.dots_offset = 0.0;
        let make_dot = |color: Color, glow: bool| {
            let mut p = Pixmap::new(size, size).unwrap();
            let c = size as f32 / 2.0;
            match theme.dots.shape {
                DotShape::Circle => {
                    if glow {
                        for shadow in &theme.dots.glow {
                            let sigma = shadow.blur * view_min / 2.0;
                            let cov = circle_coverage(size as usize, c, c, dot / 2.0).blurred(sigma);
                            paint_coverage(&mut p, &cov, 0, 0, shadow.color.unwrap_or(color));
                        }
                    }
                    fill_circle(&mut p, c, c, dot / 2.0, color);
                }
                DotShape::Cross => {
                    let mut pb = tiny_skia::PathBuilder::new();
                    let r = dot / 2.0;
                    pb.move_to(c - r, c - r);
                    pb.line_to(c + r, c + r);
                    pb.move_to(c + r, c - r);
                    pb.line_to(c - r, c + r);
                    if let Some(path) = pb.finish() {
                        let mut paint = tiny_skia::Paint::default();
                        paint.set_color(color.to_skia());
                        paint.anti_alias = true;
                        let stroke = tiny_skia::Stroke { width: (dot * 0.07).max(1.0), ..Default::default() };
                        p.stroke_path(&path, &paint, &stroke, tiny_skia::Transform::identity(), None);
                    }
                }
            }
            p
        };
        let off_dot = make_dot(theme.dots.off, false);
        self.dots_off = self.push(off_dot);
        for i in 0..4 {
            let on_dot = make_dot(theme.dots.colors[i], true);
            self.dots_on[i] = self.push(on_dot);
        }

        // Particles: one sprite per colour, sized for the largest particle.
        if let Some(amb) = &theme.ambient {
            let face_min = self.layout.face[2].min(self.layout.face[3]);
            let d = amb.size * face_min;
            let glow = amb.glow * face_min;
            let size = (d + 4.0 * glow).ceil() as u32 + 2;
            for color in &amb.colors {
                let mut p = Pixmap::new(size, size).unwrap();
                let c = size as f32 / 2.0;
                if glow > 0.0 {
                    let cov = circle_coverage(size as usize, c, c, d / 2.0).blurred(glow / 2.0);
                    paint_coverage(&mut p, &cov, 0, 0, *color);
                }
                if amb.round {
                    fill_circle(&mut p, c, c, d / 2.0, *color);
                } else {
                    fill_rect(&mut p, [c - d / 2.0, c - d / 2.0, d, d], *color);
                }
                let id = self.push(p);
                self.particles.push(id);
            }
        }

        // Date line, spaced out like the CSS caption.
        if let Some(text) = caption {
            let px = self.layout.pad * 0.14;
            let (cov, _, _, _) = on_face.text(&text.to_uppercase(), px, px * 0.32);
            if cov.w > 0 {
                let mut p = Pixmap::new(cov.w as u32 + 4, cov.h as u32 + 4).unwrap();
                let color = theme.caption.color.unwrap_or(theme.chars.on);
                paint_coverage(&mut p, &cov, 2, 2, color);
                self.caption = Some(self.push(p));
            }
        }
    }

    fn level(&self, s: &CellState, t: f64) -> f32 {
        let fade = self.theme.timing.fade as f64;
        let p = ((t - s.since) / fade).clamp(0.0, 1.0) as f32;
        let target = if s.on { 1.0 } else { 0.0 };
        s.from + (target - s.from) * ease(p)
    }

    fn set_frame(&mut self, frame: Frame, t: f64) {
        let stagger = self.theme.timing.stagger as f64;
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

    /// Instances for local time `hour:minute` at animation time `t` (seconds, monotonic).
    pub fn update(&mut self, hour: u32, minute: u32, t: f64) -> &[Instance] {
        let started = *self.started.get_or_insert(t);
        if t - started >= START_DELAY {
            let frame = self.locale.frame(hour, minute, self.options.intro);
            if self.current.as_ref() != Some(&frame) {
                self.set_frame(frame, t);
            }
        }

        let mut out = std::mem::take(&mut self.instances);
        out.clear();
        let l = &self.layout;
        let (sw, sh) = (self.sprites[self.background as usize].width, self.sprites[self.background as usize].height);
        out.push(Instance { sprite: self.background, group: 0, x: sw as f32 / 2.0, y: sh as f32 / 2.0, opacity: 1.0, scale: 1.0, rotation: 0.0 });

        // Particles (CSS amb-rise), under everything else on the face.
        if let Some(amb) = self.theme.ambient.as_ref().filter(|_| self.options.ambient) {
            let [fx, fy, fw, fh] = l.face;
            for i in 0..PARTICLES {
                let x = ((i * 37 + 11) % 100) as f32 / 100.0;
                let delay = -((i as f32 * 1.73) % PARTICLES as f32);
                let speed = 0.75 + ((i * 7) % 5) as f32 * 0.12;
                let size = 0.7 + ((i * 3) % 4) as f32 * 0.2;
                let wander = ((i * 5) % 7) as f32 - 3.0;
                let duration = amb.duration * speed;
                // f64 on purpose: hosts pass uptime-based seconds (CACurrentMediaTime is ~10^5–10^6), where f32
                // only resolves ~1/32 s and the particles would move in steps.
                let p = ((((t - delay as f64) / duration as f64) % 1.0 + 1.0) % 1.0) as f32;
                let (dx, dy, scale, opacity) = match amb.motion {
                    Motion::Rise => {
                        let (dx, dy, sc) = if p < 0.5 {
                            let k = p / 0.5;
                            (wander * 0.015 * fw * k, -0.55 * fh * k, 0.6 + 0.4 * k)
                        } else {
                            let k = (p - 0.5) / 0.5;
                            (
                                wander * 0.015 * fw + (wander * -0.01 * fw - wander * 0.015 * fw) * k,
                                -0.55 * fh + (-1.15 * fh + 0.55 * fh) * k,
                                1.0 - 0.2 * k,
                            )
                        };
                        let op = if p < 0.15 {
                            p / 0.15
                        } else if p > 0.85 {
                            (1.0 - p) / 0.15
                        } else {
                            1.0
                        };
                        (dx, dy, sc, op * amb.opacity)
                    }
                };
                let sprite = self.particles[i % self.particles.len()];
                let d = amb.size * fw.min(fh) * size;
                out.push(Instance {
                    sprite,
                    group: 1,
                    x: fx + x * fw + d / 2.0 + dx,
                    y: fy + 1.05 * fh + d / 2.0 + dy,
                    opacity,
                    scale: scale * size,
                    rotation: 0.0,
                });
            }
        }

        if let Some(caption) = self.caption {
            out.push(Instance {
                sprite: caption,
                group: 0,
                x: l.face[0] + l.face[2] / 2.0,
                y: l.face[1] + l.pad * 0.7,
                opacity: self.theme.caption.opacity * ease(((t - started) / 0.9) as f32),
                scale: 1.0,
                rotation: 0.0,
            });
        }

        // Corner dots: off underneath, on fades in; the transition pops new ones in from 1.7×.
        for corner in 0..4 {
            let (x, y) = l.dot_centre(corner);
            let visible = if self.options.dots { 1.0 } else { 0.0 };
            let lit = (corner as u8) < self.dots_lit;
            let fade = ease(((t - self.dots_since) / self.theme.timing.fade as f64) as f32);
            let on = if lit { if corner as u8 + 1 == self.dots_lit { fade } else { 1.0 } } else { 0.0 };
            let pop = if self.options.transition && lit && corner as u8 + 1 == self.dots_lit {
                1.0 + 0.7 * (1.0 - bezier(0.2, 0.7, 0.2, 1.0, ((t - self.dots_since) / 0.9) as f32))
            } else {
                1.0
            };
            out.push(Instance { sprite: self.dots_off, group: 0, x, y, opacity: visible * (1.0 - on), scale: 1.0, rotation: 0.0 });
            out.push(Instance { sprite: self.dots_on[corner], group: 0, x, y, opacity: visible * on, scale: pop, rotation: 0.0 });
        }

        // Letters: off, on and (transition) the blurred focus image on top, cross-faded.
        let cols = self.locale.cols;
        for c in 0..self.state.len() {
            let s = self.state[c];
            let level = self.level(&s, t);
            let (x, y) = l.cell_centre(cols, c);
            let breathe = if self.options.effect && s.on {
                // CSS fx-breathe: 4 s, opacity 1 → 0.6 → 1, phase shifted per letter.
                let phase = (((t + s.order as f64 * 0.15) / 4.0) % 1.0) as f32;
                let k = if phase < 0.5 { ease_in_out(phase * 2.0) } else { 1.0 - ease_in_out((phase - 0.5) * 2.0) };
                1.0 - 0.4 * k
            } else {
                1.0
            };
            let focus = if self.options.transition && s.on {
                1.0 - bezier(0.2, 0.7, 0.2, 1.0, ((t - s.since) / FOCUS) as f32)
            } else {
                0.0
            };
            let cs = &self.cells[c];
            let (off, on, foc) = (cs.off, cs.on, cs.focus);
            out.push(Instance { sprite: off, group: 0, x, y, opacity: 1.0 - level, scale: 1.0, rotation: 0.0 });
            out.push(Instance { sprite: on, group: 0, x, y, opacity: level * breathe * (1.0 - focus), scale: 1.0, rotation: 0.0 });
            out.push(Instance { sprite: foc, group: 0, x, y, opacity: level * breathe * focus, scale: 1.0, rotation: 0.0 });
        }

        self.instances = out;
        &self.instances
    }

    /// Software compositor: draws the instances into a pixmap. Used for previews and tests, and the
    /// shape of what the ESP32 blitter does.
    pub fn compose(&self, instances: &[Instance], width: u32, height: u32) -> Pixmap {
        let mut target = Pixmap::new(width, height).unwrap();
        let clip = rounded_rect_mask(width, height, self.layout.face, self.layout.radius);
        for inst in instances {
            if inst.opacity <= 0.001 {
                continue;
            }
            let s = &self.sprites[inst.sprite as usize];
            if s.rgba.is_empty() {
                continue;
            }
            let Some(src) = tiny_skia::PixmapRef::from_bytes(&s.rgba, s.width, s.height) else { continue };
            let paint = tiny_skia::PixmapPaint { opacity: inst.opacity.min(1.0), ..Default::default() };
            let transform = tiny_skia::Transform::from_translate(inst.x, inst.y)
                .pre_rotate(inst.rotation)
                .pre_scale(inst.scale, inst.scale)
                .pre_translate(-(s.width as f32) / 2.0, -(s.height as f32) / 2.0);
            let mask = if inst.group == 1 { clip.as_ref() } else { None };
            target.draw_pixmap(0, 0, src, &paint, transform, mask);
        }
        target
    }
}

fn blur_pixmap(src: &Pixmap, sigma: f32) -> Pixmap {
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
        let a = blurred[3].data[i];
        for k in 0..4 {
            let v = blurred[k].data[i].min(a.max(if k == 3 { 1.0 } else { 0.0 }));
            dst[i * 4 + k] = (v * 255.0 + 0.5).clamp(0.0, 255.0) as u8;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine(theme: &str, options: Options) -> Engine {
        let lib: qlock_core::Library = serde_json::from_str(include_str!("../../data/locales.json")).unwrap();
        let theme = Theme::from_toml(theme).unwrap();
        Engine::new(lib.locale("de").unwrap().clone(), theme, options, 400, 300, Some("So · 1. November"))
    }

    #[test]
    fn instance_list_is_stable() {
        let options = Options { dots: true, ambient: true, transition: true, effect: true, ..Options::default() };
        let mut e = engine(include_str!("../../themes/sunset.toml"), options);
        let a = e.update(15, 15, 0.0).len();
        let b = e.update(15, 15, 5.0).len();
        let c = e.update(15, 16, 9.0).len();
        assert_eq!((a, a), (b, c));
    }

    #[test]
    fn letters_light_up_after_the_start_delay() {
        let mut e = engine(include_str!("../../themes/formrausch.toml"), Options::default());
        e.update(15, 15, 0.0);
        let lit = |e: &Engine| e.state.iter().filter(|s| s.on).count();
        assert_eq!(lit(&e), 0);
        e.update(15, 15, 10.0);
        assert_eq!(lit(&e), "VIERTELNACHDREI".chars().count());
    }

    #[test]
    fn particles_move_every_frame_at_large_host_times() {
        // CACurrentMediaTime after a few days of uptime.
        let base = 345_600.0;
        let options = Options { ambient: true, ..Options::default() };
        let mut e = engine(include_str!("../../themes/sunset.toml"), options);
        let particle = |e: &mut Engine, t: f64| e.update(15, 15, t).iter().find(|i| i.group == 1).copied().unwrap();
        let mut last = particle(&mut e, base);
        for k in 1..120 {
            let now = particle(&mut e, base + k as f64 / 60.0);
            assert!(now.y != last.y || now.opacity == 0.0, "particle stood still at frame {k}");
            assert!((now.y - last.y).abs() < 20.0, "particle jumped at frame {k}");
            last = now;
        }
    }

    #[test]
    fn bezier_matches_endpoints() {
        assert!(bezier(0.25, 0.1, 0.25, 1.0, 0.0).abs() < 1e-4);
        assert!((bezier(0.25, 0.1, 0.25, 1.0, 1.0) - 1.0).abs() < 1e-4);
        assert!((ease_in_out(0.5) - 0.5).abs() < 1e-3);
    }
}
