//! A small CSS value parser: colours, lengths, shadows and background layers, as far as the themes in
//! `src/styles.css` use them. Values come from `native/data/themes.json` (see `export-themes.ts`).

use crate::theme::Color;

/// What relative units refer to when a length is resolved.
#[derive(Debug, Clone, Copy)]
pub struct Units {
    /// Letter size (`em`).
    pub em: f32,
    /// Shorter side of the screen (`vmin` = 1 % of it).
    pub view_min: f32,
    pub view_w: f32,
    pub view_h: f32,
    /// Clock face size (`cqw`, `cqh`, `cqmin`).
    pub face_w: f32,
    pub face_h: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Len {
    Px(f32),
    Pct(f32),
    Em(f32),
    Vmin(f32),
    Vw(f32),
    Vh(f32),
    Cqmin(f32),
    Cqw(f32),
    Cqh(f32),
}

impl Len {
    /// Pixels; percentages are taken of `basis`.
    pub fn px(self, u: &Units, basis: f32) -> f32 {
        match self {
            Len::Px(v) => v,
            Len::Pct(v) => v / 100.0 * basis,
            Len::Em(v) => v * u.em,
            Len::Vmin(v) => v / 100.0 * u.view_min,
            Len::Vw(v) => v / 100.0 * u.view_w,
            Len::Vh(v) => v / 100.0 * u.view_h,
            Len::Cqmin(v) => v / 100.0 * u.face_w.min(u.face_h),
            Len::Cqw(v) => v / 100.0 * u.face_w,
            Len::Cqh(v) => v / 100.0 * u.face_h,
        }
    }
}

pub fn parse_len(s: &str) -> Option<Len> {
    let s = s.trim();
    let split = s.find(|c: char| c.is_ascii_alphabetic() || c == '%').unwrap_or(s.len());
    let (num, unit) = s.split_at(split);
    let v: f32 = num.parse().ok()?;
    Some(match unit {
        "" | "px" => Len::Px(v),
        "%" => Len::Pct(v),
        "em" => Len::Em(v),
        "vmin" => Len::Vmin(v),
        "vw" => Len::Vw(v),
        "vh" | "dvh" => Len::Vh(v),
        "cqmin" => Len::Cqmin(v),
        "cqw" => Len::Cqw(v),
        "cqh" => Len::Cqh(v),
        _ => return None,
    })
}

/// Degrees from `deg`, `turn` or `rad`.
pub fn parse_angle(s: &str) -> Option<f32> {
    let s = s.trim();
    if let Some(v) = s.strip_suffix("deg") {
        v.parse().ok()
    } else if let Some(v) = s.strip_suffix("turn") {
        v.parse::<f32>().ok().map(|t| t * 360.0)
    } else if let Some(v) = s.strip_suffix("rad") {
        v.parse::<f32>().ok().map(f32::to_degrees)
    } else if s == "0" {
        Some(0.0)
    } else {
        None
    }
}

/// Splits at `sep` outside parentheses and quotes.
pub fn split_top(text: &str, sep: char) -> Vec<&str> {
    let mut parts = Vec::new();
    let (mut depth, mut quote, mut start) = (0i32, None::<char>, 0usize);
    for (i, c) in text.char_indices() {
        match quote {
            Some(q) if c == q => quote = None,
            Some(_) => {}
            None => match c {
                '"' | '\'' => quote = Some(c),
                '(' => depth += 1,
                ')' => depth -= 1,
                _ if c == sep && depth == 0 => {
                    parts.push(text[start..i].trim());
                    start = i + c.len_utf8();
                }
                _ => {}
            },
        }
    }
    parts.push(text[start..].trim());
    parts.into_iter().filter(|p| !p.is_empty()).collect()
}

/// Whitespace-separated tokens, keeping functions like `rgb(1 2 3 / 0.5)` together.
pub fn tokens(text: &str) -> Vec<&str> {
    split_top(text, ' ')
}

/// A colour, or `currentcolor` (→ `None` in [`ColorRef`]).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorRef {
    Current,
    Fixed(Color),
}

impl ColorRef {
    pub fn or(self, current: Color) -> Color {
        match self {
            ColorRef::Current => current,
            ColorRef::Fixed(c) => c,
        }
    }
}

pub fn parse_color_ref(s: &str) -> Option<ColorRef> {
    if s.eq_ignore_ascii_case("currentcolor") {
        Some(ColorRef::Current)
    } else {
        parse_color(s).map(ColorRef::Fixed)
    }
}

