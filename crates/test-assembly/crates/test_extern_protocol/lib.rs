//! Test output of `extern_protocol!` as well as `ProtocolObject<dyn T>`.
#![cfg(target_vendor = "apple")]
use core::mem::ManuallyDrop;

use objc2::rc::Retained;
use objc2::runtime::{AnyProtocol, ProtocolObject};
use objc2::{extern_protocol, ProtocolType};

extern_protocol!(
    #[allow(clippy::missing_safety_doc)]
    unsafe trait MyProtocol {
        #[allow(non_snake_case)]
        #[method(aMethod)]
        fn aMethod(&self);
    }

    unsafe impl ProtocolType for dyn MyProtocol {}
);

#[no_mangle]
unsafe fn get_protocol() -> &'static AnyProtocol {
    <dyn MyProtocol>::protocol().unwrap_unchecked()
}

#[no_mangle]
fn dyn_call(obj: &ProtocolObject<dyn MyProtocol>) {
    obj.aMethod()
}

#[no_mangle]
fn dyn_consume(obj: ManuallyDrop<Retained<ProtocolObject<dyn MyProtocol>>>) {
    obj.aMethod();
    // Use ManuallyDrop to prevent trying to handle the case where `aMethod`
    // unwinds.
    let _ = ManuallyDrop::into_inner(obj);
}
