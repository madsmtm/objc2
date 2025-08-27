//! Test that out parameters are handled correctly.
use objc2::__macros::{MsgSend, NoneFamily};
use objc2::rc::Retained;
use objc2::runtime::{NSObject, Sel};

#[export_name = "fn1_nonnull_nonnull"]
unsafe fn nonnull_nonnull(obj: &NSObject, sel: Sel, param: &mut Retained<NSObject>) -> usize {
    NoneFamily::send_message(obj, sel, (param,))
}

#[export_name = "fn2_null_nonnull"]
unsafe fn null_nonnull(obj: &NSObject, sel: Sel, param: Option<&mut Retained<NSObject>>) -> usize {
    NoneFamily::send_message(obj, sel, (param,))
}

#[export_name = "fn3_nonnull_null"]
unsafe fn nonnull_null(obj: &NSObject, sel: Sel, param: &mut Option<Retained<NSObject>>) -> usize {
    NoneFamily::send_message(obj, sel, (param,))
}

#[export_name = "fn4_null_null"]
unsafe fn null_null(
    obj: &NSObject,
    sel: Sel,
    param: Option<&mut Option<Retained<NSObject>>>,
) -> usize {
    NoneFamily::send_message(obj, sel, (param,))
}

#[export_name = "fn5_two_nonnull_nonnull"]
unsafe fn two_nonnull_nonnull(
    obj: &NSObject,
    sel: Sel,
    param1: &mut Retained<NSObject>,
    param2: &mut Retained<NSObject>,
) -> usize {
    NoneFamily::send_message(obj, sel, (param1, param2))
}

//
// Calling in specific ways that the optimizer should be able to recognize
//

// These should fully avoid any extra `retain/release`
#[export_name = "fn6_call_with_none1"]
unsafe fn call_with_none1(obj: &NSObject, sel: Sel) -> usize {
    null_nonnull(obj, sel, None)
}
#[export_name = "fn6_call_with_none2"]
unsafe fn call_with_none2(obj: &NSObject, sel: Sel) -> usize {
    null_null(obj, sel, None)
}

type Res = (usize, Option<Retained<NSObject>>);

// These should only need a `retain`
#[export_name = "fn6_call_with_none3"]
unsafe fn call_with_none3(obj: &NSObject, sel: Sel) -> Res {
    let mut param = None;
    let res = nonnull_null(obj, sel, &mut param);
    (res, param)
}
#[export_name = "fn6_call_with_none4"]
unsafe fn call_with_none4(obj: &NSObject, sel: Sel) -> Res {
    let mut param = None;
    let res = null_null(obj, sel, Some(&mut param));
    (res, param)
}

// These should need `retain/release`, but not have any branches
#[export_name = "fn7_call_with_some1"]
unsafe fn call_with_some1(obj: &NSObject, sel: Sel, mut param: Retained<NSObject>) -> Res {
    let res = null_nonnull(obj, sel, Some(&mut param));
    (res, Some(param))
}
#[export_name = "fn7_call_with_some2"]
unsafe fn call_with_some2(obj: &NSObject, sel: Sel, param: Retained<NSObject>) -> Res {
    let mut param = Some(param);
    let res = nonnull_null(obj, sel, &mut param);
    (res, param)
}
#[export_name = "fn7_call_with_some3"]
unsafe fn call_with_some3(obj: &NSObject, sel: Sel, param: Retained<NSObject>) -> Res {
    let mut param = Some(param);
    let res = null_null(obj, sel, Some(&mut param));
    (res, param)
}
