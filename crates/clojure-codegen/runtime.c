/* runtime.c — runtime nativo do caminho compilado, agora com GC mark-sweep
 * preciso e shadow-stack de roots (ADR-0002 / specs/MEMORY_MODEL.md).
 *
 * Valor tagged em uma palavra (ADR-0003, variante compilada):
 *   fixnum (n<<1)|1 ; ponteiro baixo3=000 ; imediatos NIL/TRUE/FALSE/EMPTY.
 *
 * GC: coletor tracing mark-sweep, preciso, NÃO-móvel, single-thread.
 *   - Roots: um shadow-stack (`gc_stack`) mantido pelo CÓDIGO GERADO — cada função
 *     reserva `local_count` slots (locais) e empurra/retira temporários em volta de
 *     alocações. O coletor varre [0, gc_sp) — nunca escaneia a pilha nativa.
 *   - Objetos têm header (mark + lista global de todos os objetos) para o sweep.
 *   - Gatilho: a cada N alocações; env CLJN_GC_STRESS=1 coleta a CADA alocação
 *     (usado nos testes para validar o rooting).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef intptr_t Value;

#define NIL    ((Value)2)
#define FALSEV ((Value)6)
#define TRUEV  ((Value)10)
#define EMPTY  ((Value)18)

#define IS_FIX(v)  ((v) & 1)
#define MK_FIX(n)  ((Value)(((intptr_t)(n) << 1) | 1))
#define FIX(v)     ((intptr_t)(v) >> 1)
#define IS_PTR(v)  (((v) & 7) == 0)

enum { T_STR = 1, T_CONS = 2 };

typedef struct Obj {
    uint8_t type;
    uint8_t mark;
    struct Obj *next_all; /* lista global de objetos, para o sweep */
} Obj;
typedef struct { Obj h; size_t len; char *data; } Str;
typedef struct { Obj h; Value head; Value tail; } Cons;

/* ---------- shadow-stack de roots ---------- */
#define GC_STACK_CAP (1u << 22) /* 4M slots */
static Value gc_stack[GC_STACK_CAP];
static size_t gc_sp = 0;

Value cljn_gc_enter(Value nslots) {
    size_t base = gc_sp;
    size_t n = (size_t)nslots;
    if (base + n > GC_STACK_CAP) { fprintf(stderr, "erro: overflow do shadow-stack de GC\n"); exit(1); }
    for (size_t i = 0; i < n; i++) gc_stack[base + i] = NIL; /* zera slots reservados */
    gc_sp = base + n;
    return (Value)base;
}
void cljn_gc_leave(Value base) { gc_sp = (size_t)base; }
void cljn_gc_push(Value v) {
    if (gc_sp >= GC_STACK_CAP) { fprintf(stderr, "erro: overflow do shadow-stack de GC\n"); exit(1); }
    gc_stack[gc_sp++] = v;
}
void cljn_gc_popn(Value n) { gc_sp -= (size_t)n; }
void cljn_gc_set(Value idx, Value v) { gc_stack[(size_t)idx] = v; }

/* ---------- heap + coletor ---------- */
static Obj *all_objs = NULL;
static size_t alloc_since_gc = 0;
static size_t gc_threshold = 100000;
static int gc_stress = -1;

static void gc_collect(void);

static void *xalloc(size_t n) {
    void *p = malloc(n);
    if (!p) { fprintf(stderr, "erro: sem memória\n"); exit(1); }
    return p;
}
static void die(const char *m) { fprintf(stderr, "erro: %s\n", m); exit(1); }
static int obj_type(Value v) { return (IS_PTR(v) && v != 0) ? ((Obj *)v)->type : 0; }

static int gc_off = -1;
static void maybe_gc(void) {
    if (gc_stress < 0) {
        const char *e = getenv("CLJN_GC_STRESS");
        gc_stress = (e && e[0] && e[0] != '0') ? 1 : 0;
        const char *o = getenv("CLJN_GC_OFF");
        gc_off = (o && o[0] && o[0] != '0') ? 1 : 0;
    }
    if (gc_off) return; /* diagnóstico: desliga o coletor (alocador puro) */
    if (gc_stress || alloc_since_gc >= gc_threshold) gc_collect();
}

