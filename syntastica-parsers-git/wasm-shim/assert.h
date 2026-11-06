#ifndef SYNTASTICA_WASM_SHIM_ASSERT_H_
#define SYNTASTICA_WASM_SHIM_ASSERT_H_

// tree-sitter-language's wasm `assert.h` defines `__assert_fail` as a *non-static*
// function in the header, so every scanner translation unit that includes <assert.h>
// emits a strong definition. A consumer linking two or more grammars into one wasm
// module then fails with `duplicate symbol: __assert_fail`. We fully replace that
// header here (no `include_next`) and give `__assert_fail` internal linkage so each
// translation unit gets its own copy. We also add C11 `static_assert`, which the
// base header lacks but some scanners (e.g. cpp) use.

#ifdef NDEBUG
#define assert(e) ((void)0)
#else
__attribute__((noreturn)) static void __assert_fail(const char *assertion,
                                                    const char *file,
                                                    unsigned line,
                                                    const char *function) {
    (void)assertion;
    (void)file;
    (void)line;
    (void)function;
    __builtin_trap();
}
#define assert(expression) \
  ((expression) ? (void)0 : __assert_fail(#expression, __FILE__, __LINE__, __func__))
#endif

#ifndef static_assert
#define static_assert _Static_assert
#endif

#endif // SYNTASTICA_WASM_SHIM_ASSERT_H_
