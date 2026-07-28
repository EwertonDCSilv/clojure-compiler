/*
 * Compatibility entry point for compiling the native runtime directly.
 *
 * The Rust backend embeds the same ordered fragments into one temporary C file.
 * Including runtime_all.c here preserves the same translation-unit visibility,
 * layouts, and ABI for C harnesses.
 */
#include "runtime/runtime_all.c"
