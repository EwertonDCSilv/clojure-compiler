/*
 * Core arithmetic, equality, predicates, and sequence operations.
 *
 * Numeric operations accept tagged fixnums only and terminate with Portuguese
 * diagnostics on type, divide-by-zero, or range errors. Equality is structural
 * across compatible collection representations and does not allocate.
 */
static intptr_t need_fix(Value v, const char *op) {
    if (!IS_FIX(v)) { fprintf(stderr, "erro: argumento não-numérico em %s\n", op); exit(1); }
    return FIX(v);
}
/* Validate the fixnum range before retagging a raw machine integer. */
static Value mk_fix_checked(intptr_t r, const char *op) {
    if (r < FIXNUM_MIN || r > FIXNUM_MAX) { fprintf(stderr, "erro: overflow em %s\n", op); exit(1); }
    return MK_FIX(r);
}
/* Box an IEEE-754 double. GC: allocates; heap operands stay rooted by the caller. */
static Value mk_float(double d) {
    Float *f = (Float *)obj_alloc(sizeof(Float), T_FLOAT);
    f->d = d;
    return (Value)f;
}
/* Reinterpret a raw 64-bit pattern as a double and box it (float literals). */
Value cljn_float_from_bits(Value bits) {
    double d;
    int64_t raw = (int64_t)bits;
    memcpy(&d, &raw, sizeof d);
    return mk_float(d);
}
/* Coerce a numeric Value to double; type errors are fatal. */
static double num_to_double(Value v, const char *op) {
    if (IS_FIX(v)) return (double)FIX(v);
    if (obj_type(v) == T_FLOAT) return ((Float *)v)->d;
    fprintf(stderr, "erro: argumento não-numérico em %s\n", op); exit(1);
}
/* Add: fixnum fast path; otherwise promote to double. */
Value cljn_add(Value a, Value b) {
    if (IS_FIX(a) && IS_FIX(b)) { intptr_t r; if (__builtin_add_overflow(FIX(a),FIX(b),&r)) die("overflow em +"); return mk_fix_checked(r,"+"); }
    return mk_float(num_to_double(a,"+") + num_to_double(b,"+"));
}
/* Subtract: fixnum fast path; otherwise promote to double. */
Value cljn_sub(Value a, Value b) {
    if (IS_FIX(a) && IS_FIX(b)) { intptr_t r; if (__builtin_sub_overflow(FIX(a),FIX(b),&r)) die("overflow em -"); return mk_fix_checked(r,"-"); }
    return mk_float(num_to_double(a,"-") - num_to_double(b,"-"));
}
/* Multiply: fixnum fast path; otherwise promote to double. */
Value cljn_mul(Value a, Value b) {
    if (IS_FIX(a) && IS_FIX(b)) { intptr_t r; if (__builtin_mul_overflow(FIX(a),FIX(b),&r)) die("overflow em *"); return mk_fix_checked(r,"*"); }
    return mk_float(num_to_double(a,"*") * num_to_double(b,"*"));
}
/* Divide: exact fixnum quotient when divisible, otherwise a double. */
Value cljn_div(Value a, Value b) {
    if (IS_FIX(a) && IS_FIX(b)) {
        intptr_t x = FIX(a), y = FIX(b);
        if (y == 0) die("divisão por zero");
        if (x % y == 0 && !(x == FIXNUM_MIN && y == -1)) return mk_fix_checked(x / y, "/");
        return mk_float((double)x / (double)y);
    }
    double y = num_to_double(b, "/");
    if (y == 0.0) die("divisão por zero");
    return mk_float(num_to_double(a, "/") / y);
}
/* Return truncating fixnum quotient; divide-by-zero and overflow are fatal. */
Value cljn_quot(Value a, Value b) { intptr_t y=need_fix(b,"quot"); if(y==0) die("divisão por zero"); intptr_t x=need_fix(a,"quot"); if(x==FIXNUM_MIN&&y==-1) die("overflow em quot"); return mk_fix_checked(x/y,"quot"); }
/* Return floored-modulus fixnum; divide-by-zero and type errors are fatal. */
Value cljn_mod(Value a, Value b) {
    intptr_t y=need_fix(b,"mod"); if(y==0) die("divisão por zero");
    intptr_t x=need_fix(a,"mod"), r=x%y;
    if (r!=0 && ((r<0)!=(y<0))) r+=y;
    return mk_fix_checked(r,"mod");
}
/* Increment: fixnum fast path; otherwise promote to double. */
Value cljn_inc(Value a) {
    if (IS_FIX(a)) { intptr_t r; if(__builtin_add_overflow(FIX(a),(intptr_t)1,&r)) die("overflow em inc"); return mk_fix_checked(r,"inc"); }
    return mk_float(num_to_double(a,"inc") + 1.0);
}
/* Decrement: fixnum fast path; otherwise promote to double. */
Value cljn_dec(Value a) {
    if (IS_FIX(a)) { intptr_t r; if(__builtin_sub_overflow(FIX(a),(intptr_t)1,&r)) die("overflow em dec"); return mk_fix_checked(r,"dec"); }
    return mk_float(num_to_double(a,"dec") - 1.0);
}

