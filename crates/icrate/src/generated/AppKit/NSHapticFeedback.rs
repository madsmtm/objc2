//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSHapticFeedbackPattern = NSInteger;
pub const NSHapticFeedbackPatternGeneric: NSHapticFeedbackPattern = 0;
pub const NSHapticFeedbackPatternAlignment: NSHapticFeedbackPattern = 1;
pub const NSHapticFeedbackPatternLevelChange: NSHapticFeedbackPattern = 2;

pub type NSHapticFeedbackPerformanceTime = NSUInteger;
pub const NSHapticFeedbackPerformanceTimeDefault: NSHapticFeedbackPerformanceTime = 0;
pub const NSHapticFeedbackPerformanceTimeNow: NSHapticFeedbackPerformanceTime = 1;
pub const NSHapticFeedbackPerformanceTimeDrawCompleted: NSHapticFeedbackPerformanceTime = 2;

pub type NSHapticFeedbackPerformer = NSObject;

extern_class!(
    #[derive(Debug)]
    pub struct NSHapticFeedbackManager;

    unsafe impl ClassType for NSHapticFeedbackManager {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSHapticFeedbackManager {
        #[method_id(@__retain_semantics Other defaultPerformer)]
        pub unsafe fn defaultPerformer() -> Id<NSHapticFeedbackPerformer, Shared>;
    }
);
