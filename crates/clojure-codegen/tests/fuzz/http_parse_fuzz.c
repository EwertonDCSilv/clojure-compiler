/*
 * libFuzzer entry point for the strict HTTP request parser (ADR-0013 Gate 4).
 *
 * Feeds arbitrary bytes to cljn_parse_http_request inside an exception handler so a
 * categorized throw unwinds normally. The runtime is included as one translation
 * unit for access to the internal handler stack. Build and run with
 * `scripts/fuzz-http.sh`. Between calls the shadow stack resets and the periodic
 * GC sweep reclaims dead request maps, so the working set stays bounded.
 *
 * The deterministic seed corpus lives in `crates/clojure-codegen/tests/c/
 * runtime_http.c`, which also runs under the ASan/UBSan gate.
 */
#include "../../runtime.c"

int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
    Handler h;
    h.prev = handler_top;
    h.saved_sp = gc_sp;
    h.saved_gc_disabled = gc_disabled;
    if (setjmp(h.env) == 0) {
        handler_top = &h;
        Value s = cljn_str_from((const char *)data, (long)size);
        volatile Value r = cljn_parse_http_request(s);
        (void)r;
        handler_top = h.prev;
    } else {
        handler_top = h.prev;
        gc_sp = h.saved_sp;
        gc_disabled = h.saved_gc_disabled;
    }
    return 0;
}
