use objc2::runtime::Object;
use objc2::{class, msg_send, sel};

#[cfg(gnustep)]
#[test]
fn ensure_linkage() {
    unsafe { objc2::__gnustep_hack::get_class_to_force_linkage() };
}

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
