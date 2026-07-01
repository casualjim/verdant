#ifndef SYNTASTICA_WASM_SHIM_SYS_TYPES_H_
#define SYNTASTICA_WASM_SHIM_SYS_TYPES_H_

// tree-sitter-language's wasm sysroot has no `sys/types.h`. perl's `bsearch.h`
// includes it solely for `size_t`, which clang's freestanding `stddef.h` provides.
#include <stddef.h>

#endif // SYNTASTICA_WASM_SHIM_SYS_TYPES_H_
