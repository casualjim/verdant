#ifndef SYNTASTICA_WASM_SHIM_STDIO_H_
#define SYNTASTICA_WASM_SHIM_STDIO_H_

// tree-sitter-language's wasm `stdio.h` declares `fprintf`/`snprintf`/`vsnprintf`
// but not `printf`, which some parser scanners (e.g. awk) reference in debug
// helpers. Pull in the base header, then add the missing declaration.
#include_next <stdio.h>

int printf(const char *restrict format, ...);

#endif // SYNTASTICA_WASM_SHIM_STDIO_H_
