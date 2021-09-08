use core::mem;
use objc2_sys::{objc_msg_lookup, objc_msg_lookup_super, objc_super};

use super::{Encode, Message, MessageArguments, MessageError};
use crate::runtime::{Class, Object, Sel};

pub unsafe fn send_unverified<T, A, R>(obj: *const T, sel: Sel, args: A) -> Result<R, MessageError>
where
    T: Message,
    A: MessageArguments,
    R: Encode,
{
    if obj.is_null() {
        return mem::zeroed();
    }

    let receiver = obj as *mut T as *mut Object;
    let msg_send_fn = objc_msg_lookup(receiver as *mut _, sel.as_ptr() as *const _);
    objc_try!({ A::invoke(msg_send_fn.expect("Null IMP"), receiver, sel, args) })
}

pub unsafe fn send_super_unverified<T, A, R>(
    obj: *const T,
    superclass: &Class,
    sel: Sel,
    args: A,
) -> Result<R, MessageError>
where
    T: Message,
    A: MessageArguments,
    R: Encode,
{
    let receiver = obj as *mut T as *mut Object;
    let sup = objc_super {
        receiver: receiver as *mut _,
        super_class: superclass as *const Class as *const _,
    };
    let msg_send_fn = objc_msg_lookup_super(&sup, sel.as_ptr() as *const _);
    objc_try!({ A::invoke(msg_send_fn.expect("Null IMP"), receiver, sel, args) })
}
