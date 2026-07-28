
/*
 * Strict bounded HTTP/1.x request parser (ADR-0013 Gate 4, §5).
 *
 * One-shot parse of a complete request buffer into a request map
 * {:method kw :path str :headers map :body nil|str}, or a thrown categorized
 * error map {:cljn.error/domain :http :kind k :operation :parse :status n}.
 * This is a purpose-built parser for the P1 grammar, not a general web parser.
 * It accepts only identity framing with zero or one Content-Length and rejects
 * Transfer-Encoding, chunked bodies, folded headers, NUL, bare CR/LF, invalid
 * tokens/versions, duplicate/negative lengths, and limit overflow. Incomplete
 * input is never accepted as a complete request.
 *
 * GC: parsing runs in a bounded no-GC region (input is size-limited), so partial
 * request/header structures need not be individually rooted. On a thrown error
 * the exception handler restores gc_disabled.
 *
 * Planned: an incremental socket read loop bounds and buffers bytes before
 * calling this; sockets, poll, and signal handling are separate Gate 4 work.
 */

#define HTTP_MAX_REQUEST (64 * 1024)
#define HTTP_MAX_HEADERS 100

/* Build and throw a categorized parse error (caller already disabled the GC). */
static void http_throw(const char *kind, int status) {
    Value m = cljn_map_alloc(4);
    cljn_map_set(m, 0, cljn_kw("cljn.error/domain", 17), cljn_kw("http", 4));
    cljn_map_set(m, 1, cljn_kw("kind", 4), cljn_kw(kind, (long)strlen(kind)));
    cljn_map_set(m, 2, cljn_kw("operation", 9), cljn_kw("parse", 5));
    cljn_map_set(m, 3, cljn_kw("status", 6), MK_FIX(status));
    cljn_throw(m);
}

/* RFC 7230 token character (field-name and method bytes). */
static int http_token_char(unsigned char c) {
    if (c >= 'a' && c <= 'z') return 1;
    if (c >= 'A' && c <= 'Z') return 1;
    if (c >= '0' && c <= '9') return 1;
    switch (c) {
        case '!': case '#': case '$': case '%': case '&': case '\'': case '*':
        case '+': case '-': case '.': case '^': case '_': case '`': case '|':
        case '~':
            return 1;
        default:
            return 0;
    }
}

