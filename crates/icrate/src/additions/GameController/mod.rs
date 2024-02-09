//! # Bindings to the `GameController` framework
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
pub use crate::generated::GameController::*;

use crate::common::*;

#[cfg(all(
    feature = "GameController_GCInputNames",
    feature = "Foundation_NSString"
))]
mod input_names;

#[cfg(all(
    feature = "GameController_GCInputNames",
    feature = "Foundation_NSString"
))]
pub use self::input_names::*;

// TODO: GCKeyCode = CFIndex
// NOTE: CFIndex is c_long_long on __LLP64__ / Windows 64-bit (doesn't matter for us)
#[cfg(feature = "GameController_GCKeyCodes")]
pub type GCKeyCode = c_long;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
#[cfg(feature = "GameController_GCExtendedGamepadSnapshot")]
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
#[cfg(feature = "objc2")]
#[allow(deprecated)]
#[cfg(feature = "GameController_GCExtendedGamepadSnapshot")]
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
#[cfg(feature = "objc2")]
#[allow(deprecated)]
#[cfg(feature = "GameController_GCExtendedGamepadSnapshot")]
unsafe impl objc2::RefEncode for GCExtendedGamepadSnapshotData {
    const ENCODING_REF: objc2::Encoding =
        objc2::Encoding::Pointer(&<Self as objc2::Encode>::ENCODING);
}
