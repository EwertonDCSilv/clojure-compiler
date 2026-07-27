
/* ---------- transients (mutação em lote) ----------
 * Vetor transiente ESTRUTURAL (estilo Clojure): compartilha a trie do vetor
 * persistente e muta in-place apenas os nós que possui (edit == tv->edit); nós
 * compartilhados são copiados-e-marcados no primeiro write. transient/persistent!
 * são O(1); conj!/assoc! são O(1) amortizado / O(log32 n) SEM alocar um novo
 * wrapper por passo. persistent! invalida o token (edit=NIL) e devolve um PVec
 * que compartilha a trie agora congelada. */
Value cljn_transient(Value coll) {
    if (obj_type(coll) == T_VEC) {
        PVec *o = (PVec *)coll;
        maybe_gc();
        gc_disabled++;
        Value edit = cljn_edit_new();
        TVec *tv = (TVec *)obj_alloc(sizeof(TVec), T_TVEC);
        o = (PVec *)coll; /* revalida (não-móvel) */
        tv->count = o->count;
        tv->shift = o->shift;
        tv->root = o->root; /* trie compartilhada; nós são copiados no 1º write */
        tv->tail = (Value)vnode_copy_edit((VNode *)o->tail, edit); /* tail editável */
        tv->tail_len = o->tail_len;
        tv->edit = edit;
        gc_disabled--;
        return (Value)tv;
    }
    /* mapas/sets: caixa mutável sobre o valor persistente */
    cljn_gc_push(coll);
    TBox *b = (TBox *)obj_alloc(sizeof(TBox), T_TBOX);
    cljn_gc_popn(1);
    b->inner = coll;
    return (Value)b;
}
static void tv_check(TVec *tv, const char *op) {
    if (tv->edit == NIL) { fprintf(stderr, "erro: %s: transiente já persistido\n", op); exit(1); }
}
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
            VNode *tailnode = (VNode *)tv->tail; /* cheio: empurra na trie */
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
        b->inner = cljn_conj(b->inner, x); /* b rooteado pelo caller; inner idem */
        return t;
    }
    die("conj!: não é um transient");
    return t;
}
Value cljn_assoc_bang(Value t, Value k, Value v) {
    if (obj_type(t) == T_TVEC) {
        TVec *tv = (TVec *)t;
        tv_check(tv, "assoc!");
        if (!IS_FIX(k)) die("assoc!: índice de vetor deve ser inteiro");
        int64_t i = FIX(k);
        if (i == tv->count) return cljn_conj_bang(t, v); /* append */
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
        tv_check(tv, "persistent!");
        tv->edit = NIL; /* invalida o token: writes futuros veem os nós como imutáveis */
        maybe_gc();
        gc_disabled++;
        PVec *nv = (PVec *)obj_alloc(sizeof(PVec), T_VEC);
        tv = (TVec *)t;
        nv->count = tv->count;
        nv->shift = tv->shift;
        nv->root = tv->root; /* trie compartilhada (congelada) */
        nv->tail = tv->tail;
        nv->tail_len = tv->tail_len;
        gc_disabled--;
        return (Value)nv;
    }
    die("persistent!: não é um transient");
    return t;
}
