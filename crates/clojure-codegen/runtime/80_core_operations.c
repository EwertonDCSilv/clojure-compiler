
/* ---------- aritmética ---------- */
static intptr_t need_fix(Value v, const char *op) {
    if (!IS_FIX(v)) { fprintf(stderr, "erro: argumento não-numérico em %s\n", op); exit(1); }
    return FIX(v);
}
/* Valida o intervalo de fixnum ANTES do retag (i64 tem 63 bits, fixnum 62). */
static Value mk_fix_checked(intptr_t r, const char *op) {
    if (r < FIXNUM_MIN || r > FIXNUM_MAX) { fprintf(stderr, "erro: overflow em %s\n", op); exit(1); }
    return MK_FIX(r);
}
Value cljn_add(Value a, Value b) { intptr_t r; if (__builtin_add_overflow(need_fix(a,"+"),need_fix(b,"+"),&r)) die("overflow em +"); return mk_fix_checked(r,"+"); }
Value cljn_sub(Value a, Value b) { intptr_t r; if (__builtin_sub_overflow(need_fix(a,"-"),need_fix(b,"-"),&r)) die("overflow em -"); return mk_fix_checked(r,"-"); }
Value cljn_mul(Value a, Value b) { intptr_t r; if (__builtin_mul_overflow(need_fix(a,"*"),need_fix(b,"*"),&r)) die("overflow em *"); return mk_fix_checked(r,"*"); }
Value cljn_quot(Value a, Value b) { intptr_t y=need_fix(b,"quot"); if(y==0) die("divisão por zero"); intptr_t x=need_fix(a,"quot"); if(x==FIXNUM_MIN&&y==-1) die("overflow em quot"); return mk_fix_checked(x/y,"quot"); }
Value cljn_mod(Value a, Value b) {
    intptr_t y=need_fix(b,"mod"); if(y==0) die("divisão por zero");
    intptr_t x=need_fix(a,"mod"), r=x%y;
    if (r!=0 && ((r<0)!=(y<0))) r+=y;
    return mk_fix_checked(r,"mod");
}
Value cljn_inc(Value a) { intptr_t r; if(__builtin_add_overflow(need_fix(a,"inc"),(intptr_t)1,&r)) die("overflow em inc"); return mk_fix_checked(r,"inc"); }
Value cljn_dec(Value a) { intptr_t r; if(__builtin_sub_overflow(need_fix(a,"dec"),(intptr_t)1,&r)) die("overflow em dec"); return mk_fix_checked(r,"dec"); }

static Value b2v(int b) { return b ? TRUEV : FALSEV; }
Value cljn_lt(Value a, Value b) { return b2v(need_fix(a,"<")<need_fix(b,"<")); }
Value cljn_le(Value a, Value b) { return b2v(need_fix(a,"<=")<=need_fix(b,"<=")); }
Value cljn_gt(Value a, Value b) { return b2v(need_fix(a,">")>need_fix(b,">")); }
Value cljn_ge(Value a, Value b) { return b2v(need_fix(a,">=")>=need_fix(b,">=")); }

