//! # Bindings to the `XCUIAutomation` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/xcuiautomation/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(feature = "unstable-darwin-objc", feature(darwin_objc))]
#![cfg_attr(docsrs, feature(doc_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-xc-ui-automation/0.3.2")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;

#[cfg(feature = "std")]
impl std::ops::Index<std::string::String> for XCUIElementQuery {
    type Output = Self;
    /// This will return the same thing as `XCUIElementQuery::matchingIdentifier` which is what the
    /// swift subscript implementation does:
    /// <https://developer.apple.com/documentation/xcuiautomation/xcuielementquery/matching(identifier:)>
    /// <https://developer.apple.com/documentation/xcuiautomation/xcuielementquery/subscript(_:)>
    fn index(&self, index: std::string::String) -> &Self::Output {
        // TODO: Safety evaluation.
        unsafe {
            &*objc2::rc::Retained::autorelease_return(
                self.matchingIdentifier(&objc2_foundation::NSString::from_str(index.as_str())),
            )
        }
    }
}
#[cfg(target_os = "ios")]
mod device_buttons;

// Link to XCTest instead of XCUIAutomation, since the latter is only
// available in newer Xcode versions.
#[link(name = "XCTest", kind = "framework")]
extern "C" {}
