#![doc = include_str!("../README.md")]
//!
#![cfg_attr(
    feature = "docs",
    cfg_attr(doc, doc = ::document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#))
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(rust_2018_idioms)]
#![deny(missing_docs)]

verdant_macros::parsers_ffi!();

/// Implementations of the libc functions that tree-sitter parser scanners link
/// against on `wasm32-unknown-unknown` but that nothing else in the link provides.
///
/// The `tree-sitter` runtime build compiles tree-sitter-language's stdio/stdlib/string
/// sources (`malloc`, `free`, `strncpy`, `mem*`, `abort`, ...), whose headers the
/// scanners also compile against. We must NOT redefine any of those here — doing so
/// produces duplicate-symbol link errors. Only the few stragglers below are left for us.
#[cfg(all(
    target_arch = "wasm32",
    target_vendor = "unknown",
    target_os = "unknown",
    target_env = ""
))]
#[allow(non_camel_case_types)]
mod wasm_c_bridge {
    use std::ffi::{c_void, CStr};

    type wint_t = u32;
    type size_t = usize;
    type c_char = i8;
    type int = i32;

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

    // Narrow (`int`-based) ctype family. The `wasm-shim/ctype.h` header provides
    // these as `static inline` for scanners that `#include <ctype.h>`, but a scanner
    // that calls e.g. `isdigit` without including ctype.h gets an implicit declaration
    // and emits an *external* reference, which is then undefined at link. Provide real
    // symbols here (the same way the `isw*` family above is handled) so those callers
    // resolve; wasm-ld garbage-collects any that go unreferenced. ASCII semantics,
    // matching `wasm-shim/ctype.h`.
    fn as_byte(c: int) -> Option<u8> {
        u8::try_from(c).ok()
    }

    #[no_mangle]
    extern "C" fn isascii(c: int) -> int {
        ((c as u32) < 128) as int
    }

    #[no_mangle]
    extern "C" fn isdigit(c: int) -> int {
        as_byte(c).is_some_and(|c| c.is_ascii_digit()) as int
    }

    #[no_mangle]
    extern "C" fn isalpha(c: int) -> int {
        as_byte(c).is_some_and(|c| c.is_ascii_alphabetic()) as int
    }

    #[no_mangle]
    extern "C" fn isalnum(c: int) -> int {
        as_byte(c).is_some_and(|c| c.is_ascii_alphanumeric()) as int
    }

    #[no_mangle]
    extern "C" fn isxdigit(c: int) -> int {
        as_byte(c).is_some_and(|c| c.is_ascii_hexdigit()) as int
    }

    #[no_mangle]
    extern "C" fn isspace(c: int) -> int {
        // space or 0x09..=0x0d (\t \n \v \f \r); note `is_ascii_whitespace` omits \v.
        as_byte(c).is_some_and(|c| c == b' ' || (0x09..=0x0d).contains(&c)) as int
    }

    #[no_mangle]
    extern "C" fn isblank(c: int) -> int {
        as_byte(c).is_some_and(|c| c == b' ' || c == b'\t') as int
    }

    #[no_mangle]
    extern "C" fn isupper(c: int) -> int {
        as_byte(c).is_some_and(|c| c.is_ascii_uppercase()) as int
    }

    #[no_mangle]
    extern "C" fn islower(c: int) -> int {
        as_byte(c).is_some_and(|c| c.is_ascii_lowercase()) as int
    }

    #[no_mangle]
    extern "C" fn iscntrl(c: int) -> int {
        as_byte(c).is_some_and(|c| c.is_ascii_control()) as int
    }

    #[no_mangle]
    extern "C" fn isgraph(c: int) -> int {
        as_byte(c).is_some_and(|c| c.is_ascii_graphic()) as int
    }

    #[no_mangle]
    extern "C" fn isprint(c: int) -> int {
        as_byte(c).is_some_and(|c| c.is_ascii_graphic() || c == b' ') as int
    }

    #[no_mangle]
    extern "C" fn ispunct(c: int) -> int {
        as_byte(c).is_some_and(|c| c.is_ascii_punctuation()) as int
    }

    #[no_mangle]
    extern "C" fn tolower(c: int) -> int {
        match as_byte(c) {
            Some(c) => c.to_ascii_lowercase() as int,
            None => c,
        }
    }

    #[no_mangle]
    extern "C" fn toupper(c: int) -> int {
        match as_byte(c) {
            Some(c) => c.to_ascii_uppercase() as int,
            None => c,
        }
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

    // haskell's scanner calls `setlocale(LC_ALL, "C.UTF-8")` and ignores the
    // result. wasm has no libc locale support, so this is a no-op that echoes the
    // requested locale back (a non-null return signals "accepted").
    #[no_mangle]
    extern "C" fn setlocale(_category: int, locale: *const c_char) -> *const c_char {
        locale
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
}
