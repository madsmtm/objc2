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
pub const NSEnumerationConcurrent: NSEnumerationOptions = (1 << 0);
pub const NSEnumerationReverse: NSEnumerationOptions = (1 << 1);

pub type NSSortOptions = NSUInteger;
pub const NSSortConcurrent: NSSortOptions = (1 << 0);
pub const NSSortStable: NSSortOptions = (1 << 4);

pub type NSQualityOfService = NSInteger;
pub const NSQualityOfServiceUserInteractive: NSQualityOfService = 0x21;
pub const NSQualityOfServiceUserInitiated: NSQualityOfService = 0x19;
pub const NSQualityOfServiceUtility: NSQualityOfService = 0x11;
pub const NSQualityOfServiceBackground: NSQualityOfService = 0x09;
pub const NSQualityOfServiceDefault: NSQualityOfService = -1;
