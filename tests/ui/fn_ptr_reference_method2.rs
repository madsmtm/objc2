//! Extra test for `fn_ptr_reference_method`
//! (They fail at different compilation passes).
use objc2::{class, sel};
use objc2::declare::ClassBuilder;
use objc2::runtime::{Object, Sel};

extern "C" fn my_fn(_this: &Object, _cmd: Sel, _x: &Object) {}

fn main() {
    let mut builder = ClassBuilder::new("SomeTestClass", class!(NSObject)).unwrap();
    unsafe {
        builder.add_method(sel!(both:), my_fn as extern "C" fn(&Object, Sel, &Object));
    }
}
