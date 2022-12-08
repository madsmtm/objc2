//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub static NSVariableStatusItemLength: CGFloat = -1.0;

pub static NSSquareStatusItemLength: CGFloat = -2.0;

extern_class!(
    #[derive(Debug)]
    pub struct NSStatusBar;

    unsafe impl ClassType for NSStatusBar {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSStatusBar {
        #[method_id(@__retain_semantics Other systemStatusBar)]
        pub unsafe fn systemStatusBar() -> Id<NSStatusBar, Shared>;

        #[method_id(@__retain_semantics Other statusItemWithLength:)]
        pub unsafe fn statusItemWithLength(&self, length: CGFloat) -> Id<NSStatusItem, Shared>;

        #[method(removeStatusItem:)]
        pub unsafe fn removeStatusItem(&self, item: &NSStatusItem);

        #[method(isVertical)]
        pub unsafe fn isVertical(&self) -> bool;

        #[method(thickness)]
        pub unsafe fn thickness(&self) -> CGFloat;
    }
);
