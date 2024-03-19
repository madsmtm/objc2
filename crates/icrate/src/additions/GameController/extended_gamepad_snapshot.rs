use std::os::raw::c_float;

#[cfg(feature = "objc2")]
use objc2::encode::{Encode, Encoding, RefEncode};
use objc2::runtime::Bool;

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

#[cfg(feature = "objc2")]
#[allow(deprecated)]
unsafe impl Encode for GCExtendedGamepadSnapshotData {
    const ENCODING: Encoding = Encoding::Struct(
        "GCExtendedGamepadSnapshotData",
        &[
            u16::ENCODING,
            u16::ENCODING,
            c_float::ENCODING,
            c_float::ENCODING,
            c_float::ENCODING,
            c_float::ENCODING,
            c_float::ENCODING,
            c_float::ENCODING,
            c_float::ENCODING,
            c_float::ENCODING,
            c_float::ENCODING,
            c_float::ENCODING,
            c_float::ENCODING,
            c_float::ENCODING,
            c_float::ENCODING,
            c_float::ENCODING,
            Bool::ENCODING,
            Bool::ENCODING,
            Bool::ENCODING,
        ],
    );
}

#[cfg(feature = "objc2")]
#[allow(deprecated)]
unsafe impl RefEncode for GCExtendedGamepadSnapshotData {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
