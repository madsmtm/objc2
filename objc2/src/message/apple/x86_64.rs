use core::mem;
use objc_sys::{
    objc_msgSend, objc_msgSendSuper, objc_msgSendSuper_stret, objc_msgSend_fp2ret,
    objc_msgSend_fpret, objc_msgSend_stret,
};

use super::MsgSendFn;
use crate::runtime::Imp;
use crate::{Encode, Encoding};

/// If the size of an object is larger than two eightbytes, it has class
/// MEMORY. If the type has class MEMORY, then the caller provides space for
/// the return value and passes the address of this storage.
///
/// <http://people.freebsd.org/~obrien/amd64-elf-abi.pdf>
unsafe impl<T: Encode> MsgSendFn for T {
    // TODO: Should we use objc_msgSend_fpret and objc_msgSend_fp2ret ?
    const MSG_SEND: Imp = {
        // See lines 156 to 172 in:
        // https://opensource.apple.com/source/objc4/objc4-818.2/runtime/message.h.auto.html
        if let Encoding::LongDouble = T::ENCODING {
            objc_msgSend_fpret
        } else if let Encoding::LongDoubleComplex = T::ENCODING {
            objc_msgSend_fp2ret
        } else if mem::size_of::<T>() <= 16 {
            objc_msgSend
        } else {
            objc_msgSend_stret
        }
    };
    const MSG_SEND_SUPER: Imp = {
        if mem::size_of::<T>() <= 16 {
            objc_msgSendSuper
        } else {
            objc_msgSendSuper_stret
        }
    };
}
