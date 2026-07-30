/*
 * Persistent sorted maps and sets implemented as LLRB trees.
 *
 * The current total order is fixnums, strings, keywords, then other Values.
 * Fixnums compare numerically, strings and keywords lexicographically, and
 * unequal remaining values use stable identity order. Updates path-copy and
 * rebalance O(log n) nodes.
 */
static int cmp_bytes(const char *a, size_t la, const char *b, size_t lb) {
    size_t n = la < lb ? la : lb;
    int c = memcmp(a, b, n);
    if (c != 0) return c < 0 ? -1 : 1;
    return la == lb ? 0 : (la < lb ? -1 : 1);
}
static int compare_raw(Value a, Value b) {
    if (IS_FIX(a) && IS_FIX(b)) { int64_t x = FIX(a), y = FIX(b); return x < y ? -1 : (x > y ? 1 : 0); }
    /* Ordering class: fixnum, string, keyword, then remaining Values. */
    int ca = IS_FIX(a) ? 0 : (obj_type(a) == T_STR ? 1 : (obj_type(a) == T_KW ? 2 : 3));
    int cb = IS_FIX(b) ? 0 : (obj_type(b) == T_STR ? 1 : (obj_type(b) == T_KW ? 2 : 3));
    if (ca != cb) return ca < cb ? -1 : 1;
    if (ca == 1 || ca == 2) { Str *x = (Str *)a, *y = (Str *)b; return cmp_bytes(x->data, x->len, y->data, y->len); }
    /* Equal composites compare as zero; identity breaks remaining ties. */
    if (cljn_equal_raw(a, b)) return 0;
    return a < b ? -1 : 1;
}
/* Compare two Values and return tagged fixnum -1, 0, or 1; does not allocate. */
Value cljn_compare(Value a, Value b) { return MK_FIX(compare_raw(a, b)); }

/* Build a two-element map-entry vector inside an existing no-GC context. */
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
/* LLRB rotations and color flip require the caller's no-GC region. */
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
/* Insert or update (key, value), returning a rebalanced copied subtree. */
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
/* Traverse descending while prepending, yielding an ascending final list. */
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
/* Allocate an empty persistent sorted map. O(1). */
Value cljn_sorted_map_empty(void) { return sorted_alloc(T_SMAP); }
/* Allocate an empty persistent sorted set. O(1). */
Value cljn_sorted_set_empty(void) { return sorted_alloc(T_SSET); }

/* Associate a key/value pair and return a persistent sorted collection. O(log n). */
Value cljn_sorted_assoc(Value m, Value k, Value v) {
    maybe_gc();
    gc_disabled++;
    Sorted *o = (Sorted *)m;
    int added = 0;
    Value nr = tn_insert(o->root, k, v, &added);
    /* INVARIANT: an LLRB root is black. */
    if (nr != NIL) ((TNode *)nr)->red = 0;
    Sorted *ns = (Sorted *)obj_alloc(sizeof(Sorted), obj_type(m));
    ns->count = o->count + added; ns->root = nr;
    gc_disabled--;
    return (Value)ns;
}
/* Add `x` to a sorted set by storing x as both key and value. O(log n). */
Value cljn_sorted_set_conj(Value s, Value x) { return cljn_sorted_assoc(s, x, x); }
/* Return the value for `k`, or NIL when absent. O(log n), no allocation. */
Value cljn_sorted_get(Value m, Value k) {
    Value r = tn_get(((Sorted *)m)->root, k);
    return r == MNOTFOUND ? NIL : r;
}
/* Return tagged boolean key membership. O(log n), no allocation. */
Value cljn_sorted_contains(Value m, Value k) {
    return b2v(tn_get(((Sorted *)m)->root, k) != MNOTFOUND);
}
/* Return the least set item or [key value] map entry. O(log n). */
Value cljn_sorted_first(Value m) {
    Value hv = ((Sorted *)m)->root;
    if (hv == NIL) return NIL;
    TNode *h = (TNode *)hv;
    while (h->left != NIL) h = (TNode *)h->left;
    if (obj_type(m) == T_SSET) return h->key;
    return cljn_vec_pair(h->key, h->val);
}
/* Materialize ascending keys, values, or entries into a rooted list. */
static Value sorted_seq(Value m, int mode) {
    cljn_gc_push(EMPTY);
    tn_walk_desc(((Sorted *)m)->root, mode);
    Value r = gc_stack[gc_sp - 1];
    cljn_gc_popn(1);
    return r;
}
/* Test whether every tree key belongs to another set representation. */
static int tn_all_in(Value node, Value other) {
    if (node == NIL) return 1;
    TNode *h = (TNode *)node;
    if (!tn_all_in(h->left, other)) return 0;
    if (!cljn_truthy(cljn_contains(other, h->key))) return 0;
    return tn_all_in(h->right, other);
}
/* Test whether every tree entry is present and equal in another map. */
static int tn_map_subset(Value node, Value other) {
    if (node == NIL) return 1;
    TNode *h = (TNode *)node;
    if (!tn_map_subset(h->left, other)) return 0;
    if (!cljn_truthy(cljn_map_contains(other, h->key)) ||
        !cljn_equal_raw(h->val, cljn_map_get(other, h->key))) return 0;
    return tn_map_subset(h->right, other);
}
/* Push ascending set items or map-entry vectors for apply/spread. */
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
/* Rebuild by reinserting every entry except `skip`, avoiding LLRB deletion. */
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
/* Remove key `k` by rebuilding the map; missing keys return `m`. O(n log n). */
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
