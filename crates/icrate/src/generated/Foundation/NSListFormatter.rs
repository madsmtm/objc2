use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSFormatter::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSListFormatter;
    unsafe impl ClassType for NSListFormatter {
        type Super = NSFormatter;
    }
);
extern_methods!(
    unsafe impl NSListFormatter {
        #[method_id(locale)]
        pub unsafe fn locale(&self) -> Id<NSLocale, Shared>;
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);
        #[method_id(itemFormatter)]
        pub unsafe fn itemFormatter(&self) -> Option<Id<NSFormatter, Shared>>;
        #[method(setItemFormatter:)]
        pub unsafe fn setItemFormatter(&self, itemFormatter: Option<&NSFormatter>);
        #[method_id(localizedStringByJoiningStrings:)]
        pub unsafe fn localizedStringByJoiningStrings(
            strings: &NSArray<NSString>,
        ) -> Id<NSString, Shared>;
        #[method_id(stringFromItems:)]
        pub unsafe fn stringFromItems(&self, items: &NSArray) -> Option<Id<NSString, Shared>>;
        #[method_id(stringForObjectValue:)]
        pub unsafe fn stringForObjectValue(
            &self,
            obj: Option<&Object>,
        ) -> Option<Id<NSString, Shared>>;
    }
);
