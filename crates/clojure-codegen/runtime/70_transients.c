
/* ---------- transients (mutação em lote) ---------- */
Value cljn_transient(Value coll) {
    if (obj_type(coll) == T_VEC) {
        PVec *v = (PVec *)coll;
        cljn_gc_push(coll); /* rooteia durante o obj_alloc */
        TVec *tv = (TVec *)obj_alloc(sizeof(TVec), T_TVEC);
        cljn_gc_popn(1);
        int64_t n = v->count;
        int64_t cap = n < 8 ? 8 : n;
        tv->items = (Value *)xalloc((size_t)cap * sizeof(Value));
        tv->len = n; tv->cap = cap;
        v = (PVec *)coll; /* revalida (não-móvel) */
        for (int64_t i = 0; i < n; i++) tv->items[i] = pv_nth(v, i);
        return (Value)tv;
    }
    /* mapas/sets: caixa mutável sobre o valor persistente */
    cljn_gc_push(coll);
    TBox *b = (TBox *)obj_alloc(sizeof(TBox), T_TBOX);
    cljn_gc_popn(1);
    b->inner = coll;
    return (Value)b;
}
static void tvec_grow(TVec *tv) {
    if (tv->len < tv->cap) return;
    int64_t ncap = tv->cap * 2;
    tv->items = (Value *)xrealloc(tv->items, (size_t)ncap * sizeof(Value));
    tv->cap = ncap;
}
Value cljn_conj_bang(Value t, Value x) {
    if (obj_type(t) == T_TVEC) {
        TVec *tv = (TVec *)t;
        tvec_grow(tv); /* realloc: sem obj_alloc, sem GC no meio */
        tv->items[tv->len++] = x;
        return t;
    }
    if (obj_type(t) == T_TBOX) {
        TBox *b = (TBox *)t;
        b->inner = cljn_conj(b->inner, x); /* b rooteado pelo caller; inner idem */
        return t;
    }
    die("conj!: não é um transient");
    return t;
}
Value cljn_assoc_bang(Value t, Value k, Value v) {
    if (obj_type(t) == T_TVEC) {
        TVec *tv = (TVec *)t;
        if (!IS_FIX(k)) die("assoc!: índice de vetor deve ser inteiro");
        int64_t i = FIX(k);
        if (i >= 0 && i < tv->len) { tv->items[i] = v; return t; }
        if (i == tv->len) { tvec_grow(tv); tv->items[tv->len++] = v; return t; }
        die("assoc!: índice fora dos limites");
    }
    if (obj_type(t) == T_TBOX) {
        TBox *b = (TBox *)t;
        b->inner = cljn_assoc(b->inner, k, v);
        return t;
    }
    die("assoc!: não é um transient");
    return t;
}
Value cljn_dissoc_bang(Value t, Value k) {
    if (obj_type(t) == T_TBOX) {
        TBox *b = (TBox *)t;
        b->inner = cljn_map_dissoc(b->inner, k);
        return t;
    }
    die("dissoc!: requer um mapa transiente");
    return t;
}
Value cljn_persistent_bang(Value t) {
    if (obj_type(t) == T_TBOX) return ((TBox *)t)->inner;
    if (obj_type(t) == T_TVEC) {
        TVec *tv = (TVec *)t;
        Value acc = cljn_vec_empty();
        cljn_gc_push(acc); /* acc rooteado; t rooteado pelo caller */
        for (int64_t i = 0; i < tv->len; i++) {
            acc = cljn_vec_conj(acc, ((TVec *)t)->items[i]);
            gc_stack[gc_sp - 1] = acc;
        }
        cljn_gc_popn(1);
        return acc;
    }
    die("persistent!: não é um transient");
    return t;
}
