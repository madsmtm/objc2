// Test that msg_send! is inlined into an objc_msgSend
//
// assembly-output: emit-asm
// only-x86
// compile-flags: -Copt-level=3

use objc2::runtime::Object;
use objc2::{class, msg_send};

fn main() {
    unsafe {
        let cls = class!(NSObject);
        let _obj: *mut Object = msg_send![cls, new];
    }
}
