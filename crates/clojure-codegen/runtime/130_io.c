
/* ---------- I/O nativo (ADR-0007) — subconjunto: slurp/spit/file-exists?/getenv ----------
 * Primitivas atrás da ABI C (ADR-0007): o frontend não emite syscalls nem lida com
 * descritores. Erros do SO viram mapas ex-data lançados por cljn_throw (capturáveis
 * com try/catch). Primeiro gate: Linux, síncrono, bloqueante, texto UTF-8.
 * Este é um subconjunto do gate da IO_SPEC (slurp/spit/exists + getenv); streams
 * bufferizados, handles, filesystem recursivo e reader de runtime ficam para depois. */
#include <errno.h>
#include <unistd.h>

/* Copia uma Str para um buffer C NUL-terminado (o chamador libera com free). */
static char *io_cstr(Value s) {
    Str *p = (Str *)s;
    char *c = (char *)xalloc(p->len + 1);
    if (p->len) memcpy(c, p->data, p->len);
    c[p->len] = 0;
    return c;
}
/* Categoria estável (:kind) a partir do errno. */
static const char *io_kind(int err) {
    switch (err) {
        case ENOENT: return "not-found";
        case EACCES: case EPERM: return "permission-denied";
        case EEXIST: return "already-exists";
        default: return "other";
    }
}
/* Constrói {:kind :operation :path :os-code :message} e lança (noreturn).
 * `path` deve estar rooteado pelo chamador (é usado durante as alocações). */
static void io_throw(const char *kind, const char *op, Value path, int err) {
    maybe_gc();
    gc_disabled++; /* aloca vários objetos; sem coleta no meio */
    Value m = cljn_map_alloc(5);
    cljn_map_set(m, 0, cljn_kw("kind", 4), cljn_kw(kind, (long)strlen(kind)));
    cljn_map_set(m, 1, cljn_kw("operation", 9), cljn_kw(op, (long)strlen(op)));
    cljn_map_set(m, 2, cljn_kw("path", 4), path);
    cljn_map_set(m, 3, cljn_kw("os-code", 7), MK_FIX(err));
    const char *msg = strerror(err);
    cljn_map_set(m, 4, cljn_kw("message", 7), cljn_str_from(msg, (long)strlen(msg)));
    gc_disabled--;
    cljn_throw(m);
}

/* (slurp path) -> string com o conteúdo do arquivo. Lê em blocos (sem chamada por byte). */
Value cljn_slurp(Value path) {
    if (obj_type(path) != T_STR) die("slurp: path deve ser string");
    cljn_gc_push(path); /* rooteia durante as alocações de string */
    char *cp = io_cstr(path);
    FILE *f = fopen(cp, "rb");
    free(cp);
    if (!f) io_throw(io_kind(errno), "slurp", path, errno);
    size_t cap = 4096, len = 0;
    char *buf = (char *)xalloc(cap);
    for (;;) {
        if (len == cap) { cap *= 2; buf = (char *)xrealloc(buf, cap); }
        size_t n = fread(buf + len, 1, cap - len, f);
        len += n;
        if (n == 0) {
            if (ferror(f)) { int e = errno; fclose(f); free(buf); io_throw("other", "slurp", path, e); }
            break;
        }
    }
    fclose(f);
    Value s = cljn_str_from(buf, (long)len);
    free(buf);
    cljn_gc_popn(1);
    return s;
}

/* (spit path content) -> nil. Trunca por padrão. `content` deve ser string. */
Value cljn_spit(Value path, Value content) {
    if (obj_type(path) != T_STR) die("spit: path deve ser string");
    if (obj_type(content) != T_STR) die("spit: conteúdo deve ser string");
    char *cp = io_cstr(path);
    FILE *f = fopen(cp, "wb");
    free(cp);
    if (!f) io_throw(io_kind(errno), "spit", path, errno);
    Str *c = (Str *)content;
    if (c->len && fwrite(c->data, 1, c->len, f) != (size_t)c->len) {
        int e = errno; fclose(f); io_throw("other", "spit", path, e);
    }
    fclose(f);
    return NIL;
}

/* (file-exists? path) -> bool. */
Value cljn_file_exists(Value path) {
    if (obj_type(path) != T_STR) die("file-exists?: path deve ser string");
    char *cp = io_cstr(path);
    int r = access(cp, F_OK);
    free(cp);
    return b2v(r == 0);
}

/* (getenv name) -> string | nil. */
Value cljn_getenv(Value name) {
    if (obj_type(name) != T_STR) die("getenv: nome deve ser string");
    char *cn = io_cstr(name);
    const char *v = getenv(cn);
    free(cn);
    return v ? cljn_str_from(v, (long)strlen(v)) : NIL;
}

