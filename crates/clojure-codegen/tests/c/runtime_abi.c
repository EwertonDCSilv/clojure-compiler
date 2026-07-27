#include <stdint.h>
#include <stdio.h>

typedef intptr_t Value;

#define NIL ((Value)2)
#define FALSEV ((Value)6)
#define TRUEV ((Value)10)
#define EMPTY ((Value)18)
#define MK_FIX(n) ((Value)(((uintptr_t)(intptr_t)(n) << 1) | 1u))
#define FIX(v) ((intptr_t)(v) >> 1)

Value cljn_gc_enter(Value nslots);
void cljn_gc_leave(Value base);
void cljn_gc_set(Value index, Value value);
void cljn_gc_force(void);

Value cljn_str_from(const char *bytes, long length);
Value cljn_str_concat(Value left, Value right);
Value cljn_to_str(Value value);
Value cljn_kw(const char *bytes, long length);
Value cljn_empty(void);
Value cljn_cons(Value head, Value tail);
Value cljn_first(Value value);
Value cljn_rest(Value value);
Value cljn_count(Value value);

Value cljn_vec_empty(void);
Value cljn_vec_conj(Value vector, Value value);
Value cljn_nth(Value collection, Value index);
Value cljn_nth_or(Value collection, Value index, Value not_found);
Value cljn_assoc(Value collection, Value key, Value value);

Value cljn_map_alloc(Value count);
Value cljn_map_assoc(Value map, Value key, Value value);
Value cljn_map_dissoc(Value map, Value key);
Value cljn_get(Value collection, Value key);
Value cljn_contains(Value collection, Value key);

Value cljn_set_alloc(Value capacity);
Value cljn_set_conj(Value set, Value value);

Value cljn_sorted_map_empty(void);
Value cljn_sorted_assoc(Value map, Value key, Value value);
Value cljn_sorted_get(Value map, Value key);

Value cljn_make_record(Value type_name, Value map);
Value cljn_record_type(Value record);
Value cljn_type_key(Value value);

Value cljn_make_fn(Value code, Value arity, Value capture_count);
void cljn_fn_set_free(Value function, Value index, Value value);
Value cljn_fn_free(Value function, Value index);
Value cljn_fn_code(Value function);
void cljn_register_method(Value method_id, Value key, Value implementation);
Value cljn_lookup_method(Value method_id, Value key);
void cljn_multi_register(Value multimethod_id, Value dispatch_function);
Value cljn_multi_call(Value multimethod_id, Value argc, Value argv);

Value cljn_transient(Value collection);
Value cljn_conj_bang(Value transient, Value value);
Value cljn_assoc_bang(Value transient, Value key, Value value);
Value cljn_persistent_bang(Value transient);

Value cljn_add(Value left, Value right);
Value cljn_sub(Value left, Value right);
Value cljn_mul(Value left, Value right);
Value cljn_quot(Value left, Value right);
Value cljn_mod(Value left, Value right);
Value cljn_inc(Value value);
Value cljn_dec(Value value);
Value cljn_lt(Value left, Value right);
Value cljn_le(Value left, Value right);
Value cljn_gt(Value left, Value right);
Value cljn_ge(Value left, Value right);
Value cljn_eq(Value left, Value right);
int cljn_equal_raw(Value left, Value right);
int cljn_truthy(Value value);

typedef Value (*FnCode)(Value self, int64_t argc, Value *argv);

