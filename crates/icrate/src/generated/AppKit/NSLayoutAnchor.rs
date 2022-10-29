#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSLayoutAnchor<AnchorType: Message>;
    unsafe impl<AnchorType: Message> ClassType for NSLayoutAnchor<AnchorType> {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl<AnchorType: Message> NSLayoutAnchor<AnchorType> {
        #[method_id(constraintEqualToAnchor:)]
        pub unsafe fn constraintEqualToAnchor(
            &self,
            anchor: &NSLayoutAnchor<AnchorType>,
        ) -> Id<NSLayoutConstraint, Shared>;
        #[method_id(constraintGreaterThanOrEqualToAnchor:)]
        pub unsafe fn constraintGreaterThanOrEqualToAnchor(
            &self,
            anchor: &NSLayoutAnchor<AnchorType>,
        ) -> Id<NSLayoutConstraint, Shared>;
        #[method_id(constraintLessThanOrEqualToAnchor:)]
        pub unsafe fn constraintLessThanOrEqualToAnchor(
            &self,
            anchor: &NSLayoutAnchor<AnchorType>,
        ) -> Id<NSLayoutConstraint, Shared>;
        #[method_id(constraintEqualToAnchor:constant:)]
        pub unsafe fn constraintEqualToAnchor_constant(
            &self,
            anchor: &NSLayoutAnchor<AnchorType>,
            c: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;
        #[method_id(constraintGreaterThanOrEqualToAnchor:constant:)]
        pub unsafe fn constraintGreaterThanOrEqualToAnchor_constant(
            &self,
            anchor: &NSLayoutAnchor<AnchorType>,
            c: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;
        #[method_id(constraintLessThanOrEqualToAnchor:constant:)]
        pub unsafe fn constraintLessThanOrEqualToAnchor_constant(
            &self,
            anchor: &NSLayoutAnchor<AnchorType>,
            c: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;
        #[method_id(name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;
        #[method_id(item)]
        pub unsafe fn item(&self) -> Option<Id<Object, Shared>>;
        #[method(hasAmbiguousLayout)]
        pub unsafe fn hasAmbiguousLayout(&self) -> bool;
        #[method_id(constraintsAffectingLayout)]
        pub unsafe fn constraintsAffectingLayout(&self) -> Id<NSArray<NSLayoutConstraint>, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSLayoutXAxisAnchor;
    unsafe impl ClassType for NSLayoutXAxisAnchor {
        type Super = NSLayoutAnchor;
    }
);
extern_methods!(
    unsafe impl NSLayoutXAxisAnchor {
        #[method_id(anchorWithOffsetToAnchor:)]
        pub unsafe fn anchorWithOffsetToAnchor(
            &self,
            otherAnchor: &NSLayoutXAxisAnchor,
        ) -> Id<NSLayoutDimension, Shared>;
        #[method_id(constraintEqualToSystemSpacingAfterAnchor:multiplier:)]
        pub unsafe fn constraintEqualToSystemSpacingAfterAnchor_multiplier(
            &self,
            anchor: &NSLayoutXAxisAnchor,
            multiplier: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;
        #[method_id(constraintGreaterThanOrEqualToSystemSpacingAfterAnchor:multiplier:)]
        pub unsafe fn constraintGreaterThanOrEqualToSystemSpacingAfterAnchor_multiplier(
            &self,
            anchor: &NSLayoutXAxisAnchor,
            multiplier: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;
        #[method_id(constraintLessThanOrEqualToSystemSpacingAfterAnchor:multiplier:)]
        pub unsafe fn constraintLessThanOrEqualToSystemSpacingAfterAnchor_multiplier(
            &self,
            anchor: &NSLayoutXAxisAnchor,
            multiplier: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSLayoutYAxisAnchor;
    unsafe impl ClassType for NSLayoutYAxisAnchor {
        type Super = NSLayoutAnchor;
    }
);
extern_methods!(
    unsafe impl NSLayoutYAxisAnchor {
        #[method_id(anchorWithOffsetToAnchor:)]
        pub unsafe fn anchorWithOffsetToAnchor(
            &self,
            otherAnchor: &NSLayoutYAxisAnchor,
        ) -> Id<NSLayoutDimension, Shared>;
        #[method_id(constraintEqualToSystemSpacingBelowAnchor:multiplier:)]
        pub unsafe fn constraintEqualToSystemSpacingBelowAnchor_multiplier(
            &self,
            anchor: &NSLayoutYAxisAnchor,
            multiplier: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;
        #[method_id(constraintGreaterThanOrEqualToSystemSpacingBelowAnchor:multiplier:)]
        pub unsafe fn constraintGreaterThanOrEqualToSystemSpacingBelowAnchor_multiplier(
            &self,
            anchor: &NSLayoutYAxisAnchor,
            multiplier: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;
        #[method_id(constraintLessThanOrEqualToSystemSpacingBelowAnchor:multiplier:)]
        pub unsafe fn constraintLessThanOrEqualToSystemSpacingBelowAnchor_multiplier(
            &self,
            anchor: &NSLayoutYAxisAnchor,
            multiplier: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSLayoutDimension;
    unsafe impl ClassType for NSLayoutDimension {
        type Super = NSLayoutAnchor;
    }
);
extern_methods!(
    unsafe impl NSLayoutDimension {
        #[method_id(constraintEqualToConstant:)]
        pub unsafe fn constraintEqualToConstant(
            &self,
            c: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;
        #[method_id(constraintGreaterThanOrEqualToConstant:)]
        pub unsafe fn constraintGreaterThanOrEqualToConstant(
            &self,
            c: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;
        #[method_id(constraintLessThanOrEqualToConstant:)]
        pub unsafe fn constraintLessThanOrEqualToConstant(
            &self,
            c: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;
        #[method_id(constraintEqualToAnchor:multiplier:)]
        pub unsafe fn constraintEqualToAnchor_multiplier(
            &self,
            anchor: &NSLayoutDimension,
            m: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;
        #[method_id(constraintGreaterThanOrEqualToAnchor:multiplier:)]
        pub unsafe fn constraintGreaterThanOrEqualToAnchor_multiplier(
            &self,
            anchor: &NSLayoutDimension,
            m: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;
        #[method_id(constraintLessThanOrEqualToAnchor:multiplier:)]
        pub unsafe fn constraintLessThanOrEqualToAnchor_multiplier(
            &self,
            anchor: &NSLayoutDimension,
            m: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;
        #[method_id(constraintEqualToAnchor:multiplier:constant:)]
        pub unsafe fn constraintEqualToAnchor_multiplier_constant(
            &self,
            anchor: &NSLayoutDimension,
            m: CGFloat,
            c: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;
        #[method_id(constraintGreaterThanOrEqualToAnchor:multiplier:constant:)]
        pub unsafe fn constraintGreaterThanOrEqualToAnchor_multiplier_constant(
            &self,
            anchor: &NSLayoutDimension,
            m: CGFloat,
            c: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;
        #[method_id(constraintLessThanOrEqualToAnchor:multiplier:constant:)]
        pub unsafe fn constraintLessThanOrEqualToAnchor_multiplier_constant(
            &self,
            anchor: &NSLayoutDimension,
            m: CGFloat,
            c: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;
    }
);