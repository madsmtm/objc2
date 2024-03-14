//! # Bindings to the `InputMethodKit` framework
#![allow(non_upper_case_globals)]

pub use crate::generated::InputMethodKit::*;

#[cfg(feature = "Foundation_NSString")]
#[allow(unused_imports)]
use crate::Foundation::NSString;

#[cfg(all(
    feature = "InputMethodKit_IMKInputController",
    feature = "Foundation_NSString"
))]
extern_static!(kIMKCommandMenuItemName: &'static NSString);
#[cfg(all(
    feature = "InputMethodKit_IMKInputController",
    feature = "Foundation_NSString"
))]
extern_static!(kIMKCommandClientName: &'static NSString);

#[cfg(all(
    feature = "InputMethodKit_IMKCandidates",
    feature = "Foundation_NSString"
))]
extern_static!(IMKCandidatesOpacityAttributeName: &'static NSString);
#[cfg(all(
    feature = "InputMethodKit_IMKCandidates",
    feature = "Foundation_NSString"
))]
extern_static!(IMKCandidatesSendServerKeyEventFirst: &'static NSString);

#[cfg(all(feature = "InputMethodKit_IMKServer", feature = "Foundation_NSString"))]
extern_static!(IMKModeDictionary: &'static NSString);
#[cfg(all(feature = "InputMethodKit_IMKServer", feature = "Foundation_NSString"))]
extern_static!(IMKControllerClass: &'static NSString);
#[cfg(all(feature = "InputMethodKit_IMKServer", feature = "Foundation_NSString"))]
extern_static!(IMKDelegateClass: &'static NSString);
