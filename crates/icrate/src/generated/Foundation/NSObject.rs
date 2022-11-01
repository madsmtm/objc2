//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub type NSCopying = NSObject;

pub type NSMutableCopying = NSObject;

pub type NSCoding = NSObject;

pub type NSSecureCoding = NSObject;

extern_methods!(
    /// NSCoderMethods
    unsafe impl NSObject {
        #[method(version)]
        pub unsafe fn version() -> NSInteger;

        #[method(setVersion:)]
        pub unsafe fn setVersion(aVersion: NSInteger);

        #[method(classForCoder)]
        pub unsafe fn classForCoder(&self) -> &'static Class;

        #[method_id(@__retain_semantics Other replacementObjectForCoder:)]
        pub unsafe fn replacementObjectForCoder(
            &self,
            coder: &NSCoder,
        ) -> Option<Id<Object, Shared>>;
    }
);

extern_methods!(
    /// NSDeprecatedMethods
    unsafe impl NSObject {
        #[method(poseAsClass:)]
        pub unsafe fn poseAsClass(aClass: &Class);
    }
);

pub type NSDiscardableContent = NSObject;

extern_methods!(
    /// NSDiscardableContentProxy
    unsafe impl NSObject {
        #[method_id(@__retain_semantics Other autoContentAccessingProxy)]
        pub unsafe fn autoContentAccessingProxy(&self) -> Id<Object, Shared>;
    }
);
