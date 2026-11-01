//! Renders one frame to PNG, to compare the native renderer with the web app.
//! Usage: qlock-preview <locale> <theme.toml> <HH:MM> <seconds-since-start> <width> <height> <out.png> [stretch]
use qlock_render::{Engine, Options, theme::Theme};
use std::time::Instant;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() < 8 {
        eprintln!("usage: qlock-preview <locale> <theme.toml> <HH:MM> <t> <width> <height> <out.png> [stretch]");
        std::process::exit(2);
    }
    let lib: qlock_core::Library = serde_json::from_str(include_str!("../../data/locales.json")).unwrap();
    let locale = lib.locale(&a[1]).expect("locale").clone();
    let theme = Theme::from_toml(&std::fs::read_to_string(&a[2]).unwrap()).unwrap();
    let (h, m) = a[3].split_once(':').map(|(h, m)| (h.parse().unwrap(), m.parse().unwrap())).unwrap();
    let t: f64 = a[4].parse().unwrap();
    let (w, hh): (u32, u32) = (a[5].parse().unwrap(), a[6].parse().unwrap());
    let options = Options { intro: true, dots: true, stretch: a.get(8).is_some_and(|s| s == "stretch"), effect: true, transition: true, ambient: true };

    let started = Instant::now();
    let mut engine = Engine::new(locale, theme, options, w, hh, Some("So · 1. November"));
    let build = started.elapsed();
    engine.update(h, m, 0.0);
    engine.update(h, m, 0.7);
    let started = Instant::now();
    let instances = engine.update(h, m, t).to_vec();
    let frame = started.elapsed();
    let started = Instant::now();
    let image = engine.compose(&instances, w, hh);
    let compose = started.elapsed();
    image.save_png(&a[7]).unwrap();
    eprintln!(
        "{} sprites, {} instances; build {:?}, update {:?}, software compose {:?}",
        engine.sprites().len(), instances.len(), build, frame, compose
    );
}
