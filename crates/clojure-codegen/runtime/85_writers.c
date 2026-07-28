
/* ---------- Vars dinâmicas + writers (ADR-0007 / IO_SPEC IO-0) ----------
 * Base para redirecionar a saída: *out* é uma Var dinâmica que guarda um Writer.
 * print/println escrevem no Writer corrente; with-out-str rebinda *out* para um
 * Writer de string e devolve o texto acumulado. Single-thread: as Vars são um
 * vetor global; `binding`/with-out-str salvam e restauram o valor anterior. */

/* Ids das Vars dinâmicas embutidas (devem casar com o analyzer). */
enum { VAR_OUT = 0, VAR_ERR = 1, VAR_FLUSH = 2, VAR_IN = 3, NDYNVAR = 4 };
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
/* ---------- UTF-8 (para Char e read-line/read-char) ---------- */
static int utf8_encode(uint32_t cp, char *out) {
    if (cp < 0x80) { out[0] = (char)cp; return 1; }
    if (cp < 0x800) {
        out[0] = (char)(0xC0 | (cp >> 6));
        out[1] = (char)(0x80 | (cp & 0x3F));
        return 2;
    }
    if (cp < 0x10000) {
        out[0] = (char)(0xE0 | (cp >> 12));
        out[1] = (char)(0x80 | ((cp >> 6) & 0x3F));
        out[2] = (char)(0x80 | (cp & 0x3F));
        return 3;
    }
    out[0] = (char)(0xF0 | (cp >> 18));
    out[1] = (char)(0x80 | ((cp >> 12) & 0x3F));
    out[2] = (char)(0x80 | ((cp >> 6) & 0x3F));
    out[3] = (char)(0x80 | (cp & 0x3F));
    return 4;
}
static int utf8_len(unsigned char c) {
    if (c < 0x80) return 1;
    if ((c >> 5) == 0x6) return 2;
    if ((c >> 4) == 0xE) return 3;
    if ((c >> 3) == 0x1E) return 4;
    return 1; /* byte de continuação/ inválido: trata como 1 */
}
static uint32_t utf8_decode(const char *p, int64_t avail, int *nout) {
    unsigned char c0 = (unsigned char)p[0];
    int n = utf8_len(c0);
    if (n > avail) n = 1; /* truncado */
    uint32_t cp;
    if (n == 1) cp = c0;
    else if (n == 2) cp = ((c0 & 0x1F) << 6) | (p[1] & 0x3F);
    else if (n == 3) cp = ((uint32_t)(c0 & 0x0F) << 12) | ((p[1] & 0x3F) << 6) | (p[2] & 0x3F);
    else cp = ((uint32_t)(c0 & 0x07) << 18) | ((uint32_t)(p[1] & 0x3F) << 12) | ((p[2] & 0x3F) << 6) | (p[3] & 0x3F);
    *nout = n;
    return cp;
}

static Value mk_reader(int kind, Value src) {
    Reader *r = (Reader *)obj_alloc(sizeof(Reader), T_READER);
    r->kind = kind;
    r->src = src;
    r->pos = 0;
    return (Value)r;
}
/* Leitura de Var com init preguiçoso (evita depender de ordem de construtores). */
static Value dynvar_get(int id) {
    if (dyn_vars[id] == 0) {
        if (id == VAR_OUT) dyn_vars[VAR_OUT] = mk_std_writer(WR_STDOUT);
        else if (id == VAR_ERR) dyn_vars[VAR_ERR] = mk_std_writer(WR_STDERR);
        else if (id == VAR_FLUSH) dyn_vars[VAR_FLUSH] = TRUEV;
        else if (id == VAR_IN) dyn_vars[VAR_IN] = mk_reader(RD_STDIN, NIL);
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

/* Novo Reader sobre uma string (destino de with-in-str; lê a partir do início). */
Value cljn_string_reader(Value s) {
    if (obj_type(s) != T_STR) die("with-in-str: esperava string");
    return mk_reader(RD_STRING, s);
}
/* (read-line) — lê uma linha do *in* corrente (sem o '\n'); nil no fim da entrada. */
Value cljn_read_line(void) {
    Value in = dynvar_get(VAR_IN); /* pode alocar (init preguiçoso) */
    Reader *r = (Reader *)in;
    if (r->kind == RD_STRING) {
        Str *s = (Str *)r->src;
        if (r->pos >= (int64_t)s->len) return NIL; /* fim */
        int64_t start = r->pos;
        while (r->pos < (int64_t)s->len && s->data[r->pos] != '\n') r->pos++;
        int64_t linelen = r->pos - start;
        if (r->pos < (int64_t)s->len) r->pos++; /* consome o '\n' */
        return cljn_str_from(s->data + start, (long)linelen); /* GC ok: src rooteado */
    }
    /* stdin: lê byte a byte até '\n'/EOF num buffer que cresce */
    size_t cap = 128, len = 0;
    char *buf = (char *)xalloc(cap);
    int c;
    while ((c = fgetc(stdin)) != EOF && c != '\n') {
        if (len + 1 > cap) { cap *= 2; buf = (char *)xrealloc(buf, cap); }
        buf[len++] = (char)c;
    }
    if (c == EOF && len == 0) { free(buf); return NIL; }
    Value v = cljn_str_from(buf, (long)len);
    free(buf);
    return v;
}

/* (read-char) — lê um caractere (codepoint UTF-8) do *in* corrente; nil no fim. */
Value cljn_read_char(void) {
    Value in = dynvar_get(VAR_IN);
    Reader *r = (Reader *)in;
    if (r->kind == RD_STRING) {
        Str *s = (Str *)r->src;
        if (r->pos >= (int64_t)s->len) return NIL;
        int n;
        uint32_t cp = utf8_decode(s->data + r->pos, (int64_t)s->len - r->pos, &n);
        r->pos += n;
        return MK_CHAR(cp);
    }
    int c0 = fgetc(stdin);
    if (c0 == EOF) return NIL;
    int n = utf8_len((unsigned char)c0);
    char buf[4];
    buf[0] = (char)c0;
    for (int i = 1; i < n; i++) {
        int ci = fgetc(stdin);
        if (ci == EOF) { n = i; break; }
        buf[i] = (char)ci;
    }
    int m;
    uint32_t cp = utf8_decode(buf, n, &m);
    return MK_CHAR(cp);
}
