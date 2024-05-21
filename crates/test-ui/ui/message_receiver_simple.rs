//! Test that `MessageReceiver` doesn't use our more fancy (but private)
//! conversion traits.
use objc2::runtime::{MessageReceiver, NSObject};
use objc2::sel;

fn main() {
    let obj = NSObject::new();

    // Receiver `&Retained` not allowed
    let _: usize = unsafe { MessageReceiver::send_message(&obj, sel!(hash), ()) };

    // No `bool` argument conversion
    let _: () = unsafe { MessageReceiver::send_message(&*obj, sel!(hash:), (true,)) };

    // No `bool` return conversion
    let _: bool = unsafe { MessageReceiver::send_message(&*obj, sel!(hash), ()) };
}
