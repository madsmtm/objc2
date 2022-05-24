use super::{conditional_try, Encode, MessageArguments, MessageError};
use crate::ffi;
use crate::runtime::{Class, Object, Sel};

pub(crate) unsafe fn send_unverified<A, R>(
    receiver: *mut Object,
    sel: Sel,
    args: A,
) -> Result<R, MessageError>
where
    A: MessageArguments,
    R: Encode,
{
    unsafe {
        conditional_try(|| ffi::miri::custom_msg_send(receiver.cast(), sel.as_ptr().cast(), args))
    }
}

pub(crate) unsafe fn send_super_unverified<A, R>(
    receiver: *mut Object,
    superclass: &Class,
    sel: Sel,
    args: A,
) -> Result<R, MessageError>
where
    A: MessageArguments,
    R: Encode,
{
    unsafe {
        conditional_try(|| {
            ffi::miri::custom_msg_send_super(
                receiver.cast(),
                superclass.as_ptr(),
                sel.as_ptr().cast(),
                args,
            )
        })
    }
}
