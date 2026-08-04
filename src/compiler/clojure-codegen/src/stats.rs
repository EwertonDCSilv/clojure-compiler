//! Deterministic structural lowering metrics, returned alongside the object
//! bytes by [`crate::compile_object_with_options_and_stats`]. Collecting
//! these counters never alters generated IR.

/// Deterministic aggregate structural metrics for optional lowering.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OptimizationStats {
    /// Generic callable entries emitted.
    pub generic_entries: u64,
    /// Raw-fixnum internal entries emitted.
    pub specialized_entries: u64,
    /// Direct calls through the generic callable ABI.
    pub generic_direct_calls: u64,
    /// Direct calls through the raw-fixnum ABI.
    pub specialized_direct_calls: u64,
    /// Arguments spilled solely to form generic `argv`.
    pub generic_argv_spills: u64,
    /// Compact fixed root slots reserved across generated functions.
    pub root_slots: u64,
    /// Shadow-stack frame entries emitted.
    pub root_frame_entries: u64,
    /// Stores emitted into fixed local root slots.
    pub root_stores: u64,
    /// Local definitions kept as untagged fixnum payloads.
    pub raw_fixnum_bindings: u64,
    /// Calls emitted to imported C runtime symbols.
    pub runtime_abi_calls: u64,
    /// Stores to the `gc_sp` global emitted by the codegen.
    ///
    /// With D1 (ADR-0018) this equals the number of call sites that flushed
    /// the cached pointer before entering a runtime function, not the number
    /// of individual push/pop operations.
    pub gc_sp_global_stores: u64,
}

impl OptimizationStats {
    /// Renders stable JSON without introducing a serialization dependency.
    pub fn to_json(&self) -> String {
        format!(
            concat!(
                "{{\n",
                "  \"schema\": \"clojure-compiler-optimization-stats-v1\",\n",
                "  \"generic_entries\": {},\n",
                "  \"specialized_entries\": {},\n",
                "  \"generic_direct_calls\": {},\n",
                "  \"specialized_direct_calls\": {},\n",
                "  \"generic_argv_spills\": {},\n",
                "  \"root_slots\": {},\n",
                "  \"root_frame_entries\": {},\n",
                "  \"root_stores\": {},\n",
                "  \"raw_fixnum_bindings\": {},\n",
                "  \"runtime_abi_calls\": {},\n",
                "  \"gc_sp_global_stores\": {}\n",
                "}}\n"
            ),
            self.generic_entries,
            self.specialized_entries,
            self.generic_direct_calls,
            self.specialized_direct_calls,
            self.generic_argv_spills,
            self.root_slots,
            self.root_frame_entries,
            self.root_stores,
            self.raw_fixnum_bindings,
            self.runtime_abi_calls,
            self.gc_sp_global_stores,
        )
    }
}
