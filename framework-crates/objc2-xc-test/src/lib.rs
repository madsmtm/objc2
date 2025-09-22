//! # Bindings to the `XCTest` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! WARNING: You cannot normally link to this framework unless you add something like the following:
//! ```text
//! RUSTFLAGS="-Clink-args=-F/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/Library/Frameworks"
//! ```
//!
//! And add the following when running the binary:
//! ```text
//! DYLD_FRAMEWORK_PATH=/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/Library/Frameworks
//! ```
//!
//! See [the example of using XCTest][examples-testing] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/xctest/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
//! [examples-testing]: https://github.com/madsmtm/objc2/tree/main/examples/testing-helper
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-xc-test/0.3.1")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;
