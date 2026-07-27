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

enum { T_STR = 1, T_CONS = 2, T_FN = 3, T_KW = 4, T_VEC = 5, T_MAP = 6, T_SET = 7, T_RECORD = 8,
       T_VNODE = 9, T_HMAP = 10, T_MNODE = 11, T_MCOLL = 12, T_HSET = 13,
       T_TNODE = 14, T_SMAP = 15, T_SSET = 16 };

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
typedef struct { Obj h; int64_t len; Value items[]; } Vec;    /* set (array simples) */
typedef struct { Obj h; int64_t n; Value kv[]; } Map;         /* array-map: kv[2n] (mantém ordem, <=8) */
typedef struct { Obj h; Value type_name; Value map; } Record; /* defrecord: nome + mapa */
/* Mapa grande: HAMT (hash array mapped trie). HMap = wrapper (count + raiz);
 * nós = MNode (bitmap-indexado) e MColl (colisão de hash). Chave==MNODEKEY marca
 * que o slot guarda um sub-nó. Ver hibrido: array-map (<=8) promove a HMAP. */
#define MAP_ARRAY_MAX 8
typedef struct { Obj h; int64_t count; Value root; } HMap;
typedef struct { Obj h; uint32_t bitmap; Value arr[]; } MNode; /* 2*popcount slots */
typedef struct { Obj h; uint32_t hash; int64_t n; Value pairs[]; } MColl; /* 2n */
#define MNODEKEY ((Value)26)  /* imediato reservado: "este slot guarda um sub-nó" */
#define MNOTFOUND ((Value)42) /* sentinela interno de "não encontrado" */
/* Coleções ordenadas: árvore left-leaning red-black (LLRB) persistente.
 * TNode = nó (chave, valor, filhos, cor). Sorted = wrapper (count + raiz).
 * sorted-set guarda val==key. Ordem por cljn_compare (ordem total de Clojure). */
typedef struct { Obj h; Value key; Value val; Value left; Value right; int64_t red; } TNode;
typedef struct { Obj h; int64_t count; Value root; } Sorted; /* T_SMAP / T_SSET */
/* Vetor persistente: bitmapped vector trie (32-way), como o PersistentVector de
 * Clojure. `tail` (até 32) dá conj/nth O(1) amortizado; o resto é uma árvore
 * 32-way com structural sharing (conj/assoc/nth O(log32 n)). */
#define VBITS 5
#define VWIDTH 32
#define VMASK 31
typedef struct { Obj h; Value slots[VWIDTH]; } VNode;
typedef struct { Obj h; int64_t count; int64_t shift; Value root; Value tail; int64_t tail_len; } PVec;
static Value pv_nth(PVec *v, int64_t i); /* fwd */

/* ---------- shadow-stack de roots ---------- */
/* Exportados: o código gerado (ADR-0006 Fase 3) escreve/lê diretamente, sem call.
 * `gc_sp` é um índice (contagem de slots vivos). Single-thread. */
#define GC_STACK_CAP (1u << 22) /* 4M slots */
Value gc_stack[GC_STACK_CAP];
int64_t gc_sp = 0;

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
/* Zona sem-GC: ops de runtime que alocam múltiplos objetos intermediários
 * (ex.: vector trie) incrementam isto para não coletar no meio (alocação
 * limitada; as entradas já estão rooteadas pelo chamador). */
