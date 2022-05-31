//! Test that types that are not `Encode` are not accepted.
use objc2::{class, msg_send};

fn main() {
    let cls = class!(NSData);
    unsafe {
        let _: Vec<u8> = msg_send![cls, new];

        let x: Vec<u8>;
        let _: () = msg_send![cls, newWith: x];
    }
}
