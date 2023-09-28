//! Test assembly output of `msg_send_id!` internals.
use objc2::__macro_helpers::{Alloc, CopyOrMutCopy, Init, MsgSendId, New, Other};
use objc2::rc::{Allocated, Id};
use objc2::runtime::{AnyClass, AnyObject, Sel};

#[no_mangle]
unsafe fn handle_new(cls: &AnyClass, sel: Sel) -> Option<Id<AnyObject>> {
    New::send_message_id(cls, sel, ())
}

#[no_mangle]
unsafe fn handle_new_fallible(cls: &AnyClass, sel: Sel) -> Id<AnyObject> {
    New::send_message_id(cls, sel, ())
}

#[no_mangle]
unsafe fn handle_alloc(cls: &AnyClass, sel: Sel) -> Allocated<AnyObject> {
    Alloc::send_message_id(cls, sel, ())
}

#[no_mangle]
unsafe fn handle_init(obj: Allocated<AnyObject>, sel: Sel) -> Option<Id<AnyObject>> {
    Init::send_message_id(obj, sel, ())
}

#[no_mangle]
unsafe fn handle_init_fallible(obj: Allocated<AnyObject>, sel: Sel) -> Id<AnyObject> {
    Init::send_message_id(obj, sel, ())
}

#[no_mangle]
unsafe fn handle_alloc_init(cls: &AnyClass, sel1: Sel, sel2: Sel) -> Option<Id<AnyObject>> {
    let obj = Alloc::send_message_id(cls, sel1, ());
    Init::send_message_id(obj, sel2, ())
}

#[no_mangle]
unsafe fn handle_alloc_release(cls: &AnyClass, sel: Sel) {
    let _obj: Allocated<AnyObject> = Alloc::send_message_id(cls, sel, ());
}

#[no_mangle]
unsafe fn handle_alloc_init_release(cls: &AnyClass, sel1: Sel, sel2: Sel) {
    let obj = Alloc::send_message_id(cls, sel1, ());
    let obj: Option<Id<AnyObject>> = Init::send_message_id(obj, sel2, ());
    let _obj = obj.unwrap_unchecked();
}

#[no_mangle]
unsafe fn handle_copy(obj: &AnyObject, sel: Sel) -> Option<Id<AnyObject>> {
    CopyOrMutCopy::send_message_id(obj, sel, ())
}

#[no_mangle]
unsafe fn handle_copy_fallible(obj: &AnyObject, sel: Sel) -> Id<AnyObject> {
    CopyOrMutCopy::send_message_id(obj, sel, ())
}

#[no_mangle]
unsafe fn handle_autoreleased(obj: &AnyObject, sel: Sel) -> Option<Id<AnyObject>> {
    Other::send_message_id(obj, sel, ())
}

#[no_mangle]
unsafe fn handle_autoreleased_fallible(obj: &AnyObject, sel: Sel) -> Id<AnyObject> {
    Other::send_message_id(obj, sel, ())
}

// TODO: The optimization does not happen here, fix this!
#[no_mangle]
unsafe fn handle_with_out_param(
    obj: &AnyObject,
    sel: Sel,
    param: &mut Id<AnyObject>,
) -> Option<Id<AnyObject>> {
    Other::send_message_id(obj, sel, (param,))
}
