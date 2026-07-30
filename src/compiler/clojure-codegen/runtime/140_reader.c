
/*
 * Runtime EDN reader used by read-string.
 *
 * Recursive descent produces native Values for nil, booleans, integers,
 * strings, keywords, characters, vectors, maps, sets, and lists. Symbols,
 * floats, ratios, and other reader macros are unsupported and terminate through
 * the fatal diagnostic path.
 *
 * GC: parsing runs in a bounded no-GC region because partial structures are not
 * individually rooted. Allocation is bounded by the already-resident input.
 */

typedef struct { const char *p; int64_t len; int64_t pos; } RS;

static Value rs_form(RS *r); /* fwd */

/* Reader parse errors are catchable: throw {:kind :invalid-input} so read/
 * read-string callers can recover with try/catch instead of aborting. */
static void rs_fail(void) {
    gc_disabled++;
    Value m = cljn_map_alloc(1);
    cljn_map_set(m, 0, cljn_kw("kind", 4), cljn_kw("invalid-input", 13));
    gc_disabled--;
    cljn_throw(m);
}

static int rs_peek(RS *r) { return r->pos < r->len ? (unsigned char)r->p[r->pos] : -1; }
static int rs_is_delim(int c) {
    return c == -1 || c == ' ' || c == '\t' || c == '\n' || c == '\r' || c == ',' ||
           c == '[' || c == ']' || c == '{' || c == '}' || c == '(' || c == ')' ||
           c == '"' || c == ';';
}
static void rs_ws(RS *r) {
    while (r->pos < r->len) {
        char c = r->p[r->pos];
        if (c == ' ' || c == '\t' || c == '\n' || c == '\r' || c == ',') { r->pos++; continue; }
        if (c == ';') { while (r->pos < r->len && r->p[r->pos] != '\n') r->pos++; continue; }
        break;
    }
}

static Value rs_number(RS *r) {
    int64_t start = r->pos;
    if (rs_peek(r) == '-' || rs_peek(r) == '+') r->pos++;
    while (r->pos < r->len && r->p[r->pos] >= '0' && r->p[r->pos] <= '9') r->pos++;
    if (r->pos < r->len) {
        char c = r->p[r->pos];
        if (c == '.' || c == 'e' || c == 'E' || c == '/' || c == 'M' || c == 'N')
            rs_fail();
    }
    int64_t i = start, n = 0, neg = 0;
    if (r->p[i] == '-') { neg = 1; i++; } else if (r->p[i] == '+') i++;
    for (; i < r->pos; i++) n = n * 10 + (r->p[i] - '0');
    return MK_FIX(neg ? -n : n);
}

static Value rs_string(RS *r) {
    r->pos++; /* " */
    size_t cap = 16, len = 0;
    char *buf = (char *)xalloc(cap);
    while (r->pos < r->len) {
        char c = r->p[r->pos++];
        if (c == '"') { Value s = cljn_str_from(buf, (long)len); free(buf); return s; }
        if (c == '\\' && r->pos < r->len) {
            char e = r->p[r->pos++];
            switch (e) {
                case 'n': c = '\n'; break;
                case 't': c = '\t'; break;
                case 'r': c = '\r'; break;
                case '"': c = '"'; break;
                case '\\': c = '\\'; break;
                default: c = e;
            }
        }
        if (len + 1 > cap) { cap *= 2; buf = (char *)xrealloc(buf, cap); }
        buf[len++] = c;
    }
    free(buf);
    rs_fail();
    return NIL;
}

static Value rs_keyword(RS *r) {
    r->pos++; /* : */
    int64_t start = r->pos;
    while (r->pos < r->len && !rs_is_delim((unsigned char)r->p[r->pos])) r->pos++;
    return cljn_kw(r->p + start, r->pos - start);
}

static Value rs_char(RS *r) {
    r->pos++; /* backslash */
    if (r->pos >= r->len) rs_fail();
    int64_t start = r->pos;
    r->pos++; /* Always consume the first token byte. */
    while (r->pos < r->len && !rs_is_delim((unsigned char)r->p[r->pos])) r->pos++;
    int64_t tl = r->pos - start;
    const char *t = r->p + start;
    if (tl == 1) return MK_CHAR((unsigned char)t[0]);
    if (t[0] == 'u' && tl > 1) {
        uint32_t cp = 0;
        for (int64_t i = 1; i < tl; i++) {
            char c = t[i]; int d;
            if (c >= '0' && c <= '9') d = c - '0';
            else if (c >= 'a' && c <= 'f') d = c - 'a' + 10;
            else if (c >= 'A' && c <= 'F') d = c - 'A' + 10;
            else rs_fail();
            cp = cp * 16 + (uint32_t)d;
        }
        return MK_CHAR(cp);
    }
    if (tl == 5 && !memcmp(t, "space", 5)) return MK_CHAR(' ');
    if (tl == 7 && !memcmp(t, "newline", 7)) return MK_CHAR('\n');
    if (tl == 3 && !memcmp(t, "tab", 3)) return MK_CHAR('\t');
    if (tl == 6 && !memcmp(t, "return", 6)) return MK_CHAR('\r');
    if (tl == 8 && !memcmp(t, "formfeed", 8)) return MK_CHAR('\f');
    if (tl == 9 && !memcmp(t, "backspace", 9)) return MK_CHAR('\b');
    { int n; uint32_t cp = utf8_decode(t, tl, &n); if (n == tl) return MK_CHAR(cp); }
    rs_fail();
    return NIL;
}

