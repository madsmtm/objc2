//! # Bindings to the `IOUSBHost` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/iousbhost/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-io-usb-host/0.3.0")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;

// IOKit/IOReturn.h
#[allow(dead_code)]
pub(crate) type IOReturn = core::ffi::c_int; // kern_return_t

#[allow(unused_macros)]
macro_rules! IOUSBBit {
    ($bit:expr) => {
        1 << $bit
    };
}
#[allow(unused_imports)]
pub(crate) use IOUSBBit;

#[allow(unused_macros)]
macro_rules! IOUSBBitRange {
    ($start:expr, $end:expr) => {
        !((1 << $start) - 1) & ((1 << $end) | ((1 << $end) - 1))
    };
}
#[allow(unused_imports)]
pub(crate) use {IOUSBBitRange, IOUSBBitRange as IOUSBBitRange64};

#[allow(unused_macros)]
macro_rules! IOUSBBitRangePhase {
    ($start:expr, $end:expr) => {
        $start
    };
}
#[allow(unused_imports)]
pub(crate) use IOUSBBitRangePhase;

#[cfg(feature = "IOUSBHostDefinitions")]
extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/iousbhost/iousbhostmatchingpropertykeyproductidmask?language=objc)
    pub static IOUSBHostMatchingPropertyKeyProductIDMask: &'static IOUSBHostMatchingPropertyKey;
}
