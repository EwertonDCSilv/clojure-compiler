/*
 * Dynamic Vars, writers, readers, and UTF-8 stream primitives.
 *
 * The single-threaded runtime stores built-in dynamic Vars in a global table.
 * print/println target the Writer in *out*; read-line/read-char consume *in*.
 * Rebinding helpers save and restore prior Values, including exceptional exit.
 *
 * ABI: DynVar IDs must match dyn_var_id in clojure-analyzer.
 * GC: initialized dynamic Var values are permanent roots.
 */

/* ABI: built-in dynamic Var IDs shared with the analyzer. */
enum { VAR_OUT = 0, VAR_ERR = 1, VAR_FLUSH = 2, VAR_IN = 3, VAR_ARGS = 4, NDYNVAR = 5 };
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
    w->fp = NULL;
    w->closed = 0;
    return (Value)w;
}
/* UTF-8 helpers for immediate characters and stream decoding. */
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
    return 1; /* Invalid or continuation byte advances as one byte. */
}
static uint32_t utf8_decode(const char *p, int64_t avail, int *nout) {
    unsigned char c0 = (unsigned char)p[0];
    int n = utf8_len(c0);
    if (n > avail) n = 1;
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
    r->fp = NULL;
    r->pushback = NIL;
    r->closed = 0;
    return (Value)r;
}
/* Read a dynamic Var, lazily constructing standard stream wrappers. */
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

/* Write bytes to a standard stream, file, or growable string buffer. */
static void writer_write(Value w, const char *p, size_t n) {
    Writer *wr = (Writer *)w;
    if (wr->kind == WR_STDOUT) { fwrite(p, 1, n, stdout); return; }
    if (wr->kind == WR_STDERR) { fwrite(p, 1, n, stderr); return; }
    if (wr->kind == WR_FILE) { if (wr->fp) fwrite(p, 1, n, (FILE *)wr->fp); return; }
    if (wr->len + n > wr->cap) {
        size_t c = wr->cap ? wr->cap : 64;
        while (wr->len + n > c) c *= 2;
        wr->buf = (char *)xrealloc(wr->buf, c);
        wr->cap = c;
    }
    memcpy(wr->buf + wr->len, p, n);
    wr->len += n;
}

/* Allocate an empty string-capture Writer. */
Value cljn_string_writer(void) {
    return mk_std_writer(WR_STRING);
}
/* Copy a string Writer's accumulated bytes into a runtime string. */
Value cljn_writer_to_string(Value w) {
    Writer *wr = (Writer *)w;
    return cljn_str_from(wr->buf ? wr->buf : "", (long)wr->len);
}

/* Return a built-in dynamic Var selected by tagged fixnum ID. */
Value cljn_var_get(Value id) { return dynvar_get((int)FIX(id)); }

/*
 * Capture arguments after argv[0] in the *command-line-args* dynamic Var.
 *
 * ABI: called once by generated main with raw argc and argv.
 * GC: the bounded no-GC region protects the unrooted partial vector; the final
 * vector becomes reachable from the permanently marked dynamic-Var table.
 */
void cljn_set_args(int64_t argc, char **argv) {
    gc_disabled++;
    Value v = cljn_vec_empty();
    for (int64_t i = 1; i < argc; i++) {
        v = cljn_conj(v, cljn_str_from(argv[i], (long)strlen(argv[i])));
    }
    gc_disabled--;
    dyn_vars[VAR_ARGS] = v;
}

/* Allocate a Reader positioned at the start of a runtime string. */
Value cljn_string_reader(Value s) {
    if (obj_type(s) != T_STR) die("with-in-str: esperava string");
    return mk_reader(RD_STRING, s);
}
/* Read one line from *in* without newline, returning NIL at EOF.
 * Ownership: temporary FILE buffers are freed after the result is copied. */
