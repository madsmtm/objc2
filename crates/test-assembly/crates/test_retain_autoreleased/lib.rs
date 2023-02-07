//! Test that `Id::retain_autoreleased` is inlined properly.

use objc2::rc::Id;
use objc2::runtime::{Object, Sel};
use objc2::MessageReceiver;

#[no_mangle]
unsafe fn handle(obj: &Object, sel: Sel) -> Option<Id<Object>> {
    let ptr: *mut Object = MessageReceiver::send_message(obj, sel, ());
    Id::retain_autoreleased(ptr)
}
