//! Test how static selectors work in relation to `msg_send!`.
#![cfg(target_vendor = "apple")]
use objc2::rc::Retained;
use objc2::runtime::{AnyClass, AnyObject, Sel};
use objc2::{msg_send, sel};

#[export_name = "fn1_handle_with_sel"]
unsafe fn handle_with_sel(obj: &AnyObject) -> *mut AnyObject {
    msg_send![obj, someSelector]
}

#[export_name = "fn2_handle_alloc_init"]
unsafe fn handle_alloc_init(cls: &AnyClass) -> Retained<AnyObject> {
    msg_send![msg_send![cls, alloc], init]
}

#[allow(clippy::extra_unused_type_parameters)]
fn generic<T>() -> Sel {
    sel!(generic:selector:)
}

#[export_name = "fn3_use_generic"]
unsafe fn use_generic(obj: &AnyObject) {
    let _: () = msg_send![obj, performSelector: generic::<i32>()];
    let _: () = msg_send![obj, performSelector: generic::<u32>()];
    let _: () = msg_send![obj, performSelector: generic::<f32>()];
}
