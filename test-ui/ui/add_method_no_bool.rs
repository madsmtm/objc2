//! Ensure that methods taking/returning `bool` are not allowed.
//!
//! This is not sound without a conversion step.
use objc2::declare::ClassBuilder;
use objc2::runtime::{Object, Sel};
use objc2::sel;

extern "C" fn my_bool_taking_method(obj: &Object, sel: Sel, arg: bool) {}

extern "C" fn my_bool_returning_method(obj: &Object, sel: Sel) -> bool {
    true
}

fn add(builder: &mut ClassBuilder) {
    let method: unsafe extern "C" fn(_, _, _) -> _ = my_bool_taking_method;
    builder.add_method(sel!(myBoolTakingMethod:), method);
    let method: unsafe extern "C" fn(_, _) -> _ = my_bool_returning_method;
    builder.add_method(sel!(myBoolReturningMethod), method);
}

fn main() {}
