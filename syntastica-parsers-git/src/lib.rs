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
    use std::ffi::CStr;

    type wint_t = u32;
    type c_char = i8;
    type int = i32;

    #[no_mangle]
    extern "C" fn iswcntrl(ch: wint_t) -> int {
        char::from_u32(ch).is_some_and(|ch| ch.is_control()) as int
    }

    #[no_mangle]
    extern "C" fn iswgraph(ch: wint_t) -> int {
        char::from_u32(ch).is_some_and(|ch| ch.is_ascii_graphic()) as int
    }

    #[no_mangle]
    extern "C" fn iswprint(ch: wint_t) -> int {
        char::from_u32(ch).is_some_and(|ch| ch.is_ascii_graphic() || ch == ' ') as int
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

    // haskell's scanner calls `setlocale(LC_ALL, "C.UTF-8")` and ignores the
    // result. wasm has no libc locale support, so this is a no-op that echoes the
    // requested locale back (a non-null return signals "accepted").
    #[no_mangle]
    extern "C" fn setlocale(_category: int, locale: *const c_char) -> *const c_char {
        locale
    }
}