Value cljn_read_line(void) {
    Value in = dynvar_get(VAR_IN);
    Reader *r = (Reader *)in;
    if (r->kind == RD_STRING) {
        Str *s = (Str *)r->src;
        if (r->pos >= (int64_t)s->len) return NIL;
        int64_t start = r->pos;
        while (r->pos < (int64_t)s->len && s->data[r->pos] != '\n') r->pos++;
        int64_t linelen = r->pos - start;
        if (r->pos < (int64_t)s->len) r->pos++;
        return cljn_str_from(s->data + start, (long)linelen);
    }
    /* Standard/file input grows a temporary buffer until newline or EOF. */
    FILE *fp = (r->kind == RD_FILE) ? (FILE *)r->fp : stdin;
    if (!fp) return NIL;
    size_t cap = 128, len = 0;
    char *buf = (char *)xalloc(cap);
    int c;
    while ((c = fgetc(fp)) != EOF && c != '\n') {
        if (len + 1 > cap) { cap *= 2; buf = (char *)xrealloc(buf, cap); }
        buf[len++] = (char)c;
    }
    if (c == EOF && len == 0) { free(buf); return NIL; }
    Value v = cljn_str_from(buf, (long)len);
    free(buf);
    return v;
}

/* Decode one UTF-8 character from *in*, returning NIL at EOF. */
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
    FILE *fp = (r->kind == RD_FILE) ? (FILE *)r->fp : stdin;
    if (!fp) return NIL;
    int c0 = fgetc(fp);
    if (c0 == EOF) return NIL;
    int n = utf8_len((unsigned char)c0);
    char buf[4];
    buf[0] = (char)c0;
    for (int i = 1; i < n; i++) {
        int ci = fgetc(fp);
        if (ci == EOF) { n = i; break; }
        buf[i] = (char)ci;
    }
    int m;
    uint32_t cp = utf8_decode(buf, n, &m);
    return MK_CHAR(cp);
}

/* Return NIL for a non-stream, else TRUE/FALSE for a Reader/Writer's closed flag.
   The compiled cljn.io wrappers use the NIL sentinel to raise `:invalid-input`. */
Value cljn_stream_closed(Value x) {
    int t = obj_type(x);
    if (t == T_READER) return b2v(((Reader *)x)->closed != 0);
    if (t == T_WRITER) return b2v(((Writer *)x)->closed != 0);
    return NIL;
}
/* Predicate: value is a Reader. */
Value cljn_reader_p(Value x) { return b2v(obj_type(x) == T_READER); }
/* Predicate: value is a Writer. */
Value cljn_writer_p(Value x) { return b2v(obj_type(x) == T_WRITER); }
/* Predicate: value is a file/string stream that may be closed (not a standard
   stream, which cljn.io/close! must reject). */
Value cljn_closeable_p(Value x) {
    int t = obj_type(x);
    if (t == T_READER) { int64_t k = ((Reader *)x)->kind; return b2v(k == RD_STRING || k == RD_FILE); }
    if (t == T_WRITER) { int64_t k = ((Writer *)x)->kind; return b2v(k == WR_STRING || k == WR_FILE); }
    return b2v(0);
}

/* Decode one UTF-8 character from an explicit Reader, honoring one pushed-back
   character; returns NIL at end of input. Aborts on a non-Reader (the wrapper
   validates first). */
Value cljn_read_char_from(Value rv) {
    if (obj_type(rv) != T_READER) die("read-char: esperava reader");
    Reader *r = (Reader *)rv;
    if (r->pushback != NIL) {
        Value c = r->pushback;
        r->pushback = NIL;
        return c;
    }
    if (r->kind == RD_STRING) {
        Str *s = (Str *)r->src;
        if (r->pos >= (int64_t)s->len) return NIL;
        int n;
        uint32_t cp = utf8_decode(s->data + r->pos, (int64_t)s->len - r->pos, &n);
        r->pos += n;
        return MK_CHAR(cp);
    }
    FILE *fp = (r->kind == RD_FILE) ? (FILE *)r->fp : stdin;
    if (!fp) return NIL;
    int c0 = fgetc(fp);
    if (c0 == EOF) return NIL;
    int n = utf8_len((unsigned char)c0);
    char buf[4];
    buf[0] = (char)c0;
    for (int i = 1; i < n; i++) {
        int ci = fgetc(fp);
        if (ci == EOF) { n = i; break; }
        buf[i] = (char)ci;
    }
    int m;
    uint32_t cp = utf8_decode(buf, n, &m);
    return MK_CHAR(cp);
}
/* Read one line (without newline) from an explicit Reader; NIL at end of input.
   A pushed-back character is prepended to the line. */
