// Test that msg_send! is inlined into an objc_msgSend
//
// assembly-output: emit-asm
// only-x86
// compile-flags: -Copt-level=2 -Clto=off

#![crate_type = "lib"]

use objc2::runtime::{Class, Object, Sel};
use objc2::MessageReceiver;

// CHECK-LABEL: handle:
// CHECK-NOT: j
// CHECK-NOT: call
// CHECK: jmp _objc_msgSend
#[no_mangle]
pub fn handle(obj: &Class, sel: Sel) -> *mut Object {
    unsafe { MessageReceiver::send_message(&obj, sel, ()).unwrap() }
}
