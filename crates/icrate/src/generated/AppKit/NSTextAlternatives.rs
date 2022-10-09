use super::__exported::NSString;
use crate::AppKit::generated::AppKitDefines::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTextAlternatives;
    unsafe impl ClassType for NSTextAlternatives {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTextAlternatives {
        #[method_id(initWithPrimaryString:alternativeStrings:)]
        pub unsafe fn initWithPrimaryString_alternativeStrings(
            &self,
            primaryString: &NSString,
            alternativeStrings: &NSArray<NSString>,
        ) -> Id<Self, Shared>;
        #[method_id(primaryString)]
        pub unsafe fn primaryString(&self) -> Id<NSString, Shared>;
        #[method_id(alternativeStrings)]
        pub unsafe fn alternativeStrings(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(noteSelectedAlternativeString:)]
        pub unsafe fn noteSelectedAlternativeString(&self, alternativeString: &NSString);
    }
);
