use objc2::rc::Id;
use objc2::runtime::NSObject;
use objc2::{extern_class, mutability, msg_send_id, ClassType};

extern_class!(
    struct NSMutableObject;

    unsafe impl ClassType for NSMutableObject {
        type Super = NSObject;
        type Mutability = mutability::Mutable;
        const NAME: &'static str = "NSObject";
    }
);

fn main() {
    let obj: Id<NSMutableObject> = unsafe { msg_send_id![NSMutableObject::class(), new] };
    let _ = obj.clone();
    let _ = obj.retain();
}
