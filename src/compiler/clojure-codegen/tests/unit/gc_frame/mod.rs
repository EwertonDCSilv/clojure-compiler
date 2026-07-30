//! Unit tests for gc_frame.rs.

use super::needs_gc_frame;

#[test]
fn requires_a_frame_only_for_fixed_slots_or_a_rooted_result() {
    assert!(!needs_gc_frame(0, false));
    assert!(needs_gc_frame(1, false));
    assert!(needs_gc_frame(0, true));
    assert!(needs_gc_frame(1, true));
}
