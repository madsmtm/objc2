//! # Bindings to the `WebKit` framework
//!
//!
//! ## Example
//!
//! ```ignore
#![doc = include_str!("../../../examples/browser.rs")]
//! ```
#![allow(non_snake_case)]

pub use crate::generated::WebKit::*;

use crate::common::*;

extern_methods!(
    #[cfg(feature = "WebKit_WKNavigationAction")]
    unsafe impl WKNavigationAction {
        #[cfg(feature = "WebKit_WKFrameInfo")]
        #[method_id(@__retain_semantics Other sourceFrame)]
        pub unsafe fn sourceFrame(&self) -> Option<Id<WKFrameInfo>>;
    }
);
