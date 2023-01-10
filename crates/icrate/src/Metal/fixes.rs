use std::os::raw::c_double;

use objc2::ffi::NSUInteger;

pub type CFTimeInterval = c_double;

pub const MTLResourceCPUCacheModeShift: NSUInteger = 0;
pub const MTLResourceCPUCacheModeMask: NSUInteger = 0xf << MTLResourceCPUCacheModeShift;

pub const MTLResourceStorageModeShift: NSUInteger = 4;
pub const MTLResourceStorageModeMask: NSUInteger = 0xf << MTLResourceStorageModeShift;

pub const MTLResourceHazardTrackingModeShift: NSUInteger = 8;
pub const MTLResourceHazardTrackingModeMask: NSUInteger = 0x3 << MTLResourceHazardTrackingModeShift;
