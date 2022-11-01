//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};

pub type NSTextListMarkerFormat = NSString;

extern "C" {
    static NSTextListMarkerBox: &'static NSTextListMarkerFormat;
}

extern "C" {
    static NSTextListMarkerCheck: &'static NSTextListMarkerFormat;
}

extern "C" {
    static NSTextListMarkerCircle: &'static NSTextListMarkerFormat;
}

extern "C" {
    static NSTextListMarkerDiamond: &'static NSTextListMarkerFormat;
}

extern "C" {
    static NSTextListMarkerDisc: &'static NSTextListMarkerFormat;
}

extern "C" {
    static NSTextListMarkerHyphen: &'static NSTextListMarkerFormat;
}

extern "C" {
    static NSTextListMarkerSquare: &'static NSTextListMarkerFormat;
}

extern "C" {
    static NSTextListMarkerLowercaseHexadecimal: &'static NSTextListMarkerFormat;
}

extern "C" {
    static NSTextListMarkerUppercaseHexadecimal: &'static NSTextListMarkerFormat;
}

extern "C" {
    static NSTextListMarkerOctal: &'static NSTextListMarkerFormat;
}

extern "C" {
    static NSTextListMarkerLowercaseAlpha: &'static NSTextListMarkerFormat;
}

extern "C" {
    static NSTextListMarkerUppercaseAlpha: &'static NSTextListMarkerFormat;
}

extern "C" {
    static NSTextListMarkerLowercaseLatin: &'static NSTextListMarkerFormat;
}

extern "C" {
    static NSTextListMarkerUppercaseLatin: &'static NSTextListMarkerFormat;
}

extern "C" {
    static NSTextListMarkerLowercaseRoman: &'static NSTextListMarkerFormat;
}

extern "C" {
    static NSTextListMarkerUppercaseRoman: &'static NSTextListMarkerFormat;
}

extern "C" {
    static NSTextListMarkerDecimal: &'static NSTextListMarkerFormat;
}

pub type NSTextListOptions = NSUInteger;
pub const NSTextListPrependEnclosingMarker: NSTextListOptions = 1 << 0;

extern_class!(
    #[derive(Debug)]
    pub struct NSTextList;

    unsafe impl ClassType for NSTextList {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSTextList {
        #[method_id(initWithMarkerFormat:options:)]
        pub unsafe fn initWithMarkerFormat_options(
            &self,
            format: &NSTextListMarkerFormat,
            mask: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(markerFormat)]
        pub unsafe fn markerFormat(&self) -> Id<NSTextListMarkerFormat, Shared>;

        #[method(listOptions)]
        pub unsafe fn listOptions(&self) -> NSTextListOptions;

        #[method_id(markerForItemNumber:)]
        pub unsafe fn markerForItemNumber(&self, itemNum: NSInteger) -> Id<NSString, Shared>;

        #[method(startingItemNumber)]
        pub unsafe fn startingItemNumber(&self) -> NSInteger;

        #[method(setStartingItemNumber:)]
        pub unsafe fn setStartingItemNumber(&self, startingItemNumber: NSInteger);
    }
);
