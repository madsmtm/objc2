use crate::common::*;
use crate::Foundation::NSString;

// TODO: GCKeyCode = CFIndex
// NOTE: CFIndex is c_long_long on __LLP64__ / Windows 64-bit (doesn't matter for us)
pub type GCKeyCode = c_long;

typed_extensible_enum!(
    pub type GCInputElementName = NSString;
);

typed_extensible_enum!(
    pub type GCInputButtonName = NSString;
);

typed_extensible_enum!(
    pub type GCInputAxisName = NSString;
);

typed_extensible_enum!(
    pub type GCInputSwitchName = NSString;
);

typed_extensible_enum!(
    pub type GCInputDirectionPadName = NSString;
);

extern_static!(GCInputButtonA: &'static GCInputButtonName);

extern_static!(GCInputButtonB: &'static GCInputButtonName);

extern_static!(GCInputButtonX: &'static GCInputButtonName);

extern_static!(GCInputButtonY: &'static GCInputButtonName);

extern_static!(GCInputDirectionPad: &'static GCInputDirectionPadName);

extern_static!(GCInputLeftThumbstick: &'static GCInputDirectionPadName);

extern_static!(GCInputRightThumbstick: &'static GCInputDirectionPadName);

extern_static!(GCInputLeftShoulder: &'static GCInputButtonName);

extern_static!(GCInputRightShoulder: &'static GCInputButtonName);

extern_static!(GCInputLeftTrigger: &'static GCInputButtonName);

extern_static!(GCInputRightTrigger: &'static GCInputButtonName);

extern_static!(GCInputLeftThumbstickButton: &'static GCInputButtonName);

extern_static!(GCInputRightThumbstickButton: &'static GCInputButtonName);

extern_static!(GCInputButtonHome: &'static GCInputButtonName);

extern_static!(GCInputButtonMenu: &'static GCInputButtonName);

extern_static!(GCInputButtonOptions: &'static GCInputButtonName);

extern_static!(GCInputButtonShare: &'static GCInputButtonName);

extern_static!(GCInputXboxPaddleOne: &'static GCInputButtonName);

extern_static!(GCInputXboxPaddleTwo: &'static GCInputButtonName);

extern_static!(GCInputXboxPaddleThree: &'static GCInputButtonName);

extern_static!(GCInputXboxPaddleFour: &'static GCInputButtonName);

extern_static!(GCInputDualShockTouchpadOne: &'static GCInputDirectionPadName);

extern_static!(GCInputDualShockTouchpadTwo: &'static GCInputDirectionPadName);

extern_static!(GCInputDualShockTouchpadButton: &'static GCInputButtonName);

extern_static!(GCInputSteeringWheel: &'static GCInputAxisName);

extern_static!(GCInputShifter: &'static GCInputElementName);

extern_static!(GCInputPedalAccelerator: &'static GCInputButtonName);

extern_static!(GCInputPedalBrake: &'static GCInputButtonName);

extern_static!(GCInputPedalClutch: &'static GCInputButtonName);

extern_static!(GCInputLeftPaddle: &'static GCInputButtonName);

extern_static!(GCInputRightPaddle: &'static GCInputButtonName);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
pub struct GCExtendedGamepadSnapshotData {
    pub version: u16,
    pub size: u16,
    pub dpadX: c_float,
    pub dpadY: c_float,
    pub buttonA: c_float,
    pub buttonB: c_float,
    pub buttonX: c_float,
    pub buttonY: c_float,
    pub leftShoulder: c_float,
    pub rightShoulder: c_float,
    pub leftThumbstickX: c_float,
    pub leftThumbstickY: c_float,
    pub rightThumbstickX: c_float,
    pub rightThumbstickY: c_float,
    pub leftTrigger: c_float,
    pub rightTrigger: c_float,
    pub supportsClickableThumbsticks: Bool,
    pub leftThumbstickButton: Bool,
    pub rightThumbstickButton: Bool,
}
#[cfg(feature = "objective-c")]
#[allow(deprecated)]
unsafe impl objc2::Encode for GCExtendedGamepadSnapshotData {
    const ENCODING: objc2::Encoding = (objc2::Encoding::Struct(
        "GCExtendedGamepadSnapshotData",
        &[
            <u16 as objc2::Encode>::ENCODING,
            <u16 as objc2::Encode>::ENCODING,
            <c_float as objc2::Encode>::ENCODING,
            <c_float as objc2::Encode>::ENCODING,
            <c_float as objc2::Encode>::ENCODING,
            <c_float as objc2::Encode>::ENCODING,
            <c_float as objc2::Encode>::ENCODING,
            <c_float as objc2::Encode>::ENCODING,
            <c_float as objc2::Encode>::ENCODING,
            <c_float as objc2::Encode>::ENCODING,
            <c_float as objc2::Encode>::ENCODING,
            <c_float as objc2::Encode>::ENCODING,
            <c_float as objc2::Encode>::ENCODING,
            <c_float as objc2::Encode>::ENCODING,
            <c_float as objc2::Encode>::ENCODING,
            <c_float as objc2::Encode>::ENCODING,
            <Bool as objc2::Encode>::ENCODING,
            <Bool as objc2::Encode>::ENCODING,
            <Bool as objc2::Encode>::ENCODING,
        ],
    ));
}
#[cfg(feature = "objective-c")]
#[allow(deprecated)]
unsafe impl objc2::RefEncode for GCExtendedGamepadSnapshotData {
    const ENCODING_REF: objc2::Encoding =
        objc2::Encoding::Pointer(&<Self as objc2::Encode>::ENCODING);
}
