//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};

static NSLayoutPriorityRequired: NSLayoutPriority = 1000;

static NSLayoutPriorityDefaultHigh: NSLayoutPriority = 750;

static NSLayoutPriorityDragThatCanResizeWindow: NSLayoutPriority = 510;

static NSLayoutPriorityWindowSizeStayPut: NSLayoutPriority = 500;

static NSLayoutPriorityDragThatCannotResizeWindow: NSLayoutPriority = 490;

static NSLayoutPriorityDefaultLow: NSLayoutPriority = 250;

static NSLayoutPriorityFittingSizeCompression: NSLayoutPriority = 50;

pub type NSLayoutConstraintOrientation = NSInteger;
pub const NSLayoutConstraintOrientationHorizontal: NSLayoutConstraintOrientation = 0;
pub const NSLayoutConstraintOrientationVertical: NSLayoutConstraintOrientation = 1;

pub type NSLayoutRelation = NSInteger;
pub const NSLayoutRelationLessThanOrEqual: NSLayoutRelation = -1;
pub const NSLayoutRelationEqual: NSLayoutRelation = 0;
pub const NSLayoutRelationGreaterThanOrEqual: NSLayoutRelation = 1;

pub type NSLayoutAttribute = NSInteger;
pub const NSLayoutAttributeLeft: NSLayoutAttribute = 1;
pub const NSLayoutAttributeRight: NSLayoutAttribute = 2;
pub const NSLayoutAttributeTop: NSLayoutAttribute = 3;
pub const NSLayoutAttributeBottom: NSLayoutAttribute = 4;
pub const NSLayoutAttributeLeading: NSLayoutAttribute = 5;
pub const NSLayoutAttributeTrailing: NSLayoutAttribute = 6;
pub const NSLayoutAttributeWidth: NSLayoutAttribute = 7;
pub const NSLayoutAttributeHeight: NSLayoutAttribute = 8;
pub const NSLayoutAttributeCenterX: NSLayoutAttribute = 9;
pub const NSLayoutAttributeCenterY: NSLayoutAttribute = 10;
pub const NSLayoutAttributeLastBaseline: NSLayoutAttribute = 11;
pub const NSLayoutAttributeBaseline: NSLayoutAttribute = NSLayoutAttributeLastBaseline;
pub const NSLayoutAttributeFirstBaseline: NSLayoutAttribute = 12;
pub const NSLayoutAttributeNotAnAttribute: NSLayoutAttribute = 0;

pub type NSLayoutFormatOptions = NSUInteger;
pub const NSLayoutFormatAlignAllLeft: NSLayoutFormatOptions = 1 << NSLayoutAttributeLeft;
pub const NSLayoutFormatAlignAllRight: NSLayoutFormatOptions = 1 << NSLayoutAttributeRight;
pub const NSLayoutFormatAlignAllTop: NSLayoutFormatOptions = 1 << NSLayoutAttributeTop;
pub const NSLayoutFormatAlignAllBottom: NSLayoutFormatOptions = 1 << NSLayoutAttributeBottom;
pub const NSLayoutFormatAlignAllLeading: NSLayoutFormatOptions = 1 << NSLayoutAttributeLeading;
pub const NSLayoutFormatAlignAllTrailing: NSLayoutFormatOptions = 1 << NSLayoutAttributeTrailing;
pub const NSLayoutFormatAlignAllCenterX: NSLayoutFormatOptions = 1 << NSLayoutAttributeCenterX;
pub const NSLayoutFormatAlignAllCenterY: NSLayoutFormatOptions = 1 << NSLayoutAttributeCenterY;
pub const NSLayoutFormatAlignAllLastBaseline: NSLayoutFormatOptions =
    1 << NSLayoutAttributeLastBaseline;
pub const NSLayoutFormatAlignAllFirstBaseline: NSLayoutFormatOptions =
    1 << NSLayoutAttributeFirstBaseline;
pub const NSLayoutFormatAlignAllBaseline: NSLayoutFormatOptions =
    NSLayoutFormatAlignAllLastBaseline;
