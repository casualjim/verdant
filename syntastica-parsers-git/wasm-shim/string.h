#ifndef SYNTASTICA_WASM_SHIM_STRING_H_
#define SYNTASTICA_WASM_SHIM_STRING_H_

// tree-sitter-language's wasm `string.h` declares `mem*`, `strlen`, `strncmp`.
// `strcmp` and `memchr` are defined by this crate's `wasm_c_bridge` (lib.rs) but
// undeclared; the remaining pure string helpers have no definition anywhere, so
// provide them as self-contained `static inline` (no external symbol required).
#include_next <string.h>

int strcmp(const char *lhs, const char *rhs);
void *memchr(const void *ptr, int ch, size_t count);

static inline char *strcpy(char *dst, const char *src) {
    char *d = dst;
    while ((*d++ = *src++)) {
    }
    return dst;
}

static inline char *strncpy(char *dst, const char *src, size_t n) {
    size_t i = 0;
    for (; i < n && src[i]; i++) {
        dst[i] = src[i];
    }
    for (; i < n; i++) {
        dst[i] = '\0';
    }
    return dst;
}

static inline char *strchr(const char *s, int c) {
    for (;; s++) {
        if (*s == (char)c) {
            return (char *)s;
        }
        if (!*s) {
            return (char *)0;
        }
    }
}

static inline char *strrchr(const char *s, int c) {
    const char *last = (const char *)0;
    for (;; s++) {
        if (*s == (char)c) {
            last = s;
        }
        if (!*s) {
            return (char *)last;
        }
    }
}

static inline char *strstr(const char *haystack, const char *needle) {
    if (!*needle) {
        return (char *)haystack;
    }
    for (; *haystack; haystack++) {
        const char *a = haystack;
        const char *b = needle;
        while (*a && *b && *a == *b) {
            a++;
            b++;
        }
        if (!*b) {
            return (char *)haystack;
        }
    }
    return (char *)0;
}

#endif // SYNTASTICA_WASM_SHIM_STRING_H_
