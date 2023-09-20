//! Test that out parameters are handled correctly.
use objc2::rc::Id;
use objc2::runtime::{MessageReceiver, NSObject, Sel};

#[no_mangle]
unsafe fn nonnull_nonnull(obj: &NSObject, sel: Sel, param: &mut Id<NSObject>) -> usize {
    MessageReceiver::send_message(obj, sel, (param,))
}

#[no_mangle]
unsafe fn null_nonnull(obj: &NSObject, sel: Sel, param: Option<&mut Id<NSObject>>) -> usize {
    MessageReceiver::send_message(obj, sel, (param,))
}

#[no_mangle]
unsafe fn nonnull_null(obj: &NSObject, sel: Sel, param: &mut Option<Id<NSObject>>) -> usize {
    MessageReceiver::send_message(obj, sel, (param,))
}

#[no_mangle]
unsafe fn null_null(obj: &NSObject, sel: Sel, param: Option<&mut Option<Id<NSObject>>>) -> usize {
    MessageReceiver::send_message(obj, sel, (param,))
}

#[no_mangle]
unsafe fn two_nonnull_nonnull(
    obj: &NSObject,
    sel: Sel,
    param1: &mut Id<NSObject>,
    param2: &mut Id<NSObject>,
) -> usize {
    MessageReceiver::send_message(obj, sel, (param1, param2))
}

//
// Calling in specific ways that the optimizer should be able to recognize
//

// These should fully avoid any extra `retain/release`
#[no_mangle]
unsafe fn call_with_none1(obj: &NSObject, sel: Sel) -> usize {
    null_nonnull(obj, sel, None)
}
#[no_mangle]
unsafe fn call_with_none2(obj: &NSObject, sel: Sel) -> usize {
    null_null(obj, sel, None)
}

type Res = (usize, Option<Id<NSObject>>);

// These should only need a `retain`
#[no_mangle]
unsafe fn call_with_none3(obj: &NSObject, sel: Sel) -> Res {
    let mut param = None;
    let res = nonnull_null(obj, sel, &mut param);
    (res, param)
}
#[no_mangle]
unsafe fn call_with_none4(obj: &NSObject, sel: Sel) -> Res {
    let mut param = None;
    let res = null_null(obj, sel, Some(&mut param));
    (res, param)
}

// These should need `retain/release`, but not have any branches
#[no_mangle]
unsafe fn call_with_some1(obj: &NSObject, sel: Sel, mut param: Id<NSObject>) -> Res {
    let res = null_nonnull(obj, sel, Some(&mut param));
    (res, Some(param))
}
#[no_mangle]
unsafe fn call_with_some2(obj: &NSObject, sel: Sel, param: Id<NSObject>) -> Res {
    let mut param = Some(param);
    let res = nonnull_null(obj, sel, &mut param);
    (res, param)
}
#[no_mangle]
unsafe fn call_with_some3(obj: &NSObject, sel: Sel, param: Id<NSObject>) -> Res {
    let mut param = Some(param);
    let res = null_null(obj, sel, Some(&mut param));
    (res, param)
}
