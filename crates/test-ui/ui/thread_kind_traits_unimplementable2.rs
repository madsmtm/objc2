//! Check that `AnyThread`/`MainThreadOnly` traits are not implementable manually.
use objc2::{AnyThread, MainThreadOnly};

struct CustomStruct;

unsafe impl AnyThread for CustomStruct {}

unsafe impl MainThreadOnly for CustomStruct {}

fn main() {}
