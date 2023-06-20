//! Test that `Id::autorelease_return` is tail-called properly.

use objc2::__macro_helpers::{MsgSendId, New};
use objc2::rc::Id;
use objc2::runtime::{Class, Object, Sel};

#[no_mangle]
fn simple(obj: Id<Object>) -> *mut Object {
    Id::autorelease_return(obj)
}

#[no_mangle]
unsafe fn with_body(cls: &Class, sel: Sel) -> *mut Object {
    let obj: Option<Id<Object>> = New::send_message_id(cls, sel, ());
    Id::autorelease_return(obj.unwrap_unchecked())
}
