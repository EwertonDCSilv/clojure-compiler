
/* ---------- Vars dinâmicas + writers (ADR-0007 / IO_SPEC IO-0) ----------
 * Base para redirecionar a saída: *out* é uma Var dinâmica que guarda um Writer.
 * print/println escrevem no Writer corrente; with-out-str rebinda *out* para um
 * Writer de string e devolve o texto acumulado. Single-thread: as Vars são um
 * vetor global; `binding`/with-out-str salvam e restauram o valor anterior. */

/* Ids das Vars dinâmicas embutidas (devem casar com o analyzer). */
enum { VAR_OUT = 0, VAR_ERR = 1, VAR_FLUSH = 2, NDYNVAR = 3 };
static Value dyn_vars[NDYNVAR];
static void gc_mark_dynvars(void) {
    for (int i = 0; i < NDYNVAR; i++) gc_mark(dyn_vars[i]);
}

static Value mk_std_writer(int kind) {
    Writer *w = (Writer *)obj_alloc(sizeof(Writer), T_WRITER);
    w->kind = kind;
    w->buf = NULL;
    w->len = 0;
    w->cap = 0;
    return (Value)w;
}
/* Leitura de Var com init preguiçoso (evita depender de ordem de construtores). */
static Value dynvar_get(int id) {
    if (dyn_vars[id] == 0) {
        if (id == VAR_OUT) dyn_vars[VAR_OUT] = mk_std_writer(WR_STDOUT);
        else if (id == VAR_ERR) dyn_vars[VAR_ERR] = mk_std_writer(WR_STDERR);
        else if (id == VAR_FLUSH) dyn_vars[VAR_FLUSH] = TRUEV;
        else dyn_vars[id] = NIL;
    }
    return dyn_vars[id];
}

/* Escreve n bytes no Writer: stdout/stderr direto; string acumula no buffer. */
static void writer_write(Value w, const char *p, size_t n) {
    Writer *wr = (Writer *)w;
    if (wr->kind == WR_STDOUT) { fwrite(p, 1, n, stdout); return; }
    if (wr->kind == WR_STDERR) { fwrite(p, 1, n, stderr); return; }
    if (wr->len + n > wr->cap) {
        size_t c = wr->cap ? wr->cap : 64;
        while (wr->len + n > c) c *= 2;
        wr->buf = (char *)xrealloc(wr->buf, c);
        wr->cap = c;
    }
    memcpy(wr->buf + wr->len, p, n);
    wr->len += n;
}

/* Novo Writer de string (destino de with-out-str). */
Value cljn_string_writer(void) {
    return mk_std_writer(WR_STRING);
}
/* Texto acumulado num Writer de string. */
Value cljn_writer_to_string(Value w) {
    Writer *wr = (Writer *)w;
    return cljn_str_from(wr->buf ? wr->buf : "", (long)wr->len);
}

/* Leitura de Var dinâmica como valor (id = fixnum tagged). Ex.: *out* → este writer. */
Value cljn_var_get(Value id) { return dynvar_get((int)FIX(id)); }
