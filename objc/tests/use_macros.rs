#![cfg(any(target_os = "macos", target_os = "ios"))]

extern crate objc;

use objc::runtime::Object;
use objc::{class, msg_send, sel};

#[test]
fn use_class_and_msg_send() {
    unsafe {
        let cls = class!(NSObject);
        let obj: *mut Object = msg_send![cls, new];
        let _hash: usize = msg_send![obj, hash];
        let _: () = msg_send![obj, release];
    }
}

#[test]
fn use_sel() {
    let _sel = sel!(description);
    let _sel = sel!(setObject:forKey:);
}
