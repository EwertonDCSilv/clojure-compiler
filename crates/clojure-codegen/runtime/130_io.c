/*
 * Synchronous native I/O and POSIX filesystem operations.
 *
 * The frontend emits no syscalls or descriptors. This ABI owns text/binary file
 * access, paths, directories, environment lookup, file streams, close/flush,
 * output capture, and dynamic rebinding. OS failures become ex-data maps thrown
 * through cljn_throw and are therefore catchable by native try/catch.
 *
 * Ownership: temporary C strings and I/O buffers are freed locally. FILE*
 * ownership transfers to Reader/Writer objects until close or GC sweep.
 */
#include <errno.h>
#include <unistd.h>
#include <sys/stat.h>
#include <dirent.h>

/* Copy a runtime string to a NUL-terminated buffer owned by the caller. */
static char *io_cstr(Value s) {
    Str *p = (Str *)s;
    char *c = (char *)xalloc(p->len + 1);
    if (p->len) memcpy(c, p->data, p->len);
    c[p->len] = 0;
    return c;
}
/* Map errno to the stable :kind category stored in I/O ex-data. */
static const char *io_kind(int err) {
    switch (err) {
        case ENOENT: return "not-found";
        case EACCES: case EPERM: return "permission-denied";
        case EEXIST: return "already-exists";
        default: return "other";
    }
}
/* Build {:kind :operation :path :os-code :message} and throw it.
 * GC: path must be rooted by the caller. This function does not return. */
