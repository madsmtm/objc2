//! # Bindings to the `AuthenticationServices` framework
pub use crate::generated::AuthenticationServices::*;

use objc2::runtime::NSObject;
use objc2::{extern_class, mutability, ClassType};

// TODO: UIViewController on iOS, NSViewController on macOS
#[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
pub type ASViewController = NSObject;
// TODO: UIWindow on iOS, NSWindow on macOS
#[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSWindow"))]
pub type ASPresentationAnchor = NSObject;
// TODO: UIImage on iOS, NSImage on macOS
#[cfg(feature = "AppKit_NSImage")]
pub type ASImage = NSObject;

// TODO: UIControl on iOS, NSControl on macOS
#[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDButton")]
type ASControl = NSObject;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "AuthenticationServices_ASCredentialProviderViewController",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSViewController"
    ))]
    pub struct ASCredentialProviderViewController;

    #[cfg(all(
        feature = "AuthenticationServices_ASCredentialProviderViewController",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSViewController"
    ))]
    unsafe impl ClassType for ASCredentialProviderViewController {
        type Super = ASViewController;
        type Mutability = mutability::MainThreadOnly;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "AuthenticationServices_ASAccountAuthenticationModificationViewController",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSViewController"
    ))]
    pub struct ASAccountAuthenticationModificationViewController;

    #[cfg(all(
        feature = "AuthenticationServices_ASAccountAuthenticationModificationViewController",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSViewController"
    ))]
    unsafe impl ClassType for ASAccountAuthenticationModificationViewController {
        type Super = ASViewController;
        type Mutability = mutability::MainThreadOnly;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "AuthenticationServices_ASAuthorizationAppleIDButton",
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    pub struct ASAuthorizationAppleIDButton;

    #[cfg(all(
        feature = "AuthenticationServices_ASAuthorizationAppleIDButton",
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    unsafe impl ClassType for ASAuthorizationAppleIDButton {
        type Super = ASControl;
        type Mutability = mutability::MainThreadOnly;
    }
);
