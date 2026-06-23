//! Test that out parameters for functions are handled correctly.
//!
//! In particular, we want to ensure that the assert is optimized away.
#![cfg(target_vendor = "apple")]
use objc2_core_foundation::{CFError, CFRetained, CFString};
use objc2_core_media::{CMAudioDeviceClockGetAudioDevice, CMClock};
use objc2_security::SecTrust;

#[export_name = "fn1_strong_none"]
fn strong_none(trust: &SecTrust) -> bool {
    unsafe { SecTrust::evaluate_with_error(trust, None) }
}

#[export_name = "fn2_strong_some_none"]
fn strong_some_none(trust: &SecTrust) -> Option<CFRetained<CFError>> {
    let mut err = None;
    let _ = unsafe { SecTrust::evaluate_with_error(trust, Some(&mut err)) };
    err
}

#[export_name = "fn3_autoreleasing_none"]
fn autoreleasing_none(clock: &CMClock) -> i32 {
    // We should have a branch and a CFRetain here.
    unsafe { CMAudioDeviceClockGetAudioDevice(clock, None, &mut 0, &mut 0) }
}

#[export_name = "fn4_autoreleasing_some_none"]
fn autoreleasing_some_none(clock: &CMClock) -> Option<CFRetained<CFString>> {
    let mut device_uid_out = None;
    let _ = unsafe {
        CMAudioDeviceClockGetAudioDevice(clock, Some(&mut device_uid_out), &mut 0, &mut 0)
    };
    // We should have a branch and a CFRetain here.
    device_uid_out
}
