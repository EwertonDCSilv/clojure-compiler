
/* ---------- exceções: try/catch/finally + throw ----------
 * setjmp/longjmp ficam INTEIRAMENTE dentro de cljn_try (função C): nenhum código
 * gerado pelo Cranelift atravessa o setjmp. As closures corpo/catch/finally são
 * invocadas pela ABI uniforme. `throw` desenrola até o handler mais próximo,
 * restaurando shadow-stack e gc_disabled (evita corrupção/leak em unwind). */
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

Value cljn_try(Value body, Value catch, Value finally) {
    Handler h;
    h.prev = handler_top;
    h.saved_sp = gc_sp;
    h.saved_gc_disabled = gc_disabled;
    Value result;
    if (setjmp(h.env) == 0) {
        handler_top = &h;
        result = call_fn0(body);
        handler_top = h.prev; /* caminho normal: desempilha o handler */
    } else {
        handler_top = h.prev;           /* exceções em catch/finally vão ao externo */
        gc_sp = h.saved_sp;             /* desenrola temporários do corpo */
        gc_disabled = h.saved_gc_disabled;
        Value ex = exception_value;
        if (catch == NIL) {
            if (finally != NIL) call_fn0(finally);
            return cljn_throw(ex);      /* sem catch: roda finally e repropaga */
        }
        result = call_fn1(catch, ex);   /* catch pode lançar → handler externo */
    }
    if (finally != NIL) {
        gc_stack[gc_sp++] = result;     /* rooteia o resultado durante o finally */
        call_fn0(finally);
        gc_sp--;
    }
    return result;
}
static void gc_mark_exceptions(void) { gc_mark(exception_value); }
