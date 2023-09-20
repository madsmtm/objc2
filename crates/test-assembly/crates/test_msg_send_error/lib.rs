//! Test that error parameters are handled correctly.
use objc2::__macro_helpers::{Alloc, CopyOrMutCopy, Init, MsgSendId, New, Other};
use objc2::rc::{Allocated, Id};
use objc2::runtime::{AnyClass, AnyObject, MessageReceiver, Sel};

type Result<T> = std::result::Result<T, Id<AnyObject>>;

#[no_mangle]
unsafe fn error_bool(obj: &AnyObject, sel: Sel, param: u32) -> Result<()> {
    MessageReceiver::__send_message_error(obj, sel, (param,))
}

#[no_mangle]
unsafe fn error_new(cls: &AnyClass, sel: Sel) -> Result<Id<AnyObject>> {
    New::send_message_id_error(cls, sel, ())
}

#[no_mangle]
unsafe fn error_alloc(cls: &AnyClass, sel: Sel) -> Result<Allocated<AnyObject>> {
    Alloc::send_message_id_error(cls, sel, ())
}

#[no_mangle]
unsafe fn error_init(obj: Option<Allocated<AnyObject>>, sel: Sel) -> Result<Id<AnyObject>> {
    Init::send_message_id_error(obj, sel, ())
}

#[no_mangle]
unsafe fn error_copy(obj: &AnyObject, sel: Sel) -> Result<Id<AnyObject>> {
    CopyOrMutCopy::send_message_id_error(obj, sel, ())
}

#[no_mangle]
unsafe fn error_autoreleased(obj: &AnyObject, sel: Sel) -> Result<Id<AnyObject>> {
    Other::send_message_id_error(obj, sel, ())
}
