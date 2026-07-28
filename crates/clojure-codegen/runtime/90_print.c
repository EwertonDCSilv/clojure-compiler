
/* ---------- impressão / str ---------- */
typedef struct { char *p; size_t len, cap; } SB;
static void sb_init(SB *b) { b->cap = 32; b->len = 0; b->p = xalloc(b->cap); }
static void sb_putc(SB *b, char c) {
    if (b->len + 1 > b->cap) { b->cap *= 2; b->p = realloc(b->p, b->cap); if (!b->p) die("sem memória"); }
    b->p[b->len++] = c;
}
static void sb_write(SB *b, const char *s, size_t n) { for (size_t i=0;i<n;i++) sb_putc(b,s[i]); }
static void sb_str(SB *b, const char *s) { sb_write(b, s, strlen(s)); }

static void write_val(SB *b, Value v, int for_str);
static void sb_write_hmap(SB *b, Value node, int for_str, int *first) {
    if (obj_type(node) == T_MCOLL) {
        MColl *c = (MColl *)node;
        for (int64_t i = 0; i < c->n; i++) {
            if (!*first) sb_str(b, ", ");
            *first = 0;
            write_val(b, c->pairs[2 * i], for_str);
            sb_putc(b, ' ');
            write_val(b, c->pairs[2 * i + 1], for_str);
        }
        return;
    }
    MNode *nd = (MNode *)node;
    int cnt = __builtin_popcount(nd->bitmap);
    for (int idx = 0; idx < cnt; idx++) {
        Value k = nd->arr[2 * idx];
        if (k == MNODEKEY) { sb_write_hmap(b, nd->arr[2 * idx + 1], for_str, first); }
        else {
            if (!*first) sb_str(b, ", ");
            *first = 0;
            write_val(b, k, for_str);
            sb_putc(b, ' ');
            write_val(b, nd->arr[2 * idx + 1], for_str);
        }
    }
}
static void sb_write_hset(SB *b, Value node, int for_str, int *first) {
    if (obj_type(node) == T_MCOLL) {
        MColl *c = (MColl *)node;
        for (int64_t i = 0; i < c->n; i++) {
            if (!*first) sb_putc(b, ' ');
            *first = 0;
            write_val(b, c->pairs[2 * i], for_str);
        }
        return;
    }
    MNode *nd = (MNode *)node;
    int cnt = __builtin_popcount(nd->bitmap);
    for (int idx = 0; idx < cnt; idx++) {
        Value k = nd->arr[2 * idx];
        if (k == MNODEKEY) { sb_write_hset(b, nd->arr[2 * idx + 1], for_str, first); }
        else {
            if (!*first) sb_putc(b, ' ');
            *first = 0;
            write_val(b, k, for_str);
        }
    }
}
/* percurso in-order da árvore ordenada para impressão (crescente). */
static void sb_write_tree(SB *b, Value node, int is_map, int for_str, int *first) {
    if (node == NIL) return;
    TNode *h = (TNode *)node;
    sb_write_tree(b, h->left, is_map, for_str, first);
    if (!*first) sb_str(b, is_map ? ", " : " ");
    *first = 0;
    write_val(b, h->key, for_str);
    if (is_map) { sb_putc(b, ' '); write_val(b, h->val, for_str); }
    sb_write_tree(b, h->right, is_map, for_str, first);
}
static void write_val(SB *b, Value v, int for_str) {
    if (IS_FIX(v)) { char t[32]; int n=snprintf(t,sizeof t,"%ld",(long)FIX(v)); sb_write(b,t,(size_t)n); return; }
    if (v == NIL) { if (!for_str) sb_str(b,"nil"); return; }
    if (v == TRUEV) { sb_str(b,"true"); return; }
    if (v == FALSEV) { sb_str(b,"false"); return; }
    if (v == EMPTY) { sb_str(b,"()"); return; }
    switch (obj_type(v)) {
        case T_STR: { Str *s=(Str*)v; sb_write(b,s->data,s->len); return; }
        case T_KW: { Str *s=(Str*)v; sb_putc(b,':'); sb_write(b,s->data,s->len); return; }
        case T_CONS: {
            sb_putc(b,'('); int first=1;
            while (v != EMPTY && obj_type(v)==T_CONS) {
                if(!first) sb_putc(b,' '); first=0;
                write_val(b,((Cons*)v)->head,for_str);
                v=((Cons*)v)->tail;
            }
            sb_putc(b,')'); return;
        }
        case T_VEC: {
            PVec *x=(PVec*)v; sb_putc(b,'[');
            for (int64_t i=0;i<x->count;i++){ if(i) sb_putc(b,' '); write_val(b,pv_nth(x,i),for_str); }
            sb_putc(b,']'); return;
        }
        case T_SET: {
            Vec *x=(Vec*)v; sb_str(b,"#{");
            for (int64_t i=0;i<x->len;i++){ if(i) sb_putc(b,' '); write_val(b,x->items[i],for_str); }
            sb_putc(b,'}'); return;
        }
        case T_HSET: {
            sb_str(b,"#{"); int first=1; sb_write_hset(b, ((HMap*)v)->root, for_str, &first); sb_putc(b,'}'); return;
        }
        case T_MAP: {
            Map *m=(Map*)v; sb_putc(b,'{');
            for (int64_t i=0;i<m->n;i++){ if(i) sb_str(b,", "); write_val(b,m->kv[2*i],for_str); sb_putc(b,' '); write_val(b,m->kv[2*i+1],for_str); }
            sb_putc(b,'}'); return;
        }
        case T_HMAP: {
            sb_putc(b,'{'); int first=1; sb_write_hmap(b, ((HMap*)v)->root, for_str, &first); sb_putc(b,'}'); return;
        }
        case T_SSET: {
            sb_str(b,"#{"); int first=1; sb_write_tree(b, ((Sorted*)v)->root, 0, for_str, &first); sb_putc(b,'}'); return;
        }
        case T_SMAP: {
            sb_putc(b,'{'); int first=1; sb_write_tree(b, ((Sorted*)v)->root, 1, for_str, &first); sb_putc(b,'}'); return;
        }
        case T_FN: sb_str(b, "#<fn>"); return;
        case T_RECORD: {
            Record *r = (Record *)v;
            sb_putc(b, '#');
            if (obj_type(r->type_name) == T_KW) { Str *n = (Str *)r->type_name; sb_write(b, n->data, n->len); }
            write_val(b, r->map, for_str);
            return;
        }
        default: sb_str(b,"#<obj>");
    }
}

