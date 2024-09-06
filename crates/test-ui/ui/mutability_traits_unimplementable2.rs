//! Check that `mutability` traits are not implementable manually.
use objc2::mutability;

struct CustomStruct;

unsafe impl mutability::IsAllocableAnyThread for CustomStruct {}

fn main() {}
