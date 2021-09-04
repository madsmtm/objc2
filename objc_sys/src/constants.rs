//! Various common #defines and enum constants.

use std::os::raw::c_int;

use super::{id, Class, BOOL};

/// The equivalent of true for Objective-C's [`BOOL`][`super::BOOL`] type.
#[cfg(not(target_arch = "aarch64"))]
pub const YES: BOOL = 1;
#[cfg(target_arch = "aarch64")]
pub const YES: BOOL = true;

/// The equivalent of false for Objective-C's [`BOOL`][`super::BOOL`] type.
#[cfg(not(target_arch = "aarch64"))]
pub const NO: BOOL = 0;
#[cfg(target_arch = "aarch64")]
pub const NO: BOOL = false;

/// A quick alias for a [`null_mut`][`core::ptr::null_mut`] object / instance.
#[allow(non_upper_case_globals)]
pub const nil: id = 0 as *mut _;

/// A quick alias for a [`null_mut`][`core::ptr::null_mut`] class.
#[allow(non_upper_case_globals)]
pub const Nil: Class = 0 as *mut _;

#[allow(non_camel_case_types)]
pub type objc_AssociationPolicy = usize;
pub const OBJC_ASSOCIATION_ASSIGN: objc_AssociationPolicy = 0;
pub const OBJC_ASSOCIATION_RETAIN_NONATOMIC: objc_AssociationPolicy = 1;
pub const OBJC_ASSOCIATION_COPY_NONATOMIC: objc_AssociationPolicy = 3;
pub const OBJC_ASSOCIATION_RETAIN: objc_AssociationPolicy = 769;
pub const OBJC_ASSOCIATION_COPY: objc_AssociationPolicy = 771;

pub const OBJC_SYNC_SUCCESS: c_int = 0;
pub const OBJC_SYNC_NOT_OWNING_THREAD_ERROR: c_int = -1;
/// Only relevant before macOS 10.13
pub const OBJC_SYNC_TIMED_OUT: c_int = -2;
/// Only relevant before macOS 10.13
pub const OBJC_SYNC_NOT_INITIALIZED: c_int = -3;
