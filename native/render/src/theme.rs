//! Themes, read from the CSS custom properties that `native/export-themes.ts` exports from
//! `src/styles.css`. Things CSS does with extra selectors (spiders, rotating rays, outlined letters …)
//! are listed per theme in [`Extras`].

use std::collections::HashMap;

use serde::Deserialize;

use crate::css::{self, ColorRef, Len, Shadow, parse_angle, parse_color, parse_len, parse_shadows};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const TRANSPARENT: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };

    /// `#rgb`, `#rgba`, `#rrggbb` or `#rrggbbaa`.
    pub fn parse(hex: &str) -> Option<Color> {
        let hex = hex.strip_prefix('#')?;
        let d: Vec<f32> = hex.chars().map(|c| c.to_digit(16).map(|v| v as f32)).collect::<Option<_>>()?;
        let pair = |i: usize| (d[i] * 16.0 + d[i + 1]) / 255.0;
        match d.len() {
            3 | 4 => Some(Color { r: d[0] / 15.0, g: d[1] / 15.0, b: d[2] / 15.0, a: if d.len() == 4 { d[3] / 15.0 } else { 1.0 } }),
            6 => Some(Color { r: pair(0), g: pair(2), b: pair(4), a: 1.0 }),
            8 => Some(Color { r: pair(0), g: pair(2), b: pair(4), a: pair(6) }),
            _ => None,
        }
    }

    pub fn with_alpha(self, a: f32) -> Color {
        Color { a: self.a * a, ..self }
    }

    pub fn to_skia(self) -> tiny_skia::Color {
        tiny_skia::Color::from_rgba(self.r.clamp(0.0, 1.0), self.g.clamp(0.0, 1.0), self.b.clamp(0.0, 1.0), self.a.clamp(0.0, 1.0))
            .unwrap_or(tiny_skia::Color::TRANSPARENT)
    }
}