static int gc_disabled = 0;
static void maybe_gc(void) {
    if (gc_stress < 0) {
        const char *e = getenv("CLJN_GC_STRESS");
        gc_stress = (e && e[0] && e[0] != '0') ? 1 : 0;
        const char *o = getenv("CLJN_GC_OFF");
        gc_off = (o && o[0] && o[0] != '0') ? 1 : 0;
    }
    if (gc_off || gc_disabled) return;
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
        } else if (o->type == T_SET) {
            Vec *vec = (Vec *)v;
            for (int64_t i = 0; i < vec->len; i++) gc_mark(vec->items[i]);
            return;
        } else if (o->type == T_VNODE) {
            VNode *nd = (VNode *)v;
            for (int i = 0; i < VWIDTH; i++) gc_mark(nd->slots[i]);
            return;
        } else if (o->type == T_VEC) {
            PVec *pv = (PVec *)v;
            gc_mark(pv->root);
            gc_mark(pv->tail);
            return;
        } else if (o->type == T_MAP) {
            Map *m = (Map *)v;
            for (int64_t i = 0; i < m->n * 2; i++) gc_mark(m->kv[i]);
            return;
        } else if (o->type == T_HMAP || o->type == T_HSET) {
            gc_mark(((HMap *)v)->root);
            return;
        } else if (o->type == T_SMAP || o->type == T_SSET) {
            gc_mark(((Sorted *)v)->root);
            return;
        } else if (o->type == T_TNODE) {
            TNode *nd = (TNode *)v;
            gc_mark(nd->key);
            gc_mark(nd->val);
            gc_mark(nd->left);
            v = nd->right; /* itera o filho direito */
        } else if (o->type == T_MNODE) {
            MNode *nd = (MNode *)v;
            int slots = 2 * __builtin_popcount(nd->bitmap);
            for (int i = 0; i < slots; i++) gc_mark(nd->arr[i]);
            return;
        } else if (o->type == T_MCOLL) {
            MColl *c = (MColl *)v;
            for (int64_t i = 0; i < c->n * 2; i++) gc_mark(c->pairs[i]);
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

static void gc_mark_method_table(void); /* fwd */
static void gc_collect(void) {
    for (int64_t i = 0; i < gc_sp; i++) gc_mark(gc_stack[i]);
    gc_mark_method_table(); /* raízes permanentes: chaves/impls de protocolos */
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
static int64_t hnode_push_keys(Value node); /* fwd (spread sobre T_HSET) */
static int64_t tn_push_spread(Value node, int entries); /* fwd (spread sobre sorted) */
Value cljn_spread_args(Value fixed_argc, Value coll) {
    int64_t extra = 0;
    int t = obj_type(coll);
    if (t == T_VEC) {
        PVec *v = (PVec *)coll;
        for (int64_t i = 0; i < v->count; i++) { cljn_gc_push(pv_nth(v, i)); extra++; }
    } else if (t == T_SET) {
        Vec *v = (Vec *)coll;
        for (int64_t i = 0; i < v->len; i++) { cljn_gc_push(v->items[i]); extra++; }
    } else if (t == T_HSET) {
        extra = hnode_push_keys(((HMap *)coll)->root);
    } else if (t == T_SSET || t == T_SMAP) {
        extra = tn_push_spread(((Sorted *)coll)->root, t == T_SMAP);
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
int cljn_truthy(Value v); /* fwd */
static MNode *mnode_alloc(int slots); /* fwd (HAMT) */
static Value node_get(Value node, uint32_t shift, uint32_t hash, Value key); /* fwd */
static Value node_assoc(Value node, uint32_t shift, uint32_t hash, Value key, Value val, int *added); /* fwd */
uint32_t cljn_hash(Value v); /* fwd */
static void hmap_cons_walk(Value node, int mode); /* fwd */
Value cljn_sorted_dissoc(Value m, Value k); /* fwd (sorted-map) */
Value cljn_sorted_get(Value m, Value k); /* fwd */
Value cljn_sorted_contains(Value m, Value k); /* fwd */
Value cljn_contains(Value coll, Value key); /* fwd */
static Value sorted_seq(Value m, int mode); /* fwd (sorted keys/vals/entries) */
static int tn_all_in(Value node, Value other); /* fwd (igualdade de sorted-set) */
static int tn_map_subset(Value node, Value other); /* fwd (igualdade de sorted-map) */
Value cljn_vec_conj(Value vec, Value x); /* fwd */
Value cljn_vec_empty(void); /* fwd */

/* ---------- keywords ---------- */
Value cljn_kw(const char *p, long len) {
    Str *s = (Str *)obj_alloc(sizeof(Str), T_KW);
    s->len = (size_t)len;
    s->data = (len > 0) ? xalloc((size_t)len) : NULL;
    if (len > 0) memcpy(s->data, p, (size_t)len);
    return (Value)s;
}

/* ---------- vetor persistente (bitmapped trie 32-way) ---------- */
static VNode *vnode_new(void) {
    VNode *n = (VNode *)obj_alloc(sizeof(VNode), T_VNODE);
    for (int i = 0; i < VWIDTH; i++) n->slots[i] = NIL;
    return n;
}
static VNode *vnode_copy(VNode *src) {
    VNode *n = (VNode *)obj_alloc(sizeof(VNode), T_VNODE);
    for (int i = 0; i < VWIDTH; i++) n->slots[i] = src->slots[i];
    return n;
}
Value cljn_vec_empty(void) {
    maybe_gc();    /* ponto seguro antes de desabilitar (mantém o trigger de coleta) */
    gc_disabled++; /* aloca 3 objetos; sem coleta no meio */
    PVec *v = (PVec *)obj_alloc(sizeof(PVec), T_VEC);
    v->count = 0;
    v->shift = VBITS;
    v->root = (Value)vnode_new();
    v->tail = (Value)vnode_new();
    v->tail_len = 0;
    gc_disabled--;
    return (Value)v;
}
static int64_t pv_tailoff(PVec *v) { return v->count - v->tail_len; }
static Value pv_nth(PVec *v, int64_t i) {
    if (i >= pv_tailoff(v)) return ((VNode *)v->tail)->slots[i - pv_tailoff(v)];
    VNode *node = (VNode *)v->root;
    for (int64_t level = v->shift; level > 0; level -= VBITS)
        node = (VNode *)node->slots[(i >> level) & VMASK];
    return node->slots[i & VMASK];
}
static VNode *new_path(int64_t level, VNode *node) {
    if (level == 0) return node;
    VNode *ret = vnode_new();
    ret->slots[0] = (Value)new_path(level - VBITS, node);
    return ret;
}
static VNode *push_tail(int64_t level, VNode *parent, VNode *tailnode, int64_t cnt) {
    int subidx = (int)(((cnt - 1) >> level) & VMASK);
    VNode *ret = vnode_copy(parent);
    VNode *insert;
    if (level == VBITS) {
        insert = tailnode;
    } else {
        Value child = parent->slots[subidx];
        insert = (obj_type(child) == T_VNODE)
                     ? push_tail(level - VBITS, (VNode *)child, tailnode, cnt)
                     : new_path(level - VBITS, tailnode);
    }
    ret->slots[subidx] = (Value)insert;
    return ret;
}
/* Zona sem-GC: conj aloca O(log32 n) nós; entradas rooteadas pelo chamador. */
Value cljn_vec_conj(Value vec, Value x) {
    maybe_gc(); /* ponto seguro: vec/x rooteados pelo chamador */
    gc_disabled++;
    PVec *o = (PVec *)vec;
    PVec *nv = (PVec *)obj_alloc(sizeof(PVec), T_VEC);
    nv->count = o->count + 1;
    nv->shift = o->shift;
    nv->root = o->root;
    if (o->tail_len < VWIDTH) {
        VNode *nt = vnode_copy((VNode *)o->tail);
        nt->slots[o->tail_len] = x;
        nv->tail = (Value)nt;
        nv->tail_len = o->tail_len + 1;
    } else {
        VNode *tailnode = (VNode *)o->tail;
        VNode *newroot;
        int64_t newshift = o->shift;
        if ((o->count >> VBITS) > (1LL << o->shift)) {
            newroot = vnode_new();
            newroot->slots[0] = o->root;
            newroot->slots[1] = (Value)new_path(o->shift, tailnode);
            newshift += VBITS;
        } else {
            newroot = push_tail(o->shift, (VNode *)o->root, tailnode, o->count);
        }
        VNode *nt = vnode_new();
        nt->slots[0] = x;
        nv->root = (Value)newroot;
        nv->shift = newshift;
        nv->tail = (Value)nt;
        nv->tail_len = 1;
    }
    gc_disabled--;
    return (Value)nv;
}
static VNode *do_assoc(int64_t level, VNode *node, int64_t i, Value x) {
    VNode *ret = vnode_copy(node);
    if (level == 0) {
        ret->slots[i & VMASK] = x;
    } else {
        int subidx = (int)((i >> level) & VMASK);
        ret->slots[subidx] = (Value)do_assoc(level - VBITS, (VNode *)node->slots[subidx], i, x);
    }
    return ret;
}
Value cljn_vec_assoc(Value vec, Value idx, Value x) {
    PVec *o = (PVec *)vec;
    int64_t i = IS_FIX(idx) ? FIX(idx) : -1;
    if (i < 0 || i > o->count) die("assoc: índice fora dos limites do vetor");
    if (i == o->count) return cljn_vec_conj(vec, x);
    maybe_gc(); /* ponto seguro: vec/x rooteados pelo chamador */
    gc_disabled++;
    PVec *nv = (PVec *)obj_alloc(sizeof(PVec), T_VEC);
    nv->count = o->count;
    nv->shift = o->shift;
    nv->root = o->root;
    nv->tail = o->tail;
    nv->tail_len = o->tail_len;
    if (i >= pv_tailoff(o)) {
        VNode *nt = vnode_copy((VNode *)o->tail);
        nt->slots[i - pv_tailoff(o)] = x;
        nv->tail = (Value)nt;
    } else {
        nv->root = (Value)do_assoc(o->shift, (VNode *)o->root, i, x);
    }
    gc_disabled--;
    return (Value)nv;
}
int64_t cljn_vec_count_raw(Value v) { return ((PVec *)v)->count; }

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
static Value hset_node_assoc(Value root, int64_t count, Value x, int64_t *out_count) {
    int added;
    Value nr = node_assoc(root, 0, cljn_hash(x), x, x, &added);
    *out_count = count + added;
    return nr;
}
Value cljn_set_conj(Value set, Value x) {
    maybe_gc();
    gc_disabled++;
    Value result;
    if (obj_type(set) == T_HSET) {
        HMap *o = (HMap *)set;
        int64_t c;
        Value nr = hset_node_assoc(o->root, o->count, x, &c);
        HMap *ns = (HMap *)obj_alloc(sizeof(HMap), T_HSET);
        ns->count = c; ns->root = nr;
        result = (Value)ns;
    } else {
        Vec *o = (Vec *)set;
        if (set_member(o, x)) {
            result = set;
        } else if (o->len + 1 > MAP_ARRAY_MAX) {
            /* promove array-set → HAMT-set (valor = chave) */
            MNode *root = mnode_alloc(0); root->bitmap = 0;
            HMap *hs = (HMap *)obj_alloc(sizeof(HMap), T_HSET); hs->count = 0; hs->root = (Value)root;
            cljn_gc_push((Value)hs);
            for (int64_t i = 0; i < o->len; i++) {
                HMap *cur = (HMap *)gc_stack[gc_sp - 1];
                int64_t c;
                Value nr = hset_node_assoc(cur->root, cur->count, o->items[i], &c);
                HMap *nm = (HMap *)obj_alloc(sizeof(HMap), T_HSET); nm->count = c; nm->root = nr;
                gc_stack[gc_sp - 1] = (Value)nm;
            }
            HMap *cur = (HMap *)gc_stack[gc_sp - 1];
            int64_t c;
            Value nr = hset_node_assoc(cur->root, cur->count, x, &c);
            HMap *nm = (HMap *)obj_alloc(sizeof(HMap), T_HSET); nm->count = c; nm->root = nr;
            gc_stack[gc_sp - 1] = (Value)nm;
            result = gc_stack[gc_sp - 1];
            cljn_gc_popn(1);
        } else {
            int64_t n = o->len;
            Vec *ns = (Vec *)obj_alloc(sizeof(Vec) + (size_t)(n + 1) * sizeof(Value), T_SET);
            ns->len = n + 1;
            for (int64_t i = 0; i < n; i++) ns->items[i] = o->items[i];
            ns->items[n] = x;
            result = (Value)ns;
        }
    }
    gc_disabled--;
    return result;
}
Value cljn_set_contains(Value set, Value x) {
    if (obj_type(set) == T_HSET) return b2v(node_get(((HMap *)set)->root, 0, cljn_hash(x), x) != MNOTFOUND);
    return b2v(set_member((Vec *)set, x));
}

/* ---------- hash (consistente com igualdade) ---------- */
static uint32_t hash_bytes(const char *p, size_t n) {
    uint32_t h = 2166136261u;
    for (size_t i = 0; i < n; i++) { h ^= (unsigned char)p[i]; h *= 16777619u; }
    return h;
}
uint32_t cljn_hash(Value v) {
    if (IS_FIX(v)) {
        uint64_t x = (uint64_t)(intptr_t)FIX(v);
        x = (x ^ (x >> 30)) * 0xbf58476d1ce4e5b9ull;
        x = (x ^ (x >> 27)) * 0x94d049bb133111ebull;
        return (uint32_t)(x ^ (x >> 31));
    }
    if (v == NIL) return 0;
    if (v == TRUEV) return 1;
    if (v == FALSEV) return 2;
    if (v == EMPTY) return 3;
    switch (obj_type(v)) {
        case T_STR: { Str *s = (Str *)v; return hash_bytes(s->data, s->len); }
        case T_KW:  { Str *s = (Str *)v; return hash_bytes(s->data, s->len) ^ 0x9e3779b9u; }
        default: return 7; /* chaves compostas: colisões raras; igualdade resolve */
    }
}

/* ---------- HAMT (nós) ---------- */
static MNode *mnode_alloc(int slots) { return (MNode *)obj_alloc(sizeof(MNode) + (size_t)slots * sizeof(Value), T_MNODE); }
static Value node_get(Value node, uint32_t shift, uint32_t hash, Value key) {
    if (obj_type(node) == T_MCOLL) {
        MColl *c = (MColl *)node;
        for (int64_t i = 0; i < c->n; i++) if (cljn_equal_raw(c->pairs[2 * i], key)) return c->pairs[2 * i + 1];
        return MNOTFOUND;
    }
    MNode *nd = (MNode *)node;
    uint32_t bit = 1u << ((hash >> shift) & 31);
    if (!(nd->bitmap & bit)) return MNOTFOUND;
    int idx = __builtin_popcount(nd->bitmap & (bit - 1));
    Value k = nd->arr[2 * idx];
    if (k == MNODEKEY) return node_get(nd->arr[2 * idx + 1], shift + 5, hash, key);
    if (cljn_equal_raw(k, key)) return nd->arr[2 * idx + 1];
    return MNOTFOUND;
}
static Value merge_two(uint32_t shift, uint32_t h1, Value k1, Value v1, uint32_t h2, Value k2, Value v2) {
    if (shift >= 32) {
        MColl *c = (MColl *)obj_alloc(sizeof(MColl) + 4 * sizeof(Value), T_MCOLL);
        c->hash = h1; c->n = 2;
        c->pairs[0] = k1; c->pairs[1] = v1; c->pairs[2] = k2; c->pairs[3] = v2;
        return (Value)c;
    }
    int b1 = (h1 >> shift) & 31, b2 = (h2 >> shift) & 31;
    if (b1 == b2) {
        Value sub = merge_two(shift + 5, h1, k1, v1, h2, k2, v2);
        MNode *nn = mnode_alloc(2);
        nn->bitmap = 1u << b1; nn->arr[0] = MNODEKEY; nn->arr[1] = sub;
        return (Value)nn;
    }
    MNode *nn = mnode_alloc(4);
    nn->bitmap = (1u << b1) | (1u << b2);
    if (b1 < b2) { nn->arr[0]=k1; nn->arr[1]=v1; nn->arr[2]=k2; nn->arr[3]=v2; }
    else         { nn->arr[0]=k2; nn->arr[1]=v2; nn->arr[2]=k1; nn->arr[3]=v1; }
    return (Value)nn;
}
static Value node_assoc(Value node, uint32_t shift, uint32_t hash, Value key, Value val, int *added) {
    if (obj_type(node) == T_MCOLL) {
        MColl *c = (MColl *)node;
        for (int64_t i = 0; i < c->n; i++) if (cljn_equal_raw(c->pairs[2 * i], key)) {
            *added = 0;
            MColl *nc = (MColl *)obj_alloc(sizeof(MColl) + (size_t)(2 * c->n) * sizeof(Value), T_MCOLL);
            nc->hash = c->hash; nc->n = c->n;
            for (int64_t j = 0; j < 2 * c->n; j++) nc->pairs[j] = c->pairs[j];
            nc->pairs[2 * i + 1] = val;
            return (Value)nc;
        }
        *added = 1;
        MColl *nc = (MColl *)obj_alloc(sizeof(MColl) + (size_t)(2 * (c->n + 1)) * sizeof(Value), T_MCOLL);
        nc->hash = c->hash; nc->n = c->n + 1;
        for (int64_t j = 0; j < 2 * c->n; j++) nc->pairs[j] = c->pairs[j];
        nc->pairs[2 * c->n] = key; nc->pairs[2 * c->n + 1] = val;
        return (Value)nc;
    }
    MNode *nd = (MNode *)node;
    uint32_t bit = 1u << ((hash >> shift) & 31);
    int idx = __builtin_popcount(nd->bitmap & (bit - 1));
    int cnt = __builtin_popcount(nd->bitmap);
    if (nd->bitmap & bit) {
        Value k = nd->arr[2 * idx];
        MNode *nn = mnode_alloc(2 * cnt);
        nn->bitmap = nd->bitmap;
        for (int i = 0; i < 2 * cnt; i++) nn->arr[i] = nd->arr[i];
        if (k == MNODEKEY) {
            nn->arr[2 * idx + 1] = node_assoc(nd->arr[2 * idx + 1], shift + 5, hash, key, val, added);
        } else if (cljn_equal_raw(k, key)) {
            *added = 0; nn->arr[2 * idx + 1] = val;
        } else {
            *added = 1;
            Value sub = merge_two(shift + 5, cljn_hash(k), k, nd->arr[2 * idx + 1], hash, key, val);
            nn->arr[2 * idx] = MNODEKEY; nn->arr[2 * idx + 1] = sub;
        }
        return (Value)nn;
    }
    *added = 1;
    MNode *nn = mnode_alloc(2 * (cnt + 1));
    nn->bitmap = nd->bitmap | bit;
    for (int i = 0; i < 2 * idx; i++) nn->arr[i] = nd->arr[i];
    nn->arr[2 * idx] = key; nn->arr[2 * idx + 1] = val;
    for (int i = 2 * idx; i < 2 * cnt; i++) nn->arr[i + 2] = nd->arr[i];
    return (Value)nn;
}
/* Cons cada (chave|valor) do HAMT em `gc_stack[gc_sp-1]` (acc rooteado). */
static void hmap_cons_walk(Value node, int mode /*0=keys 1=vals*/) {
    if (obj_type(node) == T_MCOLL) {
        MColl *c = (MColl *)node;
        for (int64_t i = 0; i < c->n; i++) {
            Value acc = cljn_cons(c->pairs[2 * i + mode], gc_stack[gc_sp - 1]);
            gc_stack[gc_sp - 1] = acc;
        }
        return;
    }
    MNode *nd = (MNode *)node;
    int cnt = __builtin_popcount(nd->bitmap);
    for (int idx = 0; idx < cnt; idx++) {
        Value k = nd->arr[2 * idx];
        if (k == MNODEKEY) hmap_cons_walk(nd->arr[2 * idx + 1], mode);
        else {
            Value acc = cljn_cons(mode == 0 ? k : nd->arr[2 * idx + 1], gc_stack[gc_sp - 1]);
            gc_stack[gc_sp - 1] = acc;
        }
    }
}

/* Verdadeiro se toda chave da HAMT pertence a `other` (set em qualquer repr). */
static int hnode_all_in(Value node, Value other) {
    if (obj_type(node) == T_MCOLL) {
        MColl *c = (MColl *)node;
        for (int64_t i = 0; i < c->n; i++)
            if (!cljn_truthy(cljn_contains(other, c->pairs[2 * i]))) return 0;
        return 1;
    }
    MNode *nd = (MNode *)node;
    int cnt = __builtin_popcount(nd->bitmap);
    for (int idx = 0; idx < cnt; idx++) {
        Value k = nd->arr[2 * idx];
        if (k == MNODEKEY) { if (!hnode_all_in(nd->arr[2 * idx + 1], other)) return 0; }
        else if (!cljn_truthy(cljn_contains(other, k))) return 0;
    }
    return 1;
}

/* Empurra cada chave da HAMT no gc_stack (para apply/spread sobre T_HSET). */
static int64_t hnode_push_keys(Value node) {
    int64_t extra = 0;
    if (obj_type(node) == T_MCOLL) {
        MColl *c = (MColl *)node;
        for (int64_t i = 0; i < c->n; i++) { cljn_gc_push(c->pairs[2 * i]); extra++; }
        return extra;
    }
    MNode *nd = (MNode *)node;
    int cnt = __builtin_popcount(nd->bitmap);
    for (int idx = 0; idx < cnt; idx++) {
        Value k = nd->arr[2 * idx];
        if (k == MNODEKEY) extra += hnode_push_keys(nd->arr[2 * idx + 1]);
        else { cljn_gc_push(k); extra++; }
    }
    return extra;
}

/* ---------- mapas: array-map (<=8, ordenado) + HAMT (grande) ---------- */
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
static Value hmap_from_arraymap(Map *o, Value k, Value v) {
    /* promove: array-map + (k,v) → HAMT. o/k/v rooteados pelo chamador. */
    MNode *root = mnode_alloc(0); root->bitmap = 0;
    HMap *m = (HMap *)obj_alloc(sizeof(HMap), T_HMAP);
    m->count = 0; m->root = (Value)root;
    Value hm = (Value)m;
    cljn_gc_push(hm);
    int added;
    for (int64_t i = 0; i < o->n; i++) {
        HMap *cur = (HMap *)gc_stack[gc_sp - 1];
        Value nr = node_assoc(cur->root, 0, cljn_hash(o->kv[2 * i]), o->kv[2 * i], o->kv[2 * i + 1], &added);
        HMap *nm = (HMap *)obj_alloc(sizeof(HMap), T_HMAP);
        nm->count = cur->count + added; nm->root = nr;
        gc_stack[gc_sp - 1] = (Value)nm;
    }
    HMap *cur = (HMap *)gc_stack[gc_sp - 1];
    Value nr = node_assoc(cur->root, 0, cljn_hash(k), k, v, &added);
    HMap *nm = (HMap *)obj_alloc(sizeof(HMap), T_HMAP);
    nm->count = cur->count + added; nm->root = nr;
    gc_stack[gc_sp - 1] = (Value)nm;
    Value r = gc_stack[gc_sp - 1];
    cljn_gc_popn(1);
    return r;
}
Value cljn_map_get(Value map, Value k) {
    if (obj_type(map) == T_HMAP) {
        Value r = node_get(((HMap *)map)->root, 0, cljn_hash(k), k);
        return r == MNOTFOUND ? NIL : r;
    }
    if (obj_type(map) == T_SMAP) return cljn_sorted_get(map, k);
    if (obj_type(map) != T_MAP) return NIL;
    Map *m = (Map *)map;
    int64_t i = map_index(m, k);
    return (i >= 0) ? m->kv[2 * i + 1] : NIL;
}
Value cljn_map_contains(Value map, Value k) {
    if (obj_type(map) == T_HMAP) return b2v(node_get(((HMap *)map)->root, 0, cljn_hash(k), k) != MNOTFOUND);
    if (obj_type(map) == T_SMAP) return cljn_sorted_contains(map, k);
    return b2v(obj_type(map) == T_MAP && map_index((Map *)map, k) >= 0);
}
Value cljn_map_assoc(Value map, Value k, Value v) {
    maybe_gc();
    gc_disabled++;
    Value result;
    if (obj_type(map) == T_HMAP) {
        HMap *o = (HMap *)map;
        int added;
        Value nr = node_assoc(o->root, 0, cljn_hash(k), k, v, &added);
        HMap *nm = (HMap *)obj_alloc(sizeof(HMap), T_HMAP);
        nm->count = o->count + added; nm->root = nr;
        result = (Value)nm;
    } else {
        Map *o = (Map *)map;
        int64_t at = map_index(o, k);
        int64_t n = o->n;
        if (at < 0 && n + 1 > MAP_ARRAY_MAX) {
            result = hmap_from_arraymap(o, k, v); /* promove */
        } else {
            int64_t nn = (at >= 0) ? n : n + 1;
            Map *nm = (Map *)obj_alloc(sizeof(Map) + (size_t)(2 * nn) * sizeof(Value), T_MAP);
            nm->n = nn;
            for (int64_t i = 0; i < n; i++) { nm->kv[2 * i] = o->kv[2 * i]; nm->kv[2 * i + 1] = o->kv[2 * i + 1]; }
            if (at >= 0) nm->kv[2 * at + 1] = v;
            else { nm->kv[2 * n] = k; nm->kv[2 * n + 1] = v; }
            result = (Value)nm;
        }
    }
    gc_disabled--;
    return result;
}
Value cljn_map_dissoc(Value map, Value k) {
    if (obj_type(map) == T_SMAP) return cljn_sorted_dissoc(map, k);
    if (obj_type(map) == T_HMAP) {
        HMap *m = (HMap *)map;
        if (node_get(m->root, 0, cljn_hash(k), k) == MNOTFOUND) return map;
        /* rebuild: reassoc todas as entradas exceto k (auto-dimensiona/demote). */
        maybe_gc();
        Value acc = cljn_map_alloc(0);
        cljn_gc_push(acc);
        /* itera entradas via walk que reassocia em gc_stack[sp-1] */
        /* usa uma segunda pilha lógica: acc no topo; reusa hmap_cons_walk? não — precisamos assoc */
        /* walk manual */
        /* pilha de nós simples via recursão inline: */
        extern void hmap_dissoc_walk(Value node, Value skip);
        hmap_dissoc_walk(m->root, k);
        Value r = gc_stack[gc_sp - 1];
        cljn_gc_popn(1);
        return r;
    }
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
void hmap_dissoc_walk(Value node, Value skip) {
    if (obj_type(node) == T_MCOLL) {
        MColl *c = (MColl *)node;
        for (int64_t i = 0; i < c->n; i++) if (!cljn_equal_raw(c->pairs[2 * i], skip)) {
            Value a = cljn_map_assoc(gc_stack[gc_sp - 1], c->pairs[2 * i], c->pairs[2 * i + 1]);
            gc_stack[gc_sp - 1] = a;
        }
        return;
    }
    MNode *nd = (MNode *)node;
    int cnt = __builtin_popcount(nd->bitmap);
    for (int idx = 0; idx < cnt; idx++) {
        Value k = nd->arr[2 * idx];
        if (k == MNODEKEY) hmap_dissoc_walk(nd->arr[2 * idx + 1], skip);
        else if (!cljn_equal_raw(k, skip)) {
            Value a = cljn_map_assoc(gc_stack[gc_sp - 1], k, nd->arr[2 * idx + 1]);
            gc_stack[gc_sp - 1] = a;
        }
    }
}
Value cljn_map_keys(Value map) {
    if (obj_type(map) == T_RECORD) map = ((Record *)map)->map;
    if (obj_type(map) == T_SMAP) return sorted_seq(map, 0);
    Value acc = EMPTY;
    cljn_gc_push(acc);
    if (obj_type(map) == T_HMAP) {
        hmap_cons_walk(((HMap *)map)->root, 0);
    } else {
        Map *m = (Map *)map;
        for (int64_t i = m->n - 1; i >= 0; i--) { acc = cljn_cons(m->kv[2 * i], gc_stack[gc_sp - 1]); gc_stack[gc_sp - 1] = acc; }
    }
    Value r = gc_stack[gc_sp - 1];
    cljn_gc_popn(1);
    return r;
}
/* Toda entrada do nó HAMT está presente e igual em `other` (mapa qualquer)? */
static int hmap_node_subset(Value node, Value other) {
    if (obj_type(node) == T_MCOLL) {
        MColl *c = (MColl *)node;
        for (int64_t i = 0; i < c->n; i++)
            if (!cljn_truthy(cljn_map_contains(other, c->pairs[2 * i])) ||
                !cljn_equal_raw(c->pairs[2 * i + 1], cljn_map_get(other, c->pairs[2 * i]))) return 0;
        return 1;
    }
    MNode *nd = (MNode *)node;
    int cnt = __builtin_popcount(nd->bitmap);
    for (int idx = 0; idx < cnt; idx++) {
        Value k = nd->arr[2 * idx];
        if (k == MNODEKEY) { if (!hmap_node_subset(nd->arr[2 * idx + 1], other)) return 0; }
        else if (!cljn_truthy(cljn_map_contains(other, k)) ||
                 !cljn_equal_raw(nd->arr[2 * idx + 1], cljn_map_get(other, k))) return 0;
    }
    return 1;
}
Value cljn_map_vals(Value map) {
    if (obj_type(map) == T_RECORD) map = ((Record *)map)->map;
    if (obj_type(map) == T_SMAP) return sorted_seq(map, 1);
    Value acc = EMPTY;
    cljn_gc_push(acc);
    if (obj_type(map) == T_HMAP) {
        hmap_cons_walk(((HMap *)map)->root, 1);
    } else {
        Map *m = (Map *)map;
        for (int64_t i = m->n - 1; i >= 0; i--) { acc = cljn_cons(m->kv[2 * i + 1], gc_stack[gc_sp - 1]); gc_stack[gc_sp - 1] = acc; }
    }
    Value r = gc_stack[gc_sp - 1];
    cljn_gc_popn(1);
    return r;
}

/* ---------- coleções ordenadas: árvore LLRB persistente ---------- */
/* Ordem total sobre o subconjunto suportado: números < strings < keywords < resto.
 * Dentro de cada classe: fixnum por valor; str/kw lexicográfico. Retorna -1/0/1. */
static int cmp_bytes(const char *a, size_t la, const char *b, size_t lb) {
    size_t n = la < lb ? la : lb;
    int c = memcmp(a, b, n);
    if (c != 0) return c < 0 ? -1 : 1;
    return la == lb ? 0 : (la < lb ? -1 : 1);
}
static int compare_raw(Value a, Value b) {
    if (IS_FIX(a) && IS_FIX(b)) { int64_t x = FIX(a), y = FIX(b); return x < y ? -1 : (x > y ? 1 : 0); }
    /* classe de ordenação: número=0, string=1, keyword=2, outro=3 */
    int ca = IS_FIX(a) ? 0 : (obj_type(a) == T_STR ? 1 : (obj_type(a) == T_KW ? 2 : 3));
    int cb = IS_FIX(b) ? 0 : (obj_type(b) == T_STR ? 1 : (obj_type(b) == T_KW ? 2 : 3));
    if (ca != cb) return ca < cb ? -1 : 1;
    if (ca == 1 || ca == 2) { Str *x = (Str *)a, *y = (Str *)b; return cmp_bytes(x->data, x->len, y->data, y->len); }
    /* mesma classe "outro": iguais viram 0, senão desempata por identidade estável */
    if (cljn_equal_raw(a, b)) return 0;
    return a < b ? -1 : 1;
}
Value cljn_compare(Value a, Value b) { return MK_FIX(compare_raw(a, b)); }

/* Vetor de 2 elementos [k v] (entrada de mapa). gc_disabled: sem coleta no meio. */
static Value cljn_vec_pair(Value k, Value v) {
    gc_disabled++;
    Value e = cljn_vec_empty();
    e = cljn_vec_conj(e, k);
    e = cljn_vec_conj(e, v);
    gc_disabled--;
    return e;
}

static int tn_red(Value n) { return n != NIL && ((TNode *)n)->red; }
static TNode *tn_alloc(Value key, Value val, Value left, Value right, int red) {
    TNode *n = (TNode *)obj_alloc(sizeof(TNode), T_TNODE);
    n->key = key; n->val = val; n->left = left; n->right = right; n->red = red;
    return n;
}
static TNode *tn_copy(TNode *o, Value left, Value right) {
    return tn_alloc(o->key, o->val, left, right, o->red);
}
/* rotações/flip da LLRB (assumem gc_disabled ativo: várias alocações) */
static Value tn_rot_left(TNode *h) {
    TNode *x = (TNode *)h->right;
    TNode *nh = tn_alloc(h->key, h->val, h->left, x->left, 1);
    return (Value)tn_alloc(x->key, x->val, (Value)nh, x->right, h->red);
}
static Value tn_rot_right(TNode *h) {
    TNode *x = (TNode *)h->left;
    TNode *nh = tn_alloc(h->key, h->val, x->right, h->right, 1);
    return (Value)tn_alloc(x->key, x->val, x->left, (Value)nh, h->red);
}
static Value tn_flip(TNode *h) {
    TNode *l = (TNode *)h->left, *r = (TNode *)h->right;
    TNode *nl = tn_alloc(l->key, l->val, l->left, l->right, !l->red);
    TNode *nr = tn_alloc(r->key, r->val, r->left, r->right, !r->red);
    return (Value)tn_alloc(h->key, h->val, (Value)nl, (Value)nr, !h->red);
}
static Value tn_fixup(Value hv) {
    TNode *h = (TNode *)hv;
    if (tn_red(h->right) && !tn_red(h->left)) { hv = tn_rot_left(h); h = (TNode *)hv; }
    if (tn_red(h->left) && tn_red(((TNode *)h->left)->left)) { hv = tn_rot_right(h); h = (TNode *)hv; }
    if (tn_red(h->left) && tn_red(h->right)) { hv = tn_flip(h); }
    return hv;
}
/* insere/atualiza (key,val); *added=1 se chave nova. Retorna nova sub-raiz. */
static Value tn_insert(Value hv, Value key, Value val, int *added) {
    if (hv == NIL) { *added = 1; return (Value)tn_alloc(key, val, NIL, NIL, 1); }
    TNode *h = (TNode *)hv;
    int c = compare_raw(key, h->key);
    if (c == 0) hv = (Value)tn_copy(h, h->left, h->right), ((TNode *)hv)->val = val;
    else if (c < 0) hv = (Value)tn_copy(h, tn_insert(h->left, key, val, added), h->right);
    else hv = (Value)tn_copy(h, h->left, tn_insert(h->right, key, val, added));
    return tn_fixup(hv);
}
static Value tn_get(Value hv, Value key) {
    while (hv != NIL) {
        TNode *h = (TNode *)hv;
        int c = compare_raw(key, h->key);
        if (c == 0) return h->val;
        hv = c < 0 ? h->left : h->right;
    }
    return MNOTFOUND;
}
/* Percorre em ordem DEcrescente (dir→nó→esq) fazendo prepend no acc do topo do
 * gc_stack; como cada item vai para a cabeça, o resultado final fica CREScente. */
static void tn_walk_desc(Value hv, int mode /*0=key 1=val 2=entry*/) {
    if (hv == NIL) return;
    TNode *h = (TNode *)hv;
    tn_walk_desc(h->right, mode);
    Value item;
    if (mode == 0) item = h->key;
    else if (mode == 1) item = h->val;
    else item = cljn_vec_pair(h->key, h->val); /* entrada [k v] */
    Value acc = cljn_cons(item, gc_stack[gc_sp - 1]);
    gc_stack[gc_sp - 1] = acc;
    tn_walk_desc(h->left, mode);
}

static Value sorted_alloc(int type) {
    Sorted *s = (Sorted *)obj_alloc(sizeof(Sorted), type);
    s->count = 0; s->root = NIL;
    return (Value)s;
}
Value cljn_sorted_map_empty(void) { return sorted_alloc(T_SMAP); }
Value cljn_sorted_set_empty(void) { return sorted_alloc(T_SSET); }

Value cljn_sorted_assoc(Value m, Value k, Value v) {
    maybe_gc();
    gc_disabled++;
    Sorted *o = (Sorted *)m;
    int added = 0;
    Value nr = tn_insert(o->root, k, v, &added);
    /* raiz sempre preta */
    if (nr != NIL) ((TNode *)nr)->red = 0;
    Sorted *ns = (Sorted *)obj_alloc(sizeof(Sorted), obj_type(m));
    ns->count = o->count + added; ns->root = nr;
    gc_disabled--;
    return (Value)ns;
}
Value cljn_sorted_set_conj(Value s, Value x) { return cljn_sorted_assoc(s, x, x); }
Value cljn_sorted_get(Value m, Value k) {
    Value r = tn_get(((Sorted *)m)->root, k);
    return r == MNOTFOUND ? NIL : r;
}
Value cljn_sorted_contains(Value m, Value k) {
    return b2v(tn_get(((Sorted *)m)->root, k) != MNOTFOUND);
}
/* menor chave (first): desce sempre à esquerda */
Value cljn_sorted_first(Value m) {
    Value hv = ((Sorted *)m)->root;
    if (hv == NIL) return NIL;
    TNode *h = (TNode *)hv;
    while (h->left != NIL) h = (TNode *)h->left;
    if (obj_type(m) == T_SSET) return h->key;
    return cljn_vec_pair(h->key, h->val);
}
/* lista ordenada crescente de itens (mode: 0=key 1=val 2=entry) */
static Value sorted_seq(Value m, int mode) {
    cljn_gc_push(EMPTY);
    tn_walk_desc(((Sorted *)m)->root, mode); /* prepend reverso → resulta crescente */
    Value r = gc_stack[gc_sp - 1];
    cljn_gc_popn(1);
    return r;
}
/* toda chave da árvore pertence a `other` (set qualquer repr)? */
static int tn_all_in(Value node, Value other) {
    if (node == NIL) return 1;
    TNode *h = (TNode *)node;
    if (!tn_all_in(h->left, other)) return 0;
    if (!cljn_truthy(cljn_contains(other, h->key))) return 0;
    return tn_all_in(h->right, other);
}
/* toda entrada (k,v) da árvore presente e igual em `other` (mapa qualquer repr)? */
static int tn_map_subset(Value node, Value other) {
    if (node == NIL) return 1;
    TNode *h = (TNode *)node;
    if (!tn_map_subset(h->left, other)) return 0;
    if (!cljn_truthy(cljn_map_contains(other, h->key)) ||
        !cljn_equal_raw(h->val, cljn_map_get(other, h->key))) return 0;
    return tn_map_subset(h->right, other);
}
/* Empurra itens (crescente) no gc_stack para apply/spread. entries: [k v] p/ mapa. */
static int64_t tn_push_spread(Value node, int entries) {
    if (node == NIL) return 0;
    TNode *h = (TNode *)node;
    int64_t n = tn_push_spread(h->left, entries);
    Value item = entries ? cljn_vec_pair(h->key, h->val) : h->key;
    cljn_gc_push(item);
    n++;
    n += tn_push_spread(h->right, entries);
    return n;
}
/* dissoc via rebuild: reinsere todas as entradas exceto `skip` (evita delete LLRB). */
static void tn_reassoc_walk(Value node, Value skip) {
    if (node == NIL) return;
    TNode *h = (TNode *)node;
    tn_reassoc_walk(h->left, skip);
    if (!cljn_equal_raw(h->key, skip)) {
        Value a = cljn_sorted_assoc(gc_stack[gc_sp - 1], h->key, h->val);
        gc_stack[gc_sp - 1] = a;
    }
    tn_reassoc_walk(h->right, skip);
}
Value cljn_sorted_dissoc(Value m, Value k) {
    Sorted *o = (Sorted *)m;
    if (tn_get(o->root, k) == MNOTFOUND) return m;
    maybe_gc();
    Value acc = cljn_sorted_map_empty();
    cljn_gc_push(acc);
    tn_reassoc_walk(o->root, k);
    Value r = gc_stack[gc_sp - 1];
    cljn_gc_popn(1);
    return r;
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

/* ---------- protocols (defprotocol / extend-type) ---------- */
/* Tabela global (method_id, type_key) -> impl. As entradas são malloc'd (nunca
 * liberadas) e o coletor marca key/impl como roots permanentes. */
typedef struct MethodEntry {
    int64_t method_id;
    Value key;
    Value impl;
    struct MethodEntry *next;
} MethodEntry;
static MethodEntry *method_table = NULL;

/* Chave de tipo para dispatch: records → sua keyword; builtins → fixnum distinto. */
Value cljn_type_key(Value v) {
    if (IS_FIX(v)) return MK_FIX(1000);
    if (v == NIL) return MK_FIX(1010);
    if (v == TRUEV || v == FALSEV) return MK_FIX(1011);
    if (v == EMPTY) return MK_FIX(1002);
    switch (obj_type(v)) {
        case T_STR: return MK_FIX(1001);
        case T_CONS: return MK_FIX(1002);
        case T_FN: return MK_FIX(1003);
        case T_KW: return MK_FIX(1004);
        case T_VEC: return MK_FIX(1005);
        case T_MAP: case T_HMAP: case T_SMAP: return MK_FIX(1006);
        case T_SET: case T_HSET: case T_SSET: return MK_FIX(1007);
        case T_RECORD: return ((Record *)v)->type_name;
    }
    return MK_FIX(1099);
}
void cljn_register_method(Value method_id, Value key, Value impl) {
    MethodEntry *e = xalloc(sizeof(MethodEntry));
    e->method_id = (int64_t)method_id;
    e->key = key;
    e->impl = impl;
    e->next = method_table;
    method_table = e;
}
Value cljn_lookup_method(Value method_id, Value key) {
    for (MethodEntry *e = method_table; e; e = e->next)
        if (e->method_id == (int64_t)method_id && cljn_equal_raw(e->key, key)) return e->impl;
    return NIL;
}
void cljn_no_method(Value method_id) {
    fprintf(stderr, "erro: protocolo não implementado para o tipo (método %ld)\n", (long)method_id);
    exit(1);
}
static void gc_mark_method_table(void) {
    for (MethodEntry *e = method_table; e; e = e->next) {
        gc_mark(e->key);
        gc_mark(e->impl);
    }
}

/* ---------- genéricos (dispatch por tipo) ---------- */
Value cljn_contains(Value coll, Value key);
Value cljn_get(Value coll, Value key) {
    switch (obj_type(coll)) {
        case T_RECORD: return cljn_map_get(((Record *)coll)->map, key);
        case T_MAP: case T_HMAP: return cljn_map_get(coll, key);
        case T_VEC: {
            PVec *v = (PVec *)coll;
            if (IS_FIX(key)) { int64_t i = FIX(key); if (i >= 0 && i < v->count) return pv_nth(v, i); }
            return NIL;
        }
        case T_SET: return set_member((Vec *)coll, key) ? key : NIL;
        case T_HSET: return (node_get(((HMap *)coll)->root, 0, cljn_hash(key), key) != MNOTFOUND) ? key : NIL;
        case T_SMAP: return cljn_sorted_get(coll, key);
        case T_SSET: return (tn_get(((Sorted *)coll)->root, key) != MNOTFOUND) ? key : NIL;
        default: return NIL;
    }
}
Value cljn_contains(Value coll, Value key) {
    switch (obj_type(coll)) {
        case T_RECORD: return cljn_map_contains(((Record *)coll)->map, key);
        case T_MAP: case T_HMAP: return cljn_map_contains(coll, key);
        case T_SET: case T_HSET: return cljn_set_contains(coll, key);
        case T_SMAP: case T_SSET: return cljn_sorted_contains(coll, key);
        case T_VEC: { PVec *v = (PVec *)coll; return b2v(IS_FIX(key) && FIX(key) >= 0 && FIX(key) < v->count); }
        default: return FALSEV;
    }
}
Value cljn_conj(Value coll, Value x) {
    switch (obj_type(coll)) {
        case T_VEC: return cljn_vec_conj(coll, x);
        case T_SET: case T_HSET: return cljn_set_conj(coll, x);
        case T_SSET: return cljn_sorted_set_conj(coll, x);
        case T_CONS: return cljn_cons(x, coll);
        case T_MAP: case T_HMAP:
            if (obj_type(x) == T_VEC && ((PVec *)x)->count == 2)
                return cljn_map_assoc(coll, pv_nth((PVec *)x, 0), pv_nth((PVec *)x, 1));
            die("conj em mapa requer [k v]");
            return coll;
        case T_SMAP:
            if (obj_type(x) == T_VEC && ((PVec *)x)->count == 2)
                return cljn_sorted_assoc(coll, pv_nth((PVec *)x, 0), pv_nth((PVec *)x, 1));
            die("conj em sorted-map requer [k v]");
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
        case T_MAP: case T_HMAP: return cljn_map_assoc(coll, k, v);
        case T_SMAP: return cljn_sorted_assoc(coll, k, v);
        case T_VEC: return cljn_vec_assoc(coll, k, v);
        default:
            if (coll == NIL) { Value m = cljn_map_alloc(0); return cljn_map_assoc(m, k, v); }
            die("assoc: coleção não suportada");
            return coll;
    }
}
Value cljn_nth(Value coll, Value idx) {
    int64_t i = IS_FIX(idx) ? FIX(idx) : -1;
    if (i < 0) die("nth: índice inválido");
    switch (obj_type(coll)) {
        case T_VEC: { PVec *v = (PVec *)coll; if (i < v->count) return pv_nth(v, i); break; }
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
    if (t == T_VEC) return pv_nth((PVec *)coll, i);
    Value c = coll;
    while (i-- > 0) c = ((Cons *)c)->tail;
    return ((Cons *)c)->head;
}
static int64_t seq_len(Value coll, int t) {
    if (t == T_VEC) return ((PVec *)coll)->count;
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
    {
        int as = (ta == T_SET || ta == T_HSET || ta == T_SSET);
        int bs = (tb == T_SET || tb == T_HSET || tb == T_SSET);
        if (as && bs) {
            int64_t ca = (ta == T_HSET) ? ((HMap *)a)->count : (ta == T_SSET ? ((Sorted *)a)->count : ((Vec *)a)->len);
            int64_t cb = (tb == T_HSET) ? ((HMap *)b)->count : (tb == T_SSET ? ((Sorted *)b)->count : ((Vec *)b)->len);
            if (ca != cb) return 0;
            /* toda entrada de `a` presente em `b` (contains dispatcha por repr) */
            if (ta == T_HSET) return hnode_all_in(((HMap *)a)->root, b);
            if (ta == T_SSET) return tn_all_in(((Sorted *)a)->root, b);
            Vec *x = (Vec *)a;
            for (int64_t i = 0; i < x->len; i++)
                if (!cljn_truthy(cljn_contains(b, x->items[i]))) return 0;
            return 1;
        }
    }
    {
        int am = (ta == T_MAP || ta == T_HMAP || ta == T_SMAP);
        int bm = (tb == T_MAP || tb == T_HMAP || tb == T_SMAP);
        if (am && bm) {
            int64_t ca = (ta == T_HMAP) ? ((HMap *)a)->count : (ta == T_SMAP ? ((Sorted *)a)->count : ((Map *)a)->n);
            int64_t cb = (tb == T_HMAP) ? ((HMap *)b)->count : (tb == T_SMAP ? ((Sorted *)b)->count : ((Map *)b)->n);
            if (ca != cb) return 0;
            /* toda entrada de `a` presente e igual em `b` (dispatch cobre cross-repr) */
            if (ta == T_HMAP) return hmap_node_subset(((HMap *)a)->root, b);
            if (ta == T_SMAP) return tn_map_subset(((Sorted *)a)->root, b);
            Map *x = (Map *)a;
            for (int64_t i = 0; i < x->n; i++) {
                Value k = x->kv[2 * i];
                if (!cljn_truthy(cljn_map_contains(b, k)) || !cljn_equal_raw(x->kv[2 * i + 1], cljn_map_get(b, k)))
                    return 0;
            }
            return 1;
        }
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
        case T_SET: return b2v(((Vec *)v)->len == 0);
        case T_HSET: return b2v(((HMap *)v)->count == 0);
        case T_VEC: return b2v(((PVec *)v)->count == 0);
        case T_MAP: return b2v(((Map *)v)->n == 0);
        case T_HMAP: return b2v(((HMap *)v)->count == 0);
        case T_SMAP: case T_SSET: return b2v(((Sorted *)v)->count == 0);
    }
    return FALSEV;
}
Value cljn_first(Value v) {
    if (v == EMPTY || v == NIL) return NIL;
    switch (obj_type(v)) {
        case T_CONS: return ((Cons *)v)->head;
        case T_SET: { Vec *x = (Vec *)v; return x->len > 0 ? x->items[0] : NIL; }
        case T_HSET: {
            cljn_gc_push(EMPTY);
            hmap_cons_walk(((HMap *)v)->root, 0);
            Value l = gc_stack[gc_sp - 1];
            cljn_gc_popn(1);
            return l == EMPTY ? NIL : ((Cons *)l)->head;
        }
        case T_VEC: { PVec *x = (PVec *)v; return x->count > 0 ? pv_nth(x, 0) : NIL; }
        case T_SMAP: case T_SSET: return cljn_sorted_first(v);
    }
    die("first: não é uma sequência"); return NIL;
}
Value cljn_rest(Value v) {
    if (v == EMPTY || v == NIL) return EMPTY;
    if (obj_type(v) == T_CONS) return ((Cons *)v)->tail;
    if (obj_type(v) == T_HSET) {
        cljn_gc_push(EMPTY);
        hmap_cons_walk(((HMap *)v)->root, 0);
        Value l = gc_stack[gc_sp - 1];
        cljn_gc_popn(1);
        return l == EMPTY ? EMPTY : ((Cons *)l)->tail;
    }
    if (obj_type(v) == T_SMAP || obj_type(v) == T_SSET) {
        Value l = sorted_seq(v, obj_type(v) == T_SSET ? 0 : 2);
        return l == EMPTY ? EMPTY : ((Cons *)l)->tail;
    }
    if (obj_type(v) == T_SET || obj_type(v) == T_VEC) {
        int is_vec = obj_type(v) == T_VEC;
        int64_t len = is_vec ? ((PVec *)v)->count : ((Vec *)v)->len;
        Value acc = EMPTY;
        cljn_gc_push(acc); /* rooteia o acumulador durante a construção */
        for (int64_t i = len - 1; i >= 1; i--) {
            Value el = is_vec ? pv_nth((PVec *)v, i) : ((Vec *)v)->items[i];
            acc = cljn_cons(el, acc);
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
        case T_SET: return MK_FIX(((Vec *)v)->len);
        case T_HSET: return MK_FIX(((HMap *)v)->count);
        case T_VEC: return MK_FIX(((PVec *)v)->count);
        case T_MAP: return MK_FIX(((Map *)v)->n);
        case T_HMAP: return MK_FIX(((HMap *)v)->count);
        case T_SMAP: case T_SSET: return MK_FIX(((Sorted *)v)->count);
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

static void write_val(SB *b, Value v, int for_str);
static void sb_write_hmap(SB *b, Value node, int for_str, int *first) {
    if (obj_type(node) == T_MCOLL) {
        MColl *c = (MColl *)node;
        for (int64_t i = 0; i < c->n; i++) {
            if (!*first) sb_str(b, ", ");
            *first = 0;
            write_val(b, c->pairs[2 * i], for_str);
            sb_putc(b, ' ');
            write_val(b, c->pairs[2 * i + 1], for_str);
        }
        return;
    }
    MNode *nd = (MNode *)node;
    int cnt = __builtin_popcount(nd->bitmap);
    for (int idx = 0; idx < cnt; idx++) {
        Value k = nd->arr[2 * idx];
        if (k == MNODEKEY) { sb_write_hmap(b, nd->arr[2 * idx + 1], for_str, first); }
        else {
            if (!*first) sb_str(b, ", ");
            *first = 0;
            write_val(b, k, for_str);
            sb_putc(b, ' ');
            write_val(b, nd->arr[2 * idx + 1], for_str);
        }
    }
}
static void sb_write_hset(SB *b, Value node, int for_str, int *first) {
    if (obj_type(node) == T_MCOLL) {
        MColl *c = (MColl *)node;
        for (int64_t i = 0; i < c->n; i++) {
            if (!*first) sb_putc(b, ' ');
            *first = 0;
            write_val(b, c->pairs[2 * i], for_str);
        }
        return;
    }
    MNode *nd = (MNode *)node;
    int cnt = __builtin_popcount(nd->bitmap);
    for (int idx = 0; idx < cnt; idx++) {
        Value k = nd->arr[2 * idx];
        if (k == MNODEKEY) { sb_write_hset(b, nd->arr[2 * idx + 1], for_str, first); }
        else {
            if (!*first) sb_putc(b, ' ');
            *first = 0;
            write_val(b, k, for_str);
        }
    }
}
/* percurso in-order da árvore ordenada para impressão (crescente). */
static void sb_write_tree(SB *b, Value node, int is_map, int for_str, int *first) {
    if (node == NIL) return;
    TNode *h = (TNode *)node;
    sb_write_tree(b, h->left, is_map, for_str, first);
    if (!*first) sb_str(b, is_map ? ", " : " ");
    *first = 0;
    write_val(b, h->key, for_str);
    if (is_map) { sb_putc(b, ' '); write_val(b, h->val, for_str); }
    sb_write_tree(b, h->right, is_map, for_str, first);
}
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
            PVec *x=(PVec*)v; sb_putc(b,'[');
            for (int64_t i=0;i<x->count;i++){ if(i) sb_putc(b,' '); write_val(b,pv_nth(x,i),for_str); }
            sb_putc(b,']'); return;
        }
        case T_SET: {
            Vec *x=(Vec*)v; sb_str(b,"#{");
            for (int64_t i=0;i<x->len;i++){ if(i) sb_putc(b,' '); write_val(b,x->items[i],for_str); }
            sb_putc(b,'}'); return;
        }
        case T_HSET: {
            sb_str(b,"#{"); int first=1; sb_write_hset(b, ((HMap*)v)->root, for_str, &first); sb_putc(b,'}'); return;
        }
        case T_MAP: {
            Map *m=(Map*)v; sb_putc(b,'{');
            for (int64_t i=0;i<m->n;i++){ if(i) sb_str(b,", "); write_val(b,m->kv[2*i],for_str); sb_putc(b,' '); write_val(b,m->kv[2*i+1],for_str); }
            sb_putc(b,'}'); return;
        }
        case T_HMAP: {
            sb_putc(b,'{'); int first=1; sb_write_hmap(b, ((HMap*)v)->root, for_str, &first); sb_putc(b,'}'); return;
        }
        case T_SSET: {
            sb_str(b,"#{"); int first=1; sb_write_tree(b, ((Sorted*)v)->root, 0, for_str, &first); sb_putc(b,'}'); return;
        }
        case T_SMAP: {
            sb_putc(b,'{'); int first=1; sb_write_tree(b, ((Sorted*)v)->root, 1, for_str, &first); sb_putc(b,'}'); return;
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
