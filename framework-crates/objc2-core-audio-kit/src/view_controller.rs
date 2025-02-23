#![allow(unused_imports)]

use objc2::extern_class;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::{NSResponder, NSViewController};
use objc2_foundation::NSObject;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreaudiokit/augenericviewcontroller?language=objc)
    #[unsafe(super(NSViewController, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    pub struct AUGenericViewController;
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreaudiokit/auviewcontroller?language=objc)
    #[unsafe(super(NSViewController, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    pub struct AUViewController;
);
