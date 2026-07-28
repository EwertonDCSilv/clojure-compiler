
/*
 * Garbage collector and generated-code shadow stack.
 *
 * The collector is a precise, non-moving, single-threaded mark/sweep collector.
 * Generated code reserves local slots and spills heap-capable temporaries into
 * `gc_stack`; the collector never scans the native machine stack.
 *
 * ABI: generated code also accesses `gc_stack` and `gc_sp` directly.
 * GC: every heap-capable Value live across allocation must occupy a live slot.
 */
#define GC_STACK_CAP (1u << 22) /* 4M slots */
Value gc_stack[GC_STACK_CAP];
int64_t gc_sp = 0;

/*
 * Reserve and clear `nslots` root slots, returning their untagged base index.
 *
 * ABI: `nslots` and the return value are raw counts carried in Value-sized
 * registers, not tagged fixnums. Exits fatally on shadow-stack overflow.
 * GC: clearing to NIL prevents stale pointers from becoming conservative roots.
 */
Value cljn_gc_enter(Value nslots) {
    size_t base = gc_sp;
    size_t n = (size_t)nslots;
    if (base + n > GC_STACK_CAP) { fprintf(stderr, "erro: overflow do shadow-stack de GC\n"); exit(1); }
    for (size_t i = 0; i < n; i++) gc_stack[base + i] = NIL; /* zera slots reservados */
    gc_sp = base + n;
    return (Value)base;
}
/* Restore `gc_sp` to an untagged base previously returned by cljn_gc_enter. */
void cljn_gc_leave(Value base) { gc_sp = (size_t)base; }
/* Push one tagged Value root; exits fatally if the fixed stack is full. */
void cljn_gc_push(Value v) {
    if (gc_sp >= GC_STACK_CAP) { fprintf(stderr, "erro: overflow do shadow-stack de GC\n"); exit(1); }
    gc_stack[gc_sp++] = v;
}
/* Remove an untagged number of temporary roots pushed by generated code. */
void cljn_gc_popn(Value n) { gc_sp -= (size_t)n; }
/* Store a tagged Value in an absolute untagged shadow-stack slot. */
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
static void *xrealloc(void *old, size_t n) {
    void *p = realloc(old, n);
    if (!p) { fprintf(stderr, "erro: sem memória\n"); exit(1); }
    return p;
}
static void die(const char *m) { fprintf(stderr, "erro: %s\n", m); exit(1); }
static int obj_type(Value v) { return (IS_PTR(v) && v != 0) ? ((Obj *)v)->type : 0; }

static int gc_off = -1;
/* A bounded no-GC region protects unrooted intermediate objects in compound
 * runtime operations. Inputs remain rooted by generated callers. */
static int gc_disabled = 0;

/* Non-moving allocator: bump slabs plus free lists in 16-byte size classes.
 * Sweep recycles small dead objects; objects larger than SZC_MAX use direct
 * malloc/free and have szc == 0. */
#define SZC_GRAN 16
#define SZC_MAX 512
#define NSZC (SZC_MAX / SZC_GRAN + 1)
static Obj *freelist[NSZC];
static char *slab_ptr = NULL, *slab_end = NULL;
static void *slab_bump(size_t n) {
    if (slab_ptr + n > slab_end) {
        size_t chunk = 1u << 20; /* 1 MiB */
        if (n > chunk) chunk = n;
        slab_ptr = (char *)xalloc(chunk);
        slab_end = slab_ptr + chunk;
    }
    void *p = slab_ptr;
    slab_ptr += n;
    return p;
}

static void gc_init_env(void) {
    const char *e = getenv("CLJN_GC_STRESS");
    gc_stress = (e && e[0] && e[0] != '0') ? 1 : 0;
    const char *o = getenv("CLJN_GC_OFF");
    gc_off = (o && o[0] && o[0] != '0') ? 1 : 0;
}
static void maybe_gc(void) {
    if (gc_stress < 0) gc_init_env();
    if (gc_off || gc_disabled) return;
    if (gc_stress || alloc_since_gc >= gc_threshold) gc_collect();
}

