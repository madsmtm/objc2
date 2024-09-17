//! Test assembly output of `msg_send_id!` internals.
use objc2::__macro_helpers::{Alloc, CopyOrMutCopy, Init, MsgSendId, New, Other};
use objc2::rc::{Allocated, Retained};
use objc2::runtime::{AnyClass, AnyObject, Sel};

#[no_mangle]
unsafe fn handle_new(cls: &AnyClass, sel: Sel) -> Option<Retained<AnyObject>> {
    New::send_message_id(cls, sel, ())
}

#[no_mangle]
unsafe fn handle_new_fallible(cls: &AnyClass, sel: Sel) -> Retained<AnyObject> {
    New::send_message_id(cls, sel, ())
}

#[no_mangle]
unsafe fn handle_alloc(cls: &AnyClass, sel: Sel) -> Allocated<AnyObject> {
    Alloc::send_message_id(cls, sel, ())
}

#[no_mangle]
unsafe fn handle_init(obj: Allocated<AnyObject>, sel: Sel) -> Option<Retained<AnyObject>> {
    Init::send_message_id(obj, sel, ())
}

#[no_mangle]
unsafe fn handle_init_fallible(obj: Allocated<AnyObject>, sel: Sel) -> Retained<AnyObject> {
    Init::send_message_id(obj, sel, ())
}

#[no_mangle]
unsafe fn handle_alloc_init(cls: &AnyClass, sel1: Sel, sel2: Sel) -> Option<Retained<AnyObject>> {
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
    let obj: Option<Retained<AnyObject>> = Init::send_message_id(obj, sel2, ());
    let _obj = obj.unwrap_unchecked();
}

#[no_mangle]
unsafe fn handle_copy(obj: &AnyObject, sel: Sel) -> Option<Retained<AnyObject>> {
    CopyOrMutCopy::send_message_id(obj, sel, ())
}

#[no_mangle]
unsafe fn handle_copy_fallible(obj: &AnyObject, sel: Sel) -> Retained<AnyObject> {
    CopyOrMutCopy::send_message_id(obj, sel, ())
}

#[no_mangle]
unsafe fn handle_autoreleased(obj: &AnyObject, sel: Sel) -> Option<Retained<AnyObject>> {
    Other::send_message_id(obj, sel, ())
}

// TODO: The optimization does not happen here on aarch64, fix this!
#[no_mangle]
unsafe fn handle_autoreleased_with_arg(
    obj: &AnyObject,
    sel: Sel,
    arg: u8,
) -> Option<Retained<AnyObject>> {
    Other::send_message_id(obj, sel, (arg,))
}

#[no_mangle]
unsafe fn handle_autoreleased_fallible(obj: &AnyObject, sel: Sel) -> Retained<AnyObject> {
    Other::send_message_id(obj, sel, ())
}

// TODO: The optimization does not happen here, fix this!
#[no_mangle]
unsafe fn handle_with_out_param(
    obj: &AnyObject,
    sel: Sel,
    param: &mut Retained<AnyObject>,
) -> Option<Retained<AnyObject>> {
    Other::send_message_id(obj, sel, (param,))
}
