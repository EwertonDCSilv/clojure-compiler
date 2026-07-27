
/* Introspecção para testes. */
long cljn_gc_live_objects(void) { long n=0; for (Obj*o=all_objs;o;o=o->next_all) n++; return n; }
void cljn_gc_force(void) { gc_collect(); }