/* print/println escrevem no Writer corrente (*out*), permitindo captura via
 * with-out-str; por padrão *out* é o Writer de stdout. */
void cljn_print(Value v) {
    SB b; sb_init(&b); write_val(&b,v,0);
    Value out = dynvar_get(VAR_OUT); /* pode alocar (init preguiçoso); v está rooteado */
    writer_write(out, b.p, b.len);
    free(b.p);
}
Value cljn_to_str(Value v) {
    SB b; sb_init(&b); write_val(&b,v,1);
    Value r = cljn_str_from(b.p, (long)b.len); /* pode coletar; v está rooteado */
    free(b.p); return r;
}
Value cljn_str_concat(Value a, Value b) {
    if (obj_type(a)!=T_STR || obj_type(b)!=T_STR) die("str_concat: esperava strings");
    Str *x=(Str*)a,*y=(Str*)b;
    size_t total = x->len + y->len;
    /* aloca o objeto (pode coletar; a,b rooteados) e depois o buffer. */
    Str *s=(Str*)obj_alloc(sizeof(Str),T_STR);
    s->len=total; s->data=xalloc(total?total:1);
    x=(Str*)a; y=(Str*)b; /* revalida após possível GC (não-móvel: iguais) */
    memcpy(s->data,x->data,x->len);
    memcpy(s->data+x->len,y->data,y->len);
    return (Value)s;
}
void cljn_print_space(void) { writer_write(dynvar_get(VAR_OUT), " ", 1); }
void cljn_print_newline(void) { writer_write(dynvar_get(VAR_OUT), "\n", 1); }
