#include <stdint.h>
#include <stdio.h>
#include <string.h>

typedef intptr_t Value;

#define NIL ((Value)2)
#define MK_FIX(n) ((Value)(((uintptr_t)(intptr_t)(n) << 1) | 1u))
#define FIXNUM_MAX (((intptr_t)1 << 62) - 1)

Value cljn_add(Value left, Value right);
Value cljn_quot(Value left, Value right);
Value cljn_vec_empty(void);
Value cljn_nth(Value collection, Value index);
void cljn_check_arity(Value actual, Value expected);
Value cljn_throw(Value value);

int main(int argc, char **argv) {
    if (argc != 2) {
        fputs("uso: runtime_errors <cenário>\n", stderr);
        return 64;
    }

    if (strcmp(argv[1], "division-by-zero") == 0)
        (void)cljn_quot(MK_FIX(1), MK_FIX(0));
    else if (strcmp(argv[1], "fixnum-overflow") == 0)
        (void)cljn_add(MK_FIX(FIXNUM_MAX), MK_FIX(1));
    else if (strcmp(argv[1], "non-numeric") == 0)
        (void)cljn_add(NIL, MK_FIX(1));
    else if (strcmp(argv[1], "nth-out-of-bounds") == 0)
        (void)cljn_nth(cljn_vec_empty(), MK_FIX(0));
    else if (strcmp(argv[1], "wrong-arity") == 0)
        cljn_check_arity(1, 2);
    else if (strcmp(argv[1], "uncaught-throw") == 0)
        (void)cljn_throw(MK_FIX(9));
    else {
        fprintf(stderr, "cenário desconhecido: %s\n", argv[1]);
        return 64;
    }

    fputs("cenário de erro retornou inesperadamente\n", stderr);
    return 1;
}