static Value rs_vector(RS *r) {
    r->pos++; /* [ */
    Value v = cljn_vec_empty();
    for (;;) {
        rs_ws(r);
        int c = rs_peek(r);
        if (c == ']') { r->pos++; return v; }
        if (c == -1) rs_fail();
        v = cljn_conj(v, rs_form(r));
    }
}
static Value rs_set(RS *r) {
    r->pos++; /* { (o '#' já foi consumido) */
    Value s = cljn_set_alloc(0);
    for (;;) {
        rs_ws(r);
        int c = rs_peek(r);
        if (c == '}') { r->pos++; return s; }
        if (c == -1) rs_fail();
        s = cljn_conj(s, rs_form(r));
    }
}
static Value rs_map(RS *r) {
    r->pos++; /* { */
    Value m = cljn_map_alloc(0);
    for (;;) {
        rs_ws(r);
        int c = rs_peek(r);
        if (c == '}') { r->pos++; return m; }
        if (c == -1) rs_fail();
        Value k = rs_form(r);
        rs_ws(r);
        if (rs_peek(r) == '}' || rs_peek(r) == -1) rs_fail();
        Value val = rs_form(r);
        m = cljn_assoc(m, k, val);
    }
}
static Value rs_list(RS *r) {
    r->pos++; /* ( */
    size_t cap = 8, n = 0;
    Value *arr = (Value *)xalloc(cap * sizeof(Value));
    for (;;) {
        rs_ws(r);
        int c = rs_peek(r);
        if (c == ')') { r->pos++; break; }
        if (c == -1) { free(arr); rs_fail(); }
        if (n == cap) { cap *= 2; arr = (Value *)xrealloc(arr, cap * sizeof(Value)); }
        arr[n++] = rs_form(r);
    }
    Value lst = EMPTY;
    for (int64_t i = (int64_t)n - 1; i >= 0; i--) lst = cljn_cons(arr[i], lst);
    free(arr);
    return lst;
}

static Value rs_form(RS *r) {
    rs_ws(r);
    int c = rs_peek(r);
    if (c == -1) rs_fail();
    if (c == '[') return rs_vector(r);
    if (c == '(') return rs_list(r);
    if (c == '{') return rs_map(r);
    if (c == '#') {
        r->pos++;
        if (rs_peek(r) == '{') return rs_set(r);
        rs_fail();
    }
    if (c == '"') return rs_string(r);
    if (c == ':') return rs_keyword(r);
    if (c == '\\') return rs_char(r);
    if ((c == '-' || c == '+') && r->pos + 1 < r->len &&
        r->p[r->pos + 1] >= '0' && r->p[r->pos + 1] <= '9')
        return rs_number(r);
    if (c >= '0' && c <= '9') return rs_number(r);
    /* Only nil/true/false are accepted as non-numeric bare tokens. */
    int64_t start = r->pos;
    while (r->pos < r->len && !rs_is_delim((unsigned char)r->p[r->pos])) r->pos++;
    int64_t tl = r->pos - start;
    const char *t = r->p + start;
    if (tl == 3 && !memcmp(t, "nil", 3)) return NIL;
    if (tl == 4 && !memcmp(t, "true", 4)) return TRUEV;
    if (tl == 5 && !memcmp(t, "false", 5)) return FALSEV;
    rs_fail();
    return NIL;
}

/* Parse and return the first supported EDN value in runtime string `sv`.
 * Parse errors throw {:kind :invalid-input} (catchable by read-string callers). */
Value cljn_read_string(Value sv) {
    if (obj_type(sv) != T_STR) die("read-string: esperava string");
    Str *s = (Str *)sv;
    RS r = { s->data, (int64_t)s->len, 0 };
    gc_disabled++;
    Value v = rs_form(&r);
    gc_disabled--;
    return v;
}

/* Read one form from an open string reader, advancing its position. Parse errors
 * throw {:kind :invalid-input}. Only string readers are supported. */
Value cljn_read_from(Value rv) {
    Reader *rd = (Reader *)rv;
    if (rd->kind != RD_STRING || obj_type(rd->src) != T_STR) rs_fail();
    Str *s = (Str *)rd->src;
    RS r = { s->data, (int64_t)s->len, rd->pos };
    gc_disabled++;
    Value v = rs_form(&r);
    gc_disabled--;
    rd->pos = r.pos;
    return v;
}

/* True when a string reader has only whitespace remaining; consumes that
 * whitespace so a following read-from starts at end-of-input. */
Value cljn_reader_eof(Value rv) {
    Reader *rd = (Reader *)rv;
    if (rd->kind != RD_STRING || obj_type(rd->src) != T_STR) return b2v(1);
    Str *s = (Str *)rd->src;
    RS r = { s->data, (int64_t)s->len, rd->pos };
    rs_ws(&r);
    rd->pos = r.pos;
    return b2v(r.pos >= r.len);
}
