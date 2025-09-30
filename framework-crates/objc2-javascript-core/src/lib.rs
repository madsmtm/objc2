//! # Bindings to the `JavaScriptCore` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/javascriptcore/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(feature = "unstable-darwin-objc", feature(darwin_objc))]
#![cfg_attr(docsrs, feature(doc_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-java-script-core/0.3.2")]
#![recursion_limit = "256"]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;

#[cfg(feature = "JSBase")]
/// [Apple's documentation](https://developer.apple.com/documentation/javascriptcore/jsglobalcontextref?language=objc)
pub type JSGlobalContextRef = *mut OpaqueJSContext;

#[cfg(feature = "JSBase")]
/// [Apple's documentation](https://developer.apple.com/documentation/javascriptcore/jsglobalcontextref?language=objc)
pub type JSObjectRef = *mut OpaqueJSValue;
