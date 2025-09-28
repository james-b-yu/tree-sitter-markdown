#ifndef TREE_SITTER_WASM_CTYPE_H_
#define TREE_SITTER_WASM_CTYPE_H_

static inline int isprint(int c)
{
    return c >= 0x20 && c <= 0x7E;
}

static inline int isdigit(int c)
{
    return c >= '0' && c <= '9';
}

#endif // TREE_SITTER_WASM_CTYPE_H_