static Value b2v(int b) { return b ? TRUEV : FALSEV; }
/* Return tagged boolean `a < b`; fixnum fast path, otherwise as doubles. */
Value cljn_lt(Value a, Value b) { if (IS_FIX(a)&&IS_FIX(b)) return b2v(FIX(a)<FIX(b)); return b2v(num_to_double(a,"<")<num_to_double(b,"<")); }
/* Return tagged boolean `a <= b`. */
Value cljn_le(Value a, Value b) { if (IS_FIX(a)&&IS_FIX(b)) return b2v(FIX(a)<=FIX(b)); return b2v(num_to_double(a,"<=")<=num_to_double(b,"<=")); }
/* Return tagged boolean `a > b`. */
Value cljn_gt(Value a, Value b) { if (IS_FIX(a)&&IS_FIX(b)) return b2v(FIX(a)>FIX(b)); return b2v(num_to_double(a,">")>num_to_double(b,">")); }
/* Return tagged boolean `a >= b`. */
Value cljn_ge(Value a, Value b) { if (IS_FIX(a)&&IS_FIX(b)) return b2v(FIX(a)>=FIX(b)); return b2v(num_to_double(a,">=")>=num_to_double(b,">=")); }

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
/*
 * Return raw structural equality for two Values.
 *
 * Lists and vectors compare sequentially; map and set representations compare
 * across implementations without insertion order. Complexity is O(n) for
 * ordinary finite acyclic values and may recurse through nested collections.
 */
int cljn_equal_raw(Value a, Value b) {
    if (a == b) return 1;
    int ta = obj_type(a), tb = obj_type(b);
    if ((ta == T_STR && tb == T_STR) || (ta == T_KW && tb == T_KW)) {
        Str *x = (Str *)a, *y = (Str *)b;
        return ta == tb && x->len == y->len && memcmp(x->data, y->data, x->len) == 0;
    }
    /* Floats compare by value; fixnum vs float is unequal, matching Clojure =. */
    if (ta == T_FLOAT && tb == T_FLOAT) return ((Float *)a)->d == ((Float *)b)->d;
    /* Lists and vectors share element-wise sequential equality. */
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
            /* Every element of a must be present in b. */
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
            /* Every entry of a must be present and equal in b. */
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
/* Return tagged boolean structural equality; see cljn_equal_raw. */
Value cljn_eq(Value a, Value b) { return b2v(cljn_equal_raw(a,b)); }

/* Convert a fixnum to a character, or return a character unchanged. */
Value cljn_char(Value x) {
    if (IS_CHAR(x)) return x;
    if (IS_FIX(x)) return MK_CHAR((uint32_t)FIX(x));
    die("char: esperava inteiro ou char");
    return NIL;
}
/* Convert to a fixnum: char code point, truncated float, or fixnum unchanged. */
Value cljn_int(Value x) {
    if (IS_CHAR(x)) return MK_FIX((intptr_t)CHAR_CP(x));
    if (IS_FIX(x)) return x;
    if (obj_type(x) == T_FLOAT) return mk_fix_checked((intptr_t)((Float *)x)->d, "int");
    die("int: esperava char, inteiro ou float");
    return NIL;
}
/* Return tagged boolean indicating whether `x` is an immediate character. */
Value cljn_charp(Value x) { return b2v(IS_CHAR(x)); }
/* Coerce a number to a boxed double (fixnum or float). */
Value cljn_double(Value x) {
    if (obj_type(x) == T_FLOAT) return x;
    return mk_float(num_to_double(x, "double"));
}
/* Return tagged boolean indicating whether `x` is a boxed float. */
Value cljn_floatp(Value x) { return b2v(obj_type(x) == T_FLOAT); }
/* Return tagged boolean indicating whether `x` is a string. */
Value cljn_stringp(Value x) { return b2v(obj_type(x) == T_STR); }
/* Return tagged boolean indicating whether `x` is a fixnum. */
Value cljn_intp(Value x) { return b2v(IS_FIX(x)); }
/* Return tagged boolean indicating whether `x` is a keyword. */
Value cljn_keywordp(Value x) { return b2v(obj_type(x) == T_KW); }
/* Return tagged boolean indicating whether `x` is a persistent or transient vector. */
Value cljn_vectorp(Value x) { int t = obj_type(x); return b2v(t == T_VEC || t == T_TVEC); }
/* Return tagged boolean indicating whether `x` uses any native map representation. */
Value cljn_mapp(Value x) { int t = obj_type(x); return b2v(t == T_MAP || t == T_HMAP || t == T_SMAP); }
/* Return tagged boolean indicating whether `x` is an immutable byte sequence. */
Value cljn_bytesp(Value x) { return b2v(obj_type(x) == T_BYTES); }

/* Return raw C truthiness: only NIL and FALSEV are false. */
int cljn_truthy(Value v) { return (v != NIL && v != FALSEV) ? 1 : 0; }
/* Return tagged logical negation according to Clojure truthiness. */
Value cljn_not(Value v) { return b2v(!cljn_truthy(v)); }
/* Return tagged boolean indicating exact NIL identity. */
Value cljn_nilp(Value v) { return b2v(v == NIL); }
/* Return tagged emptiness for supported collections; unsupported values are false. */
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
/*
 * Return the first item of a supported sequence or NIL when empty.
 *
 * Hash-set order follows its internal traversal. Invalid receivers are fatal.
 * May allocate while materializing a hash or sorted collection view.
 */
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
/*
 * Return the remaining items as a sequence, or EMPTY when no items remain.
 *
 * Vectors and sets are eagerly materialized as lists and therefore allocate
 * O(n) cons cells. Invalid receivers terminate through the fatal path.
 */
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
        cljn_gc_push(acc);
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
/*
 * Return a tagged fixnum count.
 *
 * Flat and tree collections are O(1); cons lists are O(n). String count is the
 * stored UTF-8 byte length in the current native runtime.
 */
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
