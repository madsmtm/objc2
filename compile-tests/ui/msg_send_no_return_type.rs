// Test that forgetting to annotate the return type fails
// See https://github.com/SSheldon/rust-objc/issues/62
use objc2::{class, msg_send};

fn main() {
    unsafe {
        let cls = class!(NSObject);
        msg_send![cls, new];
        //~^ ERROR type annotations needed
    }
}