/// One theme as exported: resolved custom properties, plus the ones set on `.dot` only.
#[derive(Debug, Clone, Deserialize)]
pub struct Exported {
    pub vars: HashMap<String, String>,
    #[serde(default)]
    pub dot: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ThemeFile {
    pub themes: HashMap<String, Exported>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Font {
    Inter,
    Amatic,
}

/// Idle letter effect (CSS `fx-*` keyframes).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FxKind {
    Breathe,
    Wave,
    Wobble,
    Bob,
    Flicker,
    Shimmer,
    Pop,
    Sway,
}

#[derive(Debug, Clone)]
pub struct Fx {
    pub kind: FxKind,
    pub duration: f32,
    /// Phase shift per letter in reading order (seconds).
    pub step: f32,
    /// Timing function as cubic-bezier control points.
    pub ease: [f32; 4],
    /// Shimmer: the gradient sweeping through the letters.
    pub gradient: Vec<(Color, f32)>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Motion {
    Fall,
    Rise,
    Sparkle,
    Confetti,
}

/// Background particles (CSS `--amb-*`).
#[derive(Debug, Clone)]
pub struct Ambient {
    pub motion: Motion,
    /// Start height as share of the face (CSS `top`).
    pub top: f32,
    /// Width as share of the face's shorter side.
    pub size: f32,
    pub ratio: f32,
    pub round: bool,
    /// SVG used as a mask for the particle shape.
    pub shape: Option<String>,
    pub duration: f32,
    pub opacity: f32,
    pub colors: [Color; 3],
}

/// What CSS does outside the custom properties, per theme.
#[derive(Debug, Clone, Default)]
pub struct Extras {
    /// Dots drawn as this bitmap instead of a circle (Formrausch cross).
    pub dot_image: Option<&'static str>,
    pub no_dots: bool,
    /// 1px ring around every dot (gradient).
    pub dot_border: Option<Color>,
    /// Solid ring outside each dot (Chaos), as share of the screen's shorter side.
    pub dot_ring: Option<(f32, Color)>,
    /// Outline around lit letters in em (Chaos `-webkit-text-stroke`).
    pub letter_stroke: Option<(f32, Color)>,
    /// Every lit letter its own hue: start, step per letter (Chaos Night).
    pub letter_hues: Option<(f32, f32)>,
    /// Particle glow (`box-shadow: 0 0 x var(--c)`) as share of the face's shorter side.
    pub particle_glow: f32,
    /// Soap-bubble particles: highlight and thin ring instead of a filled disc.
    pub bubbles: bool,
    /// The face decoration swells when particles are on (retro `deco-groove`).
    pub groove: bool,
    /// The sea: the wave decoration drifts and swells when particles are on (sunset, lagoon …).
    pub waves: bool,
    /// Rotating light rays over the face (Chaos Burst).
    pub rays: bool,
    /// Halloween: web, candles, spiders, bat.
    pub spooky: bool,
    /// The clock springs in on start (TickTack).
    pub spring_in: bool,
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub id: String,
    pub page_bg: String,
    pub face_bg: String,
    pub face_deco: String,
    /// Shorter screen side share the square face takes (CSS `--clock-size`).
    pub clock_size: f32,
    pub pad: f32,
    pub face_radius: f32,
    pub face_shadow: Vec<Shadow>,
    pub font: Font,
    pub lowercase: bool,
    pub underline: bool,
    pub weight_off: f32,
    pub weight_on: f32,
    pub char_scale: f32,
    pub char_off: Color,
    pub char_on: Color,
    pub char_on_bg: Option<Color>,
    pub char_glow: Vec<Shadow>,
    pub dot_size: f32,
    pub dot_off: Color,
    pub dot_colors: [Color; 4],
    pub dot_glow: Vec<Shadow>,
    pub progress_color: Color,
    pub progress_width: f32,
    pub caption_color: Color,
    pub caption_opacity: f32,
    pub fx: Fx,
    pub ambient: Option<Ambient>,
    /// Seconds a letter takes to light up or go dark (CSS `--fade`).
    pub fade: f32,
    /// Delay between letters in reading order (CSS `--stagger`).
    pub stagger: f32,
    pub extras: Extras,
}

fn seconds(s: &str) -> Option<f32> {
    let s = s.trim();
    if let Some(v) = s.strip_suffix("ms") {
        v.parse::<f32>().ok().map(|v| v / 1000.0)
    } else {
        s.strip_suffix('s')?.parse().ok()
    }
}

fn bezier(s: &str) -> [f32; 4] {
    match s.trim() {
        "linear" => [0.0, 0.0, 1.0, 1.0],
        "ease" => [0.25, 0.1, 0.25, 1.0],
        "ease-in" => [0.42, 0.0, 1.0, 1.0],
        "ease-out" => [0.0, 0.0, 0.58, 1.0],
        "ease-in-out" => [0.42, 0.0, 0.58, 1.0],
        other => {
            let inner = other.strip_prefix("cubic-bezier(").and_then(|r| r.strip_suffix(')')).unwrap_or("0.42,0,0.58,1");
            let v: Vec<f32> = inner.split(',').filter_map(|p| p.trim().parse().ok()).collect();
            if v.len() == 4 { [v[0], v[1], v[2], v[3]] } else { [0.42, 0.0, 0.58, 1.0] }
        }
    }
}

impl Theme {
    pub fn from_export(id: &str, e: &Exported) -> Theme {
        let var = |k: &str| e.vars.get(k).map(String::as_str).unwrap_or("");
        let num = |k: &str, d: f32| var(k).trim().parse::<f32>().unwrap_or(d);
        let color = |k: &str, d: Color| parse_color(var(k)).unwrap_or(d);
        let pct = |k: &str| match parse_len(var(k)) {
            Some(Len::Pct(v)) => v / 100.0,
            _ => 0.0,
        };
        let char_on = color("char-on", Color::WHITE);
        let dot_on = color("dot-on", Color::WHITE);
        let dot_glow_src = e.dot.get("dot-glow").map(String::as_str).unwrap_or(var("dot-glow"));

        let fx_kind = match var("fx-name") {
            "fx-wave" => FxKind::Wave,
            "fx-wobble" => FxKind::Wobble,
            "fx-bob" => FxKind::Bob,
            "fx-flicker" => FxKind::Flicker,
            "fx-shimmer" => FxKind::Shimmer,
            "fx-pop" => FxKind::Pop,
            "fx-sway" => FxKind::Sway,
            _ => FxKind::Breathe,
        };
        let gradient = match css::parse_background(var("fx-gradient")).0.first().map(|l| &l.image) {
            Some(css::Image::Linear { stops, .. }) => {
                let units = css::Units { em: 1.0, view_min: 1.0, view_w: 1.0, view_h: 1.0, face_w: 1.0, face_h: 1.0 };
                css::resolve_stops(stops, 1.0, &units)
            }
            _ => Vec::new(),
        };

        let ambient = (var("amb-display") == "block").then(|| {
            let shape = css::parse_background(var("amb-shape")).0.first().and_then(|l| match &l.image {
                css::Image::Url(u) => Some(u.clone()),
                _ => None,
            });
            let size = match parse_len(var("amb-size")) {
                Some(Len::Cqmin(v)) => v / 100.0,
                _ => 0.04,
            };
            Ambient {
                motion: match var("amb-name") {
                    "amb-rise" => Motion::Rise,
                    "amb-sparkle" => Motion::Sparkle,
                    "amb-confetti" => Motion::Confetti,
                    _ => Motion::Fall,
                },
                top: pct("amb-top"),
                size,
                ratio: num("amb-ratio", 1.0),
                round: var("amb-radius") == "50%",
                shape,
                duration: seconds(var("amb-duration")).unwrap_or(14.0),
                opacity: num("amb-opacity", 0.8),
                colors: [color("amb-c1", Color::WHITE), color("amb-c2", Color::WHITE), color("amb-c3", Color::WHITE)],
            }
        });

        let font = if var("font").contains("Amatic") { Font::Amatic } else { Font::Inter };
        let on_bg = parse_color(var("char-on-bg")).filter(|c| c.a > 0.0);
        let mut extras = Extras::default();
        match id {
            "formrausch" => extras.dot_image = Some("cross.png"),
            "ticktack" => extras.spring_in = true,
            "chaos" | "chaos-burst" | "chaos-night" => {
                let ink = Color::parse("#1a1033").unwrap();
                extras.letter_stroke = Some((0.07, ink));
                extras.dot_ring = Some((0.0035, ink));
                if id == "chaos-night" {
                    extras.letter_hues = Some((110.0, 47.0));
                }
                extras.rays = id == "chaos-burst";
            }
            "halloween" => extras.spooky = true,
            "karneval" => {
                let ink = Color::parse("#7a0f12").unwrap();
                extras.letter_stroke = Some((0.06, ink));
                extras.dot_ring = Some((0.0035, ink));
            }
            "retro" | "retro-sun" | "retro-stripes" | "retro-night" | "retro-pastel" => extras.groove = true,
            "cozy" | "sommernacht" => extras.particle_glow = 0.01,
            "lagoon" | "cotton-candy" | "lavender" | "bubblegum" | "sky-dream" | "unicorn" => extras.bubbles = true,
            _ => {}
        }
        extras.waves = matches!(id, "sunset" | "lagoon" | "sunrise" | "nightsurf");

        Theme {
            id: id.to_string(),
            page_bg: var("page-bg").to_string(),
            face_bg: var("face-bg").to_string(),
            face_deco: var("face-deco").to_string(),
            clock_size: num("clock-size", 0.8),
            pad: num("pad", 0.14),
            face_radius: pct("face-radius"),
            face_shadow: parse_shadows(var("face-shadow")),
            font,
            lowercase: var("case") == "lowercase",
            underline: var("char-decoration") == "underline",
            weight_off: num("weight-off", 200.0),
            weight_on: num("weight-on", 380.0),
            char_scale: num("char-scale", 0.62),
            char_off: color("char-off", Color::WHITE.with_alpha(0.14)),
            char_on,
            char_on_bg: on_bg,
            char_glow: parse_shadows(var("char-glow")),
            dot_size: num("dot-size", 0.22),
            dot_off: color("dot-off", Color::WHITE.with_alpha(0.14)),
            dot_colors: [1, 2, 3, 4].map(|n| color(&format!("dot-{n}"), dot_on)),
            dot_glow: parse_shadows(dot_glow_src),
            progress_color: color("progress-color", Color::WHITE.with_alpha(0.28)),
            progress_width: parse_len(var("progress-width")).map(|l| match l {
                Len::Px(v) => v,
                _ => 1.0,
            }).unwrap_or(1.0),
            caption_color: parse_color(var("caption-color")).unwrap_or(char_on),
            caption_opacity: num("caption-opacity", 0.6),
            fx: Fx {
                kind: fx_kind,
                duration: seconds(var("fx-duration")).unwrap_or(4.0),
                step: seconds(var("fx-step")).unwrap_or(0.15),
                ease: bezier(var("fx-ease")),
                gradient,
            },
            ambient,
            fade: seconds(var("fade")).unwrap_or(0.9),
            stagger: seconds(var("stagger")).unwrap_or(0.045),
            extras,
        }
    }

