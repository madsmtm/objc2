//! # Bindings to the `WebKit` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/webkit/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
//!
//!
//! ## Example
//!
//! ```ignore
#![doc = include_str!("../examples/browser.rs")]
//! ```
#![recursion_limit = "512"]
#![allow(non_snake_case)]
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-web-kit/0.3.1")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;

#[cfg(feature = "WKNavigationAction")]
impl WKNavigationAction {
    objc2::extern_methods!(
        #[cfg(feature = "WKFrameInfo")]
        #[unsafe(method(sourceFrame))]
        pub unsafe fn sourceFrame(&self) -> Option<objc2::rc::Retained<WKFrameInfo>>;
    );
}
