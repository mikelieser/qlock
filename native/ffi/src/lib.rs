//! C interface for the macOS screensaver (and any other host that composites sprites itself).
//! See `qlock.h` for the declarations Swift imports.

use std::ffi::{CStr, c_char};
use std::sync::OnceLock;

use qlock_core::Library;
use qlock_render::{Engine, Instance, Options, theme::Theme};

static LIBRARY: OnceLock<Library> = OnceLock::new();

fn library() -> &'static Library {
    LIBRARY.get_or_init(|| serde_json::from_str(include_str!("../../data/locales.json")).expect("bundled locales"))
}

/// Themes ported to the native renderer so far.
pub const THEMES: &[(&str, &str)] = &[
    ("formrausch", include_str!("../../themes/formrausch.toml")),
    ("sunset", include_str!("../../themes/sunset.toml")),
];

unsafe fn text<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() { None } else { unsafe { CStr::from_ptr(ptr) }.to_str().ok() }
}

/// Creates a clock. Unknown locales fall back to German, unknown themes to Formrausch.
/// `flags`: 1 intro, 2 dots, 4 stretch, 8 letter effect, 16 transition, 32 background particles.
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
    let theme_id = unsafe { text(theme) }.unwrap_or("formrausch");
    let source = THEMES.iter().find(|(id, _)| *id == theme_id).unwrap_or(&THEMES[0]).1;
    let theme = Theme::from_toml(source).expect("bundled theme");
    let options = Options {
        intro: flags & 1 != 0,
        dots: flags & 2 != 0,
        stretch: flags & 4 != 0,
        effect: flags & 8 != 0,
        transition: flags & 16 != 0,
        ambient: flags & 32 != 0,
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

/// Instances for this frame; the list has the same length and order every frame.
///
/// # Safety
/// `engine` must be a live engine, `out` writable. The list is valid until the next call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qlock_update(engine: *mut Engine, hour: u32, minute: u32, t: f64, out: *mut *const Instance) -> u32 {
    let list = unsafe { &mut *engine }.update(hour, minute, t);
    unsafe { *out = list.as_ptr() };
    list.len() as u32
}
