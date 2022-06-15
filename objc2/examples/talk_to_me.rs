use objc2::ffi::NSUInteger;
use objc2::rc::{Id, Owned, Shared};
use objc2::runtime::Object;
use objc2::{class, msg_send, msg_send_id};
use std::ffi::c_void;

#[cfg(feature = "apple")] // Does not work on GNUStep
#[link(name = "AVFoundation", kind = "framework")]
extern "C" {}

// Only works on macOS >= 10.15 or iOS > 7.0
fn main() {
    let text = "Hello from Rust!";
    const UTF8_ENCODING: NSUInteger = 4;

    let string = unsafe { msg_send_id![class!(NSString), alloc] };
    let text_ptr: *const c_void = text.as_ptr().cast();
    let string: Id<Object, Shared> = unsafe {
        msg_send_id![
            string,
            initWithBytes: text_ptr,
            length: text.len(),
            encoding: UTF8_ENCODING,
        ]
    }
    .unwrap();

    let synthesizer: Id<Object, Owned> =
        unsafe { msg_send_id![class!(AVSpeechSynthesizer), new] }.unwrap();

    let utterance = unsafe { msg_send_id![class!(AVSpeechUtterance), alloc] };
    let utterance: Id<Object, Owned> =
        unsafe { msg_send_id![utterance, initWithString: &*string] }.unwrap();

    // let _: () = unsafe { msg_send![&utterance, setVolume: 90.0f32 };
    // let _: () = unsafe { msg_send![&utterance, setRate: 0.50f32 };
    // let _: () = unsafe { msg_send![&utterance, setPitchMultiplier: 0.80f32 };

    let _: () = unsafe { msg_send![&synthesizer, speakUtterance: &*utterance] };
}
