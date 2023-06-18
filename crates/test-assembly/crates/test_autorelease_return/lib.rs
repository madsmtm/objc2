//! Test that `Id::autorelease_return` is tail-called properly.

use objc2::__macro_helpers::{MsgSendId, New};
use objc2::rc::Id;
use objc2::runtime::{AnyClass, AnyObject, Sel};

#[no_mangle]
fn simple(obj: Id<AnyObject>) -> *mut AnyObject {
    Id::autorelease_return(obj)
}

#[no_mangle]
unsafe fn with_body(cls: &AnyClass, sel: Sel) -> *mut AnyObject {
    let obj: Option<Id<AnyObject>> = New::send_message_id(cls, sel, ());
    Id::autorelease_return(obj.unwrap_unchecked())
}
