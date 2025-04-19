//! # Bindings to the `MetalPerformanceShaders` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/metalperformanceshaders/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-metal-performance-shaders/0.3.1")]
#![recursion_limit = "512"]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[allow(unreachable_pub)]
mod generated;
#[cfg(feature = "MPSRayIntersectorTypes")]
mod packed;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;
#[cfg(feature = "MPSRayIntersectorTypes")]
pub use self::packed::MPSPackedFloat3;

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimagebatch?language=objc)
#[cfg(all(feature = "MPSImage", feature = "MPSCore"))]
pub type MPSImageBatch = objc2_foundation::NSArray<MPSImage>;
