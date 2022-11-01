//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub type NSPersonNameComponentsFormatterStyle = NSInteger;
pub const NSPersonNameComponentsFormatterStyleDefault: NSPersonNameComponentsFormatterStyle = 0;
pub const NSPersonNameComponentsFormatterStyleShort: NSPersonNameComponentsFormatterStyle = 1;
pub const NSPersonNameComponentsFormatterStyleMedium: NSPersonNameComponentsFormatterStyle = 2;
pub const NSPersonNameComponentsFormatterStyleLong: NSPersonNameComponentsFormatterStyle = 3;
pub const NSPersonNameComponentsFormatterStyleAbbreviated: NSPersonNameComponentsFormatterStyle = 4;

pub type NSPersonNameComponentsFormatterOptions = NSUInteger;
pub const NSPersonNameComponentsFormatterPhonetic: NSPersonNameComponentsFormatterOptions = 1 << 1;

extern_class!(
    #[derive(Debug)]
    pub struct NSPersonNameComponentsFormatter;

    unsafe impl ClassType for NSPersonNameComponentsFormatter {
        type Super = NSFormatter;
    }
);

extern_methods!(
    unsafe impl NSPersonNameComponentsFormatter {
        #[method(style)]
        pub unsafe fn style(&self) -> NSPersonNameComponentsFormatterStyle;

        #[method(setStyle:)]
        pub unsafe fn setStyle(&self, style: NSPersonNameComponentsFormatterStyle);

        #[method(isPhonetic)]
        pub unsafe fn isPhonetic(&self) -> bool;

        #[method(setPhonetic:)]
        pub unsafe fn setPhonetic(&self, phonetic: bool);

        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Id<NSLocale, Shared>;

        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[method_id(@__retain_semantics Other localizedStringFromPersonNameComponents:style:options:)]
        pub unsafe fn localizedStringFromPersonNameComponents_style_options(
            components: &NSPersonNameComponents,
            nameFormatStyle: NSPersonNameComponentsFormatterStyle,
            nameOptions: NSPersonNameComponentsFormatterOptions,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other stringFromPersonNameComponents:)]
        pub unsafe fn stringFromPersonNameComponents(
            &self,
            components: &NSPersonNameComponents,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other annotatedStringFromPersonNameComponents:)]
        pub unsafe fn annotatedStringFromPersonNameComponents(
            &self,
            components: &NSPersonNameComponents,
        ) -> Id<NSAttributedString, Shared>;

        #[method_id(@__retain_semantics Other personNameComponentsFromString:)]
        pub unsafe fn personNameComponentsFromString(
            &self,
            string: &NSString,
        ) -> Option<Id<NSPersonNameComponents, Shared>>;

        #[method(getObjectValue:forString:errorDescription:)]
        pub unsafe fn getObjectValue_forString_errorDescription(
            &self,
            obj: Option<&mut Option<Id<Object, Shared>>>,
            string: &NSString,
            error: Option<&mut Option<Id<NSString, Shared>>>,
        ) -> bool;
    }
);

extern "C" {
    pub static NSPersonNameComponentKey: &'static NSString;
}

extern "C" {
    pub static NSPersonNameComponentGivenName: &'static NSString;
}

extern "C" {
    pub static NSPersonNameComponentFamilyName: &'static NSString;
}

extern "C" {
    pub static NSPersonNameComponentMiddleName: &'static NSString;
}

extern "C" {
    pub static NSPersonNameComponentPrefix: &'static NSString;
}

extern "C" {
    pub static NSPersonNameComponentSuffix: &'static NSString;
}

extern "C" {
    pub static NSPersonNameComponentNickname: &'static NSString;
}

extern "C" {
    pub static NSPersonNameComponentDelimiter: &'static NSString;
}
