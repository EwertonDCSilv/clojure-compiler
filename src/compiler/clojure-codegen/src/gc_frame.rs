//! Pure policy for selecting generated shadow-stack frames.

/// Returns whether generated code must establish a GC frame.
///
/// GC: fixed local roots always require a frame. A zero-slot function still
/// requires one when its normal result remains temporarily rooted for its caller.
pub(super) const fn needs_gc_frame(root_slot_count: usize, result_rooted: bool) -> bool {
    root_slot_count > 0 || result_rooted
}

#[cfg(test)]
#[path = "../tests/unit/gc_frame/mod.rs"]
mod tests;
