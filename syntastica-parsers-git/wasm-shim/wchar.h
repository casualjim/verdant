#ifndef SYNTASTICA_WASM_SHIM_WCHAR_H_
#define SYNTASTICA_WASM_SHIM_WCHAR_H_

// tree-sitter-language ships no <wchar.h>; some scanners (e.g. markdown) include it
// only for the wide-char typedefs. Provide the minimal set; `wint_t` matches the
// `typedef int wint_t` in tree-sitter-language's <wctype.h> (identical redefinition
// is permitted, so including both headers is safe).
typedef __WCHAR_TYPE__ wchar_t;
typedef int wint_t;

#endif // SYNTASTICA_WASM_SHIM_WCHAR_H_
