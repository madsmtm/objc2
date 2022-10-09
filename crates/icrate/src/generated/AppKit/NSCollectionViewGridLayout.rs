use super::__exported::NSColor;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSCollectionViewLayout::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSCollectionViewGridLayout;
    unsafe impl ClassType for NSCollectionViewGridLayout {
        type Super = NSCollectionViewLayout;
    }
);
extern_methods!(
    unsafe impl NSCollectionViewGridLayout {
        #[method(margins)]
        pub unsafe fn margins(&self) -> NSEdgeInsets;
        #[method(setMargins:)]
        pub unsafe fn setMargins(&self, margins: NSEdgeInsets);
        #[method(minimumInteritemSpacing)]
        pub unsafe fn minimumInteritemSpacing(&self) -> CGFloat;
        #[method(setMinimumInteritemSpacing:)]
        pub unsafe fn setMinimumInteritemSpacing(&self, minimumInteritemSpacing: CGFloat);
        #[method(minimumLineSpacing)]
        pub unsafe fn minimumLineSpacing(&self) -> CGFloat;
        #[method(setMinimumLineSpacing:)]
        pub unsafe fn setMinimumLineSpacing(&self, minimumLineSpacing: CGFloat);
        #[method(maximumNumberOfRows)]
        pub unsafe fn maximumNumberOfRows(&self) -> NSUInteger;
        #[method(setMaximumNumberOfRows:)]
        pub unsafe fn setMaximumNumberOfRows(&self, maximumNumberOfRows: NSUInteger);
        #[method(maximumNumberOfColumns)]
        pub unsafe fn maximumNumberOfColumns(&self) -> NSUInteger;
        #[method(setMaximumNumberOfColumns:)]
        pub unsafe fn setMaximumNumberOfColumns(&self, maximumNumberOfColumns: NSUInteger);
        #[method(minimumItemSize)]
        pub unsafe fn minimumItemSize(&self) -> NSSize;
        #[method(setMinimumItemSize:)]
        pub unsafe fn setMinimumItemSize(&self, minimumItemSize: NSSize);
        #[method(maximumItemSize)]
        pub unsafe fn maximumItemSize(&self) -> NSSize;
        #[method(setMaximumItemSize:)]
        pub unsafe fn setMaximumItemSize(&self, maximumItemSize: NSSize);
        #[method_id(backgroundColors)]
        pub unsafe fn backgroundColors(&self) -> Id<NSArray<NSColor>, Shared>;
        #[method(setBackgroundColors:)]
        pub unsafe fn setBackgroundColors(&self, backgroundColors: Option<&NSArray<NSColor>>);
    }
);
