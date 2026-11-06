#ifndef SYNTASTICA_WASM_SHIM_WCTYPE_H_
#define SYNTASTICA_WASM_SHIM_WCTYPE_H_

// tree-sitter-language's wasm `wctype.h` defines `wint_t`, `wchar_t` is missing,
// and only `iswalnum`/`iswalpha`/`iswdigit`/`iswspace` are provided inline. The
// remaining wide-ctype functions are defined as symbols by this crate's
// `wasm_c_bridge` (lib.rs) but undeclared. Compose with the base header, add the
// `wchar_t` typedef (clang's canonical wide-char type), and declare the rest.
//
// The base header uses `bool`/`true`/`false` without including <stdbool.h>; it
// only compiled when the scanner happened to include it first. Pull in clang's
// freestanding <stdbool.h> here so the base header is self-sufficient under -std=c11.
#include <stdbool.h>
#include_next <wctype.h>

typedef __WCHAR_TYPE__ wchar_t;

int iswblank(wint_t wc);
int iswcntrl(wint_t wc);
int iswgraph(wint_t wc);
int iswlower(wint_t wc);
int iswprint(wint_t wc);
int iswpunct(wint_t wc);
int iswupper(wint_t wc);
int iswxdigit(wint_t wc);
wint_t towlower(wint_t wc);
wint_t towupper(wint_t wc);

#endif // SYNTASTICA_WASM_SHIM_WCTYPE_H_