static Obj *obj_alloc(size_t size, int type) {
    maybe_gc();
    Obj *o = xalloc(size);
    o->type = (uint8_t)type;
    o->mark = 0;
    o->next_all = all_objs;
    all_objs = o;
    alloc_since_gc++;
    return o;
}

static void gc_mark(Value v) {
    while (IS_PTR(v) && v != 0) {
        Obj *o = (Obj *)v;
        if (o->mark) return;
        o->mark = 1;
        if (o->type == T_CONS) {
            gc_mark(((Cons *)v)->head);
            v = ((Cons *)v)->tail; /* itera a cauda (não recursa) */
        } else {
            return; /* string: folha */
        }
    }
}

static void gc_sweep(void) {
    Obj **pp = &all_objs;
    while (*pp) {
        Obj *o = *pp;
        if (o->mark) {
            o->mark = 0;
            pp = &o->next_all;
        } else {
            *pp = o->next_all;
            if (o->type == T_STR) free(((Str *)o)->data);
            free(o);
        }
    }
}

static void gc_collect(void) {
    for (size_t i = 0; i < gc_sp; i++) gc_mark(gc_stack[i]);
    gc_sweep();
    alloc_since_gc = 0;
}

/* ---------- construtores ---------- */
Value cljn_str_from(const char *p, long len) {
    Str *s = (Str *)obj_alloc(sizeof(Str), T_STR);
    s->len = (size_t)len;
    s->data = (len > 0) ? xalloc((size_t)len) : NULL;
    if (len > 0) memcpy(s->data, p, (size_t)len);
    return (Value)s;
}
Value cljn_empty(void) { return EMPTY; }
Value cljn_cons(Value h, Value t) {
    /* h e t já estão no shadow-stack (empurrados pelo código gerado). */
    Cons *c = (Cons *)obj_alloc(sizeof(Cons), T_CONS);
    c->head = h;
    c->tail = t;
    return (Value)c;
}

/* ---------- aritmética ---------- */
static intptr_t need_fix(Value v, const char *op) {
    if (!IS_FIX(v)) { fprintf(stderr, "erro: argumento não-numérico em %s\n", op); exit(1); }
    return FIX(v);
}
Value cljn_add(Value a, Value b) { intptr_t r; if (__builtin_add_overflow(need_fix(a,"+"),need_fix(b,"+"),&r)) die("overflow em +"); return MK_FIX(r); }
Value cljn_sub(Value a, Value b) { intptr_t r; if (__builtin_sub_overflow(need_fix(a,"-"),need_fix(b,"-"),&r)) die("overflow em -"); return MK_FIX(r); }
Value cljn_mul(Value a, Value b) { intptr_t r; if (__builtin_mul_overflow(need_fix(a,"*"),need_fix(b,"*"),&r)) die("overflow em *"); return MK_FIX(r); }
Value cljn_quot(Value a, Value b) { intptr_t y=need_fix(b,"quot"); if(y==0) die("divisão por zero"); return MK_FIX(need_fix(a,"quot")/y); }
Value cljn_mod(Value a, Value b) {
    intptr_t y=need_fix(b,"mod"); if(y==0) die("divisão por zero");
    intptr_t x=need_fix(a,"mod"), r=x%y;
    if (r!=0 && ((r<0)!=(y<0))) r+=y;
    return MK_FIX(r);
}
Value cljn_inc(Value a) { return cljn_add(a, MK_FIX(1)); }
Value cljn_dec(Value a) { return cljn_sub(a, MK_FIX(1)); }

static Value b2v(int b) { return b ? TRUEV : FALSEV; }
Value cljn_lt(Value a, Value b) { return b2v(need_fix(a,"<")<need_fix(b,"<")); }
Value cljn_le(Value a, Value b) { return b2v(need_fix(a,"<=")<=need_fix(b,"<=")); }
Value cljn_gt(Value a, Value b) { return b2v(need_fix(a,">")>need_fix(b,">")); }
Value cljn_ge(Value a, Value b) { return b2v(need_fix(a,">=")>=need_fix(b,">=")); }

