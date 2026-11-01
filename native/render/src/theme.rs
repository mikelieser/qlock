//! Theme data, read from `native/themes/*.toml`. Mirrors the CSS custom properties of the web themes.

use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const TRANSPARENT: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };

    /// `#rgb`, `#rrggbb` or `#rrggbbaa`.
    pub fn parse(hex: &str) -> Option<Color> {
        let hex = hex.strip_prefix('#')?;
        let digits: Vec<u8> = hex.chars().map(|c| c.to_digit(16).map(|d| d as u8)).collect::<Option<_>>()?;
        let byte = |i: usize| (digits[i] * 16 + digits[i + 1]) as f32 / 255.0;
        match digits.len() {
            3 => Some(Color {
                r: (digits[0] * 17) as f32 / 255.0,
                g: (digits[1] * 17) as f32 / 255.0,
                b: (digits[2] * 17) as f32 / 255.0,
                a: 1.0,
            }),
            6 => Some(Color { r: byte(0), g: byte(2), b: byte(4), a: 1.0 }),
            8 => Some(Color { r: byte(0), g: byte(2), b: byte(4), a: byte(6) }),
            _ => None,
        }
    }

    pub fn with_alpha(self, a: f32) -> Color {
        Color { a: self.a * a, ..self }
    }

    pub fn to_skia(self) -> tiny_skia::Color {
        tiny_skia::Color::from_rgba(self.r, self.g, self.b, self.a.clamp(0.0, 1.0)).unwrap_or(tiny_skia::Color::TRANSPARENT)
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        Color::parse(&s).ok_or_else(|| serde::de::Error::custom(format!("bad colour {s:?}")))
    }
}

/// One background layer, like one entry of a CSS `background` list.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Fill {
    Solid(Color),
    /// CSS `linear-gradient(<angle>deg, …)`; 180 runs top to bottom.
    Linear { angle: f32, stops: Vec<(Color, f32)> },
    /// CSS `radial-gradient(circle|ellipse at x y, …)` sized to the farthest corner.
    Radial {
        x: f32,
        y: f32,
        #[serde(default)]
        ellipse: bool,
        stops: Vec<(Color, f32)>,
    },
    /// An SVG path (`M Q T L H V C Z`) in a `view` box, stretched into `box` = [x, y, w, h] of the area.
    Path {
        d: String,
        view: [f32; 2],
        fill: Color,
        #[serde(rename = "box")]
        area: [f32; 4],
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct Page {
    pub fill: Fill,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Face {
    /// Corner radius as a share of the face width (CSS `--face-radius`).
    #[serde(default)]
    pub radius: f32,
    pub layers: Vec<Fill>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Shadow {
    /// Blur radius: in `em` for letters, as a share of the screen's shorter side for dots.
    pub blur: f32,
    /// Defaults to the element's own colour.
    #[serde(default)]
    pub color: Option<Color>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Chars {
    pub weight_off: f32,
    pub weight_on: f32,
    pub off: Color,
    pub on: Color,
    #[serde(default)]
    pub on_background: Option<Color>,
    #[serde(default)]
    pub glow: Vec<Shadow>,
    #[serde(default = "default_char_scale")]
    pub scale: f32,
}

fn default_char_scale() -> f32 {
    0.62
}

#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DotShape {
    #[default]
    Circle,
    Cross,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Dots {
    #[serde(default)]
    pub shape: DotShape,
    pub off: Color,
    /// Corner 1 (top left) clockwise.
    pub colors: [Color; 4],
    #[serde(default)]
    pub glow: Vec<Shadow>,
    #[serde(default = "default_dot_size")]
    pub size: f32,
}

fn default_dot_size() -> f32 {
    0.22
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Caption {
    #[serde(default)]
    pub color: Option<Color>,
    #[serde(default = "default_caption_opacity")]
    pub opacity: f32,
}

fn default_caption_opacity() -> f32 {
    0.6
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Motion {
    Rise,
}

/// Background particles (CSS `--amb-*`).
#[derive(Debug, Clone, Deserialize)]
pub struct Ambient {
    pub motion: Motion,
    /// Diameter as a share of the face's shorter side.
    pub size: f32,
    #[serde(default)]
    pub round: bool,
    /// Seconds per cycle before the per-particle factor.
    pub duration: f32,
    pub opacity: f32,
    pub colors: Vec<Color>,
    /// Glow radius as a share of the face's shorter side.
    #[serde(default)]
    pub glow: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Timing {
    /// Seconds a letter takes to light up or go dark (CSS `--fade`).
    pub fade: f32,
    /// Delay between letters in reading order (CSS `--stagger`).
    pub stagger: f32,
}

impl Default for Timing {
    fn default() -> Self {
        Timing { fade: 0.9, stagger: 0.045 }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Theme {
    pub name: String,
    pub page: Page,
    pub face: Face,
    pub chars: Chars,
    pub dots: Dots,
    #[serde(default)]
    pub caption: Caption,
    #[serde(default)]
    pub ambient: Option<Ambient>,
    #[serde(default)]
    pub timing: Timing,
}

impl Theme {
    pub fn from_toml(text: &str) -> Result<Theme, toml::de::Error> {
        toml::from_str(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colours() {
        assert_eq!(Color::parse("#fff").unwrap().r, 1.0);
        let c = Color::parse("#ff000080").unwrap();
        assert_eq!((c.r, c.g), (1.0, 0.0));
        assert!((c.a - 0.502).abs() < 0.01);
        assert!(Color::parse("red").is_none());
    }

    #[test]
    fn bundled_themes_parse() {
        for text in [include_str!("../../themes/sunset.toml"), include_str!("../../themes/formrausch.toml")] {
            Theme::from_toml(text).unwrap();
        }
    }
}
