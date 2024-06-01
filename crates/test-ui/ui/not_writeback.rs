use objc2::msg_send;
use objc2::rc::Retained;
use objc2::runtime::NSObject;

fn main() {
    let obj = &NSObject::new();

    let _: &mut Retained<NSObject> = unsafe { msg_send![obj, a] };

    let param: Retained<NSObject> = NSObject::new();
    let _: () = unsafe { msg_send![obj, a: param] };

    let param: Retained<NSObject> = NSObject::new();
    let _: () = unsafe { msg_send![obj, a: &param] };

    let param: Retained<NSObject> = NSObject::new();
    let _: () = unsafe { msg_send![obj, a: Some(&param)] };

    let param: *mut Retained<NSObject> = std::ptr::null_mut();
    let _: () = unsafe { msg_send![obj, a: param] };

    let mut param = NSObject::new();
    let mut param = &mut param;
    let param: &mut &mut Retained<NSObject> = &mut param;
    let _: () = unsafe { msg_send![obj, a: param] };
}