static int is_seq(int t) { return t == T_CONS || t == T_VEC; }
static Value seq_nth(Value coll, int t, int64_t i) {
    if (t == T_VEC) return pv_nth((PVec *)coll, i);
    Value c = coll;
    while (i-- > 0) c = ((Cons *)c)->tail;
    return ((Cons *)c)->head;
}
static int64_t seq_len(Value coll, int t) {
    if (t == T_VEC) return ((PVec *)coll)->count;
    int64_t n = 0;
    Value c = coll;
    while (obj_type(c) == T_CONS) { n++; c = ((Cons *)c)->tail; }
    return n;
}
int cljn_equal_raw(Value a, Value b) {
    if (a == b) return 1;
    int ta = obj_type(a), tb = obj_type(b);
    if ((ta == T_STR && tb == T_STR) || (ta == T_KW && tb == T_KW)) {
        Str *x = (Str *)a, *y = (Str *)b;
        return ta == tb && x->len == y->len && memcmp(x->data, y->data, x->len) == 0;
    }
    /* sequências (list/vector) comparam elemento a elemento */
    if (is_seq(ta) && is_seq(tb)) {
        int64_t la = seq_len(a, ta), lb = seq_len(b, tb);
        if (la != lb) return 0;
        for (int64_t i = 0; i < la; i++)
            if (!cljn_equal_raw(seq_nth(a, ta, i), seq_nth(b, tb, i))) return 0;
        return 1;
    }
    {
        int as = (ta == T_SET || ta == T_HSET || ta == T_SSET);
        int bs = (tb == T_SET || tb == T_HSET || tb == T_SSET);
        if (as && bs) {
            int64_t ca = (ta == T_HSET) ? ((HMap *)a)->count : (ta == T_SSET ? ((Sorted *)a)->count : ((Vec *)a)->len);
            int64_t cb = (tb == T_HSET) ? ((HMap *)b)->count : (tb == T_SSET ? ((Sorted *)b)->count : ((Vec *)b)->len);
            if (ca != cb) return 0;
            /* toda entrada de `a` presente em `b` (contains dispatcha por repr) */
            if (ta == T_HSET) return hnode_all_in(((HMap *)a)->root, b);
            if (ta == T_SSET) return tn_all_in(((Sorted *)a)->root, b);
            Vec *x = (Vec *)a;
            for (int64_t i = 0; i < x->len; i++)
                if (!cljn_truthy(cljn_contains(b, x->items[i]))) return 0;
            return 1;
        }
    }
    {
        int am = (ta == T_MAP || ta == T_HMAP || ta == T_SMAP);
        int bm = (tb == T_MAP || tb == T_HMAP || tb == T_SMAP);
        if (am && bm) {
            int64_t ca = (ta == T_HMAP) ? ((HMap *)a)->count : (ta == T_SMAP ? ((Sorted *)a)->count : ((Map *)a)->n);
            int64_t cb = (tb == T_HMAP) ? ((HMap *)b)->count : (tb == T_SMAP ? ((Sorted *)b)->count : ((Map *)b)->n);
            if (ca != cb) return 0;
            /* toda entrada de `a` presente e igual em `b` (dispatch cobre cross-repr) */
            if (ta == T_HMAP) return hmap_node_subset(((HMap *)a)->root, b);
            if (ta == T_SMAP) return tn_map_subset(((Sorted *)a)->root, b);
            Map *x = (Map *)a;
            for (int64_t i = 0; i < x->n; i++) {
                Value k = x->kv[2 * i];
                if (!cljn_truthy(cljn_map_contains(b, k)) || !cljn_equal_raw(x->kv[2 * i + 1], cljn_map_get(b, k)))
                    return 0;
            }
            return 1;
        }
    }
    if (ta == T_RECORD && tb == T_RECORD) {
        Record *x = (Record *)a, *y = (Record *)b;
        return cljn_equal_raw(x->type_name, y->type_name) && cljn_equal_raw(x->map, y->map);
    }
    return 0;
}
Value cljn_eq(Value a, Value b) { return b2v(cljn_equal_raw(a,b)); }

/* Char <-> inteiro (ADR-0007 / IO-1). (char n) inteiro->char; (int c) char->codepoint.
 * Idempotentes no próprio tipo. Também emitido para literais de char \a. */
Value cljn_char(Value x) {
    if (IS_CHAR(x)) return x;
    if (IS_FIX(x)) return MK_CHAR((uint32_t)FIX(x));
    die("char: esperava inteiro ou char");
    return NIL;
}
Value cljn_int(Value x) {
    if (IS_CHAR(x)) return MK_FIX((intptr_t)CHAR_CP(x));
    if (IS_FIX(x)) return x;
    die("int: esperava char ou inteiro");
    return NIL;
}
Value cljn_charp(Value x) { return b2v(IS_CHAR(x)); }

