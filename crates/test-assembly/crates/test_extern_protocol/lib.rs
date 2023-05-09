//! Test output of `extern_protocol!` as well as `ProtocolObject<dyn T>`.
#![cfg(feature = "apple")]
use core::mem::ManuallyDrop;

use objc2::rc::Id;
use objc2::runtime::{Protocol, ProtocolObject};
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
unsafe fn get_protocol() -> &'static Protocol {
    <dyn MyProtocol>::protocol().unwrap_unchecked()
}

#[no_mangle]
fn dyn_call(obj: &ProtocolObject<dyn MyProtocol>) {
    obj.aMethod()
}

#[no_mangle]
fn dyn_consume(obj: ManuallyDrop<Id<ProtocolObject<dyn MyProtocol>>>) {
    obj.aMethod();
    // Use ManuallyDrop to prevent trying to handle the case where `aMethod`
    // unwinds.
    let _ = ManuallyDrop::into_inner(obj);
}
