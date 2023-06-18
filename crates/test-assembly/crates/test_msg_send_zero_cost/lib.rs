//! Test that the inner part of msg_send! is inlined into an objc_msgSend
use core::mem;
use core::ptr;

use objc2::runtime::{AnyObject, Sel};
use objc2::MessageReceiver;

#[no_mangle]
unsafe fn handle(obj: &AnyObject, sel: Sel) -> *mut AnyObject {
    MessageReceiver::send_message(obj, sel, ())
}

// This will definitely not work, but is useful for making the assembly look
// closer to real-world.
#[no_mangle]
static SEL: [u8; 13] = *b"someSelector\0";
#[no_mangle]
static SEL_REF: Sel = unsafe { mem::transmute(SEL.as_ptr()) };

fn selector() -> Sel {
    unsafe { ptr::read_volatile(&SEL_REF) }
}

#[no_mangle]
unsafe fn handle_with_sel(obj: &AnyObject) -> *mut AnyObject {
    MessageReceiver::send_message(obj, selector(), ())
}