pub fn parse_color(s: &str) -> Option<Color> {
    let s = s.trim();
    if s.starts_with('#') {
        return Color::parse(s);
    }
    match s {
        "transparent" => return Some(Color::TRANSPARENT),
        "white" => return Color::parse("#fff"),
        "black" => return Color::parse("#000"),
        _ => {}
    }
    let open = s.find('(')?;
    let name = &s[..open];
    let inner = s[open + 1..].strip_suffix(')')?;
    let (main, alpha) = match inner.split_once('/') {
        Some((m, a)) => (m, Some(a.trim())),
        None => (inner, None),
    };
    let parts: Vec<&str> = main.split(|c: char| c == ',' || c.is_whitespace()).filter(|p| !p.is_empty()).collect();
    let alpha_of = |a: &str| -> Option<f32> {
        if let Some(p) = a.strip_suffix('%') { p.parse::<f32>().ok().map(|v| v / 100.0) } else { a.parse().ok() }
    };
    match name {
        "rgb" | "rgba" => {
            let channel = |p: &str| -> Option<f32> {
                if let Some(v) = p.strip_suffix('%') { v.parse::<f32>().ok().map(|v| v / 100.0) } else { p.parse::<f32>().ok().map(|v| v / 255.0) }
            };
            let a = match (alpha, parts.get(3)) {
                (Some(a), _) => alpha_of(a)?,
                (None, Some(a)) => alpha_of(a)?,
                _ => 1.0,
            };
            Some(Color { r: channel(parts.first()?)?, g: channel(parts.get(1)?)?, b: channel(parts.get(2)?)?, a })
        }
        "hsl" | "hsla" => {
            let h = parse_angle(parts.first()?).or_else(|| parts.first()?.parse().ok())?;
            let pct = |p: &str| p.trim_end_matches('%').parse::<f32>().ok().map(|v| v / 100.0);
            let a = match (alpha, parts.get(3)) {
                (Some(a), _) => alpha_of(a)?,
                (None, Some(a)) => alpha_of(a)?,
                _ => 1.0,
            };
            Some(hsl(h, pct(parts.get(1)?)?, pct(parts.get(2)?)?, a))
        }
        _ => None,
    }
}

pub fn hsl(h: f32, s: f32, l: f32, a: f32) -> Color {
    let h = h.rem_euclid(360.0) / 360.0;
    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let p = 2.0 * l - q;
    let f = |t: f32| {
        let t = t.rem_euclid(1.0);
        if t < 1.0 / 6.0 {
            p + (q - p) * 6.0 * t
        } else if t < 0.5 {
            q
        } else if t < 2.0 / 3.0 {
            p + (q - p) * (2.0 / 3.0 - t) * 6.0
        } else {
            p
        }
    };
    Color { r: f(h + 1.0 / 3.0), g: f(h), b: f(h - 1.0 / 3.0), a }
}

/// `text-shadow` / `box-shadow` entry.
#[derive(Debug, Clone, Copy)]
pub struct Shadow {
    pub x: Len,
    pub y: Len,
    pub blur: Len,
    pub spread: Len,
    pub color: ColorRef,
}

pub fn parse_shadows(s: &str) -> Vec<Shadow> {
    if s.trim() == "none" || s.trim().is_empty() {
        return Vec::new();
    }
    split_top(s, ',')
        .into_iter()
        .filter_map(|one| {
            let mut lens = Vec::new();
            let mut color = ColorRef::Current;
            for t in tokens(one) {
                if t == "inset" {
                    return None;
                }
                if let Some(l) = parse_len(t) {
                    lens.push(l);
                } else if let Some(c) = parse_color_ref(t) {
                    color = c;
                }
            }
            let at = |i: usize| lens.get(i).copied().unwrap_or(Len::Px(0.0));
            Some(Shadow { x: at(0), y: at(1), blur: at(2), spread: at(3), color })
        })
        .collect()
}

