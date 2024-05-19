//! # Bindings to the `AuthenticationServices` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/authenticationservices/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-authentication-services/0.2.0")]
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
use objc2::{extern_class, mutability, ClassType};

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
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ASCredentialProviderViewController")]
    pub struct ASCredentialProviderViewController;

    #[cfg(feature = "ASCredentialProviderViewController")]
    unsafe impl ClassType for ASCredentialProviderViewController {
        type Super = ASViewController;
        type Mutability = mutability::MainThreadOnly;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ASAccountAuthenticationModificationViewController")]
    pub struct ASAccountAuthenticationModificationViewController;

    #[cfg(feature = "ASAccountAuthenticationModificationViewController")]
    unsafe impl ClassType for ASAccountAuthenticationModificationViewController {
        type Super = ASViewController;
        type Mutability = mutability::MainThreadOnly;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ASAuthorizationAppleIDButton")]
    pub struct ASAuthorizationAppleIDButton;

    #[cfg(feature = "ASAuthorizationAppleIDButton")]
    unsafe impl ClassType for ASAuthorizationAppleIDButton {
        type Super = ASControl;
        type Mutability = mutability::MainThreadOnly;
    }
);
