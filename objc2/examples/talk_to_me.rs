use objc2::ffi::NSUInteger;
use objc2::rc::{Id, Owned, Shared};
use objc2::runtime::Object;
use objc2::{class, msg_send};
use std::ffi::c_void;

#[cfg(feature = "apple")] // Does not work on GNUStep
#[link(name = "AVFoundation", kind = "framework")]
extern "C" {}

// Only works on macOS >= 10.15 or iOS > 7.0
fn main() {
    let text = "Hello from Rust!";
    const UTF8_ENCODING: NSUInteger = 4;

    let string: *const Object = unsafe { msg_send![class!(NSString), alloc] };
    let string = unsafe {
        msg_send![
            string,
            initWithBytes: text.as_ptr() as *const c_void,
            length: text.len(),
            encoding: UTF8_ENCODING,
        ]
    };
    let string: Id<Object, Shared> = unsafe { Id::new(string).unwrap() };

    let synthesizer: *mut Object = unsafe { msg_send![class!(AVSpeechSynthesizer), new] };
    let synthesizer: Id<Object, Owned> = unsafe { Id::new(synthesizer).unwrap() };

    let utterance: *mut Object = unsafe { msg_send![class!(AVSpeechUtterance), alloc] };
    let utterance: *mut Object = unsafe { msg_send![utterance, initWithString: &*string] };
    let utterance: Id<Object, Owned> = unsafe { Id::new(utterance).unwrap() };

    // let _: () = unsafe { msg_send![&utterance, setVolume: 90.0f32 };
    // let _: () = unsafe { msg_send![&utterance, setRate: 0.50f32 };
    // let _: () = unsafe { msg_send![&utterance, setPitchMultiplier: 0.80f32 };

    let _: () = unsafe { msg_send![&synthesizer, speakUtterance: &*utterance] };
}
