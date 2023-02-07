use objc2::rc::{Id, Owned};
use objc2::runtime::NSObject;
use objc2::msg_send;

fn main() {
    let obj = &NSObject::new();

    let _: &mut Id<NSObject> = unsafe { msg_send![obj, a] };

    let param: Id<NSObject, Owned> = NSObject::new();
    let _: () = unsafe { msg_send![obj, a: param] };

    let param: Id<NSObject, Owned> = NSObject::new();
    let _: () = unsafe { msg_send![obj, a: &param] };

    let param: Id<NSObject, Owned> = NSObject::new();
    let _: () = unsafe { msg_send![obj, a: Some(&param)] };

    let param: *mut Id<NSObject> = std::ptr::null_mut();
    let _: () = unsafe { msg_send![obj, a: param] };

    let mut param = NSObject::new();
    let mut param = &mut param;
    let param: &mut &mut Id<NSObject, Owned> = &mut param;
    let _: () = unsafe { msg_send![obj, a: param] };
}
