use crate::common::*;

// TODO: UIViewController on iOS, NSViewController on macOS
pub type ASViewController = NSObject;
// TODO: UIWindow on iOS, NSWindow on macOS
pub type ASPresentationAnchor = NSObject;
// TODO: UIImage on iOS, NSImage on macOS
pub type ASImage = NSObject;

// TODO: UIControl on iOS, NSControl on macOS
pub(crate) type ASControl = NSObject;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASCredentialProviderViewController;

    unsafe impl ClassType for ASCredentialProviderViewController {
        type Super = ASViewController;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAccountAuthenticationModificationViewController;

    unsafe impl ClassType for ASAccountAuthenticationModificationViewController {
        type Super = ASViewController;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationAppleIDButton;

    unsafe impl ClassType for ASAuthorizationAppleIDButton {
        type Super = ASControl;
    }
);
