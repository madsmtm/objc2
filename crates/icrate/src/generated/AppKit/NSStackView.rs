#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSStackView;
    unsafe impl ClassType for NSStackView {
        type Super = NSView;
    }
);
extern_methods!(
    unsafe impl NSStackView {
        #[method_id(stackViewWithViews:)]
        pub unsafe fn stackViewWithViews(views: &NSArray<NSView>) -> Id<Self, Shared>;
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSStackViewDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSStackViewDelegate>);
        #[method(orientation)]
        pub unsafe fn orientation(&self) -> NSUserInterfaceLayoutOrientation;
        #[method(setOrientation:)]
        pub unsafe fn setOrientation(&self, orientation: NSUserInterfaceLayoutOrientation);
        #[method(alignment)]
        pub unsafe fn alignment(&self) -> NSLayoutAttribute;
        #[method(setAlignment:)]
        pub unsafe fn setAlignment(&self, alignment: NSLayoutAttribute);
        #[method(edgeInsets)]
        pub unsafe fn edgeInsets(&self) -> NSEdgeInsets;
        #[method(setEdgeInsets:)]
        pub unsafe fn setEdgeInsets(&self, edgeInsets: NSEdgeInsets);
        #[method(distribution)]
        pub unsafe fn distribution(&self) -> NSStackViewDistribution;
        #[method(setDistribution:)]
        pub unsafe fn setDistribution(&self, distribution: NSStackViewDistribution);
        #[method(spacing)]
        pub unsafe fn spacing(&self) -> CGFloat;
        #[method(setSpacing:)]
        pub unsafe fn setSpacing(&self, spacing: CGFloat);
        #[method(setCustomSpacing:afterView:)]
        pub unsafe fn setCustomSpacing_afterView(&self, spacing: CGFloat, view: &NSView);
        #[method(customSpacingAfterView:)]
        pub unsafe fn customSpacingAfterView(&self, view: &NSView) -> CGFloat;
        #[method(detachesHiddenViews)]
        pub unsafe fn detachesHiddenViews(&self) -> bool;
        #[method(setDetachesHiddenViews:)]
        pub unsafe fn setDetachesHiddenViews(&self, detachesHiddenViews: bool);
        #[method_id(arrangedSubviews)]
        pub unsafe fn arrangedSubviews(&self) -> Id<NSArray<NSView>, Shared>;
        #[method(addArrangedSubview:)]
        pub unsafe fn addArrangedSubview(&self, view: &NSView);
        #[method(insertArrangedSubview:atIndex:)]
        pub unsafe fn insertArrangedSubview_atIndex(&self, view: &NSView, index: NSInteger);
        #[method(removeArrangedSubview:)]
        pub unsafe fn removeArrangedSubview(&self, view: &NSView);
        #[method_id(detachedViews)]
        pub unsafe fn detachedViews(&self) -> Id<NSArray<NSView>, Shared>;
        #[method(setVisibilityPriority:forView:)]
        pub unsafe fn setVisibilityPriority_forView(
            &self,
            priority: NSStackViewVisibilityPriority,
            view: &NSView,
        );
        #[method(visibilityPriorityForView:)]
        pub unsafe fn visibilityPriorityForView(
            &self,
            view: &NSView,
        ) -> NSStackViewVisibilityPriority;
        #[method(clippingResistancePriorityForOrientation:)]
        pub unsafe fn clippingResistancePriorityForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> NSLayoutPriority;
        #[method(setClippingResistancePriority:forOrientation:)]
        pub unsafe fn setClippingResistancePriority_forOrientation(
            &self,
            clippingResistancePriority: NSLayoutPriority,
            orientation: NSLayoutConstraintOrientation,
        );
        #[method(huggingPriorityForOrientation:)]
        pub unsafe fn huggingPriorityForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> NSLayoutPriority;
        #[method(setHuggingPriority:forOrientation:)]
        pub unsafe fn setHuggingPriority_forOrientation(
            &self,
            huggingPriority: NSLayoutPriority,
            orientation: NSLayoutConstraintOrientation,
        );
    }
);
pub type NSStackViewDelegate = NSObject;
extern_methods!(
    #[doc = "NSStackViewGravityAreas"]
    unsafe impl NSStackView {
        #[method(addView:inGravity:)]
        pub unsafe fn addView_inGravity(&self, view: &NSView, gravity: NSStackViewGravity);
        #[method(insertView:atIndex:inGravity:)]
        pub unsafe fn insertView_atIndex_inGravity(
            &self,
            view: &NSView,
            index: NSUInteger,
            gravity: NSStackViewGravity,
        );
        #[method(removeView:)]
        pub unsafe fn removeView(&self, view: &NSView);
        #[method_id(viewsInGravity:)]
        pub unsafe fn viewsInGravity(
            &self,
            gravity: NSStackViewGravity,
        ) -> Id<NSArray<NSView>, Shared>;
        #[method(setViews:inGravity:)]
        pub unsafe fn setViews_inGravity(
            &self,
            views: &NSArray<NSView>,
            gravity: NSStackViewGravity,
        );
        #[method_id(views)]
        pub unsafe fn views(&self) -> Id<NSArray<NSView>, Shared>;
    }
);
extern_methods!(
    #[doc = "NSStackViewDeprecated"]
    unsafe impl NSStackView {
        #[method(hasEqualSpacing)]
        pub unsafe fn hasEqualSpacing(&self) -> bool;
        #[method(setHasEqualSpacing:)]
        pub unsafe fn setHasEqualSpacing(&self, hasEqualSpacing: bool);
    }
);