pub const NSLayoutFormatAlignmentMask: NSLayoutFormatOptions = 0xFFFF;
pub const NSLayoutFormatDirectionLeadingToTrailing: NSLayoutFormatOptions = 0 << 16;
pub const NSLayoutFormatDirectionLeftToRight: NSLayoutFormatOptions = 1 << 16;
pub const NSLayoutFormatDirectionRightToLeft: NSLayoutFormatOptions = 2 << 16;
pub const NSLayoutFormatDirectionMask: NSLayoutFormatOptions = 0x3 << 16;

extern_class!(
    #[derive(Debug)]
    pub struct NSLayoutConstraint;

    unsafe impl ClassType for NSLayoutConstraint {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSLayoutConstraint {
        #[method_id(constraintsWithVisualFormat:options:metrics:views:)]
        pub unsafe fn constraintsWithVisualFormat_options_metrics_views(
            format: &NSString,
            opts: NSLayoutFormatOptions,
            metrics: Option<&NSDictionary<NSString, Object>>,
            views: &NSDictionary<NSString, Object>,
        ) -> Id<NSArray<NSLayoutConstraint>, Shared>;

        #[method_id(constraintWithItem:attribute:relatedBy:toItem:attribute:multiplier:constant:)]
        pub unsafe fn constraintWithItem_attribute_relatedBy_toItem_attribute_multiplier_constant(
            view1: &Object,
            attr1: NSLayoutAttribute,
            relation: NSLayoutRelation,
            view2: Option<&Object>,
            attr2: NSLayoutAttribute,
            multiplier: CGFloat,
            c: CGFloat,
        ) -> Id<Self, Shared>;

        #[method(priority)]
        pub unsafe fn priority(&self) -> NSLayoutPriority;

        #[method(setPriority:)]
        pub unsafe fn setPriority(&self, priority: NSLayoutPriority);

        #[method(shouldBeArchived)]
        pub unsafe fn shouldBeArchived(&self) -> bool;

        #[method(setShouldBeArchived:)]
        pub unsafe fn setShouldBeArchived(&self, shouldBeArchived: bool);

        #[method_id(firstItem)]
        pub unsafe fn firstItem(&self) -> Option<Id<Object, Shared>>;

        #[method_id(secondItem)]
        pub unsafe fn secondItem(&self) -> Option<Id<Object, Shared>>;

        #[method(firstAttribute)]
        pub unsafe fn firstAttribute(&self) -> NSLayoutAttribute;

        #[method(secondAttribute)]
        pub unsafe fn secondAttribute(&self) -> NSLayoutAttribute;

        #[method_id(firstAnchor)]
        pub unsafe fn firstAnchor(&self) -> Id<NSLayoutAnchor, Shared>;

        #[method_id(secondAnchor)]
        pub unsafe fn secondAnchor(&self) -> Option<Id<NSLayoutAnchor, Shared>>;

        #[method(relation)]
        pub unsafe fn relation(&self) -> NSLayoutRelation;

        #[method(multiplier)]
        pub unsafe fn multiplier(&self) -> CGFloat;

        #[method(constant)]
        pub unsafe fn constant(&self) -> CGFloat;

        #[method(setConstant:)]
        pub unsafe fn setConstant(&self, constant: CGFloat);

        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;

        #[method(setActive:)]
        pub unsafe fn setActive(&self, active: bool);

        #[method(activateConstraints:)]
        pub unsafe fn activateConstraints(constraints: &NSArray<NSLayoutConstraint>);

        #[method(deactivateConstraints:)]
        pub unsafe fn deactivateConstraints(constraints: &NSArray<NSLayoutConstraint>);
    }
);

extern_methods!(
    /// NSIdentifier
    unsafe impl NSLayoutConstraint {
        #[method_id(identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString, Shared>>;

        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: Option<&NSString>);
    }
);

extern_methods!(
    unsafe impl NSLayoutConstraint {}
);

