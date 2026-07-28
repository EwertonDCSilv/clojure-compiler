
/*
 * Loopback HTTP/1.1 provider (ADR-0013 Gate 4, §3/§4).
 *
 * A narrow synchronous socket provider that the Clojure service loop drives; the
 * C side never retains or invokes a Clojure closure. open/port/accept/respond/
 * close manage the listener and one connection; the bounded request read reuses
 * the strict parser (150_http.c). Bind is 127.0.0.1 only in this slice; port 0
 * yields an ephemeral port readable through getsockname.
 *
 * ABI: no descriptor, sockaddr, or FILE* becomes a Clojure integer or collection.
 * GC: the handle is a leaf whose sweep closes leaked descriptors; explicit close
 * is idempotent. Read/accept use timeouts so a stuck peer cannot hang the loop.
 *
 * Planned (ADR-0013): self-pipe SIGINT/SIGTERM wakeup, keep-alive, statistics,
 * multi-request lifecycle, and the 1000-cycle subprocess leak gate.
 */
#include <sys/socket.h>
#include <netinet/in.h>
#include <poll.h>
#include <sys/time.h>

#define HTTP_ACCEPT_TIMEOUT_MS 30000
#define HTTP_READ_TIMEOUT_SEC 10

/* Build and throw a categorized network error. */
static void net_throw(const char *kind, const char *op, int status, int err) {
    Value m = cljn_map_alloc(5);
    cljn_map_set(m, 0, cljn_kw("cljn.error/domain", 17), cljn_kw("http", 4));
    cljn_map_set(m, 1, cljn_kw("kind", 4), cljn_kw(kind, (long)strlen(kind)));
    cljn_map_set(m, 2, cljn_kw("operation", 9), cljn_kw(op, (long)strlen(op)));
    cljn_map_set(m, 3, cljn_kw("status", 6), MK_FIX(status));
    cljn_map_set(m, 4, cljn_kw("os-code", 7), MK_FIX(err));
    cljn_throw(m);
}

/* (http-server-open port) -> handle. Binds 127.0.0.1:port (0 = ephemeral). */
Value cljn_http_server_open(Value port_v) {
    if (!IS_FIX(port_v)) die("http-server-open: porta deve ser inteiro");
    long port = FIX(port_v);
    if (port < 0 || port > 65535) die("http-server-open: porta fora de faixa");
    int fd = socket(AF_INET, SOCK_STREAM, 0);
    if (fd < 0) net_throw("socket-failed", "open", 500, errno);
    int one = 1;
    setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &one, sizeof one);
    struct sockaddr_in addr;
    memset(&addr, 0, sizeof addr);
    addr.sin_family = AF_INET;
    addr.sin_port = htons((uint16_t)port);
    addr.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    if (bind(fd, (struct sockaddr *)&addr, sizeof addr) < 0) {
        int e = errno; close(fd);
        net_throw(e == EADDRINUSE ? "address-in-use" : "bind-failed", "bind", 500, e);
    }
    if (listen(fd, 16) < 0) { int e = errno; close(fd); net_throw("listen-failed", "listen", 500, e); }
    HttpServer *s = (HttpServer *)obj_alloc(sizeof(HttpServer), T_HTTP_SERVER);
    s->listen_fd = fd;
    s->conn_fd = -1;
    return (Value)s;
}

/* (http-server-port server) -> the bound port (after ephemeral assignment). */
Value cljn_http_server_port(Value sv) {
    if (obj_type(sv) != T_HTTP_SERVER) die("http-server-port: esperava servidor");
    HttpServer *s = (HttpServer *)sv;
    struct sockaddr_in addr;
    socklen_t len = sizeof addr;
    if (getsockname(s->listen_fd, (struct sockaddr *)&addr, &len) < 0)
        net_throw("getsockname-failed", "port", 500, errno);
    return MK_FIX(ntohs(addr.sin_port));
}

/* Does the buffer hold a complete request? Returns total bytes needed, or -1 if
 * the header terminator has not arrived yet. */
