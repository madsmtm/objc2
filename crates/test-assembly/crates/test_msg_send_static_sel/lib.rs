//! Test how static selectors work in relation to `msg_send!` and `msg_send_id!`
#![cfg(target_vendor = "apple")]
use objc2::rc::Retained;
use objc2::runtime::{AnyClass, AnyObject, Sel};
use objc2::{msg_send, msg_send_id, sel};

#[no_mangle]
unsafe fn handle_with_sel(obj: &AnyObject) -> *mut AnyObject {
    msg_send![obj, someSelector]
}

#[no_mangle]
unsafe fn handle_alloc_init(cls: &AnyClass) -> Retained<AnyObject> {
    msg_send_id![msg_send_id![cls, alloc], init]
}

#[allow(clippy::extra_unused_type_parameters)]
fn generic<T>() -> Sel {
    sel!(generic:selector:)
}

#[no_mangle]
unsafe fn use_generic(obj: &AnyObject) {
    let _: () = msg_send![obj, performSelector: generic::<i32>()];
    let _: () = msg_send![obj, performSelector: generic::<u32>()];
    let _: () = msg_send![obj, performSelector: generic::<f32>()];
}
