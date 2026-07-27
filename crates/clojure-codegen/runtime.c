/*
 * Entrada compatível para compilação direta do runtime.
 *
 * O codegen Rust amalgama os mesmos módulos com include_str! para produzir
 * um único arquivo C temporário, sem mudar a unidade de tradução ou a ABI.
 */
#include "runtime/runtime_all.c"
