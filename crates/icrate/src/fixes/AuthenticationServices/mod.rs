use crate::common::*;

// TODO: UIViewController on iOS, NSViewController on macOS
pub type ASViewController = NSObject;
// TODO: UIWindow on iOS, NSWindow on macOS
pub type ASPresentationAnchor = NSObject;
// TODO: UIImage on iOS, NSImage on macOS
pub type ASImage = NSObject;

// TODO: UIControl on iOS, NSControl on macOS
#[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDButton")]
type ASControl = NSObject;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASCredentialProviderViewController")]
    pub struct ASCredentialProviderViewController;

    #[cfg(feature = "AuthenticationServices_ASCredentialProviderViewController")]
    unsafe impl ClassType for ASCredentialProviderViewController {
        type Super = ASViewController;
        type Mutability = InteriorMutable;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAccountAuthenticationModificationViewController")]
    pub struct ASAccountAuthenticationModificationViewController;

    #[cfg(feature = "AuthenticationServices_ASAccountAuthenticationModificationViewController")]
    unsafe impl ClassType for ASAccountAuthenticationModificationViewController {
        type Super = ASViewController;
        type Mutability = InteriorMutable;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDButton")]
    pub struct ASAuthorizationAppleIDButton;

    #[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDButton")]
    unsafe impl ClassType for ASAuthorizationAppleIDButton {
        type Super = ASControl;
        type Mutability = InteriorMutable;
    }
);
