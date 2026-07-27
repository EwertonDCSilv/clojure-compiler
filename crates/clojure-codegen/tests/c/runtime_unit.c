#include <stdint.h>
#include <stdio.h>

/*
 * Unidade de tradução única: permite testar invariantes internos estáticos sem
 * expô-los na ABI de produção.
 */
#include "../../runtime.c"

#define CHECK(condition)                                                        \
    do {                                                                        \
        if (!(condition)) {                                                     \
            fprintf(stderr, "CHECK falhou em %s:%d: %s\n", __FILE__, __LINE__, \
                    #condition);                                                \
            return 1;                                                           \
        }                                                                       \
    } while (0)

static int test_tagged_values_and_truthiness(void) {
    Value minimum = MK_FIX(FIXNUM_MIN);
    Value maximum = MK_FIX(FIXNUM_MAX);

    CHECK(IS_FIX(minimum));
    CHECK(IS_FIX(maximum));
    CHECK(FIX(minimum) == FIXNUM_MIN);
    CHECK(FIX(maximum) == FIXNUM_MAX);
    CHECK(FIX(MK_FIX(-42)) == -42);
    CHECK(cljn_truthy(NIL) == 0);
    CHECK(cljn_truthy(FALSEV) == 0);
    CHECK(cljn_truthy(TRUEV) == 1);
    CHECK(cljn_truthy(MK_FIX(0)) == 1);
    CHECK(cljn_not(NIL) == TRUEV);
    CHECK(cljn_nilp(NIL) == TRUEV);
    CHECK(cljn_nilp(FALSEV) == FALSEV);
    return 0;
}

static int test_shadow_stack_frames(void) {
    int64_t before = gc_sp;
    Value base = cljn_gc_enter(3);

    CHECK((int64_t)base == before);
    CHECK(gc_sp == before + 3);
    CHECK(gc_stack[base] == NIL);
    CHECK(gc_stack[base + 1] == NIL);
    CHECK(gc_stack[base + 2] == NIL);

    cljn_gc_set(base + 1, MK_FIX(42));
    CHECK(gc_stack[base + 1] == MK_FIX(42));
    cljn_gc_push(TRUEV);
    CHECK(gc_sp == before + 4);
    CHECK(gc_stack[gc_sp - 1] == TRUEV);
    cljn_gc_popn(1);
    CHECK(gc_sp == before + 3);
    cljn_gc_leave(base);
    CHECK(gc_sp == before);
    return 0;
}

static int test_allocator_reuses_swept_small_objects(void) {
    cljn_gc_force();
    long baseline = cljn_gc_live_objects();

    Value base = cljn_gc_enter(1);
    Value first = cljn_str_from("first", 5);
    cljn_gc_set(base, first);
    CHECK(cljn_gc_live_objects() == baseline + 1);
    cljn_gc_leave(base);
    cljn_gc_force();
    CHECK(cljn_gc_live_objects() == baseline);

    base = cljn_gc_enter(1);
    Value second = cljn_str_from("other", 5);
    cljn_gc_set(base, second);
    CHECK((void *)first == (void *)second);
    cljn_gc_leave(base);
    cljn_gc_force();
    CHECK(cljn_gc_live_objects() == baseline);
    return 0;
}

static int test_gc_marks_cycles_and_reclaims_them(void) {
    cljn_gc_force();
    long baseline = cljn_gc_live_objects();
    Value base = cljn_gc_enter(1);

    Value first = cljn_cons(MK_FIX(1), EMPTY);
    cljn_gc_set(base, first);
    Value second = cljn_cons(MK_FIX(2), first);
    ((Cons *)first)->tail = second;

    cljn_gc_force();
    CHECK(cljn_gc_live_objects() == baseline + 2);
    CHECK(((Cons *)((Cons *)first)->tail)->tail == first);

    cljn_gc_leave(base);
    cljn_gc_force();
    CHECK(cljn_gc_live_objects() == baseline);
    return 0;
}

static int test_vector_trie_boundaries_and_persistence(void) {
    Value base = cljn_gc_enter(4);
    Value vector = cljn_vec_empty();
    cljn_gc_set(base, vector);
    Value at_32 = NIL;
    Value at_33 = NIL;

    for (int64_t i = 0; i < 1057; i++) {
        vector = cljn_vec_conj(vector, MK_FIX(i));
        cljn_gc_set(base, vector);
        if (i == 31) {
            at_32 = vector;
            cljn_gc_set(base + 1, at_32);
        } else if (i == 32) {
            at_33 = vector;
            cljn_gc_set(base + 2, at_33);
        }
    }

    CHECK(obj_type(vector) == T_VEC);
    CHECK(((PVec *)vector)->count == 1057);
    CHECK(((PVec *)vector)->shift == 10);
    CHECK(((PVec *)at_32)->tail_len == 32);
    CHECK(((PVec *)at_33)->tail_len == 1);
    CHECK(FIX(pv_nth((PVec *)vector, 0)) == 0);
    CHECK(FIX(pv_nth((PVec *)vector, 31)) == 31);
    CHECK(FIX(pv_nth((PVec *)vector, 32)) == 32);
    CHECK(FIX(pv_nth((PVec *)vector, 1023)) == 1023);
    CHECK(FIX(pv_nth((PVec *)vector, 1024)) == 1024);
    CHECK(FIX(pv_nth((PVec *)vector, 1056)) == 1056);

    Value changed = cljn_vec_assoc(vector, MK_FIX(32), MK_FIX(9001));
    cljn_gc_set(base + 3, changed);
    CHECK(FIX(pv_nth((PVec *)vector, 32)) == 32);
    CHECK(FIX(pv_nth((PVec *)changed, 32)) == 9001);
    CHECK(((PVec *)vector)->root != ((PVec *)changed)->root);
    CHECK(((PVec *)vector)->tail == ((PVec *)changed)->tail);

    cljn_gc_force();
    CHECK(FIX(pv_nth((PVec *)vector, 1056)) == 1056);
    CHECK(FIX(pv_nth((PVec *)changed, 32)) == 9001);
    cljn_gc_leave(base);
    cljn_gc_force();
    return 0;
}

static int test_hamt_promotion_update_and_demotion(void) {
    Value base = cljn_gc_enter(4);
    Value map = cljn_map_alloc(0);
    cljn_gc_set(base, map);
    Value map_at_eight = NIL;

    for (int64_t i = 0; i < 9; i++) {
        map = cljn_map_assoc(map, MK_FIX(i), MK_FIX(i * 10));
        cljn_gc_set(base, map);
        if (i == 7) {
            map_at_eight = map;
            cljn_gc_set(base + 1, map_at_eight);
        }
    }

    CHECK(obj_type(map_at_eight) == T_MAP);
    CHECK(((Map *)map_at_eight)->n == 8);
    CHECK(obj_type(map) == T_HMAP);
    CHECK(((HMap *)map)->count == 9);
    CHECK(FIX(cljn_map_get(map, MK_FIX(8))) == 80);

    Value changed = cljn_map_assoc(map_at_eight, MK_FIX(0), MK_FIX(999));
    cljn_gc_set(base + 2, changed);
    CHECK(FIX(cljn_map_get(map_at_eight, MK_FIX(0))) == 0);
    CHECK(FIX(cljn_map_get(changed, MK_FIX(0))) == 999);

    Value demoted = cljn_map_dissoc(map, MK_FIX(4));
    cljn_gc_set(base + 3, demoted);
    CHECK(obj_type(demoted) == T_MAP);
    CHECK(((Map *)demoted)->n == 8);
    CHECK(cljn_map_contains(demoted, MK_FIX(4)) == FALSEV);

    cljn_gc_force();
    CHECK(FIX(cljn_map_get(map, MK_FIX(8))) == 80);
    CHECK(FIX(cljn_map_get(changed, MK_FIX(0))) == 999);
    cljn_gc_leave(base);
    cljn_gc_force();
    return 0;
}

static int llrb_black_height(Value node, Value *previous, int *seen) {
    if (node == NIL) return 1;
    TNode *current = (TNode *)node;

    if (tn_red(current->right)) return -1;
    if (current->red && (tn_red(current->left) || tn_red(current->right))) return -1;

    int left = llrb_black_height(current->left, previous, seen);
    if (left < 0) return -1;
    if (*seen && compare_raw(*previous, current->key) >= 0) return -1;
    *previous = current->key;
    *seen = 1;
    int right = llrb_black_height(current->right, previous, seen);
    if (right < 0 || left != right) return -1;
    return left + (current->red ? 0 : 1);
}

static int test_sorted_map_llrb_invariants(void) {
    Value base = cljn_gc_enter(1);
    Value map = cljn_sorted_map_empty();
    cljn_gc_set(base, map);

    for (int64_t i = 0; i < 128; i++) {
        int64_t key = (i * 37) % 128;
        map = cljn_sorted_assoc(map, MK_FIX(key), MK_FIX(key * 2));
        cljn_gc_set(base, map);
    }
    CHECK(((Sorted *)map)->count == 128);
    CHECK(!tn_red(((Sorted *)map)->root));
    Value previous = NIL;
    int seen = 0;
    CHECK(llrb_black_height(((Sorted *)map)->root, &previous, &seen) > 0);

    for (int64_t key = 0; key < 128; key += 3) {
        map = cljn_sorted_dissoc(map, MK_FIX(key));
        cljn_gc_set(base, map);
    }
    CHECK(((Sorted *)map)->count == 85);
    CHECK(!tn_red(((Sorted *)map)->root));
    previous = NIL;
    seen = 0;
    CHECK(llrb_black_height(((Sorted *)map)->root, &previous, &seen) > 0);

    cljn_gc_force();
    CHECK(FIX(cljn_sorted_get(map, MK_FIX(127))) == 254);
    CHECK(cljn_sorted_contains(map, MK_FIX(126)) == FALSEV);
    cljn_gc_leave(base);
    cljn_gc_force();
    return 0;
}

static int finally_calls = 0;

static Value throwing_body(Value self, int64_t argc, Value *argv) {
    (void)self;
    (void)argc;
    (void)argv;
    return cljn_throw(MK_FIX(41));
}

static Value catching_handler(Value self, int64_t argc, Value *argv) {
    (void)self;
    CHECK(argc == 1);
    return cljn_inc(argv[0]);
}

static Value finally_handler(Value self, int64_t argc, Value *argv) {
    (void)self;
    (void)argc;
    (void)argv;
    finally_calls++;
    return NIL;
}

static int test_exception_unwind_restores_runtime_state(void) {
    Value base = cljn_gc_enter(3);
    Value body = cljn_make_fn((Value)(intptr_t)&throwing_body, 0, 0);
    cljn_gc_set(base, body);
    Value catcher = cljn_make_fn((Value)(intptr_t)&catching_handler, 1, 0);
    cljn_gc_set(base + 1, catcher);
    Value finalizer = cljn_make_fn((Value)(intptr_t)&finally_handler, 0, 0);
    cljn_gc_set(base + 2, finalizer);

    int64_t expected_sp = gc_sp;
    int expected_disabled = gc_disabled;
    finally_calls = 0;
    Value result = cljn_try(body, catcher, finalizer);

    CHECK(result == MK_FIX(42));
    CHECK(finally_calls == 1);
    CHECK(gc_sp == expected_sp);
    CHECK(gc_disabled == expected_disabled);
    cljn_gc_leave(base);
    cljn_gc_force();
    return 0;
}

int main(void) {
    CHECK(test_tagged_values_and_truthiness() == 0);
    CHECK(test_shadow_stack_frames() == 0);
    CHECK(test_allocator_reuses_swept_small_objects() == 0);
    CHECK(test_gc_marks_cycles_and_reclaims_them() == 0);
    CHECK(test_vector_trie_boundaries_and_persistence() == 0);
    CHECK(test_hamt_promotion_update_and_demotion() == 0);
    CHECK(test_sorted_map_llrb_invariants() == 0);
    CHECK(test_exception_unwind_restores_runtime_state() == 0);
    puts("runtime C unit tests: ok");
    return 0;
}
