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
/* Codificação por unsigned para não fazer left shift de signed negativo (UB). */
#define MK_FIX(n)  ((Value)(((uintptr_t)(intptr_t)(n) << 1) | 1u))
#define FIX(v)     ((intptr_t)(v) >> 1)
#define IS_PTR(v)  (((v) & 7) == 0)

/* Intervalo de fixnum em alvo de 64 bits (fonte única — deve casar com o codegen). */
#define FIXNUM_MIN (-((intptr_t)1 << 62))
#define FIXNUM_MAX (((intptr_t)1 << 62) - 1)

enum { T_STR = 1, T_CONS = 2, T_FN = 3, T_KW = 4, T_VEC = 5, T_MAP = 6, T_SET = 7, T_RECORD = 8 };

typedef struct Obj {
    uint8_t type;
    uint8_t mark;
    struct Obj *next_all; /* lista global de objetos, para o sweep */
} Obj;
typedef struct { Obj h; size_t len; char *data; } Str; /* também usado p/ T_KW */
typedef struct { Obj h; Value head; Value tail; } Cons;
/* Função de primeira classe: ponteiro de código + aridade + variáveis capturadas. */
typedef struct { Obj h; void *code; int64_t arity; int64_t nfree; Value freev[]; } Fn;
/* Coleções imutáveis (persistentes por valor; estrutura simples — o vector trie /
 * HAMT com structural sharing é otimização posterior, não muda a semântica). */