extern_methods!(
    /// NSConstraintBasedLayoutInstallingConstraints
    unsafe impl NSView {
        #[method_id(leadingAnchor)]
        pub unsafe fn leadingAnchor(&self) -> Id<NSLayoutXAxisAnchor, Shared>;

        #[method_id(trailingAnchor)]
        pub unsafe fn trailingAnchor(&self) -> Id<NSLayoutXAxisAnchor, Shared>;

        #[method_id(leftAnchor)]
        pub unsafe fn leftAnchor(&self) -> Id<NSLayoutXAxisAnchor, Shared>;

        #[method_id(rightAnchor)]
        pub unsafe fn rightAnchor(&self) -> Id<NSLayoutXAxisAnchor, Shared>;

        #[method_id(topAnchor)]
        pub unsafe fn topAnchor(&self) -> Id<NSLayoutYAxisAnchor, Shared>;

        #[method_id(bottomAnchor)]
        pub unsafe fn bottomAnchor(&self) -> Id<NSLayoutYAxisAnchor, Shared>;

        #[method_id(widthAnchor)]
        pub unsafe fn widthAnchor(&self) -> Id<NSLayoutDimension, Shared>;

        #[method_id(heightAnchor)]
        pub unsafe fn heightAnchor(&self) -> Id<NSLayoutDimension, Shared>;

        #[method_id(centerXAnchor)]
        pub unsafe fn centerXAnchor(&self) -> Id<NSLayoutXAxisAnchor, Shared>;

        #[method_id(centerYAnchor)]
        pub unsafe fn centerYAnchor(&self) -> Id<NSLayoutYAxisAnchor, Shared>;

        #[method_id(firstBaselineAnchor)]
        pub unsafe fn firstBaselineAnchor(&self) -> Id<NSLayoutYAxisAnchor, Shared>;

        #[method_id(lastBaselineAnchor)]
        pub unsafe fn lastBaselineAnchor(&self) -> Id<NSLayoutYAxisAnchor, Shared>;

        #[method_id(constraints)]
        pub unsafe fn constraints(&self) -> Id<NSArray<NSLayoutConstraint>, Shared>;

        #[method(addConstraint:)]
        pub unsafe fn addConstraint(&self, constraint: &NSLayoutConstraint);

        #[method(addConstraints:)]
        pub unsafe fn addConstraints(&self, constraints: &NSArray<NSLayoutConstraint>);

        #[method(removeConstraint:)]
        pub unsafe fn removeConstraint(&self, constraint: &NSLayoutConstraint);

        #[method(removeConstraints:)]
        pub unsafe fn removeConstraints(&self, constraints: &NSArray<NSLayoutConstraint>);
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutCoreMethods
    unsafe impl NSWindow {
        #[method(updateConstraintsIfNeeded)]
        pub unsafe fn updateConstraintsIfNeeded(&self);

        #[method(layoutIfNeeded)]
        pub unsafe fn layoutIfNeeded(&self);
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutCoreMethods
    unsafe impl NSView {
        #[method(updateConstraintsForSubtreeIfNeeded)]
        pub unsafe fn updateConstraintsForSubtreeIfNeeded(&self);

        #[method(updateConstraints)]
        pub unsafe fn updateConstraints(&self);

        #[method(needsUpdateConstraints)]
        pub unsafe fn needsUpdateConstraints(&self) -> bool;

        #[method(setNeedsUpdateConstraints:)]
        pub unsafe fn setNeedsUpdateConstraints(&self, needsUpdateConstraints: bool);
    }
);

extern_methods!(
    /// NSConstraintBasedCompatibility
    unsafe impl NSView {
        #[method(translatesAutoresizingMaskIntoConstraints)]
        pub unsafe fn translatesAutoresizingMaskIntoConstraints(&self) -> bool;

        #[method(setTranslatesAutoresizingMaskIntoConstraints:)]
        pub unsafe fn setTranslatesAutoresizingMaskIntoConstraints(
            &self,
            translatesAutoresizingMaskIntoConstraints: bool,
        );

        #[method(requiresConstraintBasedLayout)]
        pub unsafe fn requiresConstraintBasedLayout() -> bool;
    }
);

extern "C" {
    static NSViewNoInstrinsicMetric: CGFloat;
}

extern "C" {
    static NSViewNoIntrinsicMetric: CGFloat;
}

extern_methods!(
    /// NSConstraintBasedLayoutLayering
    unsafe impl NSView {
        #[method(alignmentRectForFrame:)]
        pub unsafe fn alignmentRectForFrame(&self, frame: NSRect) -> NSRect;

        #[method(frameForAlignmentRect:)]
        pub unsafe fn frameForAlignmentRect(&self, alignmentRect: NSRect) -> NSRect;

        #[method(alignmentRectInsets)]
        pub unsafe fn alignmentRectInsets(&self) -> NSEdgeInsets;

        #[method(firstBaselineOffsetFromTop)]
        pub unsafe fn firstBaselineOffsetFromTop(&self) -> CGFloat;

        #[method(lastBaselineOffsetFromBottom)]
        pub unsafe fn lastBaselineOffsetFromBottom(&self) -> CGFloat;

        #[method(baselineOffsetFromBottom)]
        pub unsafe fn baselineOffsetFromBottom(&self) -> CGFloat;

        #[method(intrinsicContentSize)]
        pub unsafe fn intrinsicContentSize(&self) -> NSSize;

        #[method(invalidateIntrinsicContentSize)]
        pub unsafe fn invalidateIntrinsicContentSize(&self);

        #[method(contentHuggingPriorityForOrientation:)]
        pub unsafe fn contentHuggingPriorityForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> NSLayoutPriority;

        #[method(setContentHuggingPriority:forOrientation:)]
        pub unsafe fn setContentHuggingPriority_forOrientation(
            &self,
            priority: NSLayoutPriority,
            orientation: NSLayoutConstraintOrientation,
        );

        #[method(contentCompressionResistancePriorityForOrientation:)]
        pub unsafe fn contentCompressionResistancePriorityForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> NSLayoutPriority;

        #[method(setContentCompressionResistancePriority:forOrientation:)]
        pub unsafe fn setContentCompressionResistancePriority_forOrientation(
            &self,
            priority: NSLayoutPriority,
            orientation: NSLayoutConstraintOrientation,
        );

        #[method(isHorizontalContentSizeConstraintActive)]
        pub unsafe fn isHorizontalContentSizeConstraintActive(&self) -> bool;

        #[method(setHorizontalContentSizeConstraintActive:)]
        pub unsafe fn setHorizontalContentSizeConstraintActive(
            &self,
            horizontalContentSizeConstraintActive: bool,
        );

        #[method(isVerticalContentSizeConstraintActive)]
        pub unsafe fn isVerticalContentSizeConstraintActive(&self) -> bool;

        #[method(setVerticalContentSizeConstraintActive:)]
        pub unsafe fn setVerticalContentSizeConstraintActive(
            &self,
            verticalContentSizeConstraintActive: bool,
        );
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutLayering
    unsafe impl NSControl {
        #[method(invalidateIntrinsicContentSizeForCell:)]
        pub unsafe fn invalidateIntrinsicContentSizeForCell(&self, cell: &NSCell);
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutAnchoring
    unsafe impl NSWindow {
        #[method(anchorAttributeForOrientation:)]
        pub unsafe fn anchorAttributeForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> NSLayoutAttribute;

        #[method(setAnchorAttribute:forOrientation:)]
        pub unsafe fn setAnchorAttribute_forOrientation(
            &self,
            attr: NSLayoutAttribute,
            orientation: NSLayoutConstraintOrientation,
        );
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutFittingSize
    unsafe impl NSView {
        #[method(fittingSize)]
        pub unsafe fn fittingSize(&self) -> NSSize;
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutDebugging
    unsafe impl NSView {
        #[method_id(constraintsAffectingLayoutForOrientation:)]
        pub unsafe fn constraintsAffectingLayoutForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> Id<NSArray<NSLayoutConstraint>, Shared>;

        #[method(hasAmbiguousLayout)]
        pub unsafe fn hasAmbiguousLayout(&self) -> bool;

        #[method(exerciseAmbiguityInLayout)]
        pub unsafe fn exerciseAmbiguityInLayout(&self);
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutDebugging
    unsafe impl NSWindow {
        #[method(visualizeConstraints:)]
        pub unsafe fn visualizeConstraints(
            &self,
            constraints: Option<&NSArray<NSLayoutConstraint>>,
        );
    }
);
