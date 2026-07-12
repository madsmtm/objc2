//! Test that out parameters for functions are handled correctly.
//!
//! In particular, we want to ensure that the assert is optimized away.
#![cfg(target_vendor = "apple")]
use objc2_core_foundation::{CFDictionary, CFError, CFRetained, CFString, CFType};
use objc2_core_media::{CMAudioDeviceClockGetAudioDevice, CMClock};
use objc2_security::{SecItemCopyMatching, SecTrust};

#[export_name = "fn1_strong_none"]
fn strong_none(query: &CFDictionary<CFString, CFType>) -> i32 {
    unsafe { SecItemCopyMatching(query, None) }
}

#[export_name = "fn2_strong_some_none"]
fn strong_some_none(query: &CFDictionary<CFString, CFType>) -> Option<CFRetained<CFType>> {
    let mut result = None;
    let _ = unsafe { SecItemCopyMatching(query, Some(&mut result)) };
    result
}

#[export_name = "fn3_autoreleasing_none"]
fn autoreleasing_none(clock: &CMClock) -> i32 {
    // We should have a branch and a CFRetain here.
    unsafe { CMAudioDeviceClockGetAudioDevice(clock, None, Some(&mut 0), Some(&mut 0)) }
}

#[export_name = "fn4_autoreleasing_some_none"]
fn autoreleasing_some_none(clock: &CMClock) -> Option<CFRetained<CFString>> {
    let mut device_uid_out = None;
    let _ = unsafe {
        CMAudioDeviceClockGetAudioDevice(
            clock,
            Some(&mut device_uid_out),
            Some(&mut 0),
            Some(&mut 0),
        )
    };
    // We should have a branch and a CFRetain here.
    device_uid_out
}

#[export_name = "fn5_strong_error"]
fn strong(trust: &SecTrust) -> Result<(), CFRetained<CFError>> {
    unsafe { SecTrust::evaluate_with_error(trust) }
}

// #[export_name = "fn6_autoreleasing_error"]
// fn autoreleasing(sbuf: &CMSampleBuffer, content_key: &AVContentKey,) -> Result<(), CFRetained<CFError>> {
//     unsafe { AVSampleBufferAttachContentKey(trust) }
// }
