//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSAppearanceName = NSString;

extern_class!(
    #[derive(Debug)]
    pub struct NSAppearance;

    unsafe impl ClassType for NSAppearance {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSAppearance {
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSAppearanceName, Shared>;

        #[method_id(@__retain_semantics Other currentAppearance)]
        pub unsafe fn currentAppearance() -> Option<Id<NSAppearance, Shared>>;

        #[method(setCurrentAppearance:)]
        pub unsafe fn setCurrentAppearance(currentAppearance: Option<&NSAppearance>);

        #[method_id(@__retain_semantics Other currentDrawingAppearance)]
        pub unsafe fn currentDrawingAppearance() -> Id<NSAppearance, Shared>;

        #[method(performAsCurrentDrawingAppearance:)]
        pub unsafe fn performAsCurrentDrawingAppearance(&self, block: &Block<(), ()>);

        #[method_id(@__retain_semantics Other appearanceNamed:)]
        pub unsafe fn appearanceNamed(name: &NSAppearanceName) -> Option<Id<NSAppearance, Shared>>;

        #[method_id(@__retain_semantics Init initWithAppearanceNamed:bundle:)]
        pub unsafe fn initWithAppearanceNamed_bundle(
            this: Option<Allocated<Self>>,
            name: &NSAppearanceName,
            bundle: Option<&NSBundle>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method(allowsVibrancy)]
        pub unsafe fn allowsVibrancy(&self) -> bool;

        #[method_id(@__retain_semantics Other bestMatchFromAppearancesWithNames:)]
        pub unsafe fn bestMatchFromAppearancesWithNames(
            &self,
            appearances: &NSArray<NSAppearanceName>,
        ) -> Option<Id<NSAppearanceName, Shared>>;
    }
);

extern_static!(NSAppearanceNameAqua: &'static NSAppearanceName);

extern_static!(NSAppearanceNameDarkAqua: &'static NSAppearanceName);

extern_static!(NSAppearanceNameLightContent: &'static NSAppearanceName);

extern_static!(NSAppearanceNameVibrantDark: &'static NSAppearanceName);

extern_static!(NSAppearanceNameVibrantLight: &'static NSAppearanceName);

extern_static!(NSAppearanceNameAccessibilityHighContrastAqua: &'static NSAppearanceName);

extern_static!(NSAppearanceNameAccessibilityHighContrastDarkAqua: &'static NSAppearanceName);

extern_static!(NSAppearanceNameAccessibilityHighContrastVibrantLight: &'static NSAppearanceName);

extern_static!(NSAppearanceNameAccessibilityHighContrastVibrantDark: &'static NSAppearanceName);

pub type NSAppearanceCustomization = NSObject;
