/* runtime.c — runtime nativo mínimo do caminho compilado (Fase 4 slice).
 *
 * Representação de valor tagged em uma palavra (intptr_t), conforme ADR-0003
 * (o MVP-slice compilado usa tagged pointers; o interpretador usa enum Value):
 *   - fixnum:   (n << 1) | 1           (bit 0 = 1)
 *   - ponteiro: baixo 3 bits = 000     (objetos alinhados a 8/16 do malloc)
 *   - imediatos especiais: NIL, TRUE, FALSE, EMPTY (bit 0 = 0, baixo3 != 0)
 *
 * ALOCAÇÃO: malloc sem coletor. O GC mark-sweep com shadow-stack (specs/
 * MEMORY_MODEL.md, ADR-0002) é a PRÓXIMA etapa; por ora a memória é recuperada
 * pelo SO na saída (adequado a CLIs curtos, documentado como interino).
 *
 * NÃO é o design final — é o primeiro runtime nativo com valores heap.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef intptr_t Value;

#define NIL    ((Value)2)
#define FALSEV ((Value)6)
#define TRUEV  ((Value)10)
#define EMPTY  ((Value)18) /* lista vazia () */

#define IS_FIX(v)  ((v) & 1)
#define MK_FIX(n)  ((Value)(((intptr_t)(n) << 1) | 1))
#define FIX(v)     ((intptr_t)(v) >> 1)
#define IS_PTR(v)  (((v) & 7) == 0)

enum { T_STR = 1, T_CONS = 2 };

typedef struct { uint8_t type; } Obj;
typedef struct { uint8_t type; size_t len; char *data; } Str;
typedef struct { uint8_t type; Value head; Value tail; } Cons;

static void *xalloc(size_t n) {
    void *p = malloc(n);
    if (!p) { fprintf(stderr, "erro: sem memória\n"); exit(1); }
    return p;
}

static void die(const char *msg) { fprintf(stderr, "erro: %s\n", msg); exit(1); }

static int obj_type(Value v) { return (IS_PTR(v) && v != 0) ? ((Obj *)v)->type : 0; }

/* ---- construtores ---- */
Value cljn_str_from(const char *p, long len) {
    Str *s = xalloc(sizeof(Str));
    s->type = T_STR;
    s->len = (size_t)len;
    s->data = (len > 0) ? xalloc((size_t)len) : NULL;
    if (len > 0) memcpy(s->data, p, (size_t)len);
    return (Value)s;
}
Value cljn_empty(void) { return EMPTY; }
Value cljn_cons(Value h, Value t) {
    Cons *c = xalloc(sizeof(Cons));
    c->type = T_CONS;
    c->head = h;
    c->tail = t;
    return (Value)c;
}

/* ---- aritmética (fixnum, com checagem de overflow) ---- */
static intptr_t need_fix(Value v, const char *op) {
    if (!IS_FIX(v)) { fprintf(stderr, "erro: argumento não-numérico em %s\n", op); exit(1); }
    return FIX(v);
}
Value cljn_add(Value a, Value b) {
    intptr_t r;
    if (__builtin_add_overflow(need_fix(a, "+"), need_fix(b, "+"), &r)) die("overflow em +");
    return MK_FIX(r);
}
Value cljn_sub(Value a, Value b) {
    intptr_t r;
    if (__builtin_sub_overflow(need_fix(a, "-"), need_fix(b, "-"), &r)) die("overflow em -");
    return MK_FIX(r);
}
Value cljn_mul(Value a, Value b) {
    intptr_t r;
    if (__builtin_mul_overflow(need_fix(a, "*"), need_fix(b, "*"), &r)) die("overflow em *");
    return MK_FIX(r);
}
Value cljn_quot(Value a, Value b) {
    intptr_t y = need_fix(b, "quot");
    if (y == 0) die("divisão por zero");
    return MK_FIX(need_fix(a, "quot") / y);
}
Value cljn_mod(Value a, Value b) {
    intptr_t y = need_fix(b, "mod");
    if (y == 0) die("divisão por zero");
    intptr_t x = need_fix(a, "mod"), r = x % y;
    if (r != 0 && ((r < 0) != (y < 0))) r += y; /* mod euclidiano ao sinal do divisor */
    return MK_FIX(r);
}
Value cljn_inc(Value a) { return cljn_add(a, MK_FIX(1)); }
Value cljn_dec(Value a) { return cljn_sub(a, MK_FIX(1)); }

/* ---- comparações (retornam TRUE/FALSE) ---- */
static Value b2v(int b) { return b ? TRUEV : FALSEV; }
Value cljn_lt(Value a, Value b) { return b2v(need_fix(a, "<") < need_fix(b, "<")); }
Value cljn_le(Value a, Value b) { return b2v(need_fix(a, "<=") <= need_fix(b, "<=")); }
Value cljn_gt(Value a, Value b) { return b2v(need_fix(a, ">") > need_fix(b, ">")); }
Value cljn_ge(Value a, Value b) { return b2v(need_fix(a, ">=") >= need_fix(b, ">=")); }

