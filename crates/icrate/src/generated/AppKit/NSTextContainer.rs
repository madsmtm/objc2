#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTextContainer;
    unsafe impl ClassType for NSTextContainer {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTextContainer {
        #[method_id(initWithSize:)]
        pub unsafe fn initWithSize(&self, size: NSSize) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Id<Self, Shared>;
        #[method_id(layoutManager)]
        pub unsafe fn layoutManager(&self) -> Option<Id<NSLayoutManager, Shared>>;
        #[method(setLayoutManager:)]
        pub unsafe fn setLayoutManager(&self, layoutManager: Option<&NSLayoutManager>);
        #[method(replaceLayoutManager:)]
        pub unsafe fn replaceLayoutManager(&self, newLayoutManager: &NSLayoutManager);
        #[method_id(textLayoutManager)]
        pub unsafe fn textLayoutManager(&self) -> Option<Id<NSTextLayoutManager, Shared>>;
        #[method(size)]
        pub unsafe fn size(&self) -> NSSize;
        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: NSSize);
        #[method_id(exclusionPaths)]
        pub unsafe fn exclusionPaths(&self) -> Id<NSArray<NSBezierPath>, Shared>;
        #[method(setExclusionPaths:)]
        pub unsafe fn setExclusionPaths(&self, exclusionPaths: &NSArray<NSBezierPath>);
        #[method(lineBreakMode)]
        pub unsafe fn lineBreakMode(&self) -> NSLineBreakMode;
        #[method(setLineBreakMode:)]
        pub unsafe fn setLineBreakMode(&self, lineBreakMode: NSLineBreakMode);
        #[method(lineFragmentPadding)]
        pub unsafe fn lineFragmentPadding(&self) -> CGFloat;
        #[method(setLineFragmentPadding:)]
        pub unsafe fn setLineFragmentPadding(&self, lineFragmentPadding: CGFloat);
        #[method(maximumNumberOfLines)]
        pub unsafe fn maximumNumberOfLines(&self) -> NSUInteger;
        #[method(setMaximumNumberOfLines:)]
        pub unsafe fn setMaximumNumberOfLines(&self, maximumNumberOfLines: NSUInteger);
        #[method(lineFragmentRectForProposedRect:atIndex:writingDirection:remainingRect:)]
        pub unsafe fn lineFragmentRectForProposedRect_atIndex_writingDirection_remainingRect(
            &self,
            proposedRect: NSRect,
            characterIndex: NSUInteger,
            baseWritingDirection: NSWritingDirection,
            remainingRect: *mut NSRect,
        ) -> NSRect;
        #[method(isSimpleRectangularTextContainer)]
        pub unsafe fn isSimpleRectangularTextContainer(&self) -> bool;
        #[method(widthTracksTextView)]
        pub unsafe fn widthTracksTextView(&self) -> bool;
        #[method(setWidthTracksTextView:)]
        pub unsafe fn setWidthTracksTextView(&self, widthTracksTextView: bool);
        #[method(heightTracksTextView)]
        pub unsafe fn heightTracksTextView(&self) -> bool;
        #[method(setHeightTracksTextView:)]
        pub unsafe fn setHeightTracksTextView(&self, heightTracksTextView: bool);
        #[method_id(textView)]
        pub unsafe fn textView(&self) -> Option<Id<NSTextView, Shared>>;
        #[method(setTextView:)]
        pub unsafe fn setTextView(&self, textView: Option<&NSTextView>);
    }
);
extern_methods!(
    #[doc = "NSTextContainerDeprecated"]
    unsafe impl NSTextContainer {
        #[method_id(initWithContainerSize:)]
        pub unsafe fn initWithContainerSize(&self, aContainerSize: NSSize) -> Id<Self, Shared>;
        #[method(containerSize)]
        pub unsafe fn containerSize(&self) -> NSSize;
        #[method(setContainerSize:)]
        pub unsafe fn setContainerSize(&self, containerSize: NSSize);
        #[method(lineFragmentRectForProposedRect:sweepDirection:movementDirection:remainingRect:)]
        pub unsafe fn lineFragmentRectForProposedRect_sweepDirection_movementDirection_remainingRect(
            &self,
            proposedRect: NSRect,
            sweepDirection: NSLineSweepDirection,
            movementDirection: NSLineMovementDirection,
            remainingRect: NSRectPointer,
        ) -> NSRect;
        #[method(containsPoint:)]
        pub unsafe fn containsPoint(&self, point: NSPoint) -> bool;
    }
);