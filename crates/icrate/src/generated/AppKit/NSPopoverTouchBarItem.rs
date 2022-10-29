#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPopoverTouchBarItem;
    unsafe impl ClassType for NSPopoverTouchBarItem {
        type Super = NSTouchBarItem;
    }
);
extern_methods!(
    unsafe impl NSPopoverTouchBarItem {
        #[method_id(popoverTouchBar)]
        pub unsafe fn popoverTouchBar(&self) -> Id<NSTouchBar, Shared>;
        #[method(setPopoverTouchBar:)]
        pub unsafe fn setPopoverTouchBar(&self, popoverTouchBar: &NSTouchBar);
        #[method_id(customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString, Shared>;
        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customizationLabel: Option<&NSString>);
        #[method_id(collapsedRepresentation)]
        pub unsafe fn collapsedRepresentation(&self) -> Id<NSView, Shared>;
        #[method(setCollapsedRepresentation:)]
        pub unsafe fn setCollapsedRepresentation(&self, collapsedRepresentation: &NSView);
        #[method_id(collapsedRepresentationImage)]
        pub unsafe fn collapsedRepresentationImage(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setCollapsedRepresentationImage:)]
        pub unsafe fn setCollapsedRepresentationImage(
            &self,
            collapsedRepresentationImage: Option<&NSImage>,
        );
        #[method_id(collapsedRepresentationLabel)]
        pub unsafe fn collapsedRepresentationLabel(&self) -> Id<NSString, Shared>;
        #[method(setCollapsedRepresentationLabel:)]
        pub unsafe fn setCollapsedRepresentationLabel(
            &self,
            collapsedRepresentationLabel: &NSString,
        );
        #[method_id(pressAndHoldTouchBar)]
        pub unsafe fn pressAndHoldTouchBar(&self) -> Option<Id<NSTouchBar, Shared>>;
        #[method(setPressAndHoldTouchBar:)]
        pub unsafe fn setPressAndHoldTouchBar(&self, pressAndHoldTouchBar: Option<&NSTouchBar>);
        #[method(showsCloseButton)]
        pub unsafe fn showsCloseButton(&self) -> bool;
        #[method(setShowsCloseButton:)]
        pub unsafe fn setShowsCloseButton(&self, showsCloseButton: bool);
        #[method(showPopover:)]
        pub unsafe fn showPopover(&self, sender: Option<&Object>);
        #[method(dismissPopover:)]
        pub unsafe fn dismissPopover(&self, sender: Option<&Object>);
        #[method_id(makeStandardActivatePopoverGestureRecognizer)]
        pub unsafe fn makeStandardActivatePopoverGestureRecognizer(
            &self,
        ) -> Id<NSGestureRecognizer, Shared>;
    }
);
