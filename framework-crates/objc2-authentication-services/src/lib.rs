//! # Bindings to the `AuthenticationServices` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/authenticationservices/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-authentication-services/0.3.1")]
#![allow(unused_imports)]
#![allow(dead_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;

use objc2::runtime::NSObject;
use objc2::{extern_class, ClassType, MainThreadOnly};

// TODO: UIViewController on iOS, NSViewController on macOS
pub type ASViewController = NSObject;
// TODO: UIWindow on iOS, NSWindow on macOS
pub type ASPresentationAnchor = NSObject;
// TODO: UIImage on iOS, NSImage on macOS
pub type ASImage = NSObject;

// TODO: UIControl on iOS, NSControl on macOS
#[cfg(feature = "ASAuthorizationAppleIDButton")]
type ASControl = NSObject;

extern_class!(
    #[unsafe(super(ASViewController))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ASCredentialProviderViewController")]
    pub struct ASCredentialProviderViewController;
);

extern_class!(
    #[unsafe(super(ASViewController))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ASAccountAuthenticationModificationViewController")]
    pub struct ASAccountAuthenticationModificationViewController;
);

extern_class!(
    #[unsafe(super(ASControl))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ASAuthorizationAppleIDButton")]
    pub struct ASAuthorizationAppleIDButton;
);
