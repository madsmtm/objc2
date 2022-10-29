#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSSplitViewAutosaveName = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSSplitView;
    unsafe impl ClassType for NSSplitView {
        type Super = NSView;
    }
);
extern_methods!(
    unsafe impl NSSplitView {
        #[method(isVertical)]
        pub unsafe fn isVertical(&self) -> bool;
        #[method(setVertical:)]
        pub unsafe fn setVertical(&self, vertical: bool);
        #[method(dividerStyle)]
        pub unsafe fn dividerStyle(&self) -> NSSplitViewDividerStyle;
        #[method(setDividerStyle:)]
        pub unsafe fn setDividerStyle(&self, dividerStyle: NSSplitViewDividerStyle);
        #[method_id(autosaveName)]
        pub unsafe fn autosaveName(&self) -> Option<Id<NSSplitViewAutosaveName, Shared>>;
        #[method(setAutosaveName:)]
        pub unsafe fn setAutosaveName(&self, autosaveName: Option<&NSSplitViewAutosaveName>);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSSplitViewDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSSplitViewDelegate>);
        #[method(drawDividerInRect:)]
        pub unsafe fn drawDividerInRect(&self, rect: NSRect);
        #[method_id(dividerColor)]
        pub unsafe fn dividerColor(&self) -> Id<NSColor, Shared>;
        #[method(dividerThickness)]
        pub unsafe fn dividerThickness(&self) -> CGFloat;
        #[method(adjustSubviews)]
        pub unsafe fn adjustSubviews(&self);
        #[method(isSubviewCollapsed:)]
        pub unsafe fn isSubviewCollapsed(&self, subview: &NSView) -> bool;
        #[method(minPossiblePositionOfDividerAtIndex:)]
        pub unsafe fn minPossiblePositionOfDividerAtIndex(
            &self,
            dividerIndex: NSInteger,
        ) -> CGFloat;
        #[method(maxPossiblePositionOfDividerAtIndex:)]
        pub unsafe fn maxPossiblePositionOfDividerAtIndex(
            &self,
            dividerIndex: NSInteger,
        ) -> CGFloat;
        #[method(setPosition:ofDividerAtIndex:)]
        pub unsafe fn setPosition_ofDividerAtIndex(
            &self,
            position: CGFloat,
            dividerIndex: NSInteger,
        );
        #[method(holdingPriorityForSubviewAtIndex:)]
        pub unsafe fn holdingPriorityForSubviewAtIndex(
            &self,
            subviewIndex: NSInteger,
        ) -> NSLayoutPriority;
        #[method(setHoldingPriority:forSubviewAtIndex:)]
        pub unsafe fn setHoldingPriority_forSubviewAtIndex(
            &self,
            priority: NSLayoutPriority,
            subviewIndex: NSInteger,
        );
    }
);
extern_methods!(
    #[doc = "NSSplitViewArrangedSubviews"]
    unsafe impl NSSplitView {
        #[method(arrangesAllSubviews)]
        pub unsafe fn arrangesAllSubviews(&self) -> bool;
        #[method(setArrangesAllSubviews:)]
        pub unsafe fn setArrangesAllSubviews(&self, arrangesAllSubviews: bool);
        #[method_id(arrangedSubviews)]
        pub unsafe fn arrangedSubviews(&self) -> Id<NSArray<NSView>, Shared>;
        #[method(addArrangedSubview:)]
        pub unsafe fn addArrangedSubview(&self, view: &NSView);
        #[method(insertArrangedSubview:atIndex:)]
        pub unsafe fn insertArrangedSubview_atIndex(&self, view: &NSView, index: NSInteger);
        #[method(removeArrangedSubview:)]
        pub unsafe fn removeArrangedSubview(&self, view: &NSView);
    }
);
pub type NSSplitViewDelegate = NSObject;
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSSplitView {
        #[method(setIsPaneSplitter:)]
        pub unsafe fn setIsPaneSplitter(&self, flag: bool);
        #[method(isPaneSplitter)]
        pub unsafe fn isPaneSplitter(&self) -> bool;
    }
);
