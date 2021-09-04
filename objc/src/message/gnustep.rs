use core::mem;
use objc_sys::objc_super;

use super::{Encode, Message, MessageArguments, MessageError};
use crate::runtime::{Class, Imp, Object, Sel};

extern "C" {
    fn objc_msg_lookup(receiver: *mut Object, op: Sel) -> Imp;
    fn objc_msg_lookup_super(sup: *const objc_super, sel: Sel) -> Imp;
}

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
    let msg_send_fn = objc_msg_lookup(receiver, sel);
    objc_try!({ A::invoke(msg_send_fn, receiver, sel, args) })
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
    let msg_send_fn = objc_msg_lookup_super(&sup, sel);
    objc_try!({ A::invoke(msg_send_fn, receiver, sel, args) })
}
