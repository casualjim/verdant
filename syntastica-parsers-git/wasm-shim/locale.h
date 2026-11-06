#ifndef SYNTASTICA_WASM_SHIM_LOCALE_H_
#define SYNTASTICA_WASM_SHIM_LOCALE_H_

// tree-sitter-language's wasm sysroot has no `locale.h`. haskell's scanner calls
// `setlocale(LC_ALL, "C.UTF-8")`. Provide the declaration and `LC_ALL`; the symbol
// is stubbed by the `wasm_c_bridge` module in this crate's lib.rs.
#define LC_ALL 6

char *setlocale(int category, const char *locale);

#endif // SYNTASTICA_WASM_SHIM_LOCALE_H_
