#ifndef TREE_SITTER_WASM_WCHAR_H_
#define TREE_SITTER_WASM_WCHAR_H_

#include <wctype.h>

static inline wint_t towlower(wint_t wc)
{
    if (wc >= 'A' && wc <= 'Z')
    {
        return wc + ('a' - 'A');
    }
    return wc;
}

#endif // TREE_SITTER_WASM_WCHAR_H_