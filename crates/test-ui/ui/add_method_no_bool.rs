//! Ensure that methods taking/returning `bool` are not allowed.
//!
//! This is not sound without a conversion step.
use objc2::declare::ClassBuilder;
use objc2::runtime::{NSObject, Sel};
use objc2::sel;

extern "C" fn my_bool_taking_method(_obj: &NSObject, _sel: Sel, _arg: bool) {}

extern "C" fn my_bool_returning_method(_obj: &NSObject, _sel: Sel) -> bool {
    true
}

fn add(builder: &mut ClassBuilder) {
    let method: unsafe extern "C" fn(_, _, _) -> _ = my_bool_taking_method;
    builder.add_method(sel!(myBoolTakingMethod:), method);
    let method: unsafe extern "C" fn(_, _) -> _ = my_bool_returning_method;
    builder.add_method(sel!(myBoolReturningMethod), method);
}

fn main() {}