/* (parse-http-request s) -> request map, or throws a categorized error. */
Value cljn_parse_http_request(Value sv) {
    if (obj_type(sv) != T_STR) die("parse-http-request: esperava string");
    Str *s = (Str *)sv;
    const char *p = s->data;
    int64_t n = (int64_t)s->len;
    gc_disabled++; /* bounded input; no collection while building the map */

    if (n > HTTP_MAX_REQUEST) http_throw("request-too-large", 413);
    for (int64_t i = 0; i < n; i++) {
        if (p[i] == 0) http_throw("malformed-request", 400); /* NUL forbidden */
    }

    /* ----- request line: METHOD SP path SP HTTP/1.x CRLF ----- */
    int64_t i = 0;
    int64_t ms = i;
    while (i < n && http_token_char((unsigned char)p[i]) &&
           p[i] >= 'A' && p[i] <= 'Z')
        i++;
    if (i == ms || i >= n || p[i] != ' ') http_throw("malformed-request", 400);
    int64_t mlen = i - ms;
    i++; /* SP */

    int64_t ps = i;
    while (i < n && p[i] != ' ' && p[i] != '\r' && p[i] != '\n') i++;
    if (i == ps || i >= n || p[i] != ' ' || p[ps] != '/')
        http_throw("malformed-request", 400);
    int64_t plen = i - ps;
    int64_t path_start = ps;
    i++; /* SP */

    int64_t vs = i;
    while (i < n && p[i] != '\r' && p[i] != '\n') i++;
    int64_t vlen = i - vs;
    if (vlen != 8 || memcmp(p + vs, "HTTP/1.", 7) != 0 ||
        (p[vs + 7] != '0' && p[vs + 7] != '1'))
        http_throw("malformed-request", 400);
    if (i + 1 >= n || p[i] != '\r' || p[i + 1] != '\n')
        http_throw(i >= n ? "incomplete" : "malformed-request",
                   i >= n ? 400 : 400);
    i += 2; /* CRLF */

    /* method -> lowercase keyword; copy for the temporary lowercase buffer */
    char mbuf[32];
    if (mlen > 31) http_throw("malformed-request", 400);
    for (int64_t k = 0; k < mlen; k++) mbuf[k] = (char)(p[ms + k] - 'A' + 'a');
    Value method = cljn_kw(mbuf, (long)mlen);
    Value path = cljn_str_from(p + path_start, (long)plen);

    /* ----- headers until empty line ----- */
    Value headers = cljn_map_alloc(0);
    int64_t content_length = -1;
    int header_count = 0;
    for (;;) {
        if (i + 1 < n && p[i] == '\r' && p[i + 1] == '\n') { i += 2; break; }
        if (i >= n) http_throw("incomplete", 400);
        if (p[i] == ' ' || p[i] == '\t') http_throw("malformed-request", 400); /* folded */
        if (++header_count > HTTP_MAX_HEADERS) http_throw("headers-too-large", 431);

        int64_t ns = i;
        while (i < n && http_token_char((unsigned char)p[i])) i++;
        if (i == ns || i >= n || p[i] != ':') http_throw("malformed-request", 400);
        int64_t nlen = i - ns;
        i++; /* ':' */
        while (i < n && (p[i] == ' ' || p[i] == '\t')) i++; /* OWS */
        int64_t vstart = i;
        while (i < n && p[i] != '\r' && p[i] != '\n') i++;
        int64_t vend = i;
        while (vend > vstart && (p[vend - 1] == ' ' || p[vend - 1] == '\t')) vend--; /* trailing OWS */
        if (i + 1 >= n || p[i] != '\r' || p[i + 1] != '\n')
            http_throw(i >= n ? "incomplete" : "malformed-request", 400);
        i += 2; /* CRLF */

        /* lowercase field-name into a bounded buffer */
        char nbuf[256];
        if (nlen > 255) http_throw("headers-too-large", 431);
        for (int64_t k = 0; k < nlen; k++) {
            char c = p[ns + k];
            nbuf[k] = (c >= 'A' && c <= 'Z') ? (char)(c - 'A' + 'a') : c;
        }
        if (nlen == 17 && memcmp(nbuf, "transfer-encoding", 17) == 0)
            http_throw("malformed-request", 400); /* unsupported framing */
        if (nlen == 14 && memcmp(nbuf, "content-length", 14) == 0) {
            int64_t cl = 0;
            if (vend == vstart) http_throw("malformed-request", 400);
            for (int64_t k = vstart; k < vend; k++) {
                if (p[k] < '0' || p[k] > '9') http_throw("malformed-request", 400);
                cl = cl * 10 + (p[k] - '0');
                if (cl > HTTP_MAX_REQUEST) http_throw("request-too-large", 413);
            }
            if (content_length >= 0 && content_length != cl)
                http_throw("malformed-request", 400); /* divergent lengths */
            content_length = cl;
        }
        Value hk = cljn_kw(nbuf, (long)nlen);
        Value hv = cljn_str_from(p + vstart, (long)(vend - vstart));
        headers = cljn_assoc(headers, hk, hv);
    }

    /* ----- body: exactly Content-Length bytes, nothing trailing ----- */
    Value body = NIL;
    int64_t remaining = n - i;
    if (content_length >= 0) {
        if (remaining < content_length) http_throw("incomplete", 400);
        if (remaining > content_length) http_throw("malformed-request", 400);
        body = cljn_str_from(p + i, (long)content_length);
    } else if (remaining > 0) {
        http_throw("malformed-request", 400); /* body without Content-Length */
    }

    Value req = cljn_map_alloc(4);
    cljn_map_set(req, 0, cljn_kw("method", 6), method);
    cljn_map_set(req, 1, cljn_kw("path", 4), path);
    cljn_map_set(req, 2, cljn_kw("headers", 7), headers);
    cljn_map_set(req, 3, cljn_kw("body", 4), body);
    gc_disabled--;
    return req;
}

