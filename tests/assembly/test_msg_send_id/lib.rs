//! Test assembly output of `msg_send_id!` internals.
use objc2::__macro_helpers::{Assert, MsgSendId};
use objc2::rc::{Id, Shared};
use objc2::runtime::{Class, Object, Sel};

#[no_mangle]
unsafe fn handle_alloc(obj: &Class, sel: Sel) -> Option<Id<Object, Shared>> {
    <Assert<false, true, false, false>>::send_message_id(obj, sel, ()).unwrap()
}

#[no_mangle]
unsafe fn handle_init(obj: Option<Id<Object, Shared>>, sel: Sel) -> Option<Id<Object, Shared>> {
    <Assert<false, false, true, false>>::send_message_id(obj, sel, ()).unwrap()
}

#[no_mangle]
unsafe fn handle_alloc_init(obj: &Class, sel1: Sel, sel2: Sel) -> Option<Id<Object, Shared>> {
    let obj = <Assert<false, true, false, false>>::send_message_id(obj, sel1, ()).unwrap();
    <Assert<false, false, true, false>>::send_message_id(obj, sel2, ()).unwrap()
}

#[no_mangle]
unsafe fn handle_alloc_release(cls: &Class, sel: Sel) {
    let _obj: Id<Object, Shared> =
        <Assert<false, true, false, false>>::send_message_id(cls, sel, ())
            .unwrap()
            .unwrap_unchecked();
}

#[no_mangle]
unsafe fn handle_alloc_init_release(cls: &Class, sel1: Sel, sel2: Sel) {
    let obj = <Assert<false, true, false, false>>::send_message_id(cls, sel1, ()).unwrap();
    let _obj: Id<Object, Shared> =
        <Assert<false, false, true, false>>::send_message_id(obj, sel2, ())
            .unwrap()
            .unwrap_unchecked();
}

#[no_mangle]
unsafe fn handle_copy(obj: &Object, sel: Sel) -> Option<Id<Object, Shared>> {
    <Assert<false, false, false, true>>::send_message_id(obj, sel, ()).unwrap()
}

#[no_mangle]
unsafe fn handle_autoreleased(obj: &Object, sel: Sel) -> Option<Id<Object, Shared>> {
    <Assert<false, false, false, false>>::send_message_id(obj, sel, ()).unwrap()
}
