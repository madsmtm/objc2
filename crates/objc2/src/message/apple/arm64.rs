use super::MsgSendFn;
use crate::encode::__unstable::EncodeReturn;
use crate::ffi;
use crate::runtime::Imp;

/// `objc_msgSend_stret` is not even available in arm64.
///
/// <https://twitter.com/gparker/status/378079715824660480>
unsafe impl<T: EncodeReturn> MsgSendFn for T {
    const MSG_SEND: Imp = ffi::objc_msgSend;
    const MSG_SEND_SUPER: Imp = ffi::objc_msgSendSuper;
}
