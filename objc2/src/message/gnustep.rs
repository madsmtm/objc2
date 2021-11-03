use core::mem;
use objc_sys::{objc_msg_lookup, objc_msg_lookup_super, objc_super};

use super::{conditional_try, Encode, MessageArguments, MessageError};
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
    if receiver.is_null() {
        return unsafe { mem::zeroed() };
    }

    let sel_ptr = sel.as_ptr() as *const _;
    let msg_send_fn = unsafe { objc_msg_lookup(receiver as *mut _, sel_ptr) };
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
    let sup = objc_super {
        receiver: receiver as *mut _,
        super_class: superclass as *const Class as *const _,
    };
    let sel_ptr = sel.as_ptr() as *const _;
    let msg_send_fn = unsafe { objc_msg_lookup_super(&sup, sel_ptr) };
    let msg_send_fn = msg_send_fn.expect("Null IMP");
    unsafe { conditional_try(|| A::__invoke(msg_send_fn, receiver, sel, args)) }
}
