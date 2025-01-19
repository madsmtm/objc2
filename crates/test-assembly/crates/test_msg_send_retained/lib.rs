//! Test assembly output of `msg_send!` internals.
use objc2::__macro_helpers::{
    AllocFamily, CopyFamily, InitFamily, MsgSend, MutableCopyFamily, NewFamily, NoneFamily,
};
use objc2::rc::{Allocated, Retained};
use objc2::runtime::{AnyClass, AnyObject, Sel};

#[no_mangle]
unsafe fn handle_new(cls: &AnyClass, sel: Sel) -> Option<Retained<AnyObject>> {
    NewFamily::send_message(cls, sel, ())
}

#[no_mangle]
unsafe fn handle_new_fallible(cls: &AnyClass, sel: Sel) -> Retained<AnyObject> {
    NewFamily::send_message(cls, sel, ())
}

#[no_mangle]
unsafe fn handle_alloc(cls: &AnyClass, sel: Sel) -> Allocated<AnyObject> {
    AllocFamily::send_message(cls, sel, ())
}

#[no_mangle]
unsafe fn handle_init(obj: Allocated<AnyObject>, sel: Sel) -> Option<Retained<AnyObject>> {
    InitFamily::send_message(obj, sel, ())
}

#[no_mangle]
unsafe fn handle_init_fallible(obj: Allocated<AnyObject>, sel: Sel) -> Retained<AnyObject> {
    InitFamily::send_message(obj, sel, ())
}

#[no_mangle]
unsafe fn handle_alloc_init(cls: &AnyClass, sel1: Sel, sel2: Sel) -> Option<Retained<AnyObject>> {
    let obj = AllocFamily::send_message(cls, sel1, ());
    InitFamily::send_message(obj, sel2, ())
}

#[no_mangle]
unsafe fn handle_alloc_release(cls: &AnyClass, sel: Sel) {
    let _obj: Allocated<AnyObject> = AllocFamily::send_message(cls, sel, ());
}

#[no_mangle]
unsafe fn handle_alloc_init_release(cls: &AnyClass, sel1: Sel, sel2: Sel) {
    let obj = AllocFamily::send_message(cls, sel1, ());
    let obj: Option<Retained<AnyObject>> = InitFamily::send_message(obj, sel2, ());
    let _obj = obj.unwrap_unchecked();
}

#[no_mangle]
unsafe fn handle_copy(obj: &AnyObject, sel: Sel) -> Option<Retained<AnyObject>> {
    CopyFamily::send_message(obj, sel, ())
}

#[no_mangle]
unsafe fn handle_copy_fallible(obj: &AnyObject, sel: Sel) -> Retained<AnyObject> {
    CopyFamily::send_message(obj, sel, ())
}

#[no_mangle]
unsafe fn handle_mutable_copy(obj: &AnyObject, sel: Sel) -> Option<Retained<AnyObject>> {
    MutableCopyFamily::send_message(obj, sel, ())
}

#[no_mangle]
unsafe fn handle_mutable_copy_fallible(obj: &AnyObject, sel: Sel) -> Retained<AnyObject> {
    MutableCopyFamily::send_message(obj, sel, ())
}

#[no_mangle]
unsafe fn handle_autoreleased(obj: &AnyObject, sel: Sel) -> Option<Retained<AnyObject>> {
    NoneFamily::send_message(obj, sel, ())
}

// TODO: The optimization does not happen here on aarch64, fix this!
#[no_mangle]
unsafe fn handle_autoreleased_with_arg(
    obj: &AnyObject,
    sel: Sel,
    arg: u8,
) -> Option<Retained<AnyObject>> {
    NoneFamily::send_message(obj, sel, (arg,))
}

#[no_mangle]
unsafe fn handle_autoreleased_fallible(obj: &AnyObject, sel: Sel) -> Retained<AnyObject> {
    NoneFamily::send_message(obj, sel, ())
}

// TODO: The optimization does not happen here, fix this!
#[no_mangle]
unsafe fn handle_with_out_param(
    obj: &AnyObject,
    sel: Sel,
    param: &mut Retained<AnyObject>,
) -> Option<Retained<AnyObject>> {
    NoneFamily::send_message(obj, sel, (param,))
}
