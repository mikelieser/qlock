//! Word clock logic for every renderer (macOS screensaver, ESP32, later WebAssembly).
//!
//! The languages live in TypeScript (`src/locales`) and are exported as data by
//! `native/export-locales.ts`: every minute of the day points at a frame of lit words. This crate only
//! looks things up, so all platforms show exactly what the web app shows.
#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use serde::Deserialize;

/// Minutes per day; the tables have one entry per minute.
pub const MINUTES: usize = 1440;

#[derive(Debug, Clone, Deserialize)]
pub struct Library {
    pub locales: Vec<Locale>,
}

impl Library {
    pub fn locale(&self, id: &str) -> Option<&Locale> {
        self.locales.iter().find(|l| l.id == id)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct FrameDef {
    /// Indices into [`Locale::words`], in reading order.
    pub words: Vec<u16>,
    /// Replaces the text derived from the words, e.g. for pictures.
    #[serde(default)]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Locale {
    pub id: String,
    pub name: String,
    pub lang: String,
    pub group: String,
    pub rows: usize,
    pub cols: usize,
    /// One letter per cell, row by row.
    pub chars: Vec<String>,
    /// Each word is a list of cell indices.
    pub words: Vec<Vec<u16>>,
    pub frames: Vec<FrameDef>,
    /// `[without intro, with intro]`, one frame index per minute of the day.
    pub minutes: [Vec<u16>; 2],
    /// Corner dots lit per minute of the day.
    pub dots: Vec<u8>,
}

/// What the clock shows at one minute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    /// Lit cells in reading order (first appearance), no duplicates.
    pub cells: Vec<u16>,
    /// Corner dots lit, 0–4.
    pub dots: u8,
}

impl Locale {
    pub fn cell_count(&self) -> usize {
        self.rows * self.cols
    }

    fn frame_def(&self, minute_of_day: usize, intro: bool) -> &FrameDef {
        let index = self.minutes[intro as usize][minute_of_day % MINUTES];
        &self.frames[index as usize]
    }

    /// Lit cells and dots for a local time.
    pub fn frame(&self, hour: u32, minute: u32, intro: bool) -> Frame {
        let m = (hour as usize % 24) * 60 + minute as usize % 60;
        let mut cells: Vec<u16> = Vec::new();
        for &word in &self.frame_def(m, intro).words {
            for &cell in &self.words[word as usize] {
                if !cells.contains(&cell) {
                    cells.push(cell);
                }
            }
        }
        Frame { cells, dots: self.dots[m] }
    }

    /// The sentence as text, like the web app's tab title.
    pub fn text(&self, hour: u32, minute: u32, intro: bool) -> String {
        let def = self.frame_def((hour as usize % 24) * 60 + minute as usize % 60, intro);
        if let Some(title) = &def.title {
            return title.clone();
        }
        let mut out = String::new();
        for (i, &word) in def.words.iter().enumerate() {
            if i > 0 {
                out.push(' ');
            }
            for &cell in &self.words[word as usize] {
                out.push_str(&self.chars[cell as usize]);
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    fn library() -> Library {
        serde_json::from_str(include_str!("../../data/locales.json")).unwrap()
    }

    #[test]
    fn german_sentences() {
        let lib = library();
        let de = lib.locale("de").unwrap();
        assert_eq!(de.text(15, 15, true), "ES IST VIERTEL NACH DREI");
        assert_eq!(de.text(15, 17, false), "VIERTEL NACH DREI");
        assert_eq!(de.text(1, 0, true), "ES IST EIN UHR");
        assert_eq!(de.text(23, 55, true), "ES IST FÜNF VOR ZWÖLF");
        assert_eq!(de.frame(15, 17, true).dots, 2);
    }

    #[test]
    fn dialects_and_pictures() {
        let lib = library();
        assert_eq!(lib.locale("kob").unwrap().text(2, 20, true), "ET ES ZÄHN VÜR HALWER DREI");
        assert_eq!(lib.locale("formrausch").unwrap().text(18, 1, true), "FEIERABEND ;)");
    }

    #[test]
    fn every_locale_is_complete() {
        for locale in library().locales {
            assert_eq!(locale.chars.len(), locale.cell_count(), "{}", locale.id);
            assert_eq!(locale.dots.len(), MINUTES, "{}", locale.id);
            for intro in [false, true] {
                for m in 0..MINUTES as u32 {
                    let frame = locale.frame(m / 60, m % 60, intro);
                    assert!(!frame.cells.is_empty(), "{} at {m}", locale.id);
                    assert!(frame.cells.iter().all(|&c| (c as usize) < locale.cell_count()));
                }
            }
        }
    }
}
