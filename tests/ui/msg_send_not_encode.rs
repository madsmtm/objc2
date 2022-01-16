//! Test that return types that are not `Encode` are not accepted.
use objc2::{class, msg_send};

fn main() {
    unsafe {
        let cls = class!(NSData);
        let _: Vec<u8> = msg_send![cls, new];
    }
}