static int64_t http_request_end(const char *buf, int64_t len) {
    int64_t hend = -1;
    for (int64_t i = 0; i + 3 < len; i++) {
        if (buf[i] == '\r' && buf[i + 1] == '\n' && buf[i + 2] == '\r' && buf[i + 3] == '\n') {
            hend = i + 4;
            break;
        }
    }
    if (hend < 0) return -1;
    /* Scan headers for Content-Length to know the body size. */
    int64_t cl = 0;
    const char *cll = "content-length:";
    for (int64_t i = 0; i < hend; i++) {
        /* case-insensitive match at line start */
        if ((i == 0 || (buf[i - 1] == '\n')) && i + 15 <= hend) {
            int ok = 1;
            for (int j = 0; j < 15; j++) {
                char c = buf[i + j];
                if (c >= 'A' && c <= 'Z') c = (char)(c - 'A' + 'a');
                if (c != cll[j]) { ok = 0; break; }
            }
            if (ok) {
                int64_t k = i + 15;
                while (k < hend && (buf[k] == ' ' || buf[k] == '\t')) k++;
                while (k < hend && buf[k] >= '0' && buf[k] <= '9') { cl = cl * 10 + (buf[k] - '0'); k++; }
                break;
            }
        }
    }
    return hend + cl;
}

/* (http-server-accept server) -> request map. Blocks (with timeouts) for one
 * connection, reads a bounded complete request, and parses it strictly. */
Value cljn_http_server_accept(Value sv) {
    if (obj_type(sv) != T_HTTP_SERVER) die("http-server-accept: esperava servidor");
    HttpServer *s = (HttpServer *)sv;
    struct pollfd pfd;
    pfd.fd = s->listen_fd;
    pfd.events = POLLIN;
    int pr = poll(&pfd, 1, HTTP_ACCEPT_TIMEOUT_MS);
    if (pr == 0) net_throw("accept-timeout", "accept", 408, 0);
    if (pr < 0) net_throw("poll-failed", "accept", 500, errno);
    int c = accept(s->listen_fd, NULL, NULL);
    if (c < 0) net_throw("accept-failed", "accept", 500, errno);
    s->conn_fd = c;
    struct timeval tv;
    tv.tv_sec = HTTP_READ_TIMEOUT_SEC;
    tv.tv_usec = 0;
    setsockopt(c, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof tv);

    char *buf = (char *)xalloc(HTTP_MAX_REQUEST);
    int64_t len = 0;
    for (;;) {
        int64_t need = http_request_end(buf, len);
        if (need >= 0 && len >= need) break;         /* full request buffered */
        if (need > HTTP_MAX_REQUEST) { free(buf); net_throw("request-too-large", "read", 413, 0); }
        if (len >= HTTP_MAX_REQUEST) { free(buf); net_throw("request-too-large", "read", 413, 0); }
        ssize_t r = recv(c, buf + len, (size_t)(HTTP_MAX_REQUEST - len), 0);
        if (r == 0) { free(buf); net_throw("incomplete", "read", 400, 0); } /* peer closed early */
        if (r < 0) { free(buf); net_throw("read-timeout", "read", 408, errno); }
        len += r;
    }
    Value s_bytes = cljn_str_from(buf, (long)len); /* copies; buf freed next */
    free(buf);
    return cljn_parse_http_request(s_bytes); /* throws categorized on malformed */
}

/* (http-server-respond server response) -> nil. Serializes, writes all bytes, and
 * closes the current connection. */
Value cljn_http_server_respond(Value sv, Value response) {
    if (obj_type(sv) != T_HTTP_SERVER) die("http-server-respond: esperava servidor");
    HttpServer *s = (HttpServer *)sv;
    Value bytes = cljn_serialize_http_response(response); /* throws on invalid */
    Str *b = (Str *)bytes;
    size_t off = 0;
    while (off < b->len) {
        ssize_t w = send(s->conn_fd, b->data + off, b->len - off, MSG_NOSIGNAL);
        if (w <= 0) break; /* peer gone; still close below */
        off += (size_t)w;
    }
    if (s->conn_fd >= 0) { close(s->conn_fd); s->conn_fd = -1; }
    return NIL;
}

/* (http-server-close server) -> nil. Idempotent; closes connection and listener. */
Value cljn_http_server_close(Value sv) {
    if (obj_type(sv) != T_HTTP_SERVER) die("http-server-close: esperava servidor");
    HttpServer *s = (HttpServer *)sv;
    if (s->conn_fd >= 0) { close(s->conn_fd); s->conn_fd = -1; }
    if (s->listen_fd >= 0) { close(s->listen_fd); s->listen_fd = -1; }
    return NIL;
}
