#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::XCUIDevice;
use objc2::{extern_methods, ffi::NSInteger, Encode, Encoding, RefEncode};

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uideviceorientation?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct UIDeviceOrientation(pub NSInteger);
impl UIDeviceOrientation {
    #[doc(alias = "UIDeviceOrientationUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "UIDeviceOrientationPortrait")]
    pub const Portrait: Self = Self(1);
    #[doc(alias = "UIDeviceOrientationPortraitUpsideDown")]
    pub const PortraitUpsideDown: Self = Self(2);
    #[doc(alias = "UIDeviceOrientationLandscapeLeft")]
    pub const LandscapeLeft: Self = Self(3);
    #[doc(alias = "UIDeviceOrientationLandscapeRight")]
    pub const LandscapeRight: Self = Self(4);
    #[doc(alias = "UIDeviceOrientationFaceUp")]
    pub const FaceUp: Self = Self(5);
    #[doc(alias = "UIDeviceOrientationFaceDown")]
    pub const FaceDown: Self = Self(6);
}

unsafe impl Encode for UIDeviceOrientation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIDeviceOrientation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
impl XCUIDevice {
    extern_methods!(
        /// The orientation of the device.
        #[unsafe(method(orientation))]
        #[unsafe(method_family = none)]
        pub fn orientation(&self) -> UIDeviceOrientation;

        /// Setter for [`orientation`][Self::orientation].
        #[unsafe(method(setOrientation:))]
        #[unsafe(method_family = none)]
        pub fn setOrientation(&self, orientation: UIDeviceOrientation);
    );
}
