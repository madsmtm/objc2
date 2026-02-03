use objc2_core_foundation::cf_type;

#[allow(dead_code)]
struct TISInputSource {}

// Test old syntax still works (used by dependencies).
cf_type!(
    #[encoding_name = "__TISInputSource"]
    unsafe impl TISInputSource {}
);

fn main() {}