int cljn_truthy(Value v) { return (v != NIL && v != FALSEV) ? 1 : 0; }
Value cljn_not(Value v) { return b2v(!cljn_truthy(v)); }
Value cljn_nilp(Value v) { return b2v(v == NIL); }
Value cljn_emptyp(Value v) {
    if (v == EMPTY || v == NIL) return TRUEV;
    switch (obj_type(v)) {
        case T_STR: return b2v(((Str *)v)->len == 0);
        case T_SET: return b2v(((Vec *)v)->len == 0);
        case T_HSET: return b2v(((HMap *)v)->count == 0);
        case T_VEC: return b2v(((PVec *)v)->count == 0);
        case T_MAP: return b2v(((Map *)v)->n == 0);
        case T_HMAP: return b2v(((HMap *)v)->count == 0);
        case T_SMAP: case T_SSET: return b2v(((Sorted *)v)->count == 0);
    }
    return FALSEV;
}
Value cljn_first(Value v) {
    if (v == EMPTY || v == NIL) return NIL;
    switch (obj_type(v)) {
        case T_CONS: return ((Cons *)v)->head;
        case T_SET: { Vec *x = (Vec *)v; return x->len > 0 ? x->items[0] : NIL; }
        case T_HSET: {
            cljn_gc_push(EMPTY);
            hmap_cons_walk(((HMap *)v)->root, 0);
            Value l = gc_stack[gc_sp - 1];
            cljn_gc_popn(1);
            return l == EMPTY ? NIL : ((Cons *)l)->head;
        }
        case T_VEC: { PVec *x = (PVec *)v; return x->count > 0 ? pv_nth(x, 0) : NIL; }
        case T_SMAP: case T_SSET: return cljn_sorted_first(v);
    }
    die("first: não é uma sequência"); return NIL;
}
Value cljn_rest(Value v) {
    if (v == EMPTY || v == NIL) return EMPTY;
    if (obj_type(v) == T_CONS) return ((Cons *)v)->tail;
    if (obj_type(v) == T_HSET) {
        cljn_gc_push(EMPTY);
        hmap_cons_walk(((HMap *)v)->root, 0);
        Value l = gc_stack[gc_sp - 1];
        cljn_gc_popn(1);
        return l == EMPTY ? EMPTY : ((Cons *)l)->tail;
    }
    if (obj_type(v) == T_SMAP || obj_type(v) == T_SSET) {
        Value l = sorted_seq(v, obj_type(v) == T_SSET ? 0 : 2);
        return l == EMPTY ? EMPTY : ((Cons *)l)->tail;
    }
    if (obj_type(v) == T_SET || obj_type(v) == T_VEC) {
        int is_vec = obj_type(v) == T_VEC;
        int64_t len = is_vec ? ((PVec *)v)->count : ((Vec *)v)->len;
        Value acc = EMPTY;
        cljn_gc_push(acc); /* rooteia o acumulador durante a construção */
        for (int64_t i = len - 1; i >= 1; i--) {
            Value el = is_vec ? pv_nth((PVec *)v, i) : ((Vec *)v)->items[i];
            acc = cljn_cons(el, acc);
            gc_stack[gc_sp - 1] = acc;
        }
        cljn_gc_popn(1);
        return acc;
    }
    die("rest: não é uma sequência"); return EMPTY;
}
Value cljn_count(Value v) {
    switch (obj_type(v)) {
        case T_STR: return MK_FIX((long)((Str *)v)->len);
        case T_BYTES: return MK_FIX(((Bytes *)v)->len);
        case T_SET: return MK_FIX(((Vec *)v)->len);
        case T_HSET: return MK_FIX(((HMap *)v)->count);
        case T_VEC: return MK_FIX(((PVec *)v)->count);
        case T_MAP: return MK_FIX(((Map *)v)->n);
        case T_HMAP: return MK_FIX(((HMap *)v)->count);
        case T_SMAP: case T_SSET: return MK_FIX(((Sorted *)v)->count);
        case T_TVEC: return MK_FIX(((PVec *)v)->count);
        case T_TBOX: return cljn_count(((TBox *)v)->inner);
        case T_RECORD: return MK_FIX(((Map *)((Record *)v)->map)->n);
    }
    long n = 0;
    while (v != EMPTY && v != NIL && obj_type(v) == T_CONS) { n++; v = ((Cons *)v)->tail; }
    return MK_FIX(n);
}
