
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
        case T_TVEC: { TVec *tv = (TVec *)coll; if (IS_FIX(key)) { int64_t i = FIX(key); if (i >= 0 && i < tv->len) return tv->items[i]; } return NIL; }
        case T_TBOX: return cljn_get(((TBox *)coll)->inner, key);
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
        case T_TBOX: return cljn_contains(((TBox *)coll)->inner, key);
        case T_TVEC: { TVec *tv = (TVec *)coll; return b2v(IS_FIX(key) && FIX(key) >= 0 && FIX(key) < tv->len); }
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
/* ADR-0008: operações internas de core em espaço de IDs reservado (negativo),
 * separado dos protocolos/multimethods do programa (mids positivos). Built-ins
 * têm precedência; tipos sem tag embutida registram capabilities por type_key. */
#define CORE_ASSOC_ONE ((Value)(-1))
#define CORE_NTH ((Value)(-2))
#define CORE_NTH_OR ((Value)(-3))
static Value call_fn2(Value f, Value a, Value b);        /* fwd */
static Value call_fn3(Value f, Value a, Value b, Value c); /* fwd */

/* AssocOne: atualização unitária persistente. nil→array-map; built-in por tag;
 * senão capability registrada por type_key; senão erro. */
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
            /* capability por tipo nominal (sem tag embutida) */
            {
                Value impl = cljn_lookup_method(CORE_ASSOC_ONE, cljn_type_key(coll));
                if (impl != NIL) return call_fn3(impl, coll, k, v);
            }
            die("assoc: receptor sem suporte");
            return coll;
    }
}
/* Núcleo indexado embutido: devolve o elemento, ou MNOTFOUND se fora dos limites,
 * ou MNODEKEY se o tipo não é indexado embutido (para o caller decidir). */
static Value nth_builtin(Value coll, int64_t i) {
    if (coll == EMPTY) return MNOTFOUND; /* sequência vazia: sempre fora dos limites */
    switch (obj_type(coll)) {
        case T_VEC: { PVec *v = (PVec *)coll; return (i >= 0 && i < v->count) ? pv_nth(v, i) : MNOTFOUND; }
        case T_TVEC: { TVec *tv = (TVec *)coll; return (i >= 0 && i < tv->len) ? tv->items[i] : MNOTFOUND; }
        case T_CONS: {
            if (i < 0) return MNOTFOUND;
            Value c = coll;
            while (i-- > 0 && obj_type(c) == T_CONS) c = ((Cons *)c)->tail;
            return (obj_type(c) == T_CONS) ? ((Cons *)c)->head : MNOTFOUND;
        }
    }
    return MNODEKEY; /* não indexado embutido */
}
/* nth aridade 2: índice fora dos limites lança; tipo sem suporte lança. */
Value cljn_nth(Value coll, Value idx) {
    if (!IS_FIX(idx)) die("nth: índice deve ser inteiro");
    if (coll == NIL) return NIL;
    int64_t i = FIX(idx);
    Value r = nth_builtin(coll, i);
    if (r != MNODEKEY) { if (r == MNOTFOUND) die("nth: índice fora dos limites"); return r; }
    Value impl = cljn_lookup_method(CORE_NTH, cljn_type_key(coll));
    if (impl != NIL) return call_fn2(impl, coll, idx);
    die("nth: receptor não indexado nem sequencial");
    return NIL;
}
/* nth aridade 3: só o limite é absorvido por not-found; tipo/índice inválido lança. */
Value cljn_nth_or(Value coll, Value idx, Value nf) {
    if (!IS_FIX(idx)) die("nth: índice deve ser inteiro");
    if (coll == NIL) return nf;
    int64_t i = FIX(idx);
    Value r = nth_builtin(coll, i);
    if (r != MNODEKEY) return (r == MNOTFOUND) ? nf : r;
    Value impl = cljn_lookup_method(CORE_NTH_OR, cljn_type_key(coll));
    if (impl != NIL) return call_fn3(impl, coll, idx, nf);
    die("nth: receptor não indexado nem sequencial");
    return nf;
}
