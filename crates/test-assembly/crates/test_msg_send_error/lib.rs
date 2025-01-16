//! Test that error parameters are handled correctly.
use objc2::__macro_helpers::{Copy, Init, MsgSend, MsgSendRetained, MutableCopy, New, Other};
use objc2::rc::{Allocated, Retained};
use objc2::runtime::{AnyClass, AnyObject, NSObject, Sel};

type Result<T> = std::result::Result<T, Retained<NSObject>>;

#[no_mangle]
unsafe fn error_bool(obj: &AnyObject, sel: Sel, param: u32) -> Result<()> {
    MsgSend::send_message_error(obj, sel, (param,))
}

#[no_mangle]
unsafe fn error_new(cls: &AnyClass, sel: Sel) -> Result<Retained<AnyObject>> {
    New::send_message_retained_error(cls, sel, ())
}

// Note: Erroring allocation methods are intentionally not supported

#[no_mangle]
unsafe fn error_init(obj: Allocated<AnyObject>, sel: Sel) -> Result<Retained<AnyObject>> {
    Init::send_message_retained_error(obj, sel, ())
}

#[no_mangle]
unsafe fn error_copy(obj: &AnyObject, sel: Sel) -> Result<Retained<AnyObject>> {
    Copy::send_message_retained_error(obj, sel, ())
}

#[no_mangle]
unsafe fn error_mutable_copy(obj: &AnyObject, sel: Sel) -> Result<Retained<AnyObject>> {
    MutableCopy::send_message_retained_error(obj, sel, ())
}

#[no_mangle]
unsafe fn error_autoreleased(obj: &AnyObject, sel: Sel) -> Result<Retained<AnyObject>> {
    Other::send_message_retained_error(obj, sel, ())
}
