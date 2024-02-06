//! # Bindings to the `InputMethodKit` framework
#![allow(non_upper_case_globals)]

pub use crate::generated::InputMethodKit::*;

use crate::Foundation::*;

extern_static!(kIMKCommandClientName: &'static NSString);
extern_static!(kIMKCommandMenuItemName: &'static NSString);
extern_static!(IMKCandidatesOpacityAttributeName: &'static NSString);
extern_static!(IMKCandidatesSendServerKeyEventFirst: &'static NSString);
extern_static!(IMKControllerClass: &'static NSString);
extern_static!(IMKDelegateClass: &'static NSString);
extern_static!(IMKModeDictionary: &'static NSString);
