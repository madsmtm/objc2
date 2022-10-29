#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTextSelection;
    unsafe impl ClassType for NSTextSelection {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTextSelection {
        #[method_id(initWithRanges:affinity:granularity:)]
        pub unsafe fn initWithRanges_affinity_granularity(
            &self,
            textRanges: &NSArray<NSTextRange>,
            affinity: NSTextSelectionAffinity,
            granularity: NSTextSelectionGranularity,
        ) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(initWithRange:affinity:granularity:)]
        pub unsafe fn initWithRange_affinity_granularity(
            &self,
            range: &NSTextRange,
            affinity: NSTextSelectionAffinity,
            granularity: NSTextSelectionGranularity,
        ) -> Id<Self, Shared>;
        #[method_id(initWithLocation:affinity:)]
        pub unsafe fn initWithLocation_affinity(
            &self,
            location: &NSTextLocation,
            affinity: NSTextSelectionAffinity,
        ) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(textRanges)]
        pub unsafe fn textRanges(&self) -> Id<NSArray<NSTextRange>, Shared>;
        #[method(granularity)]
        pub unsafe fn granularity(&self) -> NSTextSelectionGranularity;
        #[method(affinity)]
        pub unsafe fn affinity(&self) -> NSTextSelectionAffinity;
        #[method(isTransient)]
        pub unsafe fn isTransient(&self) -> bool;
        #[method(anchorPositionOffset)]
        pub unsafe fn anchorPositionOffset(&self) -> CGFloat;
        #[method(setAnchorPositionOffset:)]
        pub unsafe fn setAnchorPositionOffset(&self, anchorPositionOffset: CGFloat);
        #[method(isLogical)]
        pub unsafe fn isLogical(&self) -> bool;
        #[method(setLogical:)]
        pub unsafe fn setLogical(&self, logical: bool);
        #[method_id(secondarySelectionLocation)]
        pub unsafe fn secondarySelectionLocation(&self) -> Option<Id<NSTextLocation, Shared>>;
        #[method(setSecondarySelectionLocation:)]
        pub unsafe fn setSecondarySelectionLocation(
            &self,
            secondarySelectionLocation: Option<&NSTextLocation>,
        );
        #[method_id(typingAttributes)]
        pub unsafe fn typingAttributes(
            &self,
        ) -> Id<NSDictionary<NSAttributedStringKey, Object>, Shared>;
        #[method(setTypingAttributes:)]
        pub unsafe fn setTypingAttributes(
            &self,
            typingAttributes: &NSDictionary<NSAttributedStringKey, Object>,
        );
        #[method_id(textSelectionWithTextRanges:)]
        pub unsafe fn textSelectionWithTextRanges(
            &self,
            textRanges: &NSArray<NSTextRange>,
        ) -> Id<NSTextSelection, Shared>;
    }
);