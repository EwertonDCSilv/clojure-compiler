
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
static void *xrealloc(void *old, size_t n) {
    void *p = realloc(old, n);
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

/* Alocador não-móvel: slabs (bump) + free-lists por classe de tamanho de 16 B.
 * O sweep recicla objetos mortos para o free-list em vez de `free()`, evitando
 * malloc/free por objeto e mantendo o working-set pequeno e cache-quente. Como
 * nada se move, a shadow stack e o rooting da ADR-0006 permanecem válidos.
 * Objetos > SZC_MAX usam malloc/free direto (szc==0). */
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
    /* safepoint inline (barato no caso comum; sem chamada quando em zona sem-GC) */
    if (gc_stress < 0) gc_init_env();
    if (!gc_disabled && !gc_off && (gc_stress || alloc_since_gc >= gc_threshold)) gc_collect();
    size_t asz = (size + (SZC_GRAN - 1)) & ~(size_t)(SZC_GRAN - 1);
    unsigned c = (unsigned)(asz / SZC_GRAN);
    Obj *o;
    if (c < NSZC && freelist[c]) {
        o = freelist[c];
        freelist[c] = o->next_all; /* recicla do free-list */
    } else if (c < NSZC) {
        o = (Obj *)slab_bump(asz);
    } else {
        o = (Obj *)xalloc(size); /* grande: malloc direto */
        c = 0;
    }
    o->type = (uint8_t)type;
    o->mark = 0;
    o->szc = (uint16_t)c; /* 0 = malloc'd (não reciclado) */
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
            gc_mark(nd->edit); /* token de transiente (mantém vivo p/ evitar ABA) */
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
            TVec *tv = (TVec *)v; /* transiente estrutural: raiz/tail/edit */
            gc_mark(tv->root);
            gc_mark(tv->edit);
            v = tv->tail; /* itera o tail */
        } else if (o->type == T_TBOX) {
            v = ((TBox *)v)->inner; /* itera o valor interno */
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
            if (o->type == T_WRITER) free(((Writer *)o)->buf);
            if (o->szc == 0) {
                free(o); /* grande: malloc'd */
            } else {
                o->next_all = freelist[o->szc]; /* recicla */
                freelist[o->szc] = o;
            }
        }
    }
}

/* Cache de vetores literais constantes (ADR-0009): um literal cujos elementos são
 * todos imediatos é imutável — construído uma vez no 1º uso e reusado, em vez de
 * reconstruído a cada avaliação. São raízes permanentes (marcados pelo GC). */
#define CONST_MAX 8192
Value cljn_const_cache[CONST_MAX];
static int64_t const_hi = 0;
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
    gc_mark_method_table(); /* raízes permanentes: chaves/impls de protocolos */
    gc_mark_exceptions();   /* valor de exceção em voo */
    gc_mark_multi();        /* funções de dispatch de multimethods + :default */
    gc_mark_consts();       /* vetores literais constantes cacheados */
    gc_mark_dynvars();      /* Vars dinâmicas (*out* etc.) */
    gc_sweep();
    alloc_since_gc = 0;
}