/* ---------- Path (ADR-0007 / IO-1): manipulação de caminho POSIX ----------
 * Operações puramente textuais sobre strings (separador '/'). */
Value cljn_path_join(Value a, Value b) {
    if (obj_type(a) != T_STR || obj_type(b) != T_STR) die("path-join: esperava strings");
    Str *sa = (Str *)a, *sb = (Str *)b;
    if (sb->len > 0 && sb->data[0] == '/') return b; /* b absoluto vence */
    if (sa->len == 0) return b;
    size_t alen = sa->len;
    while (alen > 1 && sa->data[alen - 1] == '/') alen--; /* remove '/' final de a */
    size_t total = alen + 1 + sb->len;
    Str *s = (Str *)obj_alloc(sizeof(Str), T_STR); /* pode coletar; a,b rooteados */
    s->len = total;
    s->data = xalloc(total ? total : 1);
    sa = (Str *)a; sb = (Str *)b; /* revalida após possível GC */
    memcpy(s->data, sa->data, alen);
    s->data[alen] = '/';
    memcpy(s->data + alen + 1, sb->data, sb->len);
    return (Value)s;
}
Value cljn_file_name(Value p) {
    if (obj_type(p) != T_STR) die("file-name: esperava string");
    Str *s = (Str *)p;
    size_t i = s->len;
    while (i > 0 && s->data[i - 1] != '/') i--;
    return cljn_str_from(s->data + i, (long)(s->len - i)); /* GC ok: p rooteado */
}
Value cljn_parent(Value p) {
    if (obj_type(p) != T_STR) die("parent: esperava string");
    Str *s = (Str *)p;
    size_t i = s->len;
    while (i > 0 && s->data[i - 1] != '/') i--;
    if (i == 0) return NIL; /* sem separador → sem pai */
    size_t end = i;
    while (end > 1 && s->data[end - 1] == '/') end--; /* mantém "/" da raiz */
    return cljn_str_from(s->data, (long)end);
}

/* (with-out-str thunk) — roda o thunk (fn de 0 arg) com *out* rebindado a um Writer
 * de string e devolve o texto acumulado. Restaura *out* mesmo se o corpo lançar
 * (repropaga a exceção), reusando a pilha de handlers de exceção. `old` e `w` ficam
 * rooteados na shadow-stack durante o corpo (o writer de stdout só é referenciado
 * por *out*, que reescrevemos). */
Value cljn_with_out_str(Value thunk) {
    Value w = cljn_string_writer();
    cljn_gc_push(w);
    Value old = dynvar_get(VAR_OUT);
    cljn_gc_push(old);
    dyn_vars[VAR_OUT] = w;
    Handler h;
    h.prev = handler_top;
    h.saved_sp = gc_sp;
    h.saved_gc_disabled = gc_disabled;
    if (setjmp(h.env) == 0) {
        handler_top = &h;
        call_fn0(thunk); /* valor do corpo é descartado */
        handler_top = h.prev;
    } else {
        handler_top = h.prev;
        gc_sp = h.saved_sp;
        gc_disabled = h.saved_gc_disabled;
        dyn_vars[VAR_OUT] = gc_stack[h.saved_sp - 1]; /* restaura *out* (old) */
        return cljn_throw(exception_value);
    }
    dyn_vars[VAR_OUT] = old;
    Value s = cljn_writer_to_string(w);
    cljn_gc_popn(2);
    return s;
}

/* (binding [v nv] corpo) — roda o thunk com a Var `id` rebindada a `nv`; devolve o
 * valor do corpo. Restaura a Var no retorno e na exceção (repropaga). Generaliza
 * cljn_with_out_str para qualquer Var dinâmica. `nv`/`old` rooteados no corpo;
 * `i` é volatile para sobreviver ao longjmp. */
Value cljn_with_binding(Value id, Value nv, Value thunk) {
    volatile int i = (int)FIX(id);
    cljn_gc_push(nv);
    Value old = dynvar_get(i);
    cljn_gc_push(old);
    dyn_vars[i] = nv;
    Handler h;
    h.prev = handler_top;
    h.saved_sp = gc_sp;
    h.saved_gc_disabled = gc_disabled;
    Value r;
    if (setjmp(h.env) == 0) {
        handler_top = &h;
        r = call_fn0(thunk);
        handler_top = h.prev;
    } else {
        handler_top = h.prev;
        gc_sp = h.saved_sp;
        gc_disabled = h.saved_gc_disabled;
        dyn_vars[i] = gc_stack[h.saved_sp - 1]; /* restaura old */
        return cljn_throw(exception_value);
    }
    dyn_vars[i] = old;
    cljn_gc_popn(2);
    return r;
}
