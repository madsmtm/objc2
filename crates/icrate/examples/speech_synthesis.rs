//! Speak synthethized text.
//!
//! This uses `NSSpeechSynthesizer` on macOS, and `AVSpeechSynthesizer` on
//! other Apple platforms. Note that `AVSpeechSynthesizer` _is_ available on
//! macOS, but only since 10.15!
//!
//! Works on macOS >= 10.7 and iOS > 7.0.
#![deny(unsafe_op_in_unsafe_fn)]

use std::thread;
use std::time::Duration;

use icrate::ns_string;
use icrate::Foundation::{NSObject, NSString};
use objc2::mutability::InteriorMutable;
use objc2::rc::Id;
use objc2::{extern_class, msg_send, msg_send_id, ClassType};

#[cfg(target_os = "macos")]
mod appkit {
    use icrate::Foundation::NSCopying;
    use std::cell::Cell;

    use super::*;

    #[link(name = "AppKit", kind = "framework")]
    extern "C" {}

    extern_class!(
        /// <https://developer.apple.com/documentation/appkit/nsspeechsynthesizer?language=objc>
        pub struct Synthesizer;

        unsafe impl ClassType for Synthesizer {
            type Super = NSObject;
            type Mutability = InteriorMutable;
            const NAME: &'static str = "NSSpeechSynthesizer";
        }
    );

    impl Synthesizer {
        // Uses default voice
        pub fn new() -> Id<Self> {
            unsafe { msg_send_id![Self::class(), new] }
        }

        fn set_rate(&self, rate: f32) {
            unsafe { msg_send![self, setRate: rate] }
        }

        fn set_volume(&self, volume: f32) {
            unsafe { msg_send![self, setVolume: volume] }
        }

        fn start_speaking(&self, s: &NSString) {
            let _: bool = unsafe { msg_send![self, startSpeakingString: s] };
        }

        pub fn speak(&self, utterance: &Utterance) {
            // Convert to the range 90-720 that `NSSpeechSynthesizer` seems to
            // support
            //
            // Note that you'd probably want a nonlinear conversion here to
            // make it match `AVSpeechSynthesizer`.
            self.set_rate(90.0 + (utterance.rate.get() * (360.0 - 90.0)));
            self.set_volume(utterance.volume.get());
            self.start_speaking(&utterance.string);
        }

        pub fn is_speaking(&self) -> bool {
            unsafe { msg_send![self, isSpeaking] }
        }
    }

    // Shim to make NSSpeechSynthesizer work similar to AVSpeechSynthesizer
    pub struct Utterance {
        rate: Cell<f32>,
        volume: Cell<f32>,
        string: Id<NSString>,
    }

    impl Utterance {
        pub fn new(string: &NSString) -> Self {
            Self {
                rate: Cell::new(0.5),
                volume: Cell::new(1.0),
                string: string.copy(),
            }
        }

        pub fn set_rate(&self, rate: f32) {
            self.rate.set(rate);
        }

        pub fn set_volume(&self, volume: f32) {
            self.volume.set(volume);
        }
    }
}

#[cfg(not(target_os = "macos"))]
mod avfaudio {
    use super::*;

    #[link(name = "AVFoundation", kind = "framework")]
    extern "C" {}

    extern_class!(
        /// <https://developer.apple.com/documentation/avfaudio/avspeechsynthesizer?language=objc>
        #[derive(Debug)]
        pub struct Synthesizer;

        unsafe impl ClassType for Synthesizer {
            type Super = NSObject;
            type Mutability = InteriorMutable;
            const NAME: &'static str = "AVSpeechSynthesizer";
        }
    );

    impl Synthesizer {
        pub fn new() -> Id<Self> {
            unsafe { msg_send_id![Self::class(), new] }
        }

        pub fn speak(&self, utterance: &Utterance) {
            unsafe { msg_send![self, speakUtterance: utterance] }
        }

        pub fn is_speaking(&self) -> bool {
            unsafe { msg_send![self, isSpeaking] }
        }
    }

    extern_class!(
        /// <https://developer.apple.com/documentation/avfaudio/avspeechutterance?language=objc>
        #[derive(Debug)]
        pub struct Utterance;

        unsafe impl ClassType for Utterance {
            type Super = NSObject;
            type Mutability = InteriorMutable;
            const NAME: &'static str = "AVSpeechUtterance";
        }
    );

    impl Utterance {
        pub fn new(string: &NSString) -> Id<Self> {
            unsafe { msg_send_id![Self::alloc(), initWithString: string] }
        }

        pub fn set_rate(&self, rate: f32) {
            unsafe { msg_send![self, setRate: rate] }
        }

        pub fn set_volume(&self, volume: f32) {
            unsafe { msg_send![self, setVolume: volume] }
        }
    }
}

#[cfg(target_os = "macos")]
use appkit::{Synthesizer, Utterance};
#[cfg(not(target_os = "macos"))]
use avfaudio::{Synthesizer, Utterance};

fn main() {
    let synthesizer = Synthesizer::new();
    let utterance = Utterance::new(ns_string!("Hello from Rust!"));
    utterance.set_rate(0.5);
    utterance.set_volume(0.5);
    synthesizer.speak(&utterance);

    // Wait until speech has properly started up
    thread::sleep(Duration::from_millis(1000));
    // Wait until finished speaking
    while synthesizer.is_speaking() {
        thread::sleep(Duration::from_millis(100));
    }
}
