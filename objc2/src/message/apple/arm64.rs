use objc2_sys::{objc_msgSend, objc_msgSendSuper};

use super::MsgSendFn;
use crate::runtime::Imp;
use crate::Encode;

/// `objc_msgSend_stret` is not even available in arm64.
///
/// <https://twitter.com/gparker/status/378079715824660480>
impl<T: Encode> MsgSendFn for T {
    const MSG_SEND: Imp = objc_msgSend;
    const MSG_SEND_SUPER: Imp = objc_msgSendSuper;
}