/// A gradient colour stop; `pos` is filled in by [`Stops::resolve`].
#[derive(Debug, Clone, Copy)]
pub struct Stop {
    pub color: Color,
    pub pos: Option<Len>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RadialSize {
    FarthestCorner,
    FarthestSide,
    ClosestSide,
    ClosestCorner,
    Circle(Len),
    Ellipse(Len, Len),
}

#[derive(Debug, Clone)]
pub enum Image {
    Linear { angle: f32, stops: Vec<Stop>, repeating: bool },
    Radial { circle: bool, size: RadialSize, at: (Len, Len), stops: Vec<Stop>, repeating: bool },
    /// Angles in the stops are given as `Len::Px(degrees)`.
    Conic { from: f32, at: (Len, Len), stops: Vec<Stop>, repeating: bool },
    Url(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Size {
    Auto,
    Cover,
    Contain,
    Explicit(Option<Len>, Option<Len>),
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub image: Image,
    pub pos: (Len, Len),
    pub size: Size,
    pub repeat: bool,
}

/// Parses a `background` shorthand list. Returns the layers top to bottom plus the background colour.
pub fn parse_background(s: &str) -> (Vec<Layer>, Option<Color>) {
    let mut layers = Vec::new();
    let mut color = None;
    if s.trim() == "none" {
        return (layers, None);
    }
    for part in split_top(s, ',') {
        let mut image = None;
        let mut pos_tokens: Vec<&str> = Vec::new();
        let mut size_tokens: Vec<&str> = Vec::new();
        let mut after_slash = false;
        let mut repeat = None;
        for t in tokens(part) {
            if t == "/" {
                after_slash = true;
                continue;
            }
            if let Some(img) = parse_image(t) {
                image = Some(img);
            } else if t == "no-repeat" {
                repeat = Some(false);
            } else if t == "repeat" {
                repeat = Some(true);
            } else if let Some(c) = parse_color(t) {
                color = Some(c);
            } else if after_slash {
                size_tokens.push(t);
            } else if let Some((p, s)) = t.split_once('/') {
                // "center/cover" without spaces.
                if !p.is_empty() {
                    pos_tokens.push(p);
                }
                if !s.is_empty() {
                    size_tokens.push(s);
                }
                after_slash = true;
            } else {
                pos_tokens.push(t);
            }
        }
        let Some(image) = image else { continue };
        let size = match size_tokens.as_slice() {
            [] => Size::Auto,
            ["cover"] => Size::Cover,
            ["contain"] => Size::Contain,
            [a] => Size::Explicit(parse_len(a), None),
            [a, b, ..] => Size::Explicit(parse_len(a), parse_len(b)),
        };
        let is_gradient = !matches!(image, Image::Url(_));
        layers.push(Layer { image, pos: parse_position(&pos_tokens), size, repeat: repeat.unwrap_or(!is_gradient) });
    }
    (layers, color)
}

/// `background-position` / `at …` tokens to x/y lengths (percentages of the free space).
pub fn parse_position(t: &[&str]) -> (Len, Len) {
    let kw = |s: &str| match s {
        "left" | "top" => Some(Len::Pct(0.0)),
        "center" => Some(Len::Pct(50.0)),
        "right" | "bottom" => Some(Len::Pct(100.0)),
        _ => parse_len(s),
    };
    match t {
        [] => (Len::Pct(0.0), Len::Pct(0.0)),
        [one] => match *one {
            "top" | "bottom" => (Len::Pct(50.0), kw(one).unwrap()),
            _ => (kw(one).unwrap_or(Len::Pct(50.0)), Len::Pct(50.0)),
        },
        [a, b, ..] => {
            // "top right" order swap.
            if matches!(*a, "top" | "bottom") || matches!(*b, "left" | "right") {
                (kw(b).unwrap_or(Len::Pct(50.0)), kw(a).unwrap_or(Len::Pct(50.0)))
            } else {
                (kw(a).unwrap_or(Len::Pct(50.0)), kw(b).unwrap_or(Len::Pct(50.0)))
            }
        }
    }
}

fn parse_stops(args: &[&str]) -> Vec<Stop> {
    let mut stops = Vec::new();
    for a in args {
        let t = tokens(a);
        let Some(color) = t.first().and_then(|c| parse_color(c)) else { continue };
        let positions: Vec<Len> = t[1..].iter().filter_map(|p| parse_len(p).or_else(|| parse_angle(p).map(Len::Px))).collect();
        match positions.as_slice() {
            [] => stops.push(Stop { color, pos: None }),
            [p] => stops.push(Stop { color, pos: Some(*p) }),
            [p, q, ..] => {
                stops.push(Stop { color, pos: Some(*p) });
                stops.push(Stop { color, pos: Some(*q) });
            }
        }
    }
    stops
}

fn parse_image(s: &str) -> Option<Image> {
    let open = s.find('(')?;
    let name = &s[..open];
    let inner = s[open + 1..].strip_suffix(')')?;
    match name {
        "url" => {
            let url = inner.trim().trim_matches(|c| c == '"' || c == '\'');
            Some(Image::Url(url.to_string()))
        }
        "linear-gradient" | "repeating-linear-gradient" => {
            let args = split_top(inner, ',');
            let (angle, rest) = match args.first().map(|a| a.trim()) {
                Some(a) if parse_angle(a).is_some() && !a.starts_with('#') => (parse_angle(a).unwrap(), &args[1..]),
                Some(a) if a.starts_with("to ") => {
                    let angle = match &a[3..] {
                        "top" => 0.0,
                        "right" => 90.0,
                        "bottom" => 180.0,
                        "left" => 270.0,
                        "bottom right" | "right bottom" => 135.0,
                        "top right" | "right top" => 45.0,
                        "bottom left" | "left bottom" => 225.0,
                        _ => 315.0,
                    };
                    (angle, &args[1..])
                }
                _ => (180.0, &args[..]),
            };
            Some(Image::Linear { angle, stops: parse_stops(rest), repeating: name.starts_with("repeating") })
        }
        "radial-gradient" | "repeating-radial-gradient" => {
            let args = split_top(inner, ',');
            let first = args.first()?;
            let is_stop = tokens(first).first().is_some_and(|t| parse_color(t).is_some());
            let (mut circle, mut size, mut at) = (false, RadialSize::FarthestCorner, (Len::Pct(50.0), Len::Pct(50.0)));
            let rest = if is_stop {
                &args[..]
            } else {
                let t = tokens(first);
                let at_index = t.iter().position(|x| *x == "at");
                let shape = &t[..at_index.unwrap_or(t.len())];
                let lens: Vec<Len> = shape.iter().filter_map(|x| parse_len(x)).collect();
                circle = shape.contains(&"circle") || (lens.len() == 1 && !shape.contains(&"ellipse"));
                size = match (shape.iter().find(|x| x.contains('-')), lens.as_slice()) {
                    (Some(&"closest-side"), _) => RadialSize::ClosestSide,
                    (Some(&"farthest-side"), _) => RadialSize::FarthestSide,
                    (Some(&"closest-corner"), _) => RadialSize::ClosestCorner,
                    (_, [r]) => RadialSize::Circle(*r),
                    (_, [rx, ry, ..]) => RadialSize::Ellipse(*rx, *ry),
                    _ => RadialSize::FarthestCorner,
                };
                if let Some(i) = at_index {
                    at = parse_position(&t[i + 1..]);
                }
                &args[1..]
            };
            Some(Image::Radial { circle, size, at, stops: parse_stops(rest), repeating: name.starts_with("repeating") })
        }
        "conic-gradient" | "repeating-conic-gradient" => {
            let args = split_top(inner, ',');
            let first = args.first()?;
            let is_stop = tokens(first).first().is_some_and(|t| parse_color(t).is_some());
            let (mut from, mut at) = (0.0, (Len::Pct(50.0), Len::Pct(50.0)));
            let rest = if is_stop {
                &args[..]
            } else {
                let t = tokens(first);
                if let Some(i) = t.iter().position(|x| *x == "from") {
                    from = t.get(i + 1).and_then(|a| parse_angle(a)).unwrap_or(0.0);
                }
                if let Some(i) = t.iter().position(|x| *x == "at") {
                    at = parse_position(&t[i + 1..]);
                }
                &args[1..]
            };
            Some(Image::Conic { from, at, stops: parse_stops(rest), repeating: name.starts_with("repeating") })
        }
        _ => None,
    }
}

/// Resolved stop list along a gradient line of length `len` (px or degrees), positions 0–1 of `len`
/// (beyond for repeating); CSS fix-up rules for missing and decreasing positions.
pub fn resolve_stops(stops: &[Stop], len: f32, u: &Units) -> Vec<(Color, f32)> {
    if stops.is_empty() {
        return Vec::new();
    }
    let mut pos: Vec<Option<f32>> = stops.iter().map(|s| s.pos.map(|p| p.px(u, len) / len.max(1e-6))).collect();
    if pos[0].is_none() {
        pos[0] = Some(0.0);
    }
    let last = pos.len() - 1;
    if pos[last].is_none() {
        pos[last] = Some(1.0);
    }
    let mut max = f32::MIN;
    for p in pos.iter_mut().flatten() {
        if *p < max {
            *p = max;
        }
        max = *p;
    }
    let mut i = 0;
    while i < pos.len() {
        if pos[i].is_none() {
            let start = i - 1;
            let mut end = i;
            while pos[end].is_none() {
                end += 1;
            }
            let (a, b) = (pos[start].unwrap(), pos[end].unwrap());
            for k in start + 1..end {
                pos[k] = Some(a + (b - a) * (k - start) as f32 / (end - start) as f32);
            }
            i = end;
        }
        i += 1;
    }
    stops.iter().zip(pos).map(|(s, p)| (s.color, p.unwrap())).collect()
}

/// Colour at position `p`, interpolated in premultiplied space like browsers. Returns premultiplied RGBA.
pub fn sample(stops: &[(Color, f32)], p: f32, repeating: bool) -> [f32; 4] {
    let pre = |c: Color| [c.r * c.a, c.g * c.a, c.b * c.a, c.a];
    if stops.is_empty() {
        return [0.0; 4];
    }
    let mut p = p;
    if repeating {
        let (a, b) = (stops[0].1, stops[stops.len() - 1].1);
        if b > a {
            p = a + (p - a).rem_euclid(b - a);
        }
    }
    if p <= stops[0].1 {
        return pre(stops[0].0);
    }
    for pair in stops.windows(2) {
        let ((ca, pa), (cb, pb)) = (pair[0], pair[1]);
        if p <= pb {
            let k = if pb > pa { (p - pa) / (pb - pa) } else { 1.0 };
            let (x, y) = (pre(ca), pre(cb));
            return [x[0] + (y[0] - x[0]) * k, x[1] + (y[1] - x[1]) * k, x[2] + (y[2] - x[2]) * k, x[3] + (y[3] - x[3]) * k];
        }
    }
    pre(stops[stops.len() - 1].0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colours_and_shadows() {
        let c = parse_color("rgb(255 170 120 / 0.85)").unwrap();
        assert!((c.g - 170.0 / 255.0).abs() < 1e-4 && (c.a - 0.85).abs() < 1e-4);
        assert_eq!(parse_color("transparent").unwrap().a, 0.0);
        let s = parse_shadows("0 0 0.3em rgb(255 170 120 / 0.85), 0 0.09em 0 #1a1033");
        assert_eq!(s.len(), 2);
        assert_eq!(s[1].y, Len::Em(0.09));
        assert_eq!(parse_shadows("0 0 1.2vmin currentcolor")[0].color, ColorRef::Current);
    }

    #[test]
    fn backgrounds() {
        let (layers, color) = parse_background(
            "url(\"data:image/svg+xml,%3Csvg%3E\") bottom / 100% 16% no-repeat, radial-gradient(circle at 50% 92%, #ffd27a 0 7%, transparent 30%), linear-gradient(#211c42 0%, #e8706a 100%)",
        );
        assert_eq!(layers.len(), 3);
        assert!(color.is_none());
        assert_eq!(layers[0].pos, (Len::Pct(50.0), Len::Pct(100.0)));
        assert_eq!(layers[0].size, Size::Explicit(Some(Len::Pct(100.0)), Some(Len::Pct(16.0))));
        match &layers[1].image {
            Image::Radial { circle, stops, .. } => assert!(*circle && stops.len() == 3),
            _ => panic!(),
        }
        let (layers, color) = parse_background("#111 url('./assets/themes/formrausch/pattern.jpg')");
        assert_eq!(layers.len(), 1);
        assert!(color.is_some() && layers[0].repeat);
        match &parse_background("radial-gradient(ellipse 3% 7% at 80% 92%, rgb(240 235 200 / 0.3), transparent)").0[0].image {
            Image::Radial { size: RadialSize::Ellipse(..), .. } => {}
            other => panic!("{other:?}"),
        }
        match &parse_background("repeating-conic-gradient(from 200deg at 100% 0%, rgb(255 255 255 / 0.12) 0 4deg, transparent 4deg 9deg)").0[0].image {
            Image::Conic { from, stops, repeating, .. } => assert!(*from == 200.0 && stops.len() == 4 && *repeating),
            other => panic!("{other:?}"),
        }
    }
}
