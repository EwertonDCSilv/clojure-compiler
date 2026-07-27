
/* ---------- multimethods (defmulti/defmethod) ----------
 * Reusa a method_table (chave = valor de dispatch, casada por cljn_equal_raw).
 * A função de dispatch de cada multimethod fica num registro próprio por mid. */
typedef struct MultiEntry { int64_t mid; Value fn; struct MultiEntry *next; } MultiEntry;
static MultiEntry *multi_table = NULL;
static Value cljn_default_kw = 0; /* keyword :default cacheada (root) */

void cljn_multi_register(Value mid, Value fn) {
    MultiEntry *e = xalloc(sizeof(MultiEntry));
    e->mid = (int64_t)mid; e->fn = fn; e->next = multi_table; multi_table = e;
}
static Value multi_dispatch_fn(int64_t mid) {
    for (MultiEntry *e = multi_table; e; e = e->next) if (e->mid == mid) return e->fn;
    return NIL;
}
Value cljn_multi_call(Value mid, Value argc, Value argv_) {
    int64_t n = (int64_t)argc;
    Value *argv = (Value *)argv_;
    Value df = multi_dispatch_fn((int64_t)mid);
    if (obj_type(df) != T_FN) { fprintf(stderr, "erro: multimethod sem função de dispatch\n"); exit(1); }
    Value dv = ((FnCode)((Fn *)df)->code)(df, n, argv);
    gc_stack[gc_sp++] = dv; /* rooteia dv durante lookup e alloc de :default */
    Value impl = cljn_lookup_method(mid, dv);
    if (impl == NIL) {
        if (cljn_default_kw == 0) cljn_default_kw = cljn_kw("default", 7);
        impl = cljn_lookup_method(mid, cljn_default_kw);
    }
    gc_sp--;
    if (obj_type(impl) != T_FN) {
        fputs("erro: sem método de multimethod para o valor de dispatch: ", stderr);
        SB b; sb_init(&b); write_val(&b, dv, 0); fwrite(b.p, 1, b.len, stderr); free(b.p);
        fputc('\n', stderr);
        exit(1);
    }
    return ((FnCode)((Fn *)impl)->code)(impl, n, argv);
}
static void gc_mark_multi(void) {
    for (MultiEntry *e = multi_table; e; e = e->next) gc_mark(e->fn);
    if (cljn_default_kw) gc_mark(cljn_default_kw);
}