typedef struct { Obj h; int64_t len; Value items[]; } Vec;    /* vetor e set */
typedef struct { Obj h; int64_t n; Value kv[]; } Map;         /* array-map: kv[2n] */
typedef struct { Obj h; Value type_name; Value map; } Record; /* defrecord: nome + mapa */

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
        } else if (o->type == T_FN) {
            Fn *f = (Fn *)v;
            for (int64_t i = 0; i < f->nfree; i++) gc_mark(f->freev[i]);
            return;
        } else if (o->type == T_VEC || o->type == T_SET) {
            Vec *vec = (Vec *)v;
            for (int64_t i = 0; i < vec->len; i++) gc_mark(vec->items[i]);
            return;
        } else if (o->type == T_MAP) {
            Map *m = (Map *)v;
            for (int64_t i = 0; i < m->n * 2; i++) gc_mark(m->kv[i]);
            return;
        } else if (o->type == T_RECORD) {
            Record *r = (Record *)v;
            gc_mark(r->type_name);
            gc_mark(r->map);
            return;
        } else {
            return; /* string/keyword: folha */
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

/* ---------- funções de primeira classe ---------- */
Value cljn_make_fn(Value code, Value arity, Value nfree) {
    /* Capturas (se houver) estão no shadow-stack; obj_alloc pode coletar. */
    Fn *f = (Fn *)obj_alloc(sizeof(Fn) + (size_t)nfree * sizeof(Value), T_FN);
    f->code = (void *)code;
    f->arity = (int64_t)arity;
    f->nfree = (int64_t)nfree;
    for (int64_t i = 0; i < f->nfree; i++) f->freev[i] = NIL; /* zera antes de qualquer GC */
    return (Value)f;
}
void cljn_fn_set_free(Value fn, Value i, Value v) { ((Fn *)fn)->freev[(size_t)i] = v; }
Value cljn_fn_free(Value fn, Value i) { return ((Fn *)fn)->freev[(size_t)i]; }
Value cljn_fn_code(Value fn) { return (Value)((Fn *)fn)->code; }
void cljn_check_fn(Value fn) {
    if (obj_type(fn) != T_FN) die("valor chamado não é uma função");
}

/* Convenção de chamada uniforme: entry(self, argc, argv) -> valor.
 * Os argumentos ficam contíguos no topo do shadow-stack; `argv` aponta pra lá
 * (portanto já rooteados). */
Value cljn_argv(Value argc) {
    return (Value)&gc_stack[gc_sp - (size_t)argc];
}
void cljn_check_arity(Value argc, Value expected) {
    if ((int64_t)argc != (int64_t)expected) {
        fprintf(stderr, "erro: aridade errada (esperava %ld, recebeu %ld)\n",
                (long)expected, (long)argc);
        exit(1);
    }
}
void cljn_check_arity_min(Value argc, Value minv) {
    if ((int64_t)argc < (int64_t)minv) {
        fprintf(stderr, "erro: aridade errada (esperava ao menos %ld, recebeu %ld)\n",
                (long)minv, (long)argc);
        exit(1);
    }
}
/* Coleta argv[nfixed..argc) numa lista (para o parâmetro `& rest`). */
Value cljn_collect_rest(Value argc, Value argv, Value nfixed) {
    Value *a = (Value *)argv;
    int64_t n = (int64_t)argc, nf = (int64_t)nfixed;
    Value acc = EMPTY;
    cljn_gc_push(acc);
    for (int64_t i = n - 1; i >= nf; i--) {
        acc = cljn_cons(a[i], acc);
        gc_stack[gc_sp - 1] = acc;
    }
    cljn_gc_popn(1);
    return acc;
}
/* apply: empurra os elementos de `coll` no topo do shadow-stack (após os
 * `fixed_argc` já empurrados) e devolve o argc total (raw). `coll` está rooteado. */
Value cljn_spread_args(Value fixed_argc, Value coll) {
    int64_t extra = 0;
    int t = obj_type(coll);
    if (t == T_VEC || t == T_SET) {
        Vec *v = (Vec *)coll;
        for (int64_t i = 0; i < v->len; i++) { cljn_gc_push(v->items[i]); extra++; }
    } else {
        Value c = coll;
        while (c != EMPTY && c != NIL && obj_type(c) == T_CONS) {
            cljn_gc_push(((Cons *)c)->head);
            c = ((Cons *)c)->tail;
            extra++;
        }
    }
    return (Value)((int64_t)fixed_argc + extra);
}

static Value b2v(int b); /* fwd */
int cljn_equal_raw(Value a, Value b); /* fwd */

/* ---------- keywords ---------- */
Value cljn_kw(const char *p, long len) {
    Str *s = (Str *)obj_alloc(sizeof(Str), T_KW);
    s->len = (size_t)len;
    s->data = (len > 0) ? xalloc((size_t)len) : NULL;
    if (len > 0) memcpy(s->data, p, (size_t)len);
    return (Value)s;
}

/* ---------- vetores (imutáveis) ---------- */
Value cljn_vec_alloc(Value n) {
    int64_t k = (int64_t)n;
    Vec *v = (Vec *)obj_alloc(sizeof(Vec) + (size_t)k * sizeof(Value), T_VEC);
    v->len = k;
    for (int64_t i = 0; i < k; i++) v->items[i] = NIL; /* zera antes de qualquer GC */
    return (Value)v;
}
void cljn_vec_set(Value vec, Value i, Value x) { ((Vec *)vec)->items[(int64_t)i] = x; }
Value cljn_vec_conj(Value vec, Value x) {
    Vec *o = (Vec *)vec;
    int64_t n = o->len;
    Vec *nv = (Vec *)obj_alloc(sizeof(Vec) + (size_t)(n + 1) * sizeof(Value), T_VEC);
    o = (Vec *)vec; /* revalida (não-móvel: igual) */
    nv->len = n + 1;
    for (int64_t i = 0; i < n; i++) nv->items[i] = o->items[i];
    nv->items[n] = x;
    return (Value)nv;
}
Value cljn_vec_assoc(Value vec, Value idx, Value x) {
    Vec *o = (Vec *)vec;
    int64_t i = IS_FIX(idx) ? FIX(idx) : -1; /* índice vem tagged */
    if (i < 0 || i > o->len) die("assoc: índice fora dos limites do vetor");
    if (i == o->len) return cljn_vec_conj(vec, x);
    int64_t n = o->len;
    Vec *nv = (Vec *)obj_alloc(sizeof(Vec) + (size_t)n * sizeof(Value), T_VEC);
    o = (Vec *)vec;
    nv->len = n;
    for (int64_t j = 0; j < n; j++) nv->items[j] = o->items[j];
    nv->items[i] = x;
    return (Value)nv;
}

/* ---------- sets (imutáveis) ---------- */
Value cljn_set_alloc(Value n) {
    int64_t k = (int64_t)n;
    Vec *s = (Vec *)obj_alloc(sizeof(Vec) + (size_t)k * sizeof(Value), T_SET);
    s->len = 0; /* cresce durante a construção; capacidade k */
    for (int64_t i = 0; i < k; i++) s->items[i] = NIL;
    return (Value)s;
}
static int set_member(Vec *s, Value x) {
    for (int64_t i = 0; i < s->len; i++) if (cljn_equal_raw(s->items[i], x)) return 1;
    return 0;
}
void cljn_set_add(Value set, Value x) { /* construção */
    Vec *s = (Vec *)set;
    if (!set_member(s, x)) s->items[s->len++] = x;
}
Value cljn_set_conj(Value set, Value x) {
    Vec *o = (Vec *)set;
    if (set_member(o, x)) return set;
    int64_t n = o->len;
    Vec *ns = (Vec *)obj_alloc(sizeof(Vec) + (size_t)(n + 1) * sizeof(Value), T_SET);
    o = (Vec *)set;
    ns->len = n + 1;
    for (int64_t i = 0; i < n; i++) ns->items[i] = o->items[i];
    ns->items[n] = x;
    return (Value)ns;
}
Value cljn_set_contains(Value set, Value x) { return b2v(set_member((Vec *)set, x)); }

/* ---------- mapas (array-map imutável) ---------- */
Value cljn_map_alloc(Value n) {
    int64_t k = (int64_t)n;
    Map *m = (Map *)obj_alloc(sizeof(Map) + (size_t)(2 * k) * sizeof(Value), T_MAP);
    m->n = k;
    for (int64_t i = 0; i < 2 * k; i++) m->kv[i] = NIL;
    return (Value)m;
}
void cljn_map_set(Value map, Value i, Value k, Value v) {
    Map *m = (Map *)map;
    int64_t idx = (int64_t)i;
    m->kv[2 * idx] = k;
    m->kv[2 * idx + 1] = v;
}
static int64_t map_index(Map *m, Value k) {
    for (int64_t i = 0; i < m->n; i++) if (cljn_equal_raw(m->kv[2 * i], k)) return i;
    return -1;
}
Value cljn_map_get(Value map, Value k) {
    if (obj_type(map) != T_MAP) return NIL;
    Map *m = (Map *)map;
    int64_t i = map_index(m, k);
    return (i >= 0) ? m->kv[2 * i + 1] : NIL;
}
Value cljn_map_contains(Value map, Value k) {
    return b2v(obj_type(map) == T_MAP && map_index((Map *)map, k) >= 0);
}
Value cljn_map_assoc(Value map, Value k, Value v) {
    Map *o = (Map *)map;
    int64_t at = map_index(o, k);
    int64_t n = o->n;
    int64_t nn = (at >= 0) ? n : n + 1;
    Map *nm = (Map *)obj_alloc(sizeof(Map) + (size_t)(2 * nn) * sizeof(Value), T_MAP);
    o = (Map *)map;
    nm->n = nn;
    for (int64_t i = 0; i < n; i++) { nm->kv[2 * i] = o->kv[2 * i]; nm->kv[2 * i + 1] = o->kv[2 * i + 1]; }
    if (at >= 0) {
        nm->kv[2 * at + 1] = v;
    } else {
        nm->kv[2 * n] = k;
        nm->kv[2 * n + 1] = v;
    }
    return (Value)nm;
}
Value cljn_map_dissoc(Value map, Value k) {
    Map *o = (Map *)map;
    int64_t at = map_index(o, k);
    if (at < 0) return map;
    int64_t n = o->n;
    Map *nm = (Map *)obj_alloc(sizeof(Map) + (size_t)(2 * (n - 1)) * sizeof(Value), T_MAP);
    o = (Map *)map;
    nm->n = n - 1;
    int64_t j = 0;
    for (int64_t i = 0; i < n; i++) {
        if (i == at) continue;
        nm->kv[2 * j] = o->kv[2 * i];
        nm->kv[2 * j + 1] = o->kv[2 * i + 1];
        j++;
    }
    return (Value)nm;
}
/* Constrói uma lista em C rooteando o acumulador no shadow-stack (senão o GC
 * coletaria `acc` no meio da construção). `map`/coll já está rooteado por quem chama. */
Value cljn_map_keys(Value map) {
    if (obj_type(map) == T_RECORD) map = ((Record *)map)->map;
    Map *m = (Map *)map;
    Value acc = EMPTY;
    cljn_gc_push(acc);
    for (int64_t i = m->n - 1; i >= 0; i--) {
        acc = cljn_cons(m->kv[2 * i], acc);
        gc_stack[gc_sp - 1] = acc;
    }
    cljn_gc_popn(1);
    return acc;
}
Value cljn_map_vals(Value map) {
    if (obj_type(map) == T_RECORD) map = ((Record *)map)->map;
    Map *m = (Map *)map;
    Value acc = EMPTY;
    cljn_gc_push(acc);
    for (int64_t i = m->n - 1; i >= 0; i--) {
        acc = cljn_cons(m->kv[2 * i + 1], acc);
        gc_stack[gc_sp - 1] = acc;
    }
    cljn_gc_popn(1);
    return acc;
}

/* ---------- records (defrecord) ---------- */
Value cljn_make_record(Value type_name, Value map) {
    Record *r = (Record *)obj_alloc(sizeof(Record), T_RECORD);
    r->type_name = type_name;
    r->map = map;
    return (Value)r;
}
Value cljn_record_type(Value r) { return ((Record *)r)->type_name; }
Value cljn_record_map(Value r) { return ((Record *)r)->map; }

/* ---------- genéricos (dispatch por tipo) ---------- */
Value cljn_contains(Value coll, Value key);
Value cljn_get(Value coll, Value key) {
    switch (obj_type(coll)) {
        case T_RECORD: return cljn_map_get(((Record *)coll)->map, key);
        case T_MAP: return cljn_map_get(coll, key);
        case T_VEC: {
            Vec *v = (Vec *)coll;
            if (IS_FIX(key)) { int64_t i = FIX(key); if (i >= 0 && i < v->len) return v->items[i]; }
            return NIL;
        }
        case T_SET: return set_member((Vec *)coll, key) ? key : NIL;
        default: return NIL;
    }
}
Value cljn_contains(Value coll, Value key) {
    switch (obj_type(coll)) {
        case T_RECORD: return cljn_map_contains(((Record *)coll)->map, key);
        case T_MAP: return cljn_map_contains(coll, key);
        case T_SET: return cljn_set_contains(coll, key);
        case T_VEC: { Vec *v = (Vec *)coll; return b2v(IS_FIX(key) && FIX(key) >= 0 && FIX(key) < v->len); }
        default: return FALSEV;
    }
}
Value cljn_conj(Value coll, Value x) {
    switch (obj_type(coll)) {
        case T_VEC: return cljn_vec_conj(coll, x);
        case T_SET: return cljn_set_conj(coll, x);
        case T_CONS: return cljn_cons(x, coll);
        case T_MAP:
            if (obj_type(x) == T_VEC && ((Vec *)x)->len == 2)
                return cljn_map_assoc(coll, ((Vec *)x)->items[0], ((Vec *)x)->items[1]);
            die("conj em mapa requer [k v]");
            return coll;
        default:
            if (coll == EMPTY || coll == NIL) return cljn_cons(x, EMPTY);
            die("conj: coleção não suportada");
            return coll;
    }
}
Value cljn_assoc(Value coll, Value k, Value v) {
    switch (obj_type(coll)) {
        case T_RECORD: {
            Record *r = (Record *)coll;
            Value nm = cljn_map_assoc(r->map, k, v); /* r rooteado via coll */
            cljn_gc_push(nm);                        /* rooteia nm p/ o make_record */
            Value rec = cljn_make_record(((Record *)coll)->type_name, nm);
            cljn_gc_popn(1);
            return rec;
        }
        case T_MAP: return cljn_map_assoc(coll, k, v);
        case T_VEC: return cljn_vec_assoc(coll, k, v);
        default:
            if (coll == NIL) { Value m = cljn_map_alloc(MK_FIX(0)); return cljn_map_assoc(m, k, v); }
            die("assoc: coleção não suportada");
            return coll;
    }
}
Value cljn_nth(Value coll, Value idx) {
    int64_t i = IS_FIX(idx) ? FIX(idx) : -1;
    if (i < 0) die("nth: índice inválido");
    switch (obj_type(coll)) {
        case T_VEC: { Vec *v = (Vec *)coll; if (i < v->len) return v->items[i]; break; }
        case T_CONS: { Value c = coll; while (i-- > 0 && obj_type(c) == T_CONS) c = ((Cons *)c)->tail; if (obj_type(c) == T_CONS) return ((Cons *)c)->head; break; }
    }
    die("nth: índice fora dos limites");
    return NIL;
}

/* ---------- aritmética ---------- */
static intptr_t need_fix(Value v, const char *op) {
    if (!IS_FIX(v)) { fprintf(stderr, "erro: argumento não-numérico em %s\n", op); exit(1); }
    return FIX(v);
}
/* Valida o intervalo de fixnum ANTES do retag (i64 tem 63 bits, fixnum 62). */
static Value mk_fix_checked(intptr_t r, const char *op) {
    if (r < FIXNUM_MIN || r > FIXNUM_MAX) { fprintf(stderr, "erro: overflow em %s\n", op); exit(1); }
    return MK_FIX(r);
}
Value cljn_add(Value a, Value b) { intptr_t r; if (__builtin_add_overflow(need_fix(a,"+"),need_fix(b,"+"),&r)) die("overflow em +"); return mk_fix_checked(r,"+"); }
Value cljn_sub(Value a, Value b) { intptr_t r; if (__builtin_sub_overflow(need_fix(a,"-"),need_fix(b,"-"),&r)) die("overflow em -"); return mk_fix_checked(r,"-"); }
Value cljn_mul(Value a, Value b) { intptr_t r; if (__builtin_mul_overflow(need_fix(a,"*"),need_fix(b,"*"),&r)) die("overflow em *"); return mk_fix_checked(r,"*"); }
Value cljn_quot(Value a, Value b) { intptr_t y=need_fix(b,"quot"); if(y==0) die("divisão por zero"); intptr_t x=need_fix(a,"quot"); if(x==FIXNUM_MIN&&y==-1) die("overflow em quot"); return mk_fix_checked(x/y,"quot"); }
Value cljn_mod(Value a, Value b) {
    intptr_t y=need_fix(b,"mod"); if(y==0) die("divisão por zero");
    intptr_t x=need_fix(a,"mod"), r=x%y;
    if (r!=0 && ((r<0)!=(y<0))) r+=y;
    return mk_fix_checked(r,"mod");
}
Value cljn_inc(Value a) { intptr_t r; if(__builtin_add_overflow(need_fix(a,"inc"),(intptr_t)1,&r)) die("overflow em inc"); return mk_fix_checked(r,"inc"); }
Value cljn_dec(Value a) { intptr_t r; if(__builtin_sub_overflow(need_fix(a,"dec"),(intptr_t)1,&r)) die("overflow em dec"); return mk_fix_checked(r,"dec"); }

static Value b2v(int b) { return b ? TRUEV : FALSEV; }
Value cljn_lt(Value a, Value b) { return b2v(need_fix(a,"<")<need_fix(b,"<")); }
Value cljn_le(Value a, Value b) { return b2v(need_fix(a,"<=")<=need_fix(b,"<=")); }
Value cljn_gt(Value a, Value b) { return b2v(need_fix(a,">")>need_fix(b,">")); }
Value cljn_ge(Value a, Value b) { return b2v(need_fix(a,">=")>=need_fix(b,">=")); }

static int is_seq(int t) { return t == T_CONS || t == T_VEC; }
static Value seq_nth(Value coll, int t, int64_t i) {
    if (t == T_VEC) return ((Vec *)coll)->items[i];
    Value c = coll;
    while (i-- > 0) c = ((Cons *)c)->tail;
    return ((Cons *)c)->head;
}
static int64_t seq_len(Value coll, int t) {
    if (t == T_VEC) return ((Vec *)coll)->len;
    int64_t n = 0;
    Value c = coll;
    while (obj_type(c) == T_CONS) { n++; c = ((Cons *)c)->tail; }
    return n;
}
int cljn_equal_raw(Value a, Value b) {
    if (a == b) return 1;
    int ta = obj_type(a), tb = obj_type(b);
    if ((ta == T_STR && tb == T_STR) || (ta == T_KW && tb == T_KW)) {
        Str *x = (Str *)a, *y = (Str *)b;
        return ta == tb && x->len == y->len && memcmp(x->data, y->data, x->len) == 0;
    }
    /* sequências (list/vector) comparam elemento a elemento */
    if (is_seq(ta) && is_seq(tb)) {
        int64_t la = seq_len(a, ta), lb = seq_len(b, tb);
        if (la != lb) return 0;
        for (int64_t i = 0; i < la; i++)
            if (!cljn_equal_raw(seq_nth(a, ta, i), seq_nth(b, tb, i))) return 0;
        return 1;
    }
    if (ta == T_SET && tb == T_SET) {
        Vec *x = (Vec *)a, *y = (Vec *)b;
        if (x->len != y->len) return 0;
        for (int64_t i = 0; i < x->len; i++) if (!set_member(y, x->items[i])) return 0;
        return 1;
    }
    if (ta == T_MAP && tb == T_MAP) {
        Map *x = (Map *)a, *y = (Map *)b;
        if (x->n != y->n) return 0;
        for (int64_t i = 0; i < x->n; i++) {
            int64_t j = map_index(y, x->kv[2 * i]);
            if (j < 0 || !cljn_equal_raw(x->kv[2 * i + 1], y->kv[2 * j + 1])) return 0;
        }
        return 1;
    }
    if (ta == T_RECORD && tb == T_RECORD) {
        Record *x = (Record *)a, *y = (Record *)b;
        return cljn_equal_raw(x->type_name, y->type_name) && cljn_equal_raw(x->map, y->map);
    }
    return 0;
}
Value cljn_eq(Value a, Value b) { return b2v(cljn_equal_raw(a,b)); }

int cljn_truthy(Value v) { return (v != NIL && v != FALSEV) ? 1 : 0; }
Value cljn_not(Value v) { return b2v(!cljn_truthy(v)); }
Value cljn_nilp(Value v) { return b2v(v == NIL); }
Value cljn_emptyp(Value v) {
    if (v == EMPTY || v == NIL) return TRUEV;
    switch (obj_type(v)) {
        case T_STR: return b2v(((Str *)v)->len == 0);
        case T_VEC: case T_SET: return b2v(((Vec *)v)->len == 0);
        case T_MAP: return b2v(((Map *)v)->n == 0);
    }
    return FALSEV;
}
Value cljn_first(Value v) {
    if (v == EMPTY || v == NIL) return NIL;
    switch (obj_type(v)) {
        case T_CONS: return ((Cons *)v)->head;
        case T_VEC: case T_SET: { Vec *x = (Vec *)v; return x->len > 0 ? x->items[0] : NIL; }
    }
    die("first: não é uma sequência"); return NIL;
}
Value cljn_rest(Value v) {
    if (v == EMPTY || v == NIL) return EMPTY;
    if (obj_type(v) == T_CONS) return ((Cons *)v)->tail;
    if (obj_type(v) == T_VEC || obj_type(v) == T_SET) {
        Vec *x = (Vec *)v;
        Value acc = EMPTY;
        cljn_gc_push(acc); /* rooteia o acumulador durante a construção */
        for (int64_t i = x->len - 1; i >= 1; i--) {
            acc = cljn_cons(x->items[i], acc);
            gc_stack[gc_sp - 1] = acc;
        }
        cljn_gc_popn(1);
        return acc;
    }
    die("rest: não é uma sequência"); return EMPTY;
}
Value cljn_count(Value v) {
    switch (obj_type(v)) {
        case T_STR: return MK_FIX((long)((Str *)v)->len);
        case T_VEC: case T_SET: return MK_FIX(((Vec *)v)->len);
        case T_MAP: return MK_FIX(((Map *)v)->n);
        case T_RECORD: return MK_FIX(((Map *)((Record *)v)->map)->n);
    }
    long n = 0;
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
        case T_KW: { Str *s=(Str*)v; sb_putc(b,':'); sb_write(b,s->data,s->len); return; }
        case T_CONS: {
            sb_putc(b,'('); int first=1;
            while (v != EMPTY && obj_type(v)==T_CONS) {
                if(!first) sb_putc(b,' '); first=0;
                write_val(b,((Cons*)v)->head,for_str);
                v=((Cons*)v)->tail;
            }
            sb_putc(b,')'); return;
        }
        case T_VEC: {
            Vec *x=(Vec*)v; sb_putc(b,'[');
            for (int64_t i=0;i<x->len;i++){ if(i) sb_putc(b,' '); write_val(b,x->items[i],for_str); }
            sb_putc(b,']'); return;
        }
        case T_SET: {
            Vec *x=(Vec*)v; sb_str(b,"#{");
            for (int64_t i=0;i<x->len;i++){ if(i) sb_putc(b,' '); write_val(b,x->items[i],for_str); }
            sb_putc(b,'}'); return;
        }
        case T_MAP: {
            Map *m=(Map*)v; sb_putc(b,'{');
            for (int64_t i=0;i<m->n;i++){ if(i) sb_str(b,", "); write_val(b,m->kv[2*i],for_str); sb_putc(b,' '); write_val(b,m->kv[2*i+1],for_str); }
            sb_putc(b,'}'); return;
        }
        case T_FN: sb_str(b, "#<fn>"); return;
        case T_RECORD: {
            Record *r = (Record *)v;
            sb_putc(b, '#');
            if (obj_type(r->type_name) == T_KW) { Str *n = (Str *)r->type_name; sb_write(b, n->data, n->len); }
            write_val(b, r->map, for_str);
            return;
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
