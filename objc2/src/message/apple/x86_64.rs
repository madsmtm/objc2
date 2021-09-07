use core::mem;
use objc2_sys::{objc_msgSend, objc_msgSendSuper, objc_msgSendSuper_stret, objc_msgSend_stret};

use super::MsgSendFn;
use crate::runtime::Imp;
use crate::Encode;

/// If the size of an object is larger than two eightbytes, it has class
/// MEMORY. If the type has class MEMORY, then the caller provides space for
/// the return value and passes the address of this storage.
///
/// <http://people.freebsd.org/~obrien/amd64-elf-abi.pdf>
impl<T: Encode> MsgSendFn for T {
    // TODO: Should we use objc_msgSend_fpret and objc_msgSend_fp2ret ?
    const MSG_SEND: Imp = {
        if mem::size_of::<T>() <= 16 {
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
