/*
 * Native Value ABI, heap layouts, and shared runtime declarations.
 *
 * A Value is one machine word. Fixnums use `(n << 1) | 1`, aligned heap
 * pointers have low bits 000, and NIL/TRUE/FALSE/EMPTY are reserved immediates.
 * Character code points use low bits 100.
 *
 * ABI: tag numbers, immediate constants, field order, and offsets must match
 * clojure-codegen/src/lib.rs. All generated callable entries use
 * `(self, argc, argv) -> Value`.
 *
 * GC: heap objects begin with Obj and never move. Generated code owns the
 * precise shadow stack. CLJN_GC_STRESS=1 collects at every allocation to test
 * rooting contracts.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <setjmp.h>
#include <unistd.h>

typedef intptr_t Value;

#define NIL    ((Value)2)
#define FALSEV ((Value)6)
#define TRUEV  ((Value)10)
#define EMPTY  ((Value)18)

#define IS_FIX(v)  ((v) & 1)
/* Shift through unsigned arithmetic to avoid signed-negative shift UB. */
#define MK_FIX(n)  ((Value)(((uintptr_t)(intptr_t)(n) << 1) | 1u))
#define FIX(v)     ((intptr_t)(v) >> 1)
#define IS_PTR(v)  (((v) & 7) == 0)

/* Characters are immediate Unicode scalar values and are invisible to GC. */
#define IS_CHAR(v)  (((v) & 7) == 4)
#define MK_CHAR(cp) ((Value)(((uintptr_t)(uint32_t)(cp) << 3) | 4u))
#define CHAR_CP(v)  ((uint32_t)((uintptr_t)(v) >> 3))

/* ABI: 64-bit fixnum range must match the Rust backend. */
#define FIXNUM_MIN (-((intptr_t)1 << 62))
#define FIXNUM_MAX (((intptr_t)1 << 62) - 1)

enum { T_STR = 1, T_CONS = 2, T_FN = 3, T_KW = 4, T_VEC = 5, T_MAP = 6, T_SET = 7, T_RECORD = 8,
       T_VNODE = 9, T_HMAP = 10, T_MNODE = 11, T_MCOLL = 12, T_HSET = 13,
       T_TNODE = 14, T_SMAP = 15, T_SSET = 16, T_TVEC = 17, T_TBOX = 18, T_EDIT = 19,
       T_WRITER = 20, T_READER = 21, T_BYTES = 22, T_FLOAT = 23, T_HTTP_SERVER = 24 };

typedef struct Obj {
    uint8_t type;
    uint8_t mark;
    uint16_t szc;         /* classe de tamanho (para reciclar no free-list) */
    struct Obj *next_all; /* lista global de objetos, para o sweep */
} Obj;
typedef struct { Obj h; size_t len; char *data; } Str; /* também usado p/ T_KW */
typedef struct { Obj h; Value head; Value tail; } Cons;
/* First-class function: entry address, canonical arity, and captured Values. */
typedef struct { Obj h; void *code; int64_t arity; int64_t nfree; Value freev[]; } Fn;
/* Small immutable collections stored as flat arrays. */
typedef struct { Obj h; int64_t len; Value items[]; } Vec;    /* set (array simples) */
typedef struct { Obj h; int64_t n; Value kv[]; } Map;         /* array-map: kv[2n] (mantém ordem, <=8) */
typedef struct { Obj h; Value type_name; Value map; } Record; /* defrecord: nome + mapa */
/* Large map HAMT: HMap owns count/root, MNode is bitmap-indexed, and MColl is a
 * same-hash collision node. Maps promote after MAP_ARRAY_MAX pairs. */
#define MAP_ARRAY_MAX 8
typedef struct { Obj h; int64_t count; Value root; } HMap;
typedef struct { Obj h; uint32_t bitmap; Value arr[]; } MNode; /* 2*popcount slots */
typedef struct { Obj h; uint32_t hash; int64_t n; Value pairs[]; } MColl; /* 2n */
#define MNODEKEY ((Value)26)  /* imediato reservado: "este slot guarda um sub-nó" */
#define MNOTFOUND ((Value)42) /* sentinela interno de "não encontrado" */
/* Persistent left-leaning red-black tree. Sorted sets store value == key and
 * ordering is defined by cljn_compare. */
typedef struct { Obj h; Value key; Value val; Value left; Value right; int64_t red; } TNode;
typedef struct { Obj h; int64_t count; Value root; } Sorted; /* T_SMAP / T_SSET */
/* Structural vector transients own editable trie nodes through a unique token.
 * Map/set transients are mutable boxes around persistent values. */
typedef struct { Obj h; } Edit;
/* Writer destination for standard streams, string capture, or FILE*. */
enum { WR_STDOUT = 0, WR_STDERR = 1, WR_STRING = 2, WR_FILE = 3 };
typedef struct { Obj h; int64_t kind; char *buf; size_t len; size_t cap; void *fp; } Writer; /* T_WRITER; fp=FILE* p/ WR_FILE */
/* Reader source for stdin, an in-memory Str cursor, or FILE*. */
enum { RD_STDIN = 0, RD_STRING = 1, RD_FILE = 2 };
typedef struct { Obj h; int64_t kind; Value src; int64_t pos; void *fp; } Reader; /* T_READER; fp=FILE* p/ RD_FILE */
/* Immutable binary array. GC treats it as a leaf and sweep frees `data`. */
typedef struct { Obj h; int64_t len; uint8_t *data; } Bytes; /* T_BYTES */
/* Boxed IEEE-754 double. GC leaf; arithmetic promotes fixnums to this. */
typedef struct { Obj h; double d; } Float; /* T_FLOAT */
/* HTTP server handle (ADR-0013 Gate 4): owns the listener and one connection
 * descriptor. GC leaf; the sweep closes any still-open descriptor (leak safety).
 * No Clojure closure or OS address escapes into a Value. */
typedef struct { Obj h; int listen_fd; int conn_fd; } HttpServer; /* T_HTTP_SERVER */
typedef struct { Obj h; Value inner; } TBox; /* T_TBOX (mapa/set transiente) */
/* Persistent 32-way bitmapped vector trie. A tail of up to 32 values gives
 * amortized O(1) conj; tree access and updates are O(log32 n). VNode.edit is NIL
 * for persistent nodes or an ownership token for transient nodes. */
#define VBITS 5
#define VWIDTH 32
#define VMASK 31
typedef struct { Obj h; Value edit; Value slots[VWIDTH]; } VNode;
typedef struct { Obj h; int64_t count; int64_t shift; Value root; Value tail; int64_t tail_len; } PVec;
/* Transient vector extends the PVec field prefix with its ownership token. */
typedef struct { Obj h; int64_t count; int64_t shift; Value root; Value tail; int64_t tail_len; Value edit; } TVec; /* T_TVEC */
static Value pv_nth(PVec *v, int64_t i); /* fwd */
