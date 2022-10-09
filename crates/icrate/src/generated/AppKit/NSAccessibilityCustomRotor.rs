use super::__exported::NSAccessibilityCustomRotorItemLoadDelegate;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSAccessibilityProtocols::*;
use crate::Foundation::generated::Foundation::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSAccessibilityCustomRotor;
    unsafe impl ClassType for NSAccessibilityCustomRotor {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSAccessibilityCustomRotor {
        #[method_id(initWithLabel:itemSearchDelegate:)]
        pub unsafe fn initWithLabel_itemSearchDelegate(
            &self,
            label: &NSString,
            itemSearchDelegate: &NSAccessibilityCustomRotorItemSearchDelegate,
        ) -> Id<Self, Shared>;
        #[method_id(initWithRotorType:itemSearchDelegate:)]
        pub unsafe fn initWithRotorType_itemSearchDelegate(
            &self,
            rotorType: NSAccessibilityCustomRotorType,
            itemSearchDelegate: &NSAccessibilityCustomRotorItemSearchDelegate,
        ) -> Id<Self, Shared>;
        #[method(type)]
        pub unsafe fn type_(&self) -> NSAccessibilityCustomRotorType;
        #[method(setType:)]
        pub unsafe fn setType(&self, type_: NSAccessibilityCustomRotorType);
        #[method_id(label)]
        pub unsafe fn label(&self) -> Id<NSString, Shared>;
        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: &NSString);
        #[method_id(itemSearchDelegate)]
        pub unsafe fn itemSearchDelegate(
            &self,
        ) -> Option<Id<NSAccessibilityCustomRotorItemSearchDelegate, Shared>>;
        #[method(setItemSearchDelegate:)]
        pub unsafe fn setItemSearchDelegate(
            &self,
            itemSearchDelegate: Option<&NSAccessibilityCustomRotorItemSearchDelegate>,
        );
        #[method_id(itemLoadingDelegate)]
        pub unsafe fn itemLoadingDelegate(
            &self,
        ) -> Option<Id<NSAccessibilityElementLoading, Shared>>;
        #[method(setItemLoadingDelegate:)]
        pub unsafe fn setItemLoadingDelegate(
            &self,
            itemLoadingDelegate: Option<&NSAccessibilityElementLoading>,
        );
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSAccessibilityCustomRotorSearchParameters;
    unsafe impl ClassType for NSAccessibilityCustomRotorSearchParameters {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSAccessibilityCustomRotorSearchParameters {
        #[method_id(currentItem)]
        pub unsafe fn currentItem(
            &self,
        ) -> Option<Id<NSAccessibilityCustomRotorItemResult, Shared>>;
        #[method(setCurrentItem:)]
        pub unsafe fn setCurrentItem(
            &self,
            currentItem: Option<&NSAccessibilityCustomRotorItemResult>,
        );
        #[method(searchDirection)]
        pub unsafe fn searchDirection(&self) -> NSAccessibilityCustomRotorSearchDirection;
        #[method(setSearchDirection:)]
        pub unsafe fn setSearchDirection(
            &self,
            searchDirection: NSAccessibilityCustomRotorSearchDirection,
        );
        #[method_id(filterString)]
        pub unsafe fn filterString(&self) -> Id<NSString, Shared>;
        #[method(setFilterString:)]
        pub unsafe fn setFilterString(&self, filterString: &NSString);
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSAccessibilityCustomRotorItemResult;
    unsafe impl ClassType for NSAccessibilityCustomRotorItemResult {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSAccessibilityCustomRotorItemResult {
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithTargetElement:)]
        pub unsafe fn initWithTargetElement(
            &self,
            targetElement: &NSAccessibilityElement,
        ) -> Id<Self, Shared>;
        #[method_id(initWithItemLoadingToken:customLabel:)]
        pub unsafe fn initWithItemLoadingToken_customLabel(
            &self,
            itemLoadingToken: &NSAccessibilityLoadingToken,
            customLabel: &NSString,
        ) -> Id<Self, Shared>;
        #[method_id(targetElement)]
        pub unsafe fn targetElement(&self) -> Option<Id<NSAccessibilityElement, Shared>>;
        #[method_id(itemLoadingToken)]
        pub unsafe fn itemLoadingToken(&self) -> Option<Id<NSAccessibilityLoadingToken, Shared>>;
        #[method(targetRange)]
        pub unsafe fn targetRange(&self) -> NSRange;
        #[method(setTargetRange:)]
        pub unsafe fn setTargetRange(&self, targetRange: NSRange);
        #[method_id(customLabel)]
        pub unsafe fn customLabel(&self) -> Option<Id<NSString, Shared>>;
        #[method(setCustomLabel:)]
        pub unsafe fn setCustomLabel(&self, customLabel: Option<&NSString>);
    }
);
pub type NSAccessibilityCustomRotorItemSearchDelegate = NSObject;