#define CHECK(condition)                                                        \
    do {                                                                        \
        if (!(condition)) {                                                     \
            fprintf(stderr, "CHECK falhou em %s:%d: %s\n", __FILE__, __LINE__, \
                    #condition);                                                \
            return 1;                                                           \
        }                                                                       \
    } while (0)

static int test_arithmetic_and_predicates(void) {
    CHECK(FIX(cljn_add(MK_FIX(20), MK_FIX(22))) == 42);
    CHECK(FIX(cljn_sub(MK_FIX(20), MK_FIX(22))) == -2);
    CHECK(FIX(cljn_mul(MK_FIX(-6), MK_FIX(7))) == -42);
    CHECK(FIX(cljn_quot(MK_FIX(-43), MK_FIX(10))) == -4);
    CHECK(FIX(cljn_mod(MK_FIX(-43), MK_FIX(10))) == 7);
    CHECK(FIX(cljn_mod(MK_FIX(43), MK_FIX(-10))) == -7);
    CHECK(FIX(cljn_inc(MK_FIX(41))) == 42);
    CHECK(FIX(cljn_dec(MK_FIX(43))) == 42);
    CHECK(cljn_lt(MK_FIX(1), MK_FIX(2)) == TRUEV);
    CHECK(cljn_le(MK_FIX(2), MK_FIX(2)) == TRUEV);
    CHECK(cljn_gt(MK_FIX(3), MK_FIX(2)) == TRUEV);
    CHECK(cljn_ge(MK_FIX(2), MK_FIX(3)) == FALSEV);
    CHECK(cljn_eq(MK_FIX(42), MK_FIX(42)) == TRUEV);
    CHECK(cljn_truthy(NIL) == 0);
    CHECK(cljn_truthy(FALSEV) == 0);
    CHECK(cljn_truthy(MK_FIX(0)) == 1);
    return 0;
}

static int test_heap_values_and_collections(void) {
    Value base = cljn_gc_enter(16);

    Value hello = cljn_str_from("hello", 5);
    cljn_gc_set(base, hello);
    Value suffix = cljn_str_from(" runtime", 8);
    cljn_gc_set(base + 1, suffix);
    Value joined = cljn_str_concat(hello, suffix);
    cljn_gc_set(base + 2, joined);
    Value expected = cljn_str_from("hello runtime", 13);
    cljn_gc_set(base + 3, expected);
    CHECK(cljn_equal_raw(joined, expected));
    CHECK(FIX(cljn_count(joined)) == 13);

    Value number_string = cljn_to_str(MK_FIX(-42));
    cljn_gc_set(base + 4, number_string);
    Value expected_number = cljn_str_from("-42", 3);
    cljn_gc_set(base + 5, expected_number);
    CHECK(cljn_equal_raw(number_string, expected_number));

    Value vector = cljn_vec_empty();
    cljn_gc_set(base + 6, vector);
    for (int64_t i = 0; i < 70; i++) {
        vector = cljn_vec_conj(vector, MK_FIX(i));
        cljn_gc_set(base + 6, vector);
    }
    CHECK(FIX(cljn_count(vector)) == 70);
    CHECK(FIX(cljn_nth(vector, MK_FIX(0))) == 0);
    CHECK(FIX(cljn_nth(vector, MK_FIX(32))) == 32);
    CHECK(FIX(cljn_nth(vector, MK_FIX(69))) == 69);
    CHECK(FIX(cljn_nth_or(vector, MK_FIX(70), MK_FIX(777))) == 777);

    Value original_vector = vector;
    cljn_gc_set(base + 7, original_vector);
    Value changed_vector = cljn_assoc(vector, MK_FIX(32), MK_FIX(999));
    cljn_gc_set(base + 8, changed_vector);
    CHECK(FIX(cljn_nth(original_vector, MK_FIX(32))) == 32);
    CHECK(FIX(cljn_nth(changed_vector, MK_FIX(32))) == 999);

    Value list = cljn_empty();
    cljn_gc_set(base + 9, list);
    for (int64_t i = 5; i >= 0; i--) {
        list = cljn_cons(MK_FIX(i), list);
        cljn_gc_set(base + 9, list);
    }
    CHECK(FIX(cljn_count(list)) == 6);
    CHECK(FIX(cljn_first(list)) == 0);
    CHECK(FIX(cljn_first(cljn_rest(list))) == 1);

    Value map = cljn_map_alloc(0);
    cljn_gc_set(base + 10, map);
    for (int64_t i = 0; i < 20; i++) {
        map = cljn_map_assoc(map, MK_FIX(i), MK_FIX(i * 10));
        cljn_gc_set(base + 10, map);
    }
    CHECK(FIX(cljn_count(map)) == 20);
    CHECK(FIX(cljn_get(map, MK_FIX(12))) == 120);
    CHECK(cljn_contains(map, MK_FIX(19)) == TRUEV);
    CHECK(cljn_contains(map, MK_FIX(20)) == FALSEV);
    Value removed = cljn_map_dissoc(map, MK_FIX(12));
    cljn_gc_set(base + 11, removed);
    CHECK(FIX(cljn_count(removed)) == 19);
    CHECK(cljn_contains(removed, MK_FIX(12)) == FALSEV);
    CHECK(FIX(cljn_get(map, MK_FIX(12))) == 120);

    Value set = cljn_set_alloc(0);
    cljn_gc_set(base + 12, set);
    for (int64_t i = 0; i < 20; i++) {
        set = cljn_set_conj(set, MK_FIX(i));
        cljn_gc_set(base + 12, set);
    }
    set = cljn_set_conj(set, MK_FIX(10));
    cljn_gc_set(base + 12, set);
    CHECK(FIX(cljn_count(set)) == 20);
    CHECK(cljn_contains(set, MK_FIX(10)) == TRUEV);
    CHECK(cljn_contains(set, MK_FIX(30)) == FALSEV);

    Value sorted = cljn_sorted_map_empty();
    cljn_gc_set(base + 13, sorted);
    for (int64_t i = 9; i >= 0; i--) {
        sorted = cljn_sorted_assoc(sorted, MK_FIX(i), MK_FIX(i + 100));
        cljn_gc_set(base + 13, sorted);
    }
    CHECK(FIX(cljn_count(sorted)) == 10);
    CHECK(FIX(cljn_sorted_get(sorted, MK_FIX(4))) == 104);

    Value record_type = cljn_kw("example/Point", 13);
    cljn_gc_set(base + 14, record_type);
    Value record = cljn_make_record(record_type, map);
    cljn_gc_set(base + 15, record);
    CHECK(cljn_equal_raw(cljn_record_type(record), record_type));
    CHECK(cljn_equal_raw(cljn_type_key(record), record_type));
    CHECK(FIX(cljn_get(record, MK_FIX(3))) == 30);

    cljn_gc_force();
    CHECK(FIX(cljn_nth(changed_vector, MK_FIX(32))) == 999);
    CHECK(FIX(cljn_get(map, MK_FIX(12))) == 120);
    CHECK(cljn_contains(set, MK_FIX(10)) == TRUEV);
    CHECK(FIX(cljn_sorted_get(sorted, MK_FIX(4))) == 104);
    cljn_gc_leave(base);
    return 0;
}

static int test_transient_round_trip(void) {
    Value base = cljn_gc_enter(3);
    Value vector = cljn_vec_empty();
    cljn_gc_set(base, vector);
    vector = cljn_vec_conj(vector, MK_FIX(1));
    cljn_gc_set(base, vector);
    vector = cljn_vec_conj(vector, MK_FIX(2));
    cljn_gc_set(base, vector);

    Value transient = cljn_transient(vector);
    cljn_gc_set(base + 1, transient);
    CHECK(cljn_conj_bang(transient, MK_FIX(3)) == transient);
    CHECK(cljn_assoc_bang(transient, MK_FIX(0), MK_FIX(10)) == transient);
    Value persistent = cljn_persistent_bang(transient);
    cljn_gc_set(base + 2, persistent);

    CHECK(FIX(cljn_count(persistent)) == 3);
    CHECK(FIX(cljn_nth(persistent, MK_FIX(0))) == 10);
    CHECK(FIX(cljn_nth(persistent, MK_FIX(2))) == 3);
    CHECK(FIX(cljn_count(vector)) == 2);
    CHECK(FIX(cljn_nth(vector, MK_FIX(0))) == 1);

    cljn_gc_force();
    CHECK(FIX(cljn_nth(persistent, MK_FIX(2))) == 3);
    cljn_gc_leave(base);
    return 0;
}

static Value captured_adder(Value self, int64_t argc, Value *argv) {
    if (argc != 1) return NIL;
    return cljn_add(cljn_fn_free(self, 0), argv[0]);
}

static Value identity_dispatch(Value self, int64_t argc, Value *argv) {
    (void)self;
    if (argc != 1) return NIL;
    return argv[0];
}

static Value multimethod_implementation(Value self, int64_t argc, Value *argv) {
    (void)self;
    if (argc != 1) return NIL;
    return cljn_add(MK_FIX(500), argv[0]);
}

static int test_functions_protocols_and_multimethods(void) {
    Value base = cljn_gc_enter(3);
    Value function =
        cljn_make_fn((Value)(intptr_t)&captured_adder, 1, 1);
    cljn_gc_set(base, function);
    cljn_fn_set_free(function, 0, MK_FIX(40));
    CHECK(cljn_fn_free(function, 0) == MK_FIX(40));
    CHECK(cljn_fn_code(function) == (Value)(intptr_t)&captured_adder);
    Value args[] = {MK_FIX(2)};
    FnCode code = (FnCode)(intptr_t)cljn_fn_code(function);
    CHECK(code(function, 1, args) == MK_FIX(42));

    Value fixnum_type = cljn_type_key(MK_FIX(1));
    cljn_register_method(700, fixnum_type, function);
    Value implementation = cljn_lookup_method(700, fixnum_type);
    CHECK(implementation == function);
    CHECK(cljn_lookup_method(700, cljn_type_key(NIL)) == NIL);
    CHECK(((FnCode)(intptr_t)cljn_fn_code(implementation))(
              implementation, 1, args) == MK_FIX(42));

    Value dispatch =
        cljn_make_fn((Value)(intptr_t)&identity_dispatch, 1, 0);
    cljn_gc_set(base + 1, dispatch);
    Value method =
        cljn_make_fn((Value)(intptr_t)&multimethod_implementation, 1, 0);
    cljn_gc_set(base + 2, method);
    cljn_multi_register(800, dispatch);
    cljn_register_method(800, MK_FIX(7), method);
    Value dispatch_args[] = {MK_FIX(7)};
    CHECK(cljn_multi_call(800, 1, (Value)(intptr_t)dispatch_args) ==
          MK_FIX(507));

    cljn_gc_force();
    CHECK(cljn_lookup_method(700, fixnum_type) == function);
    CHECK(cljn_multi_call(800, 1, (Value)(intptr_t)dispatch_args) ==
          MK_FIX(507));
    cljn_gc_leave(base);
    return 0;
}

int main(void) {
    CHECK(test_arithmetic_and_predicates() == 0);
    CHECK(test_heap_values_and_collections() == 0);
    CHECK(test_transient_round_trip() == 0);
    CHECK(test_functions_protocols_and_multimethods() == 0);
    puts("runtime C ABI integration: ok");
    return 0;
}
