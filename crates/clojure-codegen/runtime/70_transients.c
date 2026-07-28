
/*
 * Batched mutation through transient collections.
 *
 * A transient vector shares the persistent trie and mutates only nodes carrying
 * its unique edit token; other nodes copy on first write. Map/set transients are
 * mutable boxes around persistent values. persistent! invalidates vector
 * ownership and returns a wrapper sharing the frozen trie.
 *
 * INVARIANT: mutation after vector persistent! is rejected. The analyzer only
 * threads transient receivers along uniqueness-proven paths.
 */
/* Convert a persistent collection to its transient representation. O(1). */
Value cljn_transient(Value coll) {
    if (obj_type(coll) == T_VEC) {
        PVec *o = (PVec *)coll;
        maybe_gc();
        gc_disabled++;
        Value edit = cljn_edit_new();
        TVec *tv = (TVec *)obj_alloc(sizeof(TVec), T_TVEC);
        o = (PVec *)coll;
        tv->count = o->count;
        tv->shift = o->shift;
        tv->root = o->root;
        tv->tail = (Value)vnode_copy_edit((VNode *)o->tail, edit);
        tv->tail_len = o->tail_len;
        tv->edit = edit;
        gc_disabled--;
        return (Value)tv;
    }
    /* Maps and sets use a mutable box around their persistent value. */
    cljn_gc_push(coll);
    TBox *b = (TBox *)obj_alloc(sizeof(TBox), T_TBOX);
    cljn_gc_popn(1);
    b->inner = coll;
    return (Value)b;
}
static void tv_check(TVec *tv, const char *op) {
    if (tv->edit == NIL) { fprintf(stderr, "erro: %s: transiente já persistido\n", op); exit(1); }
}
/*
 * Mutate transient `t` by adjoining `x` and return the same transient.
 *
 * Complexity: amortized O(1) for vectors. Invalid receivers or invalidated
 * vector tokens terminate through the fatal path.
 */
Value cljn_conj_bang(Value t, Value x) {
    if (obj_type(t) == T_TVEC) {
        TVec *tv = (TVec *)t;
        tv_check(tv, "conj!");
        maybe_gc();
        gc_disabled++;
        if (tv->tail_len < VWIDTH) {
            VNode *tail = vnode_editable((VNode *)tv->tail, tv->edit);
            tail->slots[tv->tail_len] = x;
            tv->tail = (Value)tail;
            tv->tail_len++;
        } else {
            VNode *tailnode = (VNode *)tv->tail;
            Value edit = tv->edit;
            VNode *newroot;
            int64_t newshift = tv->shift;
            if ((tv->count >> VBITS) > (1LL << tv->shift)) {
                newroot = vnode_new_edit(edit);
                newroot->slots[0] = tv->root;
                newroot->slots[1] = (Value)new_path_edit(tv->shift, tailnode, edit);
                newshift += VBITS;
            } else {
                newroot = tv_push_tail(tv->shift, (VNode *)tv->root, tailnode, tv->count, edit);
            }
            VNode *nt = vnode_new_edit(edit);
            nt->slots[0] = x;
            tv->root = (Value)newroot;
            tv->shift = newshift;
            tv->tail = (Value)nt;
            tv->tail_len = 1;
        }
        tv->count++;
        gc_disabled--;
        return t;
    }
    if (obj_type(t) == T_TBOX) {
        TBox *b = (TBox *)t;
        b->inner = cljn_conj(b->inner, x);
        return t;
    }
    die("conj!: não é um transient");
    return t;
}
/* Mutate one transient association and return the same transient. O(log32 n)
 * for vectors; boxed collections delegate to persistent association. */
Value cljn_assoc_bang(Value t, Value k, Value v) {
    if (obj_type(t) == T_TVEC) {
        TVec *tv = (TVec *)t;
        tv_check(tv, "assoc!");
        if (!IS_FIX(k)) die("assoc!: índice de vetor deve ser inteiro");
        int64_t i = FIX(k);
        if (i == tv->count) return cljn_conj_bang(t, v);
        if (i < 0 || i > tv->count) die("assoc!: índice fora dos limites");
        maybe_gc();
        gc_disabled++;
        int64_t tailoff = tv->count - tv->tail_len;
        if (i >= tailoff) {
            VNode *tail = vnode_editable((VNode *)tv->tail, tv->edit);
            tail->slots[i - tailoff] = v;
            tv->tail = (Value)tail;
        } else {
            tv->root = (Value)tv_do_assoc(tv->shift, (VNode *)tv->root, i, v, tv->edit);
        }
        gc_disabled--;
        return t;
    }
    if (obj_type(t) == T_TBOX) {
        TBox *b = (TBox *)t;
        b->inner = cljn_assoc(b->inner, k, v);
        return t;
    }
    die("assoc!: não é um transient");
    return t;
}
/* Remove a key from a boxed transient map and return the same box. */
Value cljn_dissoc_bang(Value t, Value k) {
    if (obj_type(t) == T_TBOX) {
        TBox *b = (TBox *)t;
        b->inner = cljn_map_dissoc(b->inner, k);
        return t;
    }
    die("dissoc!: requer um mapa transiente");
    return t;
}
/*
 * Freeze a transient and return its persistent value.
 *
 * Vector freezing is O(1) and invalidates the edit token. Boxed map/set
 * transients return their inner persistent collection.
 */
Value cljn_persistent_bang(Value t) {
    if (obj_type(t) == T_TBOX) return ((TBox *)t)->inner;
    if (obj_type(t) == T_TVEC) {
        TVec *tv = (TVec *)t;
        tv_check(tv, "persistent!");
        tv->edit = NIL;
        maybe_gc();
        gc_disabled++;
        PVec *nv = (PVec *)obj_alloc(sizeof(PVec), T_VEC);
        tv = (TVec *)t;
        nv->count = tv->count;
        nv->shift = tv->shift;
        nv->root = tv->root;
        nv->tail = tv->tail;
        nv->tail_len = tv->tail_len;
        gc_disabled--;
        return (Value)nv;
    }
    die("persistent!: não é um transient");
    return t;
}
