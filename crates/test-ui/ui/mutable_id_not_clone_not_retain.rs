use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{extern_class, msg_send_id, mutability, ClassType};

extern_class!(
    struct NSMutableObject;

    unsafe impl ClassType for NSMutableObject {
        type Super = NSObject;
        type Mutability = mutability::Mutable;
        const NAME: &'static str = "NSObject";
    }
);

fn main() {
    let obj: Retained<NSMutableObject> = unsafe { msg_send_id![NSMutableObject::class(), new] };
    let _ = obj.clone();
    let _ = obj.retain();
}
