/*
 * Scalar constructors, cons cells, first-class functions, and call ABI.
 *
 * ABI: callable entries receive `(self, argc, argv)` and return a tagged Value.
 * Counts and indices passed to helpers are raw machine integers unless stated.
 * GC: generated callers root heap-capable arguments before allocating helpers.
 */
/* Allocate and copy `len` raw UTF-8 bytes into a runtime string. O(len). */
Value cljn_str_from(const char *p, long len) {
    Str *s = (Str *)obj_alloc(sizeof(Str), T_STR);
    s->len = (size_t)len;
    s->data = (len > 0) ? xalloc((size_t)len) : NULL;
    if (len > 0) memcpy(s->data, p, (size_t)len);
    return (Value)s;
}
/* Return the immediate empty-list sentinel; does not allocate. */
Value cljn_empty(void) { return EMPTY; }
/* Allocate a cons cell from already-rooted head and tail Values. O(1). */
Value cljn_cons(Value h, Value t) {
    Cons *c = (Cons *)obj_alloc(sizeof(Cons), T_CONS);
    c->head = h;
    c->tail = t;
    return (Value)c;
}

/* Allocate a function object with raw code/arity/count and NIL capture slots. */
Value cljn_make_fn(Value code, Value arity, Value nfree) {
    /* GC: capture values remain rooted until generated code fills freev. */
    Fn *f = (Fn *)obj_alloc(sizeof(Fn) + (size_t)nfree * sizeof(Value), T_FN);
    f->code = (void *)code;
    f->arity = (int64_t)arity;
    f->nfree = (int64_t)nfree;
    for (int64_t i = 0; i < f->nfree; i++) f->freev[i] = NIL;
    return (Value)f;
}
/* Store captured Value `v` at raw index `i`; does not allocate. */
void cljn_fn_set_free(Value fn, Value i, Value v) { ((Fn *)fn)->freev[(size_t)i] = v; }
/* Return a captured Value from raw index `i`; does not allocate. */
Value cljn_fn_free(Value fn, Value i) { return ((Fn *)fn)->freev[(size_t)i]; }
/* Return the raw native entry address carried by a function object. */
Value cljn_fn_code(Value fn) { return (Value)((Fn *)fn)->code; }
/* Validate that `fn` is callable as a function, or exit through the fatal path. */
void cljn_check_fn(Value fn) {
    if (obj_type(fn) != T_FN) die("valor chamado não é uma função");
}

/* Return a raw pointer to `argc` arguments at the shadow-stack top. */
Value cljn_argv(Value argc) {
    return (Value)&gc_stack[gc_sp - (size_t)argc];
}
/* Validate exact raw argument count; emits a Portuguese fatal diagnostic. */
void cljn_check_arity(Value argc, Value expected) {
    if ((int64_t)argc != (int64_t)expected) {
        fprintf(stderr, "erro: aridade errada (esperava %ld, recebeu %ld)\n",
                (long)expected, (long)argc);
        exit(1);
    }
}
/* Validate minimum raw argument count; emits a Portuguese fatal diagnostic. */
void cljn_check_arity_min(Value argc, Value minv) {
    if ((int64_t)argc < (int64_t)minv) {
        fprintf(stderr, "erro: aridade errada (esperava ao menos %ld, recebeu %ld)\n",
                (long)minv, (long)argc);
        exit(1);
    }
}
/* Materialize argv[nfixed..argc) as a list for a rest parameter. O(argc). */
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
/* Push `coll` elements after fixed arguments and return total raw argc.
 * GC: `coll` is rooted by the caller; pushed elements become argument roots. */
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

/* Allocate and copy `len` raw UTF-8 bytes into a keyword object. O(len). */
Value cljn_kw(const char *p, long len) {
    Str *s = (Str *)obj_alloc(sizeof(Str), T_KW);
    s->len = (size_t)len;
    s->data = (len > 0) ? xalloc((size_t)len) : NULL;
    if (len > 0) memcpy(s->data, p, (size_t)len);
    return (Value)s;
}
