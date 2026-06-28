#ifndef SYNTASTICA_WASM_SHIM_ASSERT_H_
#define SYNTASTICA_WASM_SHIM_ASSERT_H_

// tree-sitter-language's wasm `assert.h` provides `assert` but not the C11
// `static_assert` macro, which some scanners (e.g. cpp) use. Compose and add it.
#include_next <assert.h>

#ifndef static_assert
#define static_assert _Static_assert
#endif

#endif // SYNTASTICA_WASM_SHIM_ASSERT_H_
