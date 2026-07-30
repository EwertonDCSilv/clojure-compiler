
/*
 * Test-only GC introspection ABI.
 *
 * These functions expose collector state to the C harness and are not emitted
 * as language primitives.
 */
/* Return the raw number of objects currently linked in the heap. O(live). */
long cljn_gc_live_objects(void) { long n=0; for (Obj*o=all_objs;o;o=o->next_all) n++; return n; }
/* Force a complete collection immediately. */
void cljn_gc_force(void) { gc_collect(); }
