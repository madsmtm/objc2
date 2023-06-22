//! Test that `Id::retain_autoreleased` is inlined properly.

use objc2::rc::Id;
use objc2::runtime::{AnyObject, Sel};
use objc2::MessageReceiver;

#[no_mangle]
unsafe fn handle(obj: &AnyObject, sel: Sel) -> Option<Id<AnyObject>> {
    let ptr: *mut AnyObject = MessageReceiver::send_message(obj, sel, ());
    Id::retain_autoreleased(ptr)
}
