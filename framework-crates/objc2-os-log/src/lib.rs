//! # Bindings to the `OSLog` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/oslog/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-os-log/0.3.0")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;

// os/activity.h
#[allow(dead_code, non_camel_case_types)]
pub(crate) type os_activity_id_t = u64;
// os/signpost.h
#[allow(dead_code, non_camel_case_types)]
pub(crate) type os_signpost_id_t = u64;

unsafe impl objc2_foundation::NSCoding for OSLogEntry {}