static void io_throw(const char *kind, const char *op, Value path, int err) {
    maybe_gc();
    gc_disabled++;
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

/* Read a complete file in blocks and return a newly allocated runtime string. */
Value cljn_slurp(Value path) {
    if (obj_type(path) != T_STR) die("slurp: path deve ser string");
    cljn_gc_push(path);
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

/* Truncate/write string content to path and return NIL. */
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

/* Return tagged boolean path existence; type errors are fatal. */
Value cljn_file_exists(Value path) {
    if (obj_type(path) != T_STR) die("file-exists?: path deve ser string");
    char *cp = io_cstr(path);
    int r = access(cp, F_OK);
    free(cp);
    return b2v(r == 0);
}

/* Copy an environment value into a runtime string, or return NIL when absent. */
Value cljn_getenv(Value name) {
    if (obj_type(name) != T_STR) die("getenv: nome deve ser string");
    char *cn = io_cstr(name);
    const char *v = getenv(cn);
    free(cn);
    return v ? cljn_str_from(v, (long)strlen(v)) : NIL;
}

/* Create one directory, throwing structured I/O ex-data on failure. */
Value cljn_mkdir(Value path) {
    if (obj_type(path) != T_STR) die("mkdir: path deve ser string");
    char *cp = io_cstr(path);
    int r = mkdir(cp, 0777);
    int e = errno;
    free(cp);
    if (r != 0) io_throw(io_kind(e), "mkdir", path, e);
    return NIL;
}
/* Create every missing path component like `mkdir -p`; EEXIST is accepted. */
Value cljn_mkdirs(Value path) {
    if (obj_type(path) != T_STR) die("mkdirs: path deve ser string");
    char *cp = io_cstr(path);
    for (char *p = cp + (cp[0] == '/' ? 1 : 0); *p; p++) {
        if (*p == '/') {
            *p = 0;
            if (cp[0] && mkdir(cp, 0777) != 0 && errno != EEXIST) {
                int e = errno; free(cp); io_throw(io_kind(e), "mkdirs", path, e);
            }
            *p = '/';
        }
    }
    if (mkdir(cp, 0777) != 0 && errno != EEXIST) {
        int e = errno; free(cp); io_throw(io_kind(e), "mkdirs", path, e);
    }
    free(cp);
    return NIL;
}
/* Return a vector of directory entry names excluding "." and "..". */
Value cljn_list_dir(Value path) {
    if (obj_type(path) != T_STR) die("list-dir: path deve ser string");
    char *cp = io_cstr(path);
    DIR *d = opendir(cp);
    free(cp);
    if (!d) io_throw(io_kind(errno), "list-dir", path, errno);
    gc_disabled++;
    Value v = cljn_vec_empty();
    struct dirent *e;
    while ((e = readdir(d)) != NULL) {
        if (!strcmp(e->d_name, ".") || !strcmp(e->d_name, "..")) continue;
        v = cljn_conj(v, cljn_str_from(e->d_name, (long)strlen(e->d_name)));
    }
    gc_disabled--;
    closedir(d);
    return v;
}
/* Remove a file or empty directory and return NIL. */
Value cljn_delete_file(Value path) {
    if (obj_type(path) != T_STR) die("delete-file: path deve ser string");
    char *cp = io_cstr(path);
    int r = remove(cp);
    int e = errno;
    free(cp);
    if (r != 0) io_throw(io_kind(e), "delete-file", path, e);
    return NIL;
}
/* Rename one filesystem entry and return NIL. */
Value cljn_rename(Value from, Value to) {
    if (obj_type(from) != T_STR || obj_type(to) != T_STR) die("rename: esperava strings");
    char *cf = io_cstr(from);
    char *ct = io_cstr(to);
    int r = rename(cf, ct);
    int e = errno;
    free(cf); free(ct);
    if (r != 0) io_throw(io_kind(e), "rename", from, e);
    return NIL;
}
/* Return tagged boolean indicating whether path names a directory. */
Value cljn_directoryp(Value path) {
    if (obj_type(path) != T_STR) die("directory?: path deve ser string");
    char *cp = io_cstr(path);
    struct stat st;
    int ok = (stat(cp, &st) == 0) && S_ISDIR(st.st_mode);
    free(cp);
    return b2v(ok);
}
/* Return tagged boolean indicating whether path names a regular file. */
Value cljn_filep(Value path) {
    if (obj_type(path) != T_STR) die("file?: path deve ser string");
    char *cp = io_cstr(path);
    struct stat st;
    int ok = (stat(cp, &st) == 0) && S_ISREG(st.st_mode);
    free(cp);
    return b2v(ok);
}
/* Return the file size in bytes; throws structured I/O ex-data if stat fails. */
Value cljn_file_size(Value path) {
    if (obj_type(path) != T_STR) die("file-size: path deve ser string");
    char *cp = io_cstr(path);
    struct stat st;
    int r = stat(cp, &st);
    int e = errno;
    free(cp);
    if (r != 0) io_throw(io_kind(e), "file-size", path, e);
    return MK_FIX((intptr_t)st.st_size);
}
/* Return the last-modified time in whole seconds since the epoch. */
Value cljn_file_modified(Value path) {
    if (obj_type(path) != T_STR) die("file-modified: path deve ser string");
    char *cp = io_cstr(path);
    struct stat st;
    int r = stat(cp, &st);
    int e = errno;
    free(cp);
    if (r != 0) io_throw(io_kind(e), "file-modified", path, e);
    return MK_FIX((intptr_t)st.st_mtime);
}

/* Allocate an immutable byte array and its owned external data buffer. */
static Value bytes_alloc(int64_t n) {
    Bytes *b = (Bytes *)obj_alloc(sizeof(Bytes), T_BYTES);
    b->len = n;
    b->data = (uint8_t *)xalloc(n ? n : 1);
    return (Value)b;
}
/* Copy a runtime string's UTF-8 bytes into an immutable byte array. */
Value cljn_bytes(Value s) {
    if (obj_type(s) != T_STR) die("bytes: esperava string");
    cljn_gc_push(s);
    Value bv = bytes_alloc(((Str *)s)->len);
    Str *ss = (Str *)s;
    memcpy(((Bytes *)bv)->data, ss->data, ss->len);
    cljn_gc_popn(1);
    return bv;
}
/* Copy raw bytes to a runtime string without UTF-8 validation. */
Value cljn_bytes_to_string(Value b) {
    if (obj_type(b) != T_BYTES) die("bytes->string: esperava bytes");
    Bytes *bb = (Bytes *)b;
    return cljn_str_from((char *)bb->data, (long)bb->len);
}
/* Build an immutable byte array from a vector of fixnums 0..255.
   ABI: aborts on a non-vector, non-fixnum element, or out-of-range value; the
   compiled cljn.io/bytes wrapper validates first, so reaching those is an
   internal error. GC: roots the source vector across allocation. */
Value cljn_bytes_of_vec(Value v) {
    if (obj_type(v) != T_VEC) die("bytes: esperava vetor");
    int64_t n = ((PVec *)v)->count;
    cljn_gc_push(v);
    Value bv = bytes_alloc(n);
    for (int64_t i = 0; i < n; i++) {
        Value e = cljn_nth(v, MK_FIX(i));
        if (!IS_FIX(e)) die("bytes: elemento não é inteiro");
        int64_t byte = FIX(e);
        if (byte < 0 || byte > 255) die("bytes: byte fora de 0..255");
        ((Bytes *)bv)->data[i] = (uint8_t)byte;
    }
    cljn_gc_popn(1);
    return bv;
}
/* Convert an immutable byte array to a persistent vector of fixnums 0..255.
   GC: roots the source and disables collection while conjoining. */
Value cljn_bytes_to_vec(Value b) {
    if (obj_type(b) != T_BYTES) die("bytes->vector: esperava bytes");
    cljn_gc_push(b);
    gc_disabled++;
    Value v = cljn_vec_empty();
    Bytes *bb = (Bytes *)b;
    for (int64_t i = 0; i < bb->len; i++) {
        v = cljn_conj(v, MK_FIX((int64_t)bb->data[i]));
    }
    gc_disabled--;
    cljn_gc_popn(1);
    return v;
}
/* Return TRUE when the byte array holds well-formed UTF-8, else FALSE.
   Rejects overlong forms, surrogates, and out-of-range code points. */
Value cljn_valid_utf8(Value b) {
    if (obj_type(b) != T_BYTES) return b2v(0);
    Bytes *bb = (Bytes *)b;
    const uint8_t *d = bb->data;
    int64_t n = bb->len, i = 0;
    while (i < n) {
        uint8_t c = d[i];
        int extra;
        uint32_t cp;
        if (c < 0x80) {
            i++;
            continue;
        } else if ((c & 0xE0) == 0xC0) {
            extra = 1;
            cp = (uint32_t)(c & 0x1F);
            if (c < 0xC2) return b2v(0);
        } else if ((c & 0xF0) == 0xE0) {
            extra = 2;
            cp = (uint32_t)(c & 0x0F);
        } else if ((c & 0xF8) == 0xF0) {
            extra = 3;
            cp = (uint32_t)(c & 0x07);
            if (c > 0xF4) return b2v(0);
        } else {
            return b2v(0);
        }
        if (i + extra >= n) return b2v(0);
        for (int k = 1; k <= extra; k++) {
            uint8_t cc = d[i + k];
            if ((cc & 0xC0) != 0x80) return b2v(0);
            cp = (cp << 6) | (uint32_t)(cc & 0x3F);
        }
        if (cp > 0x10FFFF) return b2v(0);
        if (cp >= 0xD800 && cp <= 0xDFFF) return b2v(0);
        if (extra == 2 && cp < 0x800) return b2v(0);
        if (extra == 3 && cp < 0x10000) return b2v(0);
        i += extra + 1;
    }
    return b2v(1);
}
/* Return byte at tagged fixnum index as a fixnum from 0 through 255. */
Value cljn_bget(Value b, Value i) {
    if (obj_type(b) != T_BYTES) die("bget: esperava bytes");
    if (!IS_FIX(i)) die("bget: índice deve ser inteiro");
    int64_t idx = FIX(i);
    Bytes *bb = (Bytes *)b;
    if (idx < 0 || idx >= bb->len) die("bget: índice fora dos limites");
    return MK_FIX((int64_t)bb->data[idx]);
}
/* Read a complete file in binary mode into an immutable byte array. */
Value cljn_slurp_bytes(Value path) {
    if (obj_type(path) != T_STR) die("slurp-bytes: path deve ser string");
    cljn_gc_push(path);
    char *cp = io_cstr(path);
    FILE *f = fopen(cp, "rb");
    free(cp);
    if (!f) io_throw(io_kind(errno), "slurp-bytes", path, errno);
    size_t cap = 4096, len = 0;
    char *buf = (char *)xalloc(cap);
    for (;;) {
        if (len == cap) { cap *= 2; buf = (char *)xrealloc(buf, cap); }
        size_t n = fread(buf + len, 1, cap - len, f);
        len += n;
        if (n == 0) {
            if (ferror(f)) { int e = errno; fclose(f); free(buf); io_throw("other", "slurp-bytes", path, e); }
            break;
        }
    }
    fclose(f);
    Value bv = bytes_alloc((int64_t)len);
    memcpy(((Bytes *)bv)->data, buf, len);
    free(buf);
    cljn_gc_popn(1);
    return bv;
}
/* Write an immutable byte array to a truncated binary file and return NIL. */
Value cljn_spit_bytes(Value path, Value b) {
    if (obj_type(path) != T_STR) die("spit-bytes: path deve ser string");
    if (obj_type(b) != T_BYTES) die("spit-bytes: esperava bytes");
    char *cp = io_cstr(path);
    FILE *f = fopen(cp, "wb");
    free(cp);
    if (!f) io_throw(io_kind(errno), "spit-bytes", path, errno);
    Bytes *bb = (Bytes *)b;
    if (bb->len && fwrite(bb->data, 1, bb->len, f) != (size_t)bb->len) {
        int e = errno; fclose(f); io_throw("other", "spit-bytes", path, e);
    }
    fclose(f);
    return NIL;
}

/*
 * Join two paths textually with POSIX separator semantics.
 *
 * An absolute second operand wins. The result does not normalize `.` or `..`.
 */
Value cljn_path_join(Value a, Value b) {
    if (obj_type(a) != T_STR || obj_type(b) != T_STR) die("path-join: esperava strings");
    Str *sa = (Str *)a, *sb = (Str *)b;
    if (sb->len > 0 && sb->data[0] == '/') return b;
    if (sa->len == 0) return b;
    size_t alen = sa->len;
    while (alen > 1 && sa->data[alen - 1] == '/') alen--;
    size_t total = alen + 1 + sb->len;
    Str *s = (Str *)obj_alloc(sizeof(Str), T_STR);
    s->len = total;
    s->data = xalloc(total ? total : 1);
    sa = (Str *)a; sb = (Str *)b;
    memcpy(s->data, sa->data, alen);
    s->data[alen] = '/';
    memcpy(s->data + alen + 1, sb->data, sb->len);
    return (Value)s;
}
/* Return the textual final path component as a newly allocated string. */
Value cljn_file_name(Value p) {
    if (obj_type(p) != T_STR) die("file-name: esperava string");
    Str *s = (Str *)p;
    size_t i = s->len;
    while (i > 0 && s->data[i - 1] != '/') i--;
    return cljn_str_from(s->data + i, (long)(s->len - i));
}
/* Return the textual parent path, or NIL when no separator exists. */
Value cljn_parent(Value p) {
    if (obj_type(p) != T_STR) die("parent: esperava string");
    Str *s = (Str *)p;
    size_t i = s->len;
    while (i > 0 && s->data[i - 1] != '/') i--;
    if (i == 0) return NIL;
    size_t end = i;
    while (end > 1 && s->data[end - 1] == '/') end--;
    return cljn_str_from(s->data, (long)end);
}

/* Open a truncated binary FILE* and transfer ownership to a Writer object. */
Value cljn_writer(Value path) {
    if (obj_type(path) != T_STR) die("writer: path deve ser string");
    cljn_gc_push(path);
    char *cp = io_cstr(path);
    FILE *f = fopen(cp, "wb");
    free(cp);
    if (!f) io_throw(io_kind(errno), "writer", path, errno);
    Writer *w = (Writer *)obj_alloc(sizeof(Writer), T_WRITER);
    w->kind = WR_FILE;
    w->buf = NULL;
    w->len = 0;
    w->cap = 0;
    w->fp = f;
    cljn_gc_popn(1);
    return (Value)w;
}
/* Open a binary FILE* and transfer ownership to a Reader object. */
Value cljn_reader(Value path) {
    if (obj_type(path) != T_STR) die("reader: path deve ser string");
    cljn_gc_push(path);
    char *cp = io_cstr(path);
    FILE *f = fopen(cp, "rb");
    free(cp);
    if (!f) io_throw(io_kind(errno), "reader", path, errno);
    Reader *r = (Reader *)obj_alloc(sizeof(Reader), T_READER);
    r->kind = RD_FILE;
    r->src = NIL;
    r->pos = 0;
    r->fp = f;
    cljn_gc_popn(1);
    return (Value)r;
}
/* Idempotently close a file Reader/Writer; standard/string streams are no-ops. */
Value cljn_close(Value x) {
    int t = obj_type(x);
    if (t == T_WRITER) {
        Writer *w = (Writer *)x;
        if (w->kind == WR_FILE && w->fp) { fclose((FILE *)w->fp); w->fp = NULL; }
    } else if (t == T_READER) {
        Reader *r = (Reader *)x;
        if (r->kind == RD_FILE && r->fp) { fclose((FILE *)r->fp); r->fp = NULL; }
    }
    return NIL;
}
/* Flush current *out* when it wraps stdout, stderr, or FILE*. */
Value cljn_flush(void) {
    Writer *w = (Writer *)dynvar_get(VAR_OUT);
    if (w->kind == WR_STDOUT) fflush(stdout);
    else if (w->kind == WR_STDERR) fflush(stderr);
    else if (w->kind == WR_FILE && w->fp) fflush((FILE *)w->fp);
    return NIL;
}

/* Invoke zero-arity thunk with *out* rebound to string capture.
 * Restores *out* and rethrows on exception; returns captured text normally.
 * GC: capture writer and prior *out* remain rooted across the thunk. */
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
        call_fn0(thunk);
        handler_top = h.prev;
    } else {
        handler_top = h.prev;
        gc_sp = h.saved_sp;
        gc_disabled = h.saved_gc_disabled;
        dyn_vars[VAR_OUT] = gc_stack[h.saved_sp - 1];
        return cljn_throw(exception_value);
    }
    dyn_vars[VAR_OUT] = old;
    Value s = cljn_writer_to_string(w);
    cljn_gc_popn(2);
    return s;
}

/* Invoke thunk with dynamic Var `id` rebound to `nv`.
 * Restores on normal and exceptional exit. `i` is volatile across longjmp.
 * GC: the new and prior Values remain rooted during the body. */
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
        dyn_vars[i] = gc_stack[h.saved_sp - 1];
        return cljn_throw(exception_value);
    }
    dyn_vars[i] = old;
    cljn_gc_popn(2);
    return r;
}
