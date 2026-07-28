/*
 * Unidade de tradução amalgamada do runtime nativo.
 *
 * A ordem deve permanecer igual à lista RUNTIME_MODULES em
 * clojure-codegen/src/lib.rs. Os fragmentos não são compilados isoladamente.
 */
#include "00_types.c"
#include "10_gc.c"
#include "20_values_and_functions.c"
#include "30_vector.c"
#include "40_hash_collections.c"
#include "50_sorted_collections.c"
#include "60_records_and_dispatch.c"
#include "70_transients.c"
#include "80_core_operations.c"
#include "90_print.c"
#include "100_exceptions.c"
#include "110_multimethods.c"
#include "120_test_introspection.c"
#include "130_io.c"
