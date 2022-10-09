use crate::AppKit::generated::NSTouchBarItem::*;
use crate::AppKit::generated::NSUserInterfaceCompression::*;
use crate::AppKit::generated::NSUserInterfaceLayout::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSGroupTouchBarItem;
    unsafe impl ClassType for NSGroupTouchBarItem {
        type Super = NSTouchBarItem;
    }
);
extern_methods!(
    unsafe impl NSGroupTouchBarItem {
        #[method_id(groupItemWithIdentifier:items:)]
        pub unsafe fn groupItemWithIdentifier_items(
            identifier: &NSTouchBarItemIdentifier,
            items: &NSArray<NSTouchBarItem>,
        ) -> Id<Self, Shared>;
        #[method_id(groupItemWithIdentifier:items:allowedCompressionOptions:)]
        pub unsafe fn groupItemWithIdentifier_items_allowedCompressionOptions(
            identifier: &NSTouchBarItemIdentifier,
            items: &NSArray<NSTouchBarItem>,
            allowedCompressionOptions: &NSUserInterfaceCompressionOptions,
        ) -> Id<Self, Shared>;
        #[method_id(alertStyleGroupItemWithIdentifier:)]
        pub unsafe fn alertStyleGroupItemWithIdentifier(
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self, Shared>;
        #[method_id(groupTouchBar)]
        pub unsafe fn groupTouchBar(&self) -> Id<NSTouchBar, Shared>;
        #[method(setGroupTouchBar:)]
        pub unsafe fn setGroupTouchBar(&self, groupTouchBar: &NSTouchBar);
        #[method_id(customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString, Shared>;
        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customizationLabel: Option<&NSString>);
        #[method(groupUserInterfaceLayoutDirection)]
        pub unsafe fn groupUserInterfaceLayoutDirection(&self) -> NSUserInterfaceLayoutDirection;
        #[method(setGroupUserInterfaceLayoutDirection:)]
        pub unsafe fn setGroupUserInterfaceLayoutDirection(
            &self,
            groupUserInterfaceLayoutDirection: NSUserInterfaceLayoutDirection,
        );
        #[method(prefersEqualWidths)]
        pub unsafe fn prefersEqualWidths(&self) -> bool;
        #[method(setPrefersEqualWidths:)]
        pub unsafe fn setPrefersEqualWidths(&self, prefersEqualWidths: bool);
        #[method(preferredItemWidth)]
        pub unsafe fn preferredItemWidth(&self) -> CGFloat;
        #[method(setPreferredItemWidth:)]
        pub unsafe fn setPreferredItemWidth(&self, preferredItemWidth: CGFloat);
        #[method_id(effectiveCompressionOptions)]
        pub unsafe fn effectiveCompressionOptions(
            &self,
        ) -> Id<NSUserInterfaceCompressionOptions, Shared>;
        #[method_id(prioritizedCompressionOptions)]
        pub unsafe fn prioritizedCompressionOptions(
            &self,
        ) -> Id<NSArray<NSUserInterfaceCompressionOptions>, Shared>;
        #[method(setPrioritizedCompressionOptions:)]
        pub unsafe fn setPrioritizedCompressionOptions(
            &self,
            prioritizedCompressionOptions: &NSArray<NSUserInterfaceCompressionOptions>,
        );
    }
);
