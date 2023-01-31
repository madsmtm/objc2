//! Test that error parameters are handled correctly.
use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::{Class, Object, Sel};
use objc2::MessageReceiver;
use objc2::__macro_helpers::{Alloc, CopyOrMutCopy, Init, MsgSendId, New, Other};

type Result<T> = std::result::Result<T, Id<Object, Shared>>;

#[no_mangle]
unsafe fn error_bool(obj: &Object, sel: Sel, param: u32) -> Result<()> {
    MessageReceiver::__send_message_error(obj, sel, (param,))
}

#[no_mangle]
unsafe fn error_new(cls: &Class, sel: Sel) -> Result<Id<Object, Shared>> {
    New::send_message_id_error(cls, sel, ())
}

#[no_mangle]
unsafe fn error_alloc(cls: &Class, sel: Sel) -> Result<Allocated<Object>> {
    Alloc::send_message_id_error(cls, sel, ())
}

#[no_mangle]
unsafe fn error_init(obj: Option<Allocated<Object>>, sel: Sel) -> Result<Id<Object, Shared>> {
    Init::send_message_id_error(obj, sel, ())
}

#[no_mangle]
unsafe fn error_copy(obj: &Object, sel: Sel) -> Result<Id<Object, Shared>> {
    CopyOrMutCopy::send_message_id_error(obj, sel, ())
}

#[no_mangle]
unsafe fn error_autoreleased(obj: &Object, sel: Sel) -> Result<Id<Object, Shared>> {
    Other::send_message_id_error(obj, sel, ())
}
