use super::{conditional_try, Encode, MessageArguments, MessageError};
use crate::ffi;
use crate::runtime::{Class, Imp, Object, Sel};

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;
#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;
#[cfg(target_arch = "arm")]
#[path = "arm.rs"]
mod arch;
#[cfg(target_arch = "aarch64")]
#[path = "arm64.rs"]
mod arch;

/// On the above architectures we can statically find the correct method to
/// call from the return type, by looking at it's `Encode` implementation.
#[allow(clippy::missing_safety_doc)]
unsafe trait MsgSendFn: Encode {
    const MSG_SEND: Imp;
    const MSG_SEND_SUPER: Imp;
}

#[inline(always)]
pub(crate) unsafe fn send_unverified<A, R>(
    receiver: *mut Object,
    sel: Sel,
    args: A,
) -> Result<R, MessageError>
where
    A: MessageArguments,
    R: Encode,
{
    let msg_send_fn = R::MSG_SEND;
    unsafe { conditional_try(|| A::__invoke(msg_send_fn, receiver, sel, args)) }
}

#[inline]
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
    let sup = ffi::objc_super {
        receiver: receiver as *mut _,
        super_class: superclass as *const Class as *const _,
    };
    let receiver = &sup as *const ffi::objc_super as *mut Object;
    let msg_send_fn = R::MSG_SEND_SUPER;
    unsafe { conditional_try(|| A::__invoke(msg_send_fn, receiver, sel, args)) }
}
