//! Use `NSSpeechSynthesizer` to speak synthesized text.
//!
//! Works on macOS 10.7+
#![deny(unsafe_op_in_unsafe_fn)]
use std::ops::{Deref, DerefMut};
use std::thread;
use std::time::Duration;

use objc2::rc::{Id, Owned};
use objc2::runtime::Class;
use objc2::{class, msg_send, msg_send_bool, msg_send_id};
use objc2::{Encoding, Message, RefEncode};

use objc2_foundation::{NSObject, NSString};

#[cfg(all(feature = "apple", target_os = "macos"))]
#[link(name = "AppKit", kind = "framework")]
extern "C" {}

/// <https://developer.apple.com/documentation/appkit/nsspeechsynthesizer?language=objc>
#[repr(C)]
pub struct NSSpeechSynthesizer {
    inner: NSObject,
}

unsafe impl RefEncode for NSSpeechSynthesizer {
    const ENCODING_REF: Encoding<'static> = Encoding::Object;
}

unsafe impl Message for NSSpeechSynthesizer {}

impl Deref for NSSpeechSynthesizer {
    type Target = NSObject;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for NSSpeechSynthesizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// TODO: Unsure about when to use `&mut` here?
impl NSSpeechSynthesizer {
    pub fn class() -> &'static Class {
        class!(NSSpeechSynthesizer)
    }

    // Uses default voice
    pub fn new() -> Id<Self, Owned> {
        unsafe { msg_send_id![Self::class(), new].unwrap() }
    }

    pub fn set_rate(&mut self, rate: f32) {
        unsafe { msg_send![self, setRate: rate] }
    }

    pub fn set_volume(&mut self, volume: f32) {
        unsafe { msg_send![self, setVolume: volume] }
    }

    pub fn start_speaking(&mut self, s: &str) {
        let s = NSString::from_str(s);
        unsafe { msg_send![self, startSpeakingString: &*s] }
    }

    pub fn speaking(&self) -> bool {
        unsafe { msg_send_bool![self, isSpeaking] }
    }
}

fn main() {
    let mut synthesizer = NSSpeechSynthesizer::new();
    synthesizer.set_rate(150.0);
    synthesizer.set_volume(1.0);
    synthesizer.start_speaking("Hello from Rust");

    while synthesizer.speaking() {
        // Wait until finished speaking
        thread::sleep(Duration::from_millis(100));
    }
}
