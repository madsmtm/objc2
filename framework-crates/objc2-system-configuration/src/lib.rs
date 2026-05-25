//! # Bindings to the `SystemConfiguration` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/systemconfiguration/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(feature = "unstable-darwin-objc", feature(darwin_objc))]
#![cfg_attr(docsrs, feature(doc_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-system-configuration/0.3.2")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// TODO(breaking): Figure out `SCNetworkInterfaceRefreshConfiguration`'s ABI
#[allow(improper_ctypes_definitions)]
mod generated;
#[cfg(feature = "SCNetwork")]
mod network;
#[cfg(feature = "SCNetworkReachability")]
mod network_reachability;
#[cfg(feature = "libc")]
mod sockaddr;

#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;
#[cfg(feature = "SCNetwork")]
#[allow(unused_imports, unreachable_pub)]
pub use self::network::*;

#[allow(dead_code)]
pub(crate) type Boolean = u8; // unsigned char
