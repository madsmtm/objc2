//! # Bindings to the `GameController` framework
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
pub use crate::generated::GameController::*;

#[cfg(feature = "GameController_GCExtendedGamepadSnapshot")]
mod extended_gamepad_snapshot;
#[cfg(all(
    feature = "GameController_GCInputNames",
    feature = "Foundation_NSString"
))]
mod input_names;

#[cfg(feature = "GameController_GCExtendedGamepadSnapshot")]
#[allow(deprecated)]
pub use self::extended_gamepad_snapshot::GCExtendedGamepadSnapshotData;
#[cfg(all(
    feature = "GameController_GCInputNames",
    feature = "Foundation_NSString"
))]
pub use self::input_names::*;

// TODO: GCKeyCode = CFIndex
// NOTE: CFIndex is c_long_long on __LLP64__ / Windows 64-bit (doesn't matter for us)
#[cfg(feature = "GameController_GCKeyCodes")]
pub type GCKeyCode = std::os::raw::c_long;
