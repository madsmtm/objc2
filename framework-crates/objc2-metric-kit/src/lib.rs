//! # Bindings to the `MetricKit` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/metrickit/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(feature = "unstable-docsrs", feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-metric-kit/0.2.0")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
#[cfg(feature = "MXMetricManager")]
mod manager;

#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;
#[cfg(feature = "MXMetricManager")]
#[allow(unused_imports, unreachable_pub)]
pub use self::manager::*;
