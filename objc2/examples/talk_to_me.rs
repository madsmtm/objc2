//! Use `AVSpeechSynthesizer` to speak synthesized text.
//!
//! **Untested**!
//!
//! Works on macOS >= 10.15 or iOS > 7.0!

#[cfg(not(talk_to_me_example))]
fn main() {
    panic!("pass the `--cfg talk_to_me_example` flag to run this example!");
}

#[cfg(talk_to_me_example)]
fn main() {
    use objc2::rc::{Id, Owned};
    use objc2::runtime::Object;
    use objc2::{class, msg_send, msg_send_bool, msg_send_id, ns_string};

    #[link(name = "AVFoundation", kind = "framework")]
    extern "C" {}

    let string = ns_string!("Hello from Rust!");

    let synthesizer: Id<Object, Owned> =
        unsafe { msg_send_id![class!(AVSpeechSynthesizer), new] }.unwrap();

    let utterance = unsafe { msg_send_id![class!(AVSpeechUtterance), alloc] };
    let utterance: Id<Object, Owned> =
        unsafe { msg_send_id![utterance, initWithString: string] }.unwrap();

    // let _: () = unsafe { msg_send![&utterance, setVolume: 90.0f32 };
    // let _: () = unsafe { msg_send![&utterance, setRate: 0.50f32 };
    // let _: () = unsafe { msg_send![&utterance, setPitchMultiplier: 0.80f32 };

    let _: () = unsafe { msg_send![&synthesizer, speakUtterance: &*utterance] };

    while unsafe { msg_send_bool![&synthesizer, isSpeaking] } {
        // Busy loop while speaking
    }
}