static Obj *obj_alloc(size_t size, int type) {
    /* Allocation is a safepoint unless a bounded no-GC region is active. */
    if (gc_stress < 0) gc_init_env();
    if (!gc_disabled && !gc_off && (gc_stress || alloc_since_gc >= gc_threshold)) gc_collect();
    size_t asz = (size + (SZC_GRAN - 1)) & ~(size_t)(SZC_GRAN - 1);
    unsigned c = (unsigned)(asz / SZC_GRAN);
    Obj *o;
    if (c < NSZC && freelist[c]) {
        o = freelist[c];
        freelist[c] = o->next_all; /* Reuse one same-size object. */
    } else if (c < NSZC) {
        o = (Obj *)slab_bump(asz);
    } else {
        o = (Obj *)xalloc(size); /* Large object: direct allocation. */
        c = 0;
    }
    o->type = (uint8_t)type;
    o->mark = 0;
    o->szc = (uint16_t)c; /* Zero denotes a direct allocation. */
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
            v = ((Cons *)v)->tail; /* Iterate the tail to bound recursion. */
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
            gc_mark(nd->edit); /* Keep the transient ownership token alive. */
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
        } else if (o->type == T_TVEC) {
            TVec *tv = (TVec *)v;
            gc_mark(tv->root);
            gc_mark(tv->edit);
            v = tv->tail; /* Iterate the tail. */
        } else if (o->type == T_TBOX) {
            v = ((TBox *)v)->inner;
        } else if (o->type == T_READER) {
            v = ((Reader *)v)->src;
        } else if (o->type == T_TNODE) {
            TNode *nd = (TNode *)v;
            gc_mark(nd->key);
            gc_mark(nd->val);
            gc_mark(nd->left);
            v = nd->right; /* Iterate the right child. */
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
            return; /* Remaining object types are leaves. */
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
            if (o->type == T_WRITER) {
                Writer *w = (Writer *)o;
                free(w->buf);
                if (w->kind == WR_FILE && w->fp) fclose((FILE *)w->fp);
            }
            if (o->type == T_READER) {
                Reader *r = (Reader *)o;
                if (r->kind == RD_FILE && r->fp) fclose((FILE *)r->fp);
            }
            if (o->type == T_BYTES) free(((Bytes *)o)->data);
            if (o->szc == 0) {
                free(o); /* Directly allocated large object. */
            } else {
                o->next_all = freelist[o->szc];
                freelist[o->szc] = o;
            }
        }
    }
}

/* Immediate-only vector literals are cached after first construction.
 * GC: registered constants are permanent roots. */
#define CONST_MAX 8192
Value cljn_const_cache[CONST_MAX];
static int64_t const_hi = 0;
/*
 * Store `v` in the permanent constant cache at raw index `id`.
 *
 * ABI: CONST_MAX must match CONST_CACHE_MAX in clojure-codegen/src/lib.rs.
 * GC: does not allocate; the value becomes a root at the next collection.
 */
void cljn_const_register(Value id, Value v) {
    int64_t i = (int64_t)id;
    cljn_const_cache[i] = v;
    if (i + 1 > const_hi) const_hi = i + 1;
}
static void gc_mark_consts(void) {
    for (int64_t i = 0; i < const_hi; i++) gc_mark(cljn_const_cache[i]);
}

static void gc_mark_method_table(void); /* fwd */
static void gc_mark_exceptions(void);   /* fwd */
static void gc_mark_multi(void);        /* fwd */
static void gc_mark_dynvars(void);      /* fwd */
static void gc_collect(void) {
    for (int64_t i = 0; i < gc_sp; i++) gc_mark(gc_stack[i]);
    gc_mark_method_table(); /* Permanent protocol keys and implementations. */
    gc_mark_exceptions();   /* Exception value currently in flight. */
    gc_mark_multi();        /* Multimethod dispatch functions and :default. */
    gc_mark_consts();       /* Cached constant vector literals. */
    gc_mark_dynvars();      /* Built-in dynamic Vars. */
    gc_sweep();
    alloc_since_gc = 0;
}
