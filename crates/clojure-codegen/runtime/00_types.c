/* runtime.c — runtime nativo do caminho compilado, agora com GC mark-sweep
 * preciso e shadow-stack de roots (ADR-0002 / specs/MEMORY_MODEL.md).
 *
 * Valor tagged em uma palavra (ADR-0003, variante compilada):
 *   fixnum (n<<1)|1 ; ponteiro baixo3=000 ; imediatos NIL/TRUE/FALSE/EMPTY.
 *
 * GC: coletor tracing mark-sweep, preciso, NÃO-móvel, single-thread.
 *   - Roots: um shadow-stack (`gc_stack`) mantido pelo CÓDIGO GERADO — cada função
 *     reserva `local_count` slots (locais) e empurra/retira temporários em volta de
 *     alocações. O coletor varre [0, gc_sp) — nunca escaneia a pilha nativa.
 *   - Objetos têm header (mark + lista global de todos os objetos) para o sweep.
 *   - Gatilho: a cada N alocações; env CLJN_GC_STRESS=1 coleta a CADA alocação
 *     (usado nos testes para validar o rooting).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <setjmp.h>

typedef intptr_t Value;

#define NIL    ((Value)2)
#define FALSEV ((Value)6)
#define TRUEV  ((Value)10)
#define EMPTY  ((Value)18)

#define IS_FIX(v)  ((v) & 1)
/* Codificação por unsigned para não fazer left shift de signed negativo (UB). */
#define MK_FIX(n)  ((Value)(((uintptr_t)(intptr_t)(n) << 1) | 1u))
#define FIX(v)     ((intptr_t)(v) >> 1)
#define IS_PTR(v)  (((v) & 7) == 0)

/* Intervalo de fixnum em alvo de 64 bits (fonte única — deve casar com o codegen). */
#define FIXNUM_MIN (-((intptr_t)1 << 62))
#define FIXNUM_MAX (((intptr_t)1 << 62) - 1)

enum { T_STR = 1, T_CONS = 2, T_FN = 3, T_KW = 4, T_VEC = 5, T_MAP = 6, T_SET = 7, T_RECORD = 8,
       T_VNODE = 9, T_HMAP = 10, T_MNODE = 11, T_MCOLL = 12, T_HSET = 13,
       T_TNODE = 14, T_SMAP = 15, T_SSET = 16, T_TVEC = 17, T_TBOX = 18 };

typedef struct Obj {
    uint8_t type;
    uint8_t mark;
    uint16_t szc;         /* classe de tamanho (para reciclar no free-list) */
    struct Obj *next_all; /* lista global de objetos, para o sweep */
} Obj;
typedef struct { Obj h; size_t len; char *data; } Str; /* também usado p/ T_KW */
typedef struct { Obj h; Value head; Value tail; } Cons;
/* Função de primeira classe: ponteiro de código + aridade + variáveis capturadas. */
typedef struct { Obj h; void *code; int64_t arity; int64_t nfree; Value freev[]; } Fn;
/* Coleções imutáveis (persistentes por valor; estrutura simples — o vector trie /
 * HAMT com structural sharing é otimização posterior, não muda a semântica). */
typedef struct { Obj h; int64_t len; Value items[]; } Vec;    /* set (array simples) */
typedef struct { Obj h; int64_t n; Value kv[]; } Map;         /* array-map: kv[2n] (mantém ordem, <=8) */
typedef struct { Obj h; Value type_name; Value map; } Record; /* defrecord: nome + mapa */
/* Mapa grande: HAMT (hash array mapped trie). HMap = wrapper (count + raiz);
 * nós = MNode (bitmap-indexado) e MColl (colisão de hash). Chave==MNODEKEY marca
 * que o slot guarda um sub-nó. Ver hibrido: array-map (<=8) promove a HMAP. */
#define MAP_ARRAY_MAX 8
typedef struct { Obj h; int64_t count; Value root; } HMap;
typedef struct { Obj h; uint32_t bitmap; Value arr[]; } MNode; /* 2*popcount slots */
typedef struct { Obj h; uint32_t hash; int64_t n; Value pairs[]; } MColl; /* 2n */
#define MNODEKEY ((Value)26)  /* imediato reservado: "este slot guarda um sub-nó" */
#define MNOTFOUND ((Value)42) /* sentinela interno de "não encontrado" */
/* Coleções ordenadas: árvore left-leaning red-black (LLRB) persistente.
 * TNode = nó (chave, valor, filhos, cor). Sorted = wrapper (count + raiz).
 * sorted-set guarda val==key. Ordem por cljn_compare (ordem total de Clojure). */
typedef struct { Obj h; Value key; Value val; Value left; Value right; int64_t red; } TNode;
typedef struct { Obj h; int64_t count; Value root; } Sorted; /* T_SMAP / T_SSET */
/* Transients (mutação em lote). Vetor transiente = buffer mutável crescente
 * (conj! O(1) amortizado); mapa/set transiente = caixa mutável sobre o valor
 * persistente (semântica correta; persistent! O(1)). */
typedef struct { Obj h; int64_t len; int64_t cap; Value *items; } TVec; /* T_TVEC */
typedef struct { Obj h; Value inner; } TBox;                            /* T_TBOX */
/* Vetor persistente: bitmapped vector trie (32-way), como o PersistentVector de
 * Clojure. `tail` (até 32) dá conj/nth O(1) amortizado; o resto é uma árvore
 * 32-way com structural sharing (conj/assoc/nth O(log32 n)). */
#define VBITS 5
#define VWIDTH 32
#define VMASK 31
typedef struct { Obj h; Value slots[VWIDTH]; } VNode;
typedef struct { Obj h; int64_t count; int64_t shift; Value root; Value tail; int64_t tail_len; } PVec;
static Value pv_nth(PVec *v, int64_t i); /* fwd */