    /// The glow colour of a shadow on an element with colour `own`.
    pub fn shadow_color(s: &Shadow, own: Color) -> Color {
        s.color.or(own)
    }
}

impl ColorRef {
    pub fn is_current(self) -> bool {
        matches!(self, ColorRef::Current)
    }
}

/// Angle helper re-exported for effects.
pub fn angle(s: &str) -> f32 {
    parse_angle(s).unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all() -> ThemeFile {
        serde_json::from_str(include_str!("../../data/themes.json")).unwrap()
    }

    #[test]
    fn colours() {
        assert_eq!(Color::parse("#fff").unwrap().r, 1.0);
        let c = Color::parse("#ff000080").unwrap();
        assert!((c.a - 0.502).abs() < 0.01);
        assert!(Color::parse("red").is_none());
    }

    #[test]
    fn every_exported_theme_reads() {
        let file = all();
        assert!(file.themes.len() >= 30);
        for (id, e) in &file.themes {
            let t = Theme::from_export(id, e);
            assert!(!t.face_bg.is_empty(), "{id}");
            assert!(t.weight_on > t.weight_off || t.font == Font::Amatic || t.weight_on >= 380.0, "{id}");
        }
        let sunset = Theme::from_export("sunset", &file.themes["sunset"]);
        assert_eq!(sunset.fx.kind, FxKind::Shimmer);
        assert!(!sunset.fx.gradient.is_empty());
        assert!(sunset.ambient.is_none() && sunset.extras.waves);
        let lagoon = Theme::from_export("lagoon", &file.themes["lagoon"]);
        assert_eq!(lagoon.ambient.as_ref().unwrap().motion, Motion::Rise);
        assert!(sunset.dot_glow[0].color.is_current());
    }
}
