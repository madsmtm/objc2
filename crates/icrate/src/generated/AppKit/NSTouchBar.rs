#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSTouchBarCustomizationIdentifier = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSTouchBar;
    unsafe impl ClassType for NSTouchBar {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTouchBar {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(customizationIdentifier)]
        pub unsafe fn customizationIdentifier(
            &self,
        ) -> Option<Id<NSTouchBarCustomizationIdentifier, Shared>>;
        #[method(setCustomizationIdentifier:)]
        pub unsafe fn setCustomizationIdentifier(
            &self,
            customizationIdentifier: Option<&NSTouchBarCustomizationIdentifier>,
        );
        #[method_id(customizationAllowedItemIdentifiers)]
        pub unsafe fn customizationAllowedItemIdentifiers(
            &self,
        ) -> Id<NSArray<NSTouchBarItemIdentifier>, Shared>;
        #[method(setCustomizationAllowedItemIdentifiers:)]
        pub unsafe fn setCustomizationAllowedItemIdentifiers(
            &self,
            customizationAllowedItemIdentifiers: &NSArray<NSTouchBarItemIdentifier>,
        );
        #[method_id(customizationRequiredItemIdentifiers)]
        pub unsafe fn customizationRequiredItemIdentifiers(
            &self,
        ) -> Id<NSArray<NSTouchBarItemIdentifier>, Shared>;
        #[method(setCustomizationRequiredItemIdentifiers:)]
        pub unsafe fn setCustomizationRequiredItemIdentifiers(
            &self,
            customizationRequiredItemIdentifiers: &NSArray<NSTouchBarItemIdentifier>,
        );
        #[method_id(defaultItemIdentifiers)]
        pub unsafe fn defaultItemIdentifiers(
            &self,
        ) -> Id<NSArray<NSTouchBarItemIdentifier>, Shared>;
        #[method(setDefaultItemIdentifiers:)]
        pub unsafe fn setDefaultItemIdentifiers(
            &self,
            defaultItemIdentifiers: &NSArray<NSTouchBarItemIdentifier>,
        );
        #[method_id(itemIdentifiers)]
        pub unsafe fn itemIdentifiers(&self) -> Id<NSArray<NSTouchBarItemIdentifier>, Shared>;
        #[method_id(principalItemIdentifier)]
        pub unsafe fn principalItemIdentifier(
            &self,
        ) -> Option<Id<NSTouchBarItemIdentifier, Shared>>;
        #[method(setPrincipalItemIdentifier:)]
        pub unsafe fn setPrincipalItemIdentifier(
            &self,
            principalItemIdentifier: Option<&NSTouchBarItemIdentifier>,
        );
        #[method_id(escapeKeyReplacementItemIdentifier)]
        pub unsafe fn escapeKeyReplacementItemIdentifier(
            &self,
        ) -> Option<Id<NSTouchBarItemIdentifier, Shared>>;
        #[method(setEscapeKeyReplacementItemIdentifier:)]
        pub unsafe fn setEscapeKeyReplacementItemIdentifier(
            &self,
            escapeKeyReplacementItemIdentifier: Option<&NSTouchBarItemIdentifier>,
        );
        #[method_id(templateItems)]
        pub unsafe fn templateItems(&self) -> Id<NSSet<NSTouchBarItem>, Shared>;
        #[method(setTemplateItems:)]
        pub unsafe fn setTemplateItems(&self, templateItems: &NSSet<NSTouchBarItem>);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSTouchBarDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSTouchBarDelegate>);
        #[method_id(itemForIdentifier:)]
        pub unsafe fn itemForIdentifier(
            &self,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Option<Id<NSTouchBarItem, Shared>>;
        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;
        #[method(isAutomaticCustomizeTouchBarMenuItemEnabled)]
        pub unsafe fn isAutomaticCustomizeTouchBarMenuItemEnabled() -> bool;
        #[method(setAutomaticCustomizeTouchBarMenuItemEnabled:)]
        pub unsafe fn setAutomaticCustomizeTouchBarMenuItemEnabled(
            automaticCustomizeTouchBarMenuItemEnabled: bool,
        );
    }
);
pub type NSTouchBarDelegate = NSObject;
pub type NSTouchBarProvider = NSObject;
extern_methods!(
    #[doc = "NSTouchBarProvider"]
    unsafe impl NSResponder {
        #[method_id(touchBar)]
        pub unsafe fn touchBar(&self) -> Option<Id<NSTouchBar, Shared>>;
        #[method(setTouchBar:)]
        pub unsafe fn setTouchBar(&self, touchBar: Option<&NSTouchBar>);
        #[method_id(makeTouchBar)]
        pub unsafe fn makeTouchBar(&self) -> Option<Id<NSTouchBar, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSTouchBarCustomization"]
    unsafe impl NSApplication {
        #[method(isAutomaticCustomizeTouchBarMenuItemEnabled)]
        pub unsafe fn isAutomaticCustomizeTouchBarMenuItemEnabled(&self) -> bool;
        #[method(setAutomaticCustomizeTouchBarMenuItemEnabled:)]
        pub unsafe fn setAutomaticCustomizeTouchBarMenuItemEnabled(
            &self,
            automaticCustomizeTouchBarMenuItemEnabled: bool,
        );
        #[method(toggleTouchBarCustomizationPalette:)]
        pub unsafe fn toggleTouchBarCustomizationPalette(&self, sender: Option<&Object>);
    }
);
