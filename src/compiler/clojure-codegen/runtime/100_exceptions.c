
/*
 * Native throw and try/catch/finally control flow.
 *
 * setjmp/longjmp remain entirely inside cljn_try; generated Cranelift frames do
 * not contain setjmp state. Throw unwinds to the nearest Handler, which restores
 * both shadow-stack depth and no-GC nesting before invoking user handlers.
 *
 * ABI: body, catch, and finally are closures using (self, argc, argv).
 */
typedef Value (*FnCode)(Value, int64_t, Value *);
static Value call_fn0(Value f) {
    return ((FnCode)((Fn *)f)->code)(f, 0, &gc_stack[gc_sp]);
}
static Value call_fn1(Value f, Value a) {
    gc_stack[gc_sp++] = a; /* arg no topo; argv aponta pra ele (rooteado) */
    Value r = ((FnCode)((Fn *)f)->code)(f, 1, &gc_stack[gc_sp - 1]);
    gc_sp--;
    return r;
}
static Value call_fn2(Value f, Value a, Value b) {
    gc_stack[gc_sp] = a; gc_stack[gc_sp + 1] = b; gc_sp += 2;
    Value r = ((FnCode)((Fn *)f)->code)(f, 2, &gc_stack[gc_sp - 2]);
    gc_sp -= 2;
    return r;
}
static Value call_fn3(Value f, Value a, Value b, Value c) {
    gc_stack[gc_sp] = a; gc_stack[gc_sp + 1] = b; gc_stack[gc_sp + 2] = c; gc_sp += 3;
    Value r = ((FnCode)((Fn *)f)->code)(f, 3, &gc_stack[gc_sp - 3]);
    gc_sp -= 3;
    return r;
}
typedef struct Handler {
    jmp_buf env;
    struct Handler *prev;
    size_t saved_sp;
    int saved_gc_disabled;
} Handler;
static Handler *handler_top = NULL;
static Value exception_value = NIL;

/*
 * Throw tagged Value `v` to the nearest active handler.
 *
 * Never returns normally when a handler exists. Without a handler, prints the
 * value and terminates. GC: exception_value is a permanent in-flight root.
 */
Value cljn_throw(Value v) {
    exception_value = v;
    if (handler_top == NULL) {
        SB b; sb_init(&b); write_val(&b, v, 0);
        fputs("exceção não capturada: ", stderr);
        fwrite(b.p, 1, b.len, stderr); fputc('\n', stderr);
        free(b.p);
        exit(1);
    }
    longjmp(handler_top->env, 1);
}

/*
 * Invoke body with optional catch and finally closures.
 *
 * Catch receives the thrown Value. Finally runs after normal or caught exit;
 * when no catch exists it runs before rethrow. Restores GC bookkeeping on
 * longjmp and returns the body/catch result.
 */
Value cljn_try(Value body, Value catch, Value finally) {
    Handler h;
    h.prev = handler_top;
    h.saved_sp = gc_sp;
    h.saved_gc_disabled = gc_disabled;
    Value result;
    if (setjmp(h.env) == 0) {
        handler_top = &h;
        result = call_fn0(body);
        handler_top = h.prev;
    } else {
        handler_top = h.prev;
        gc_sp = h.saved_sp;
        gc_disabled = h.saved_gc_disabled;
        Value ex = exception_value;
        if (catch == NIL) {
            if (finally != NIL) call_fn0(finally);
            return cljn_throw(ex);
        }
        result = call_fn1(catch, ex);
    }
    if (finally != NIL) {
        gc_stack[gc_sp++] = result;
        call_fn0(finally);
        gc_sp--;
    }
    return result;
}
static void gc_mark_exceptions(void) { gc_mark(exception_value); }
