//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug)]
    pub struct NSTintConfiguration;

    unsafe impl ClassType for NSTintConfiguration {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSTintConfiguration {
        #[method_id(@__retain_semantics Other defaultTintConfiguration)]
        pub unsafe fn defaultTintConfiguration() -> Id<NSTintConfiguration, Shared>;

        #[method_id(@__retain_semantics Other monochromeTintConfiguration)]
        pub unsafe fn monochromeTintConfiguration() -> Id<NSTintConfiguration, Shared>;

        #[method_id(@__retain_semantics Other tintConfigurationWithPreferredColor:)]
        pub unsafe fn tintConfigurationWithPreferredColor(color: &NSColor) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other tintConfigurationWithFixedColor:)]
        pub unsafe fn tintConfigurationWithFixedColor(color: &NSColor) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other baseTintColor)]
        pub unsafe fn baseTintColor(&self) -> Option<Id<NSColor, Shared>>;

        #[method_id(@__retain_semantics Other equivalentContentTintColor)]
        pub unsafe fn equivalentContentTintColor(&self) -> Option<Id<NSColor, Shared>>;

        #[method(adaptsToUserAccentColor)]
        pub unsafe fn adaptsToUserAccentColor(&self) -> bool;
    }
);
