use objc2::{class, msg_send};

fn main() {
    unsafe {
        let cls = class!(NSObject);
        let _obj = msg_send![cls, new];
    }
}
