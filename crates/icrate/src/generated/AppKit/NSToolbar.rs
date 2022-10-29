#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSToolbarIdentifier = NSString;
pub type NSToolbarItemIdentifier = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSToolbar;
    unsafe impl ClassType for NSToolbar {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSToolbar {
        #[method_id(initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            &self,
            identifier: &NSToolbarIdentifier,
        ) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method(insertItemWithItemIdentifier:atIndex:)]
        pub unsafe fn insertItemWithItemIdentifier_atIndex(
            &self,
            itemIdentifier: &NSToolbarItemIdentifier,
            index: NSInteger,
        );
        #[method(removeItemAtIndex:)]
        pub unsafe fn removeItemAtIndex(&self, index: NSInteger);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSToolbarDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSToolbarDelegate>);
        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;
        #[method(setVisible:)]
        pub unsafe fn setVisible(&self, visible: bool);
        #[method(runCustomizationPalette:)]
        pub unsafe fn runCustomizationPalette(&self, sender: Option<&Object>);
        #[method(customizationPaletteIsRunning)]
        pub unsafe fn customizationPaletteIsRunning(&self) -> bool;
        #[method(displayMode)]
        pub unsafe fn displayMode(&self) -> NSToolbarDisplayMode;
        #[method(setDisplayMode:)]
        pub unsafe fn setDisplayMode(&self, displayMode: NSToolbarDisplayMode);
        #[method_id(selectedItemIdentifier)]
        pub unsafe fn selectedItemIdentifier(&self) -> Option<Id<NSToolbarItemIdentifier, Shared>>;
        #[method(setSelectedItemIdentifier:)]
        pub unsafe fn setSelectedItemIdentifier(
            &self,
            selectedItemIdentifier: Option<&NSToolbarItemIdentifier>,
        );
        #[method(sizeMode)]
        pub unsafe fn sizeMode(&self) -> NSToolbarSizeMode;
        #[method(setSizeMode:)]
        pub unsafe fn setSizeMode(&self, sizeMode: NSToolbarSizeMode);
        #[method(showsBaselineSeparator)]
        pub unsafe fn showsBaselineSeparator(&self) -> bool;
        #[method(setShowsBaselineSeparator:)]
        pub unsafe fn setShowsBaselineSeparator(&self, showsBaselineSeparator: bool);
        #[method(allowsUserCustomization)]
        pub unsafe fn allowsUserCustomization(&self) -> bool;
        #[method(setAllowsUserCustomization:)]
        pub unsafe fn setAllowsUserCustomization(&self, allowsUserCustomization: bool);
        #[method_id(identifier)]
        pub unsafe fn identifier(&self) -> Id<NSToolbarIdentifier, Shared>;
        #[method_id(items)]
        pub unsafe fn items(&self) -> Id<NSArray<NSToolbarItem>, Shared>;
        #[method_id(visibleItems)]
        pub unsafe fn visibleItems(&self) -> Option<Id<NSArray<NSToolbarItem>, Shared>>;
        #[method_id(centeredItemIdentifier)]
        pub unsafe fn centeredItemIdentifier(&self) -> Option<Id<NSToolbarItemIdentifier, Shared>>;
        #[method(setCenteredItemIdentifier:)]
        pub unsafe fn setCenteredItemIdentifier(
            &self,
            centeredItemIdentifier: Option<&NSToolbarItemIdentifier>,
        );
        #[method(autosavesConfiguration)]
        pub unsafe fn autosavesConfiguration(&self) -> bool;
        #[method(setAutosavesConfiguration:)]
        pub unsafe fn setAutosavesConfiguration(&self, autosavesConfiguration: bool);
        #[method(setConfigurationFromDictionary:)]
        pub unsafe fn setConfigurationFromDictionary(
            &self,
            configDict: &NSDictionary<NSString, Object>,
        );
        #[method_id(configurationDictionary)]
        pub unsafe fn configurationDictionary(&self) -> Id<NSDictionary<NSString, Object>, Shared>;
        #[method(validateVisibleItems)]
        pub unsafe fn validateVisibleItems(&self);
        #[method(allowsExtensionItems)]
        pub unsafe fn allowsExtensionItems(&self) -> bool;
        #[method(setAllowsExtensionItems:)]
        pub unsafe fn setAllowsExtensionItems(&self, allowsExtensionItems: bool);
    }
);
pub type NSToolbarDelegate = NSObject;
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSToolbar {
        #[method_id(fullScreenAccessoryView)]
        pub unsafe fn fullScreenAccessoryView(&self) -> Option<Id<NSView, Shared>>;
        #[method(setFullScreenAccessoryView:)]
        pub unsafe fn setFullScreenAccessoryView(&self, fullScreenAccessoryView: Option<&NSView>);
        #[method(fullScreenAccessoryViewMinHeight)]
        pub unsafe fn fullScreenAccessoryViewMinHeight(&self) -> CGFloat;
        #[method(setFullScreenAccessoryViewMinHeight:)]
        pub unsafe fn setFullScreenAccessoryViewMinHeight(
            &self,
            fullScreenAccessoryViewMinHeight: CGFloat,
        );
        #[method(fullScreenAccessoryViewMaxHeight)]
        pub unsafe fn fullScreenAccessoryViewMaxHeight(&self) -> CGFloat;
        #[method(setFullScreenAccessoryViewMaxHeight:)]
        pub unsafe fn setFullScreenAccessoryViewMaxHeight(
            &self,
            fullScreenAccessoryViewMaxHeight: CGFloat,
        );
    }
);