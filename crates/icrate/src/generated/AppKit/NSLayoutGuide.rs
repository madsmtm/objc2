#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSLayoutGuide;
    unsafe impl ClassType for NSLayoutGuide {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSLayoutGuide {
        #[method(frame)]
        pub unsafe fn frame(&self) -> NSRect;
        #[method_id(owningView)]
        pub unsafe fn owningView(&self) -> Option<Id<NSView, Shared>>;
        #[method(setOwningView:)]
        pub unsafe fn setOwningView(&self, owningView: Option<&NSView>);
        #[method_id(identifier)]
        pub unsafe fn identifier(&self) -> Id<NSUserInterfaceItemIdentifier, Shared>;
        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: &NSUserInterfaceItemIdentifier);
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
        #[method(hasAmbiguousLayout)]
        pub unsafe fn hasAmbiguousLayout(&self) -> bool;
        #[method_id(constraintsAffectingLayoutForOrientation:)]
        pub unsafe fn constraintsAffectingLayoutForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> Id<NSArray<NSLayoutConstraint>, Shared>;
    }
);
extern_methods!(
    #[doc = "NSLayoutGuideSupport"]
    unsafe impl NSView {
        #[method(addLayoutGuide:)]
        pub unsafe fn addLayoutGuide(&self, guide: &NSLayoutGuide);
        #[method(removeLayoutGuide:)]
        pub unsafe fn removeLayoutGuide(&self, guide: &NSLayoutGuide);
        #[method_id(layoutGuides)]
        pub unsafe fn layoutGuides(&self) -> Id<NSArray<NSLayoutGuide>, Shared>;
    }
);
