#![doc = include_str!("../README.md")]
//!
#![cfg_attr(
    feature = "docs",
    cfg_attr(doc, doc = ::document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#))
)]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![warn(rust_2018_idioms)]
#![deny(missing_docs)]

syntastica_macros::parsers_ffi!();

/// Basic implementation of libc functions that tree-sitter parsers link against on
/// `wasm32-unknown-unknown`. The `tree-sitter-language` crate ships headers and stdio/stdlib/string
/// sources (consumed by the `tree-sitter` runtime's own build), but not implementations of the
/// wide-ctype or assertion helpers that parser scanners need. We provide those here in Rust.
#[cfg(all(
    target_arch = "wasm32",
    target_vendor = "unknown",
    target_os = "unknown",
    target_env = ""
))]
#[allow(non_camel_case_types)]
mod wasm_c_bridge {
    use std::{
        alloc::{GlobalAlloc, Layout, System},
        collections::HashMap,
        ffi::{c_void, CStr},
        sync::{LazyLock, Mutex},
    };

    type wint_t = u32;
    type size_t = usize;
    type c_char = i8;
    type int = i32;

    #[no_mangle]
    extern "C" fn abort() {
        panic!("program aborted");
    }

    #[no_mangle]
    extern "C" fn towupper(wc: wint_t) -> wint_t {
        let Some(char) = char::from_u32(wc) else {
            return wc;
        };
        let mut uppercase = char.to_uppercase();
        if uppercase.len() == 1 {
            uppercase.next().unwrap() as wint_t
        } else {
            wc
        }
    }

    #[no_mangle]
    extern "C" fn towlower(wc: wint_t) -> wint_t {
        let Some(char) = char::from_u32(wc) else {
            return wc;
        };
        let mut lowercase = char.to_lowercase();
        if lowercase.len() == 1 {
            lowercase.next().unwrap() as wint_t
        } else {
            wc
        }
    }

    #[no_mangle]
    extern "C" fn iswalnum(ch: wint_t) -> int {
        char::from_u32(ch).is_some_and(|ch| ch.is_alphanumeric()) as int
    }

    #[no_mangle]
    extern "C" fn iswalpha(ch: wint_t) -> int {
        char::from_u32(ch).is_some_and(|ch| ch.is_alphabetic()) as int
    }

    #[no_mangle]
    extern "C" fn iswblank(ch: wint_t) -> int {
        (ch == b' ' as wint_t || ch == b'\t' as wint_t) as int
    }

    #[no_mangle]
    extern "C" fn iswcntrl(ch: wint_t) -> int {
        char::from_u32(ch).is_some_and(|ch| ch.is_control()) as int
    }

    #[no_mangle]
    extern "C" fn iswdigit(ch: wint_t) -> int {
        char::from_u32(ch).is_some_and(|ch| ch.is_numeric()) as int
    }

    #[no_mangle]
    extern "C" fn iswgraph(ch: wint_t) -> int {
        char::from_u32(ch).is_some_and(|ch| ch.is_ascii_graphic()) as int
    }

    #[no_mangle]
    extern "C" fn iswlower(ch: wint_t) -> int {
        char::from_u32(ch).is_some_and(|ch| ch.is_lowercase()) as int
    }

    #[no_mangle]
    extern "C" fn iswprint(ch: wint_t) -> int {
        char::from_u32(ch).is_some_and(|ch| ch.is_ascii_graphic() || ch == ' ') as int
    }

    #[no_mangle]
    extern "C" fn iswpunct(ch: wint_t) -> int {
        char::from_u32(ch).is_some_and(|ch| ch.is_ascii_punctuation()) as int
    }

    #[no_mangle]
    extern "C" fn iswspace(ch: wint_t) -> int {
        char::from_u32(ch).is_some_and(|ch| ch.is_whitespace()) as int
    }

    #[no_mangle]
    extern "C" fn iswupper(ch: wint_t) -> int {
        char::from_u32(ch).is_some_and(|ch| ch.is_uppercase()) as int
    }

    #[no_mangle]
    extern "C" fn iswxdigit(ch: wint_t) -> int {
        char::from_u32(ch).is_some_and(|ch| ch.is_ascii_hexdigit()) as int
    }

    #[no_mangle]
    unsafe extern "C" fn __assert2(
        file: *const c_char,
        line: int,
        func: *const c_char,
        error: *const c_char,
    ) {
        let file = CStr::from_ptr(file).to_string_lossy();
        let func = CStr::from_ptr(func).to_string_lossy();
        let error = CStr::from_ptr(error).to_string_lossy();
        panic!("assertion failed in {file} on line {line} in {func}: {error}");
    }

    #[no_mangle]
    unsafe extern "C" fn strcmp(lhs: *const c_char, rhs: *const c_char) -> int {
        let lhs = CStr::from_ptr(lhs);
        let rhs = CStr::from_ptr(rhs);
        lhs.cmp(rhs) as int
    }

    #[no_mangle]
    unsafe extern "C" fn strncpy(
        dest: *mut c_char,
        src: *const c_char,
        count: size_t,
    ) -> *mut c_char {
        for i in 0..count {
            let cp = src.add(i).read();
            dest.add(i).write(cp)
        }
        dest
    }

    #[no_mangle]
    unsafe extern "C" fn memchr(ptr: *const c_void, ch: int, count: size_t) -> *mut c_void {
        let ptr = ptr as *const u8;
        let ch = ch as u8;
        for i in 0..count {
            if ptr.add(i).read() == ch {
                return ptr.add(i) as *mut _;
            }
        }
        std::ptr::null_mut()
    }

    static LAYOUTS: LazyLock<Mutex<HashMap<usize, Layout>>> = LazyLock::new(Default::default);
    const MIN_ALIGN: usize = 8;

    #[no_mangle]
    unsafe extern "C" fn malloc(size: size_t) -> *mut c_void {
        let layout = Layout::from_size_align_unchecked(size, MIN_ALIGN);
        let ptr = System.alloc(layout);
        if !ptr.is_null() {
            LAYOUTS.lock().unwrap().insert(ptr as usize, layout);
        }
        ptr as *mut _
    }

    #[no_mangle]
    unsafe extern "C" fn calloc(num: size_t, size: size_t) -> *mut c_void {
        let layout = Layout::from_size_align_unchecked(num * size, MIN_ALIGN);
        let ptr = System.alloc_zeroed(layout);
        if !ptr.is_null() {
            LAYOUTS.lock().unwrap().insert(ptr as usize, layout);
        }
        ptr as *mut _
    }

    #[no_mangle]
    unsafe extern "C" fn realloc(ptr: *mut c_void, new_size: size_t) -> *mut c_void {
        if ptr.is_null() {
            return malloc(new_size);
        }
        let layout = *LAYOUTS
            .lock()
            .unwrap()
            .get(&(ptr as usize))
            .unwrap_unchecked();
        let ptr = System.realloc(ptr as *mut _, layout, new_size);
        if !ptr.is_null() {
            LAYOUTS.lock().unwrap().insert(
                ptr as usize,
                Layout::from_size_align_unchecked(new_size, layout.align()),
            );
        }
        ptr as *mut _
    }

    #[no_mangle]
    unsafe extern "C" fn free(ptr: *mut c_void) {
        if ptr.is_null() {
            return;
        }
        let layout = LAYOUTS
            .lock()
            .unwrap()
            .remove(&(ptr as usize))
            .unwrap_unchecked();
        System.dealloc(ptr as *mut _, layout);
    }
}
