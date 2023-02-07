//! Test assembly output of `msg_send_id!` internals.
use objc2::__macro_helpers::{Alloc, CopyOrMutCopy, Init, MsgSendId, New, Other};
use objc2::rc::{Allocated, Id};
use objc2::runtime::{Class, Object, Sel};

#[no_mangle]
unsafe fn handle_new(cls: &Class, sel: Sel) -> Option<Id<Object>> {
    New::send_message_id(cls, sel, ())
}

#[no_mangle]
unsafe fn handle_new_fallible(cls: &Class, sel: Sel) -> Id<Object> {
    New::send_message_id(cls, sel, ())
}

#[no_mangle]
unsafe fn handle_alloc(cls: &Class, sel: Sel) -> Option<Allocated<Object>> {
    Alloc::send_message_id(cls, sel, ())
}

#[no_mangle]
unsafe fn handle_alloc_fallible(cls: &Class, sel: Sel) -> Allocated<Object> {
    Alloc::send_message_id(cls, sel, ())
}

#[no_mangle]
unsafe fn handle_init(obj: Option<Allocated<Object>>, sel: Sel) -> Option<Id<Object>> {
    Init::send_message_id(obj, sel, ())
}

#[no_mangle]
unsafe fn handle_init_fallible(obj: Option<Allocated<Object>>, sel: Sel) -> Id<Object> {
    Init::send_message_id(obj, sel, ())
}

#[no_mangle]
unsafe fn handle_alloc_init(cls: &Class, sel1: Sel, sel2: Sel) -> Option<Id<Object>> {
    let obj = Alloc::send_message_id(cls, sel1, ());
    Init::send_message_id(obj, sel2, ())
}

#[no_mangle]
unsafe fn handle_alloc_release(cls: &Class, sel: Sel) {
    let obj: Option<Allocated<Object>> = Alloc::send_message_id(cls, sel, ());
    let _obj = obj.unwrap_unchecked();
}

#[no_mangle]
unsafe fn handle_alloc_init_release(cls: &Class, sel1: Sel, sel2: Sel) {
    let obj = Alloc::send_message_id(cls, sel1, ());
    let obj: Option<Id<Object>> = Init::send_message_id(obj, sel2, ());
    let _obj = obj.unwrap_unchecked();
}

#[no_mangle]
unsafe fn handle_copy(obj: &Object, sel: Sel) -> Option<Id<Object>> {
    CopyOrMutCopy::send_message_id(obj, sel, ())
}

#[no_mangle]
unsafe fn handle_copy_fallible(obj: &Object, sel: Sel) -> Id<Object> {
    CopyOrMutCopy::send_message_id(obj, sel, ())
}

#[no_mangle]
unsafe fn handle_autoreleased(obj: &Object, sel: Sel) -> Option<Id<Object>> {
    Other::send_message_id(obj, sel, ())
}

#[no_mangle]
unsafe fn handle_autoreleased_fallible(obj: &Object, sel: Sel) -> Id<Object> {
    Other::send_message_id(obj, sel, ())
}

// TODO: The optimization does not happen here, fix this!
#[no_mangle]
unsafe fn handle_with_out_param(
    obj: &Object,
    sel: Sel,
    param: &mut Id<Object>,
) -> Option<Id<Object>> {
    Other::send_message_id(obj, sel, (param,))
}