/* Reason phrase for a status code (empty when unknown; the status is authoritative). */
static const char *http_reason(int64_t s) {
    switch (s) {
        case 200: return "OK";
        case 201: return "Created";
        case 202: return "Accepted";
        case 204: return "No Content";
        case 301: return "Moved Permanently";
        case 302: return "Found";
        case 303: return "See Other";
        case 304: return "Not Modified";
        case 400: return "Bad Request";
        case 401: return "Unauthorized";
        case 403: return "Forbidden";
        case 404: return "Not Found";
        case 405: return "Method Not Allowed";
        case 408: return "Request Timeout";
        case 413: return "Payload Too Large";
        case 431: return "Request Header Fields Too Large";
        case 500: return "Internal Server Error";
        case 503: return "Service Unavailable";
        default: return "";
    }
}
static void sb_num(SB *b, int64_t x) {
    char t[24];
    int n = snprintf(t, sizeof t, "%ld", (long)x);
    sb_write(b, t, (size_t)n);
}
static void http_ser_throw(const char *kind) {
    Value m = cljn_map_alloc(3);
    cljn_map_set(m, 0, cljn_kw("cljn.error/domain", 17), cljn_kw("http", 4));
    cljn_map_set(m, 1, cljn_kw("kind", 4), cljn_kw(kind, (long)strlen(kind)));
    cljn_map_set(m, 2, cljn_kw("operation", 9), cljn_kw("serialize", 9));
    cljn_throw(m);
}

/* (serialize-http-response resp) -> string com os bytes da resposta HTTP/1.1.
 * Revalida status/headers/corpo, computa Content-Length e força Connection: close
 * (ADR-0013 §5). Emite os headers da aplicação na ordem do array-map (determinística
 * para <=8 headers); ordenação lexical e HAMT grande são refinamentos (Gate 5).
 * Não confia na normalização em Clojure. GC: região sem-GC; `resp` rooteado. */
Value cljn_serialize_http_response(Value resp) {
    int rt = obj_type(resp);
    if (rt != T_MAP && rt != T_HMAP) die("serialize-http-response: esperava mapa");
    gc_disabled++;
    Value status_v = cljn_map_get(resp, cljn_kw("status", 6));
    if (!IS_FIX(status_v)) http_ser_throw("invalid-status");
    int64_t status = FIX(status_v);
    if (status < 100 || status > 599) http_ser_throw("invalid-status");

    Value body = cljn_map_get(resp, cljn_kw("body", 4));
    const char *body_data = NULL;
    int64_t body_len = 0;
    if (body == NIL) {
        /* no body */
    } else if (obj_type(body) == T_STR) {
        body_data = ((Str *)body)->data;
        body_len = (int64_t)((Str *)body)->len;
    } else if (obj_type(body) == T_BYTES) {
        body_data = (const char *)((Bytes *)body)->data;
        body_len = ((Bytes *)body)->len;
    } else {
        http_ser_throw("invalid-body");
    }

    SB b;
    sb_init(&b);
    sb_str(&b, "HTTP/1.1 ");
    sb_num(&b, status);
    sb_putc(&b, ' ');
    sb_str(&b, http_reason(status));
    sb_str(&b, "\r\n");

    /* Application headers (array-map only in P1). */
    Value headers = cljn_map_get(resp, cljn_kw("headers", 7));
    if (headers != NIL) {
        if (obj_type(headers) != T_MAP) { free(b.p); http_ser_throw("unsupported-headers"); }
        Map *hm = (Map *)headers;
        for (int64_t k = 0; k < hm->n; k++) {
            Value hk = hm->kv[2 * k];
            Value hv = hm->kv[2 * k + 1];
            if (obj_type(hk) != T_KW || obj_type(hv) != T_STR) { free(b.p); http_ser_throw("invalid-header"); }
            Str *kn = (Str *)hk;
            Str *vs = (Str *)hv;
            /* Reject framing headers the serializer owns, and CR/LF/NUL injection. */
            if ((kn->len == 14 && memcmp(kn->data, "content-length", 14) == 0) ||
                (kn->len == 17 && memcmp(kn->data, "transfer-encoding", 17) == 0)) {
                free(b.p); http_ser_throw("reserved-header");
            }
            for (size_t j = 0; j < kn->len; j++)
                if (!http_token_char((unsigned char)kn->data[j])) { free(b.p); http_ser_throw("invalid-header"); }
            for (size_t j = 0; j < vs->len; j++) {
                char c = vs->data[j];
                if (c == '\r' || c == '\n' || c == 0) { free(b.p); http_ser_throw("invalid-header"); }
            }
            sb_write(&b, kn->data, kn->len);
            sb_str(&b, ": ");
            sb_write(&b, vs->data, vs->len);
            sb_str(&b, "\r\n");
        }
    }
    sb_str(&b, "content-length: ");
    sb_num(&b, body_len);
    sb_str(&b, "\r\nconnection: close\r\n\r\n");
    if (body_len) sb_write(&b, body_data, (size_t)body_len);

    Value out = cljn_str_from(b.p, (long)b.len);
    free(b.p);
    gc_disabled--;
    return out;
}
