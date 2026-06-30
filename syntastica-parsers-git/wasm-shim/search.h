#ifndef SYNTASTICA_WASM_SHIM_SEARCH_H_
#define SYNTASTICA_WASM_SHIM_SEARCH_H_

// tree-sitter-language's wasm sysroot has no `search.h`, so `#include <search.h>`
// fails outright. lalrpop's scanner includes it but never uses any of its symbols
// (it rolls its own `binsearch` precisely because libc `bsearch` is unavailable in
// wasm). An empty stub satisfies the include without pulling in libc surface.

#endif // SYNTASTICA_WASM_SHIM_SEARCH_H_
