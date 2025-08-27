//! Test assembly output of `msg_send!` internals.
use objc2::__macros::{
    AllocFamily, CopyFamily, InitFamily, MsgSend, MutableCopyFamily, NewFamily, NoneFamily,
};
use objc2::rc::{Allocated, Retained};
use objc2::runtime::{AnyClass, AnyObject, Sel};

#[export_name = "fn01_handle_new"]
unsafe fn handle_new(cls: &AnyClass, sel: Sel) -> Option<Retained<AnyObject>> {
    NewFamily::send_message(cls, sel, ())
}

#[export_name = "fn02_handle_new_fallible"]
unsafe fn handle_new_fallible(cls: &AnyClass, sel: Sel) -> Retained<AnyObject> {
    NewFamily::send_message(cls, sel, ())
}

#[export_name = "fn03_handle_alloc"]
unsafe fn handle_alloc(cls: &AnyClass, sel: Sel) -> Allocated<AnyObject> {
    AllocFamily::send_message(cls, sel, ())
}

#[export_name = "fn04_handle_init"]
unsafe fn handle_init(obj: Allocated<AnyObject>, sel: Sel) -> Option<Retained<AnyObject>> {
    InitFamily::send_message(obj, sel, ())
}

#[export_name = "fn05_handle_init_fallible"]
unsafe fn handle_init_fallible(obj: Allocated<AnyObject>, sel: Sel) -> Retained<AnyObject> {
    InitFamily::send_message(obj, sel, ())
}

#[export_name = "fn06_handle_alloc_init"]
unsafe fn handle_alloc_init(cls: &AnyClass, sel1: Sel, sel2: Sel) -> Option<Retained<AnyObject>> {
    let obj = AllocFamily::send_message(cls, sel1, ());
    InitFamily::send_message(obj, sel2, ())
}

#[export_name = "fn07_handle_alloc_release"]
unsafe fn handle_alloc_release(cls: &AnyClass, sel: Sel) {
    let _obj: Allocated<AnyObject> = AllocFamily::send_message(cls, sel, ());
}

#[export_name = "fn08_handle_alloc_init_release"]
unsafe fn handle_alloc_init_release(cls: &AnyClass, sel1: Sel, sel2: Sel) {
    let obj = AllocFamily::send_message(cls, sel1, ());
    let obj: Option<Retained<AnyObject>> = InitFamily::send_message(obj, sel2, ());
    let _obj = obj.unwrap_unchecked();
}

#[export_name = "fn09_handle_copy"]
unsafe fn handle_copy(obj: &AnyObject, sel: Sel) -> Option<Retained<AnyObject>> {
    CopyFamily::send_message(obj, sel, ())
}

#[export_name = "fn10_handle_copy_fallible"]
unsafe fn handle_copy_fallible(obj: &AnyObject, sel: Sel) -> Retained<AnyObject> {
    CopyFamily::send_message(obj, sel, ())
}

#[export_name = "fn11_handle_mutable_copy"]
unsafe fn handle_mutable_copy(obj: &AnyObject, sel: Sel) -> Option<Retained<AnyObject>> {
    MutableCopyFamily::send_message(obj, sel, ())
}

#[export_name = "fn12_handle_mutable_copy_fallible"]
unsafe fn handle_mutable_copy_fallible(obj: &AnyObject, sel: Sel) -> Retained<AnyObject> {
    MutableCopyFamily::send_message(obj, sel, ())
}

#[export_name = "fn13_handle_autoreleased"]
unsafe fn handle_autoreleased(obj: &AnyObject, sel: Sel) -> Option<Retained<AnyObject>> {
    NoneFamily::send_message(obj, sel, ())
}

// TODO: The optimization does not happen here on aarch64, fix this!
#[export_name = "fn14_handle_autoreleased_with_arg"]
unsafe fn handle_autoreleased_with_arg(
    obj: &AnyObject,
    sel: Sel,
    arg: u8,
) -> Option<Retained<AnyObject>> {
    NoneFamily::send_message(obj, sel, (arg,))
}

#[export_name = "fn15_handle_autoreleased_fallible"]
unsafe fn handle_autoreleased_fallible(obj: &AnyObject, sel: Sel) -> Retained<AnyObject> {
    NoneFamily::send_message(obj, sel, ())
}

// TODO: The optimization does not happen here, fix this!
#[export_name = "fn16_handle_with_out_param"]
unsafe fn handle_with_out_param(
    obj: &AnyObject,
    sel: Sel,
    param: &mut Retained<AnyObject>,
) -> Option<Retained<AnyObject>> {
    NoneFamily::send_message(obj, sel, (param,))
}
