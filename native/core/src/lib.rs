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
    /// Words lit all day on one date (birthdays …).
    #[serde(default)]
    pub dated: Vec<Dated>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Dated {
    pub month: u32,
    pub day: u32,
    pub words: Vec<u16>,
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
        self.frame_on(hour, minute, intro, None)
    }

    /// Like [`Locale::frame`], plus the words of `date` (month, day).
    pub fn frame_on(&self, hour: u32, minute: u32, intro: bool, date: Option<(u32, u32)>) -> Frame {
        let m = (hour as usize % 24) * 60 + minute as usize % 60;
        let mut cells: Vec<u16> = Vec::new();
        let dated = self.dated.iter().filter(|d| Some((d.month, d.day)) == date).flat_map(|d| d.words.iter());
        for &word in self.frame_def(m, intro).words.iter().chain(dated) {
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
    fn date_words() {
        let lib = library();
        let hitchhiker = lib.locale("anhalter").unwrap();
        let plain = hitchhiker.frame(13, 45, true).cells.len();
        assert_eq!(hitchhiker.frame_on(13, 45, true, Some((5, 26))).cells.len(), plain);
        assert_eq!(hitchhiker.frame_on(13, 45, true, Some((5, 25))).cells.len(), plain + "HANDTUCH".len());
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

/// Easter Sunday (Gregorian, anonymous algorithm) as (month 1–12, day).
pub fn easter(year: i32) -> (u32, u32) {
    let a = year % 19;
    let b = year / 100;
    let c = year % 100;
    let (d, e) = (b / 4, b % 4);
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let (i, k) = (c / 4, c % 4);
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    (((h + l - 7 * m + 114) / 31) as u32, ((h + l - 7 * m + 114) % 31 + 1) as u32)
}

/// 0 = Sunday … 6 = Saturday (1970-01-01 was a Thursday).
fn weekday_of(days_since_epoch: i64) -> i64 {
    (days_since_epoch + 4).rem_euclid(7)
}

/// Days since 1970-01-01 of a civil date (Howard Hinnant's algorithm).
fn days(year: i32, month: u32, day: u32) -> i64 {
    let y = i64::from(year) - i64::from(month <= 2);
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = (i64::from(month) + 9) % 12;
    let doy = (153 * mp + 2) / 5 + i64::from(day) - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// The festive theme for a date, as the web app's `festival()` (src/core/festival.ts):
/// Karneval Weiberfastnacht–Veilchendienstag, Ostern Palmsonntag–Ostermontag, Halloween 24.–31.10.,
/// Weihnachten 1. Advent–26.12., Silvester 27.–31.12.
pub fn festival(year: i32, month: u32, day: u32) -> Option<&'static str> {
    let today = days(year, month, day);
    let (em, ed) = easter(year);
    let offset = today - days(year, em, ed);
    // Fourth Advent is the Sunday on or before 24 December (1970-01-01 was a Thursday).
    let eve = days(year, 12, 24);
    let weekday = (eve + 4).rem_euclid(7);
    let advent = eve - weekday - 21;
    if (-52..=-47).contains(&offset) {
        Some("karneval")
    } else if (-7..=1).contains(&offset) {
        Some("ostern")
    } else if offset == 39 {
        Some("vatertag")
    } else if month == 5 && i64::from(day) == 1 + (7 - weekday_of(days(year, 5, 1))) % 7 + 7 {
        Some("muttertag")
    } else if month == 10 && day >= 24 {
        Some("halloween")
    } else if month == 12 && day == 6 {
        Some("nikolaus")
    } else if month == 12 && day >= 27 {
        Some("silvester")
    } else if today >= advent && (month < 12 || day <= 26) {
        Some("weihnachten")
    } else {
        None
    }
}

#[cfg(test)]
mod festival_tests {
    use super::*;

    #[test]
    fn same_dates_as_the_web() {
        assert_eq!(easter(2026), (4, 5));
        assert_eq!(easter(2027), (3, 28));
        let cases: &[(i32, u32, u32, Option<&str>)] = &[
            (2026, 1, 1, None),
            (2026, 2, 12, Some("karneval")),
            (2026, 2, 17, Some("karneval")),
            (2026, 2, 18, None),
            (2026, 3, 29, Some("ostern")),
            (2026, 4, 6, Some("ostern")),
            (2026, 4, 7, None),
            (2026, 5, 10, Some("muttertag")),
            (2026, 5, 9, None),
            (2027, 5, 9, Some("muttertag")),
            (2026, 5, 14, Some("vatertag")),
            (2026, 5, 15, None),
            (2026, 12, 6, Some("nikolaus")),
            (2026, 12, 7, Some("weihnachten")),
            (2026, 10, 23, None),
            (2026, 10, 31, Some("halloween")),
            (2026, 11, 28, None),
            (2026, 11, 29, Some("weihnachten")),
            (2026, 12, 26, Some("weihnachten")),
            (2026, 12, 27, Some("silvester")),
            (2027, 11, 28, Some("weihnachten")),
            (2027, 11, 27, None),
            (2023, 12, 3, Some("weihnachten")),
            (2023, 12, 2, None),
        ];
        for &(y, m, d, expected) in cases {
            assert_eq!(festival(y, m, d), expected, "{y}-{m}-{d}");
        }
    }
}

/// Home for now: Koblenz. Later the hub sends the place.
pub const HOME: (f64, f64) = (50.36, 7.59);

/// Sun elevation in degrees at a Unix time for a place, as `sunElevation` in src/core/daylight.ts.
pub fn sun_elevation(unix: f64, lat: f64, lon: f64) -> f64 {
    let rad = core::f64::consts::PI / 180.0;
    let day = libm_floor(unix / 86_400.0) as i64;
    // Day of the year (1-based) and hours since midnight UTC.
    let (y, _, _) = civil(day);
    let day_of_year = (day - days(y, 1, 1) + 1) as f64;
    let hours = (unix - day as f64 * 86_400.0) / 3600.0;
    let g = 2.0 * core::f64::consts::PI / 365.0 * (day_of_year - 1.0 + (hours - 12.0) / 24.0);
    let eq_time = 229.18
        * (0.000075 + 0.001868 * cos(g) - 0.032077 * sin(g) - 0.014615 * cos(2.0 * g) - 0.040849 * sin(2.0 * g));
    let decl = 0.006918 - 0.399912 * cos(g) + 0.070257 * sin(g) - 0.006758 * cos(2.0 * g) + 0.000907 * sin(2.0 * g)
        - 0.002697 * cos(3.0 * g)
        + 0.00148 * sin(3.0 * g);
    let solar_minutes = hours * 60.0 + eq_time + 4.0 * lon;
    let hour_angle = (solar_minutes / 4.0 - 180.0) * rad;
    let cos_zenith = sin(lat * rad) * sin(decl) + cos(lat * rad) * cos(decl) * cos(hour_angle);
    90.0 - acos(cos_zenith.clamp(-1.0, 1.0)) / rad
}

/// Theme by the sun, as `daylight` in src/core/daylight.ts: night below −6°, sunrise/sunset while the
/// sun is below 8° in the morning/evening, the lagoon in between.
pub fn daylight(unix: f64, lat: f64, lon: f64) -> &'static str {
    let e = sun_elevation(unix, lat, lon);
    if e < -6.0 {
        "nightsurf"
    } else if e >= 8.0 {
        "lagoon"
    } else if sun_elevation(unix + 600.0, lat, lon) > e {
        "sunrise"
    } else {
        "sunset"
    }
}

fn civil(days_since_epoch: i64) -> (i32, u32, u32) {
    let z = days_since_epoch + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    ((yoe + era * 400 + i64::from(m <= 2)) as i32, m, d)
}

// no_std: float functions from libm.
fn libm_floor(x: f64) -> f64 {
    let t = x as i64 as f64;
    if t > x { t - 1.0 } else { t }
}
fn sin(x: f64) -> f64 {
    libm::sin(x)
}
fn cos(x: f64) -> f64 {
    libm::cos(x)
}
fn acos(x: f64) -> f64 {
    libm::acos(x)
}

#[cfg(test)]
mod daylight_tests {
    use super::*;

    fn unix(y: i32, mo: u32, d: u32, h: u32, mi: u32) -> f64 {
        (days(y, mo, d) * 86_400 + i64::from(h) * 3600 + i64::from(mi) * 60) as f64
    }

    #[test]
    fn same_answers_as_the_web() {
        let (lat, lon) = HOME;
        assert!(sun_elevation(unix(2026, 6, 21, 11, 30), lat, lon) > 60.0);
        assert!(sun_elevation(unix(2026, 12, 21, 11, 30), lat, lon) < 18.0);
        let cases = [
            (unix(2026, 11, 1, 2, 0), "nightsurf"),
            (unix(2026, 11, 1, 6, 30), "sunrise"),
            (unix(2026, 11, 1, 11, 0), "lagoon"),
            (unix(2026, 11, 1, 15, 45), "sunset"),
            (unix(2026, 6, 21, 3, 30), "sunrise"),
            (unix(2026, 6, 21, 19, 30), "sunset"),
            (unix(2026, 6, 21, 21, 0), "nightsurf"),
        ];
        for (t, expected) in cases {
            assert_eq!(daylight(t, lat, lon), expected, "{t}");
        }
    }
}
