use objc2::{extern_class, ClassType};
use objc2_foundation::{NSCopying, NSMutableCopying, NSObject};

extern_class!(
    struct MyObj;

    unsafe impl ClassType for MyObj {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for MyObj {}
unsafe impl NSMutableCopying for MyObj {}

#[allow(unreachable_code)]
fn main() {
    let obj: &MyObj = unimplemented!();
    let _ = obj.copy();
    let _ = obj.mutableCopy();
}
