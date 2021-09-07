use core::mem;
use objc_sys::{
    objc_msgSend, objc_msgSendSuper, objc_msgSendSuper_stret, objc_msgSend_fpret,
    objc_msgSend_stret,
};

use super::MsgSendFn;
use crate::runtime::Imp;
use crate::{Encode, Encoding};

/// Structures 1 or 2 bytes in size are placed in EAX.
/// Structures 4 or 8 bytes in size are placed in: EAX and EDX.
/// Structures of other sizes are placed at the address supplied by the caller.
///
/// <https://developer.apple.com/library/mac/documentation/DeveloperTools/Conceptual/LowLevelABI/130-IA-32_Function_Calling_Conventions/IA32.html>
impl<T: Encode> MsgSendFn for T {
    const MSG_SEND: Imp = {
        if let Encoding::Float | Encoding::Double = T::ENCODING {
            objc_msgSend_fpret
        } else if let 0 | 1 | 2 | 4 | 8 = mem::size_of::<T>() {
            objc_msgSend
        } else {
            objc_msgSend_stret
        }
    };
    const MSG_SEND_SUPER: Imp = {
        if let 0 | 1 | 2 | 4 | 8 = mem::size_of::<T>() {
            objc_msgSendSuper
        } else {
            objc_msgSendSuper_stret
        }
    };
}
