#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::XCUIDevice;
use objc2::extern_methods;
use objc2_ui_kit::UIDeviceOrientation;

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
