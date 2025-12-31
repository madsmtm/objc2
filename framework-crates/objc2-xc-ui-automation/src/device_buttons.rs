#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
use crate::XCUIDevice;
use objc2::{extern_methods, Encode, Encoding, RefEncode};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct XCUIDeviceButton(pub objc2_foundation::NSInteger);
impl XCUIDeviceButton {
    #[doc(alias = "XCUIDeviceButtonHome")]
    pub const Home: Self = Self(1);
    #[doc(alias = "XCUIDeviceButtonVolumeUp")]
    pub const VolumeUp: Self = Self(2);
    #[doc(alias = "XCUIDeviceButtonVolumeDown")]
    pub const VolumeDown: Self = Self(3);
    #[cfg(not(any(target_os = "tvos", target_os = "visionos")))]
    #[doc(alias = "XCUIDeviceButtonAction")]
    pub const Action: Self = Self(4);
    #[cfg(not(any(target_os = "tvos", target_os = "visionos", target_os = "watchos")))]
    #[doc(alias = "XCUIDeviceButtonCamera")]
    pub const Camera: Self = Self(5);
}

unsafe impl Encode for XCUIDeviceButton {
    const ENCODING: Encoding = objc2_foundation::NSInteger::ENCODING;
}

unsafe impl RefEncode for XCUIDeviceButton {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
impl XCUIDevice {
    extern_methods!(
        #[unsafe(method(hasHardwareButton:))]
        #[unsafe(method_family = none)]
        pub fn hasHardwareButton(&self, button: XCUIDeviceButton) -> bool;

        /// Simulates the user pressing a physical button.
        #[unsafe(method(pressButton:))]
        #[unsafe(method_family = none)]
        pub fn pressButton(&self, button: XCUIDeviceButton);
    );
}
