//! Test assembly output of `msg_send_id!` internals.
use objc2::__macro_helpers::{MsgSendId, RetainSemantics};
use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::{Class, Object, Sel};

#[no_mangle]
unsafe fn handle_new(cls: &Class, sel: Sel) -> Option<Id<Object, Shared>> {
    <RetainSemantics<true, false, false, false>>::send_message_id(cls, sel, ())
}

#[no_mangle]
unsafe fn handle_new_fallible(cls: &Class, sel: Sel) -> Id<Object, Shared> {
    <RetainSemantics<true, false, false, false>>::send_message_id(cls, sel, ())
}

#[no_mangle]
unsafe fn handle_alloc(cls: &Class, sel: Sel) -> Option<Allocated<Object>> {
    <RetainSemantics<false, true, false, false>>::send_message_id(cls, sel, ())
}

#[no_mangle]
unsafe fn handle_alloc_fallible(cls: &Class, sel: Sel) -> Allocated<Object> {
    <RetainSemantics<false, true, false, false>>::send_message_id(cls, sel, ())
}

#[no_mangle]
unsafe fn handle_init(obj: Option<Allocated<Object>>, sel: Sel) -> Option<Id<Object, Shared>> {
    <RetainSemantics<false, false, true, false>>::send_message_id(obj, sel, ())
}

#[no_mangle]
unsafe fn handle_init_fallible(obj: Option<Allocated<Object>>, sel: Sel) -> Id<Object, Shared> {
    <RetainSemantics<false, false, true, false>>::send_message_id(obj, sel, ())
}

#[no_mangle]
unsafe fn handle_alloc_init(cls: &Class, sel1: Sel, sel2: Sel) -> Option<Id<Object, Shared>> {
    let obj = <RetainSemantics<false, true, false, false>>::send_message_id(cls, sel1, ());
    <RetainSemantics<false, false, true, false>>::send_message_id(obj, sel2, ())
}

#[no_mangle]
unsafe fn handle_alloc_release(cls: &Class, sel: Sel) {
    let obj: Option<Allocated<Object>> =
        <RetainSemantics<false, true, false, false>>::send_message_id(cls, sel, ());
    let _obj = obj.unwrap_unchecked();
}

#[no_mangle]
unsafe fn handle_alloc_init_release(cls: &Class, sel1: Sel, sel2: Sel) {
    let obj = <RetainSemantics<false, true, false, false>>::send_message_id(cls, sel1, ());
    let obj: Option<Id<Object, Shared>> =
        <RetainSemantics<false, false, true, false>>::send_message_id(obj, sel2, ());
    let _obj = obj.unwrap_unchecked();
}

#[no_mangle]
unsafe fn handle_copy(obj: &Object, sel: Sel) -> Option<Id<Object, Shared>> {
    <RetainSemantics<false, false, false, true>>::send_message_id(obj, sel, ())
}

#[no_mangle]
unsafe fn handle_copy_fallible(obj: &Object, sel: Sel) -> Id<Object, Shared> {
    <RetainSemantics<false, false, false, true>>::send_message_id(obj, sel, ())
}

#[no_mangle]
unsafe fn handle_autoreleased(obj: &Object, sel: Sel) -> Option<Id<Object, Shared>> {
    <RetainSemantics<false, false, false, false>>::send_message_id(obj, sel, ())
}

#[no_mangle]
unsafe fn handle_autoreleased_fallible(obj: &Object, sel: Sel) -> Id<Object, Shared> {
    <RetainSemantics<false, false, false, false>>::send_message_id(obj, sel, ())
}
