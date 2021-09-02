use core::mem;

use super::MsgSendFn;
use crate::runtime::Imp;
use crate::{Encode, Encoding};

// TODO: C-unwind
extern "C" {
    fn objc_msgSend();
    fn objc_msgSend_stret();

    fn objc_msgSendSuper();
    fn objc_msgSendSuper_stret();
}

/// Double-word sized fundamental data types don't use stret, but any
/// composite type larger than 4 bytes does.
///
/// <http://infocenter.arm.com/help/topic/com.arm.doc.ihi0042e/IHI0042E_aapcs.pdf>
impl<T: Encode> MsgSendFn for T {
    const MSG_SEND: Imp = {
        if let Encoding::LongLong | Encoding::ULongLong | Encoding::Double = T::ENCODING {
            objc_msgSend
        } else if mem::size_of::<T>() <= 4 {
            objc_msgSend
        } else {
            objc_msgSend_stret
        }
    };
    const MSG_SEND_SUPER: Imp = {
        if let Encoding::LongLong | Encoding::ULongLong | Encoding::Double = T::ENCODING {
            objc_msgSendSuper
        } else if mem::size_of::<T>() <= 4 {
            objc_msgSendSuper
        } else {
            objc_msgSendSuper_stret
        }
    };
}
