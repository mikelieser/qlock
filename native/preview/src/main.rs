//! Renders one frame to PNG, to compare the native renderer with the web app.
//! Usage: qlock-preview <locale> <theme-id> <HH:MM[:SS]> <t> <width> <height> <out.png> [all] [stretch] [effect]
//!        [transition] [ambient] [progress] [date] [nodots]
use qlock_render::{Engine, Options, theme::{Theme, ThemeFile}};
use std::time::Instant;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() < 8 {
        eprintln!("usage: qlock-preview <locale> <theme-id> <HH:MM[:SS]> <t> <width> <height> <out.png> [flags…]");
        std::process::exit(2);
    }
    let lib: qlock_core::Library = serde_json::from_str(include_str!("../../data/locales.json")).unwrap();
    let file: ThemeFile = serde_json::from_str(include_str!("../../data/themes.json")).unwrap();
    let locale = lib.locale(&a[1]).expect("locale").clone();
    let theme = Theme::from_export(&a[2], file.themes.get(&a[2]).expect("theme"));
    let parts: Vec<f32> = a[3].split(':').map(|p| p.parse().unwrap()).collect();
    let (h, m, s) = (parts[0] as u32, parts[1] as u32, parts.get(2).copied().unwrap_or(0.0));
    let t: f64 = a[4].parse().unwrap();
    let (w, hh): (u32, u32) = (a[5].parse().unwrap(), a[6].parse().unwrap());
    let flag = |f: &str| a[8..].iter().any(|x| x == f);
    let options = Options {
        intro: true,
        dots: !flag("nodots"),
        stretch: flag("stretch"),
        effect: flag("effect"),
        transition: flag("transition"),
        ambient: flag("ambient"),
        progress: flag("progress"),
        all: flag("all"),
    };
    let caption = flag("date").then_some("So · 1. November");

    let started = Instant::now();
    let mut engine = Engine::new(locale, theme, options, w, hh, caption);
    let build = started.elapsed();
    // Walk up to t in small steps, as a host would.
    let mut now = 0.0;
    while now < t {
        engine.update(h, m, s, now);
        now += 0.25;
    }
    let started = Instant::now();
    let instances = engine.update(h, m, s, t).to_vec();
    let frame = started.elapsed();
    let image = engine.compose(&instances, w, hh);
    image.save_png(&a[7]).unwrap();
    eprintln!("{} sprites, {} instances; build {:?}, update {:?}", engine.sprites().len(), instances.len(), build, frame);
}
