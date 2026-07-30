/*
 * Persistent and transient 32-way bitmapped vector tries.
 *
 * Persistent updates path-copy O(log32 n) nodes and share the remainder.
 * Transient helpers mutate nodes carrying the caller's unique edit token.
 * GC: compound operations enter bounded no-GC regions because intermediate
 * nodes are not individually rooted.
 */
static VNode *vnode_new(void) {
    VNode *n = (VNode *)obj_alloc(sizeof(VNode), T_VNODE);
    n->edit = NIL;
    for (int i = 0; i < VWIDTH; i++) n->slots[i] = NIL;
    return n;
}
static VNode *vnode_copy(VNode *src) {
    VNode *n = (VNode *)obj_alloc(sizeof(VNode), T_VNODE);
    n->edit = NIL;
    memcpy(n->slots, src->slots, sizeof(n->slots));
    return n;
}
/* Allocate a unique transient ownership token. */
Value cljn_edit_new(void) { return (Value)obj_alloc(sizeof(Edit), T_EDIT); }
static VNode *vnode_new_edit(Value edit) { VNode *n = vnode_new(); n->edit = edit; return n; }
static VNode *vnode_copy_edit(VNode *src, Value edit) { VNode *n = vnode_copy(src); n->edit = edit; return n; }
/* Return an owned node or copy it and attach the requested edit token. */
static VNode *vnode_editable(VNode *node, Value edit) {
    return (node->edit == edit) ? node : vnode_copy_edit(node, edit);
}
static VNode *new_path_edit(int64_t level, VNode *node, Value edit) {
    if (level == 0) return node;
    VNode *ret = vnode_new_edit(edit);
    ret->slots[0] = (Value)new_path_edit(level - VBITS, node, edit);
    return ret;
}
static VNode *tv_push_tail(int64_t level, VNode *parent, VNode *tailnode, int64_t cnt, Value edit) {
    int subidx = (int)(((cnt - 1) >> level) & VMASK);
    VNode *ret = vnode_editable(parent, edit);
    VNode *insert;
    if (level == VBITS) {
        insert = tailnode;
    } else {
        Value child = ret->slots[subidx];
        insert = (obj_type(child) == T_VNODE)
                     ? tv_push_tail(level - VBITS, (VNode *)child, tailnode, cnt, edit)
                     : new_path_edit(level - VBITS, tailnode, edit);
    }
    ret->slots[subidx] = (Value)insert;
    return ret;
}
static VNode *tv_do_assoc(int64_t level, VNode *node, int64_t i, Value x, Value edit) {
    VNode *ret = vnode_editable(node, edit);
    if (level == 0) {
        ret->slots[i & VMASK] = x;
    } else {
        int subidx = (int)((i >> level) & VMASK);
        ret->slots[subidx] = (Value)tv_do_assoc(level - VBITS, (VNode *)ret->slots[subidx], i, x, edit);
    }
    return ret;
}
/*
 * Allocate an empty persistent vector.
 *
 * GC: performs one safepoint before allocating the wrapper, root, and tail in a
 * bounded no-GC region. Complexity: O(1).
 */
Value cljn_vec_empty(void) {
    maybe_gc();
    gc_disabled++;
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
/* Return `vec` with `x` appended using structural sharing.
 * GC: vec and x are rooted by the caller. Complexity: amortized O(1). */
Value cljn_vec_conj(Value vec, Value x) {
    maybe_gc();
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
/*
 * Return `vec` with tagged fixnum `idx` associated to `x`.
 *
 * An index equal to count appends; other out-of-range indices are fatal.
 * GC: vec and x are rooted by the caller. Complexity: O(log32 n).
 */
Value cljn_vec_assoc(Value vec, Value idx, Value x) {
    PVec *o = (PVec *)vec;
    int64_t i = IS_FIX(idx) ? FIX(idx) : -1;
    if (i < 0 || i > o->count) die("assoc: índice fora dos limites do vetor");
    if (i == o->count) return cljn_vec_conj(vec, x);
    maybe_gc();
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
/* Return the vector count as an untagged integer; does not allocate. */
int64_t cljn_vec_count_raw(Value v) { return ((PVec *)v)->count; }
