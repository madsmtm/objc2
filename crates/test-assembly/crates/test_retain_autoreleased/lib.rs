//! Test that `Retained::retain_autoreleased` is inlined properly.

use objc2::rc::Retained;
use objc2::runtime::{AnyObject, MessageReceiver, Sel};

#[no_mangle]
unsafe fn handle(obj: &AnyObject, sel: Sel) -> Option<Retained<AnyObject>> {
    let ptr: *mut AnyObject = MessageReceiver::send_message(obj, sel, ());
    Retained::retain_autoreleased(ptr)
}