int cljn_equal_raw(Value a, Value b) {
    if (a == b) return 1;
    int ta = obj_type(a), tb = obj_type(b);
    if (ta == T_STR && tb == T_STR) {
        Str *x = (Str *)a, *y = (Str *)b;
        return x->len == y->len && memcmp(x->data, y->data, x->len) == 0;
    }
    if (ta == T_CONS && tb == T_CONS) {
        return cljn_equal_raw(((Cons *)a)->head, ((Cons *)b)->head) &&
               cljn_equal_raw(((Cons *)a)->tail, ((Cons *)b)->tail);
    }
    return 0;
}
Value cljn_eq(Value a, Value b) { return b2v(cljn_equal_raw(a, b)); }

/* ---- predicados / seq ---- */
int cljn_truthy(Value v) { return (v != NIL && v != FALSEV) ? 1 : 0; }
Value cljn_not(Value v) { return b2v(!cljn_truthy(v)); }
Value cljn_nilp(Value v) { return b2v(v == NIL); }
Value cljn_emptyp(Value v) {
    if (v == EMPTY || v == NIL) return TRUEV;
    if (obj_type(v) == T_STR) return b2v(((Str *)v)->len == 0);
    return FALSEV;
}
Value cljn_first(Value v) {
    if (v == EMPTY || v == NIL) return NIL;
    if (obj_type(v) == T_CONS) return ((Cons *)v)->head;
    die("first: não é uma sequência");
    return NIL;
}
Value cljn_rest(Value v) {
    if (v == EMPTY || v == NIL) return EMPTY;
    if (obj_type(v) == T_CONS) return ((Cons *)v)->tail;
    die("rest: não é uma sequência");
    return EMPTY;
}
Value cljn_count(Value v) {
    long n = 0;
    if (obj_type(v) == T_STR) return MK_FIX((long)((Str *)v)->len);
    while (v != EMPTY && v != NIL && obj_type(v) == T_CONS) {
        n++;
        v = ((Cons *)v)->tail;
    }
    return MK_FIX(n);
}

/* ---- impressão / str ---- */
typedef struct { char *p; size_t len, cap; } SB;
static void sb_init(SB *b) { b->cap = 32; b->len = 0; b->p = xalloc(b->cap); }
static void sb_putc(SB *b, char c) {
    if (b->len + 1 > b->cap) { b->cap *= 2; b->p = realloc(b->p, b->cap); if (!b->p) die("sem memória"); }
    b->p[b->len++] = c;
}
static void sb_write(SB *b, const char *s, size_t n) { for (size_t i = 0; i < n; i++) sb_putc(b, s[i]); }
static void sb_str(SB *b, const char *s) { sb_write(b, s, strlen(s)); }

/* for_str: str-mode (nil→"") vs print-mode (nil→"nil"). */
static void write_val(SB *b, Value v, int for_str) {
    if (IS_FIX(v)) {
        char buf[32];
        int n = snprintf(buf, sizeof buf, "%ld", (long)FIX(v));
        sb_write(b, buf, (size_t)n);
        return;
    }
    if (v == NIL) { if (!for_str) sb_str(b, "nil"); return; }
    if (v == TRUEV) { sb_str(b, "true"); return; }
    if (v == FALSEV) { sb_str(b, "false"); return; }
    if (v == EMPTY) { sb_str(b, "()"); return; }
    switch (obj_type(v)) {
        case T_STR: { Str *s = (Str *)v; sb_write(b, s->data, s->len); return; }
        case T_CONS: {
            sb_putc(b, '(');
            int first = 1;
            while (v != EMPTY && obj_type(v) == T_CONS) {
                if (!first) sb_putc(b, ' ');
                first = 0;
                write_val(b, ((Cons *)v)->head, for_str);
                v = ((Cons *)v)->tail;
            }
            sb_putc(b, ')');
            return;
        }
        default: sb_str(b, "#<obj>");
    }
}

void cljn_print(Value v) {
    SB b;
    sb_init(&b);
    write_val(&b, v, 0);
    fwrite(b.p, 1, b.len, stdout);
    free(b.p);
}
Value cljn_to_str(Value v) {
    SB b;
    sb_init(&b);
    write_val(&b, v, 1);
    Value r = cljn_str_from(b.p, (long)b.len);
    free(b.p);
    return r;
}
Value cljn_str_concat(Value a, Value b) {
    if (obj_type(a) != T_STR || obj_type(b) != T_STR) die("str_concat: esperava strings");
    Str *x = (Str *)a, *y = (Str *)b;
    Str *s = xalloc(sizeof(Str));
    s->type = T_STR;
    s->len = x->len + y->len;
    s->data = xalloc(s->len ? s->len : 1);
    memcpy(s->data, x->data, x->len);
    memcpy(s->data + x->len, y->data, y->len);
    return (Value)s;
}
void cljn_print_space(void) { fputc(' ', stdout); }
void cljn_print_newline(void) { fputc('\n', stdout); }
