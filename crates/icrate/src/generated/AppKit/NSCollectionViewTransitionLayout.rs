//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

pub type NSCollectionViewTransitionLayoutAnimatedKey = NSString;

extern_class!(
    #[derive(Debug)]
    pub struct NSCollectionViewTransitionLayout;

    unsafe impl ClassType for NSCollectionViewTransitionLayout {
        type Super = NSCollectionViewLayout;
    }
);

extern_methods!(
    unsafe impl NSCollectionViewTransitionLayout {
        #[method(transitionProgress)]
        pub unsafe fn transitionProgress(&self) -> CGFloat;

        #[method(setTransitionProgress:)]
        pub unsafe fn setTransitionProgress(&self, transitionProgress: CGFloat);

        #[method_id(currentLayout)]
        pub unsafe fn currentLayout(&self) -> Id<NSCollectionViewLayout, Shared>;

        #[method_id(nextLayout)]
        pub unsafe fn nextLayout(&self) -> Id<NSCollectionViewLayout, Shared>;

        #[method_id(initWithCurrentLayout:nextLayout:)]
        pub unsafe fn initWithCurrentLayout_nextLayout(
            &self,
            currentLayout: &NSCollectionViewLayout,
            newLayout: &NSCollectionViewLayout,
        ) -> Id<Self, Shared>;

        #[method(updateValue:forAnimatedKey:)]
        pub unsafe fn updateValue_forAnimatedKey(
            &self,
            value: CGFloat,
            key: &NSCollectionViewTransitionLayoutAnimatedKey,
        );

        #[method(valueForAnimatedKey:)]
        pub unsafe fn valueForAnimatedKey(
            &self,
            key: &NSCollectionViewTransitionLayoutAnimatedKey,
        ) -> CGFloat;
    }
);
