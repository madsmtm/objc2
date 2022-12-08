//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug)]
    pub struct NSMovie;

    unsafe impl ClassType for NSMovie {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSMovie {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithMovie:)]
        pub unsafe fn initWithMovie(
            this: Option<Allocated<Self>>,
            movie: &QTMovie,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other QTMovie)]
        pub unsafe fn QTMovie(&self) -> Option<Id<QTMovie, Shared>>;
    }
);
