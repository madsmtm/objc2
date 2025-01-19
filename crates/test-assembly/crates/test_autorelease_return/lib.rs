//! Test that `Retained::autorelease_return` is tail-called properly.

use objc2::__macro_helpers::{MsgSend, NewFamily};
use objc2::rc::Retained;
use objc2::runtime::{AnyClass, AnyObject, Sel};

#[no_mangle]
fn simple(obj: Retained<AnyObject>) -> *mut AnyObject {
    Retained::autorelease_return(obj)
}

#[no_mangle]
unsafe fn with_body(cls: &AnyClass, sel: Sel) -> *mut AnyObject {
    let obj: Option<Retained<AnyObject>> = NewFamily::send_message(cls, sel, ());
    Retained::autorelease_return(obj.unwrap_unchecked())
}
