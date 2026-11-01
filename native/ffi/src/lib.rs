//! C interface for the macOS screensaver (and any other host that composites sprites itself).
//! See `qlock.h` for the declarations Swift imports.

use std::ffi::{CStr, c_char};
use std::sync::OnceLock;

use qlock_core::Library;
use qlock_render::{Engine, Instance, Options, theme::{Theme, ThemeFile}};

static LIBRARY: OnceLock<Library> = OnceLock::new();
static THEMES: OnceLock<ThemeFile> = OnceLock::new();

fn library() -> &'static Library {
    LIBRARY.get_or_init(|| serde_json::from_str(include_str!("../../data/locales.json")).expect("bundled locales"))
}

fn themes() -> &'static ThemeFile {
    THEMES.get_or_init(|| serde_json::from_str(include_str!("../../data/themes.json")).expect("bundled themes"))
}

unsafe fn text<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() { None } else { unsafe { CStr::from_ptr(ptr) }.to_str().ok() }
}

/// Creates a clock. Unknown locales fall back to German, unknown themes to Formrausch.
/// `flags`: 1 intro, 2 dots, 4 stretch, 8 letter effect, 16 transition, 32 background particles,
/// 64 seconds frame, 128 all words.
///
/// # Safety
/// String pointers must be null or valid NUL-terminated UTF-8.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qlock_engine_new(
    locale: *const c_char,
    theme: *const c_char,
    width: u32,
    height: u32,
    flags: u32,
    caption: *const c_char,
) -> *mut Engine {
    let lib = library();
    let locale = unsafe { text(locale) }.and_then(|id| lib.locale(id)).or_else(|| lib.locale("de")).unwrap().clone();
    let all = themes();
    // Renamed themes keep working from old settings.
    let requested = unsafe { text(theme) }.map(|id| if id == "katja" { "halloween" } else { id });
    let theme_id = requested.filter(|id| all.themes.contains_key(*id)).unwrap_or("formrausch");
    let theme = Theme::from_export(theme_id, &all.themes[theme_id]);
    let options = Options {
        intro: flags & 1 != 0,
        dots: flags & 2 != 0,
        stretch: flags & 4 != 0,
        effect: flags & 8 != 0,
        transition: flags & 16 != 0,
        ambient: flags & 32 != 0,
        progress: flags & 64 != 0,
        all: flags & 128 != 0,
    };
    let caption = unsafe { text(caption) };
    Box::into_raw(Box::new(Engine::new(locale, theme, options, width.max(1), height.max(1), caption)))
}

/// # Safety
/// `engine` must come from `qlock_engine_new` and not be used afterwards.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qlock_engine_free(engine: *mut Engine) {
    if !engine.is_null() {
        drop(unsafe { Box::from_raw(engine) });
    }
}

/// # Safety
/// `engine` must be a live engine.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qlock_sprite_count(engine: *const Engine) -> u32 {
    unsafe { &*engine }.sprites().len() as u32
}

/// Premultiplied RGBA, valid as long as the engine lives.
///
/// # Safety
/// `engine` must be a live engine, `width`/`height` writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qlock_sprite(engine: *const Engine, index: u32, width: *mut u32, height: *mut u32) -> *const u8 {
    let sprite = &unsafe { &*engine }.sprites()[index as usize];
    unsafe {
        *width = sprite.width;
        *height = sprite.height;
    }
    sprite.rgba.as_ptr()
}

/// Face rectangle and corner radius in pixels: x, y, width, height, radius.
///
/// # Safety
/// `engine` must be a live engine, `out` must hold 5 floats.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qlock_face(engine: *const Engine, out: *mut f32) {
    let e = unsafe { &*engine };
    let [x, y, w, h] = e.face();
    let values = [x, y, w, h, e.face_radius()];
    unsafe { std::ptr::copy_nonoverlapping(values.as_ptr(), out, 5) };
}

/// Instances for this frame; the list has the same length and order every frame, sprite ids may change
/// and new sprites may appear (check `qlock_sprite_count`).
///
/// # Safety
/// `engine` must be a live engine, `out` writable. The list is valid until the next call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qlock_update(engine: *mut Engine, hour: u32, minute: u32, second: f32, t: f64, out: *mut *const Instance) -> u32 {
    let list = unsafe { &mut *engine }.update(hour, minute, second, t);
    unsafe { *out = list.as_ptr() };
    list.len() as u32
}

/// Frees the sprite pixels after the host copied them into its own images. Sprites made later keep
/// their pixels until the next call.
///
/// # Safety
/// `engine` must be a live engine; earlier `qlock_sprite` pointers become invalid.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qlock_release_pixels(engine: *mut Engine) {
    unsafe { &mut *engine }.release_pixels();
}
