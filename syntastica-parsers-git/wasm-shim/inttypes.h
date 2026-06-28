#ifndef SYNTASTICA_WASM_SHIM_INTTYPES_H_
#define SYNTASTICA_WASM_SHIM_INTTYPES_H_

// tree-sitter-language's wasm `inttypes.h` only defines `PRId32`; parser scanners
// reference the wider fixed-width format-macro set. wasm32 is ILP32, so 32-bit
// types map to `int`/`unsigned` and 64-bit types to `long long`.
#include <stdint.h>

#define PRId8  "d"
#define PRIi8  "i"
#define PRIu8  "u"
#define PRIo8  "o"
#define PRIx8  "x"
#define PRIX8  "X"

#define PRId16 "d"
#define PRIi16 "i"
#define PRIu16 "u"
#define PRIo16 "o"
#define PRIx16 "x"
#define PRIX16 "X"

#define PRId32 "d"
#define PRIi32 "i"
#define PRIu32 "u"
#define PRIo32 "o"
#define PRIx32 "x"
#define PRIX32 "X"

#define PRId64 "lld"
#define PRIi64 "lli"
#define PRIu64 "llu"
#define PRIo64 "llo"
#define PRIx64 "llx"
#define PRIX64 "llX"

#endif // SYNTASTICA_WASM_SHIM_INTTYPES_H_