int cljn_equal_raw(Value a, Value b) {
    if (a == b) return 1;
    int ta = obj_type(a), tb = obj_type(b);
    if (ta == T_STR && tb == T_STR) {
        Str *x=(Str*)a,*y=(Str*)b;
        return x->len==y->len && memcmp(x->data,y->data,x->len)==0;
    }
    if (ta == T_CONS && tb == T_CONS)
        return cljn_equal_raw(((Cons*)a)->head,((Cons*)b)->head) &&
               cljn_equal_raw(((Cons*)a)->tail,((Cons*)b)->tail);
    return 0;
}
Value cljn_eq(Value a, Value b) { return b2v(cljn_equal_raw(a,b)); }

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
    die("first: não é uma sequência"); return NIL;
}
Value cljn_rest(Value v) {
    if (v == EMPTY || v == NIL) return EMPTY;
    if (obj_type(v) == T_CONS) return ((Cons *)v)->tail;
    die("rest: não é uma sequência"); return EMPTY;
}
Value cljn_count(Value v) {
    long n = 0;
    if (obj_type(v) == T_STR) return MK_FIX((long)((Str *)v)->len);
    while (v != EMPTY && v != NIL && obj_type(v) == T_CONS) { n++; v = ((Cons *)v)->tail; }
    return MK_FIX(n);
}

/* ---------- impressão / str ---------- */
typedef struct { char *p; size_t len, cap; } SB;
static void sb_init(SB *b) { b->cap = 32; b->len = 0; b->p = xalloc(b->cap); }
static void sb_putc(SB *b, char c) {
    if (b->len + 1 > b->cap) { b->cap *= 2; b->p = realloc(b->p, b->cap); if (!b->p) die("sem memória"); }
    b->p[b->len++] = c;
}
static void sb_write(SB *b, const char *s, size_t n) { for (size_t i=0;i<n;i++) sb_putc(b,s[i]); }
static void sb_str(SB *b, const char *s) { sb_write(b, s, strlen(s)); }

static void write_val(SB *b, Value v, int for_str) {
    if (IS_FIX(v)) { char t[32]; int n=snprintf(t,sizeof t,"%ld",(long)FIX(v)); sb_write(b,t,(size_t)n); return; }
    if (v == NIL) { if (!for_str) sb_str(b,"nil"); return; }
    if (v == TRUEV) { sb_str(b,"true"); return; }
    if (v == FALSEV) { sb_str(b,"false"); return; }
    if (v == EMPTY) { sb_str(b,"()"); return; }
    switch (obj_type(v)) {
        case T_STR: { Str *s=(Str*)v; sb_write(b,s->data,s->len); return; }
        case T_CONS: {
            sb_putc(b,'('); int first=1;
            while (v != EMPTY && obj_type(v)==T_CONS) {
                if(!first) sb_putc(b,' '); first=0;
                write_val(b,((Cons*)v)->head,for_str);
                v=((Cons*)v)->tail;
            }
            sb_putc(b,')'); return;
        }
        default: sb_str(b,"#<obj>");
    }
}

void cljn_print(Value v) { SB b; sb_init(&b); write_val(&b,v,0); fwrite(b.p,1,b.len,stdout); free(b.p); }
Value cljn_to_str(Value v) {
    SB b; sb_init(&b); write_val(&b,v,1);
    Value r = cljn_str_from(b.p, (long)b.len); /* pode coletar; v está rooteado */
    free(b.p); return r;
}
Value cljn_str_concat(Value a, Value b) {
    if (obj_type(a)!=T_STR || obj_type(b)!=T_STR) die("str_concat: esperava strings");
    Str *x=(Str*)a,*y=(Str*)b;
    size_t total = x->len + y->len;
    /* aloca o objeto (pode coletar; a,b rooteados) e depois o buffer. */
    Str *s=(Str*)obj_alloc(sizeof(Str),T_STR);
    s->len=total; s->data=xalloc(total?total:1);
    x=(Str*)a; y=(Str*)b; /* revalida após possível GC (não-móvel: iguais) */
    memcpy(s->data,x->data,x->len);
    memcpy(s->data+x->len,y->data,y->len);
    return (Value)s;
}
void cljn_print_space(void) { fputc(' ', stdout); }
void cljn_print_newline(void) { fputc('\n', stdout); }

/* Introspecção para testes. */
long cljn_gc_live_objects(void) { long n=0; for (Obj*o=all_objs;o;o=o->next_all) n++; return n; }
void cljn_gc_force(void) { gc_collect(); }
