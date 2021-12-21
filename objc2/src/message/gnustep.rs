use core::mem;

use super::{conditional_try, MessageArguments, MessageError};
use crate::encode::Encode;
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
    // If `receiver` is NULL, objc_msg_lookup will return a standard C-method
    // taking two arguments, the receiver and the selector. Transmuting and
    // calling such a function with multiple parameters is UB, so instead we
    // just return NULL directly.
    if receiver.is_null() {
        // SAFETY: Caller guarantees that messages to NULL-receivers only
        // return pointers, and a mem::zeroed pointer is just a NULL-pointer.
        return unsafe { mem::zeroed() };
    }

    let sel_ptr = sel.as_ptr() as *const _;
    let msg_send_fn = unsafe { ffi::objc_msg_lookup(receiver as *mut _, sel_ptr) };
    let msg_send_fn = msg_send_fn.expect("Null IMP");
    unsafe { conditional_try(|| A::__invoke(msg_send_fn, receiver, sel, args)) }
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
    if receiver.is_null() {
        // SAFETY: Same as in `send_unverified`.
        return unsafe { mem::zeroed() };
    }

    let sup = ffi::objc_super {
        receiver: receiver as *mut _,
        super_class: superclass as *const Class as *const _,
    };
    let sel_ptr = sel.as_ptr() as *const _;
    let msg_send_fn = unsafe { ffi::objc_msg_lookup_super(&sup, sel_ptr) };
    let msg_send_fn = msg_send_fn.expect("Null IMP");
    unsafe { conditional_try(|| A::__invoke(msg_send_fn, receiver, sel, args)) }
}
