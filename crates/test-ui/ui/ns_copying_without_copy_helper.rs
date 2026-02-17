use objc2::{extern_class, extern_conformance};
use objc2_foundation::{NSCopying, NSMutableCopying, NSObject};

extern_class!(
    #[unsafe(super(NSObject))]
    struct MyObj;
);

extern_conformance!(
    unsafe impl NSCopying for MyObj {}
);
extern_conformance!(
    unsafe impl NSMutableCopying for MyObj {}
);

#[allow(unreachable_code)]
fn main() {
    let obj: &MyObj = unimplemented!();
    let _ = obj.copy();
    let _ = obj.mutableCopy();
}
