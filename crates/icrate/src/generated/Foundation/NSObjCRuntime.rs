//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};

pub type NSExceptionName = NSString;

pub type NSRunLoopMode = NSString;

pub type NSComparisonResult = NSInteger;
pub const NSOrderedAscending: NSComparisonResult = -1;
pub const NSOrderedSame: NSComparisonResult = 0;
pub const NSOrderedDescending: NSComparisonResult = 1;

pub type NSEnumerationOptions = NSUInteger;
pub const NSEnumerationConcurrent: NSEnumerationOptions = 1;
pub const NSEnumerationReverse: NSEnumerationOptions = 2;

pub type NSSortOptions = NSUInteger;
pub const NSSortConcurrent: NSSortOptions = 1;
pub const NSSortStable: NSSortOptions = 16;

pub type NSQualityOfService = NSInteger;
pub const NSQualityOfServiceUserInteractive: NSQualityOfService = 33;
pub const NSQualityOfServiceUserInitiated: NSQualityOfService = 25;
pub const NSQualityOfServiceUtility: NSQualityOfService = 17;
pub const NSQualityOfServiceBackground: NSQualityOfService = 9;
pub const NSQualityOfServiceDefault: NSQualityOfService = -1;
