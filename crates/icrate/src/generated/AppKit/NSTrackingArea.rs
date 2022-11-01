//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

pub type NSTrackingAreaOptions = NSUInteger;
pub const NSTrackingMouseEnteredAndExited: NSTrackingAreaOptions = 0x01;
pub const NSTrackingMouseMoved: NSTrackingAreaOptions = 0x02;
pub const NSTrackingCursorUpdate: NSTrackingAreaOptions = 0x04;
pub const NSTrackingActiveWhenFirstResponder: NSTrackingAreaOptions = 0x10;
pub const NSTrackingActiveInKeyWindow: NSTrackingAreaOptions = 0x20;
pub const NSTrackingActiveInActiveApp: NSTrackingAreaOptions = 0x40;
pub const NSTrackingActiveAlways: NSTrackingAreaOptions = 0x80;
pub const NSTrackingAssumeInside: NSTrackingAreaOptions = 0x100;
pub const NSTrackingInVisibleRect: NSTrackingAreaOptions = 0x200;
pub const NSTrackingEnabledDuringMouseDrag: NSTrackingAreaOptions = 0x400;

extern_class!(
    #[derive(Debug)]
    pub struct NSTrackingArea;

    unsafe impl ClassType for NSTrackingArea {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSTrackingArea {
        #[method_id(@__retain_semantics Init initWithRect:options:owner:userInfo:)]
        pub unsafe fn initWithRect_options_owner_userInfo(
            this: Option<Allocated<Self>>,
            rect: NSRect,
            options: NSTrackingAreaOptions,
            owner: Option<&Object>,
            userInfo: Option<&NSDictionary<Object, Object>>,
        ) -> Id<Self, Shared>;

        #[method(rect)]
        pub unsafe fn rect(&self) -> NSRect;

        #[method(options)]
        pub unsafe fn options(&self) -> NSTrackingAreaOptions;

        #[method_id(@__retain_semantics Other owner)]
        pub unsafe fn owner(&self) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary<Object, Object>, Shared>>;
    }
);