Value cljn_read_line_from(Value rv) {
    if (obj_type(rv) != T_READER) die("read-line: esperava reader");
    Reader *r = (Reader *)rv;
    char prefix[4];
    int prefix_len = 0;
    if (r->pushback != NIL) {
        prefix_len = utf8_encode(CHAR_CP(r->pushback), prefix);
        r->pushback = NIL;
    }
    if (r->kind == RD_STRING) {
        Str *s = (Str *)r->src;
        if (prefix_len == 0 && r->pos >= (int64_t)s->len) return NIL;
        int64_t start = r->pos;
        while (r->pos < (int64_t)s->len && s->data[r->pos] != '\n') r->pos++;
        int64_t linelen = r->pos - start;
        if (r->pos < (int64_t)s->len) r->pos++;
        if (prefix_len == 0) return cljn_str_from(s->data + start, (long)linelen);
        size_t total = (size_t)prefix_len + (size_t)linelen;
        char *buf = (char *)xalloc(total);
        memcpy(buf, prefix, (size_t)prefix_len);
        memcpy(buf + prefix_len, s->data + start, (size_t)linelen);
        Value v = cljn_str_from(buf, (long)total);
        free(buf);
        return v;
    }
    FILE *fp = (r->kind == RD_FILE) ? (FILE *)r->fp : stdin;
    if (!fp) return prefix_len ? cljn_str_from(prefix, prefix_len) : NIL;
    size_t cap = 128, len = (size_t)prefix_len;
    char *buf = (char *)xalloc(cap);
    if (prefix_len) memcpy(buf, prefix, (size_t)prefix_len);
    int c;
    while ((c = fgetc(fp)) != EOF && c != '\n') {
        if (len + 1 > cap) { cap *= 2; buf = (char *)xrealloc(buf, cap); }
        buf[len++] = (char)c;
    }
    if (c == EOF && len == 0) { free(buf); return NIL; }
    Value v = cljn_str_from(buf, (long)len);
    free(buf);
    return v;
}
/* Push one immediate Char back into an explicit Reader so the next read returns
   it, requiring that at least one character was already consumed. Returns TRUE
   on success or FALSE when there is nothing to unread. */
Value cljn_unread_char_to(Value rv, Value ch) {
    if (obj_type(rv) != T_READER) die("unread-char: esperava reader");
    if (!IS_CHAR(ch)) die("unread-char: esperava char");
    Reader *r = (Reader *)rv;
    if (r->kind == RD_STRING && r->pos == 0 && r->pushback == NIL) return b2v(0);
    r->pushback = ch;
    return b2v(1);
}
/* Append a runtime string's UTF-8 bytes to an explicit Writer. */
Value cljn_write_to(Value wv, Value s) {
    if (obj_type(wv) != T_WRITER) die("write!: esperava writer");
    if (obj_type(s) != T_STR) die("write!: esperava string");
    Str *ss = (Str *)s;
    writer_write(wv, ss->data, ss->len);
    return NIL;
}
/* Flush an explicit Writer's file descriptor; string writers are a no-op. */
Value cljn_flush_writer(Value wv) {
    if (obj_type(wv) != T_WRITER) die("flush!: esperava writer");
    Writer *w = (Writer *)wv;
    if (w->kind == WR_FILE && w->fp) fflush((FILE *)w->fp);
    return NIL;
}
