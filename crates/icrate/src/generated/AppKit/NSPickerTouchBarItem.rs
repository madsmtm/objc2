#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPickerTouchBarItem;
    unsafe impl ClassType for NSPickerTouchBarItem {
        type Super = NSTouchBarItem;
    }
);
extern_methods!(
    unsafe impl NSPickerTouchBarItem {
        #[method_id(pickerTouchBarItemWithIdentifier:labels:selectionMode:target:action:)]
        pub unsafe fn pickerTouchBarItemWithIdentifier_labels_selectionMode_target_action(
            identifier: &NSTouchBarItemIdentifier,
            labels: &NSArray<NSString>,
            selectionMode: NSPickerTouchBarItemSelectionMode,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;
        #[method_id(pickerTouchBarItemWithIdentifier:images:selectionMode:target:action:)]
        pub unsafe fn pickerTouchBarItemWithIdentifier_images_selectionMode_target_action(
            identifier: &NSTouchBarItemIdentifier,
            images: &NSArray<NSImage>,
            selectionMode: NSPickerTouchBarItemSelectionMode,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;
        #[method(controlRepresentation)]
        pub unsafe fn controlRepresentation(&self) -> NSPickerTouchBarItemControlRepresentation;
        #[method(setControlRepresentation:)]
        pub unsafe fn setControlRepresentation(
            &self,
            controlRepresentation: NSPickerTouchBarItemControlRepresentation,
        );
        #[method_id(collapsedRepresentationLabel)]
        pub unsafe fn collapsedRepresentationLabel(&self) -> Id<NSString, Shared>;
        #[method(setCollapsedRepresentationLabel:)]
        pub unsafe fn setCollapsedRepresentationLabel(
            &self,
            collapsedRepresentationLabel: &NSString,
        );
        #[method_id(collapsedRepresentationImage)]
        pub unsafe fn collapsedRepresentationImage(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setCollapsedRepresentationImage:)]
        pub unsafe fn setCollapsedRepresentationImage(
            &self,
            collapsedRepresentationImage: Option<&NSImage>,
        );
        #[method(selectedIndex)]
        pub unsafe fn selectedIndex(&self) -> NSInteger;
        #[method(setSelectedIndex:)]
        pub unsafe fn setSelectedIndex(&self, selectedIndex: NSInteger);
        #[method_id(selectionColor)]
        pub unsafe fn selectionColor(&self) -> Option<Id<NSColor, Shared>>;
        #[method(setSelectionColor:)]
        pub unsafe fn setSelectionColor(&self, selectionColor: Option<&NSColor>);
        #[method(selectionMode)]
        pub unsafe fn selectionMode(&self) -> NSPickerTouchBarItemSelectionMode;
        #[method(setSelectionMode:)]
        pub unsafe fn setSelectionMode(&self, selectionMode: NSPickerTouchBarItemSelectionMode);
        #[method(numberOfOptions)]
        pub unsafe fn numberOfOptions(&self) -> NSInteger;
        #[method(setNumberOfOptions:)]
        pub unsafe fn setNumberOfOptions(&self, numberOfOptions: NSInteger);
        #[method(setImage:atIndex:)]
        pub unsafe fn setImage_atIndex(&self, image: Option<&NSImage>, index: NSInteger);
        #[method_id(imageAtIndex:)]
        pub unsafe fn imageAtIndex(&self, index: NSInteger) -> Option<Id<NSImage, Shared>>;
        #[method(setLabel:atIndex:)]
        pub unsafe fn setLabel_atIndex(&self, label: &NSString, index: NSInteger);
        #[method_id(labelAtIndex:)]
        pub unsafe fn labelAtIndex(&self, index: NSInteger) -> Option<Id<NSString, Shared>>;
        #[method_id(target)]
        pub unsafe fn target(&self) -> Option<Id<Object, Shared>>;
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);
        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;
        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);
        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;
        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);
        #[method(setEnabled:atIndex:)]
        pub unsafe fn setEnabled_atIndex(&self, enabled: bool, index: NSInteger);
        #[method(isEnabledAtIndex:)]
        pub unsafe fn isEnabledAtIndex(&self, index: NSInteger) -> bool;
        #[method_id(customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString, Shared>;
        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customizationLabel: Option<&NSString>);
    }
);
