#include <stdint.h>
#include <stdio.h>
#include <string.h>

/*
 * Single translation unit so the HTTP parser/serializer fuzz can drive the strict
 * grammar and exercise every internal path, including the exception handler stack
 * (ADR-0013 Gate 4). Run under `make test-runtime-sanitize` for ASan/UBSan.
 */
#include "../../runtime.c"

/* Parse one buffer inside an exception handler so a categorized throw unwinds. */
static void parse_one(const char *buf, long len) {
    Handler h;
    h.prev = handler_top;
    h.saved_sp = gc_sp;
    h.saved_gc_disabled = gc_disabled;
    if (setjmp(h.env) == 0) {
        handler_top = &h;
        Value s = cljn_str_from(buf, len);
        volatile Value r = cljn_parse_http_request(s);
        (void)r;
        handler_top = h.prev;
    } else {
        handler_top = h.prev;
        gc_sp = h.saved_sp;
        gc_disabled = h.saved_gc_disabled;
    }
}

int main(void) {
    const char *corpus[] = {
        "", "\r", "\n", "\r\n", "\r\n\r\n", "GET", "GET ", "GET /", "GET / ",
        "GET / HTTP/1.1", "GET / HTTP/1.1\r", "GET / HTTP/1.1\r\n",
        "GET / HTTP/1.1\r\n\r\n",
        "GET / HTTP/1.1\r\nhost: example\r\n\r\n",
        "GET /a/b/c HTTP/1.0\r\nAccept: */*\r\nHost: x\r\n\r\n",
        "POST /x HTTP/1.1\r\ncontent-length: 3\r\n\r\nabc",
        "POST /x HTTP/1.1\r\ncontent-length: 3\r\n\r\nab",
        "POST /x HTTP/1.1\r\ncontent-length: 3\r\n\r\nabcd",
        "POST /x HTTP/1.1\r\ncontent-length: 999999999999\r\n\r\n",
        "POST / HTTP/1.1\r\ncontent-length: 5\r\ncontent-length: 6\r\n\r\nhello",
        "POST / HTTP/1.1\r\ncontent-length: -1\r\n\r\n",
        "GET / HTTP/1.1\r\ntransfer-encoding: chunked\r\n\r\n",
        "GET / HTTP/1.1\n\n",
        "GET / HTTP/1.1\r\n bad-fold: x\r\n\r\n",
        "GET / HTTP/1.1\r\n:novalue\r\n\r\n",
        "GET / HTTP/1.1\r\nbad header: x\r\n\r\n",
        "GET /x HTTP/2.0\r\n\r\n",
        "get / HTTP/1.1\r\n\r\n",
        0
    };
    /* Every prefix of every input proves no over-read on incomplete data. */
    for (int i = 0; corpus[i]; i++) {
        const char *b = corpus[i];
        size_t L = strlen(b);
        for (size_t k = 0; k <= L; k++) parse_one(b, (long)k);
    }
    char nasty[] = "GET /\x00\xff HTTP/1.1\r\n\r\n";
    parse_one(nasty, (long)(sizeof(nasty) - 1));
    static char big[70000];
    memset(big, 'a', sizeof(big));
    parse_one(big, (long)sizeof(big));
    static char many[8000];
    size_t o = 0;
    o += (size_t)snprintf(many + o, sizeof(many) - o, "GET / HTTP/1.1\r\n");
    for (int i = 0; i < 200 && o + 8 < sizeof(many); i++)
        o += (size_t)snprintf(many + o, sizeof(many) - o, "h%d: v\r\n", i);
    o += (size_t)snprintf(many + o, sizeof(many) - o, "\r\n");
    parse_one(many, (long)o);

    /* Serializer: valid maps plus header injection and framing rejections. */
    for (int st = 90; st <= 610; st += 7) {
        Handler h;
        h.prev = handler_top;
        h.saved_sp = gc_sp;
        h.saved_gc_disabled = gc_disabled;
        if (setjmp(h.env) == 0) {
            handler_top = &h;
            Value hdr = cljn_map_alloc(1);
            cljn_map_set(hdr, 0, cljn_kw("x-test", 6),
                         cljn_str_from(st % 3 ? "ok" : "bad\r\nInject: 1", st % 3 ? 2 : 14));
            Value r = cljn_map_alloc(3);
            cljn_map_set(r, 0, cljn_kw("status", 6), MK_FIX(st));
            cljn_map_set(r, 1, cljn_kw("headers", 7), hdr);
            cljn_map_set(r, 2, cljn_kw("body", 4), st % 2 ? NIL : cljn_str_from("body", 4));
            volatile Value out = cljn_serialize_http_response(r);
            (void)out;
            handler_top = h.prev;
        } else {
            handler_top = h.prev;
            gc_sp = h.saved_sp;
            gc_disabled = h.saved_gc_disabled;
        }
    }
    printf("runtime C HTTP fuzz: ok\n");
    return 0;
}
