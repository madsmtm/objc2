//! Check that `AllocAnyThread`/`MainThreadOnly` traits are not implementable manually.
use objc2::{AllocAnyThread, MainThreadOnly};

struct CustomStruct;

unsafe impl AllocAnyThread for CustomStruct {}

unsafe impl MainThreadOnly for CustomStruct {}

fn main() {}
