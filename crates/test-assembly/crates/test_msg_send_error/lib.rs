//! Test that error parameters are handled correctly.
use objc2::__macros::{
    CopyFamily, InitFamily, MsgSendError, MutableCopyFamily, NewFamily, NoneFamily,
};
use objc2::rc::{Allocated, Retained};
use objc2::runtime::{AnyClass, AnyObject, NSObject, Sel};

type Result<T> = std::result::Result<T, Retained<NSObject>>;

#[export_name = "fn1_error_bool"]
unsafe fn error_bool(obj: &AnyObject, sel: Sel, param: u32) -> Result<()> {
    NoneFamily::send_message_error(obj, sel, (param,))
}

#[export_name = "fn2_error_new"]
unsafe fn error_new(cls: &AnyClass, sel: Sel) -> Result<Retained<AnyObject>> {
    NewFamily::send_message_error(cls, sel, ())
}

// Note: Erroring allocation methods are intentionally not supported

#[export_name = "fn3_error_init"]
unsafe fn error_init(obj: Allocated<AnyObject>, sel: Sel) -> Result<Retained<AnyObject>> {
    InitFamily::send_message_error(obj, sel, ())
}

#[export_name = "fn4_error_copy"]
unsafe fn error_copy(obj: &AnyObject, sel: Sel) -> Result<Retained<AnyObject>> {
    CopyFamily::send_message_error(obj, sel, ())
}

#[export_name = "fn5_error_mutable_copy"]
unsafe fn error_mutable_copy(obj: &AnyObject, sel: Sel) -> Result<Retained<AnyObject>> {
    MutableCopyFamily::send_message_error(obj, sel, ())
}

#[export_name = "fn6_error_autoreleased"]
unsafe fn error_autoreleased(obj: &AnyObject, sel: Sel) -> Result<Retained<AnyObject>> {
    NoneFamily::send_message_error(obj, sel, ())
}
