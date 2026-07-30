/*
 * Hash sets and maps.
 *
 * Small collections use insertion-ordered flat arrays. Maps and sets promote
 * after MAP_ARRAY_MAX entries to a persistent bitmap-indexed HAMT with explicit
 * collision nodes. Updates path-copy affected nodes.
 *
 * GC: callers root input Values; multi-allocation updates use bounded no-GC
 * regions or keep the current accumulator at the shadow-stack top.
 */
/* Allocate flat set storage with raw capacity `n`; construction-only. */
Value cljn_set_alloc(Value n) {
    int64_t k = (int64_t)n;
    Vec *s = (Vec *)obj_alloc(sizeof(Vec) + (size_t)k * sizeof(Value), T_SET);
    s->len = 0;
    for (int64_t i = 0; i < k; i++) s->items[i] = NIL;
    return (Value)s;
}
static int set_member(Vec *s, Value x) {
    for (int64_t i = 0; i < s->len; i++) if (cljn_equal_raw(s->items[i], x)) return 1;
    return 0;
}
/* Add `x` to construction-time flat set storage; does not allocate. */
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
/* Return a persistent set containing `x`, promoting to HAMT as needed. */
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
            /* Promote flat set to HAMT set, storing value == key. */
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
/* Return tagged boolean membership. Expected O(1), worst-case O(n) collisions. */
Value cljn_set_contains(Value set, Value x) {
    if (obj_type(set) == T_HSET) return b2v(node_get(((HMap *)set)->root, 0, cljn_hash(x), x) != MNOTFOUND);
    return b2v(set_member((Vec *)set, x));
}

/* Hash one Value consistently with cljn_equal_raw; does not allocate. */
static uint32_t hash_bytes(const char *p, size_t n) {
    uint32_t h = 2166136261u;
    for (size_t i = 0; i < n; i++) { h ^= (unsigned char)p[i]; h *= 16777619u; }
    return h;
}
/*
 * Return the 32-bit structural hash used by HAMT navigation.
 *
 * Composite keys currently share a fallback hash; equality resolves collisions.
 */
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
        default: return 7;
    }
}

/* HAMT node lookup descends five hash bits per level. */
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
/* Cons every HAMT key or value into the rooted accumulator at gc_sp - 1. */
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

/* Test whether every HAMT key belongs to `other`. */
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

/* Push every HAMT key for apply/spread and return the number pushed. */
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

/* Allocate a flat map with raw pair count `n`; fields are filled separately. */
Value cljn_map_alloc(Value n) {
    int64_t k = (int64_t)n;
    Map *m = (Map *)obj_alloc(sizeof(Map) + (size_t)(2 * k) * sizeof(Value), T_MAP);
    m->n = k;
    for (int64_t i = 0; i < 2 * k; i++) m->kv[i] = NIL;
    return (Value)m;
}
/* Set construction-time pair `i`; does not allocate or validate bounds. */
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
    /* Promote array-map plus (k,v) to HAMT; inputs are caller-rooted. */
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
/* Look up key `k`, returning NIL when absent; does not allocate. */
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
/* Return a tagged boolean indicating whether key `k` is present. */
Value cljn_map_contains(Value map, Value k) {
    if (obj_type(map) == T_HMAP) return b2v(node_get(((HMap *)map)->root, 0, cljn_hash(k), k) != MNOTFOUND);
    if (obj_type(map) == T_SMAP) return cljn_sorted_contains(map, k);
    return b2v(obj_type(map) == T_MAP && map_index((Map *)map, k) >= 0);
}
/*
 * Return a persistent map associating `k` with `v`.
 *
 * Promotes a flat map to HAMT past MAP_ARRAY_MAX. Expected O(1); flat-map and
 * collision paths are O(n). GC: all inputs are rooted by the caller.
 */
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
            result = hmap_from_arraymap(o, k, v);
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
/*
 * Return a persistent map without key `k`.
 *
 * HAMT removal currently rebuilds through association. Missing keys return the
 * original map. GC: rebuild keeps its accumulator rooted.
 */
Value cljn_map_dissoc(Value map, Value k) {
    if (obj_type(map) == T_SMAP) return cljn_sorted_dissoc(map, k);
    if (obj_type(map) == T_HMAP) {
        HMap *m = (HMap *)map;
        if (node_get(m->root, 0, cljn_hash(k), k) == MNOTFOUND) return map;
        /* Rebuild every entry except k, allowing representation selection. */
        maybe_gc();
        Value acc = cljn_map_alloc(0);
        cljn_gc_push(acc);
        /* The recursive walk updates the rooted accumulator at gc_sp - 1. */
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
/* Reassociate all HAMT entries except `skip` into the rooted accumulator. */
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
/* Materialize map or record keys as a list. O(n), allocates n cons cells. */
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
/* Test whether every HAMT entry is present and equal in `other`. */
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
/* Materialize map or record values as a list. O(n), allocates n cons cells. */
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
