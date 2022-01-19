use objc2::runtime::Object;
use objc2::{class, msg_send};

fn main() {
    // Get a class
    let cls = class!(NSObject);

    let obj: *mut Object = unsafe { msg_send![cls, new] };

    let x = unsafe { msg_send![obj, isEqual: obj] };
    if x {}
}
