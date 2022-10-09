use crate::AppKit::generated::AppKitDefines::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSGeometry::*;
use crate::Foundation::generated::NSObjCRuntime::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSUserInterfaceCompressionOptions;
    unsafe impl ClassType for NSUserInterfaceCompressionOptions {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSUserInterfaceCompressionOptions {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Id<Self, Shared>;
        #[method_id(initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(&self, identifier: &NSString) -> Id<Self, Shared>;
        #[method_id(initWithCompressionOptions:)]
        pub unsafe fn initWithCompressionOptions(
            &self,
            options: &NSSet<NSUserInterfaceCompressionOptions>,
        ) -> Id<Self, Shared>;
        #[method(containsOptions:)]
        pub unsafe fn containsOptions(&self, options: &NSUserInterfaceCompressionOptions) -> bool;
        #[method(intersectsOptions:)]
        pub unsafe fn intersectsOptions(&self, options: &NSUserInterfaceCompressionOptions)
            -> bool;
        #[method(isEmpty)]
        pub unsafe fn isEmpty(&self) -> bool;
        #[method_id(optionsByAddingOptions:)]
        pub unsafe fn optionsByAddingOptions(
            &self,
            options: &NSUserInterfaceCompressionOptions,
        ) -> Id<NSUserInterfaceCompressionOptions, Shared>;
        #[method_id(optionsByRemovingOptions:)]
        pub unsafe fn optionsByRemovingOptions(
            &self,
            options: &NSUserInterfaceCompressionOptions,
        ) -> Id<NSUserInterfaceCompressionOptions, Shared>;
        #[method_id(hideImagesOption)]
        pub unsafe fn hideImagesOption() -> Id<NSUserInterfaceCompressionOptions, Shared>;
        #[method_id(hideTextOption)]
        pub unsafe fn hideTextOption() -> Id<NSUserInterfaceCompressionOptions, Shared>;
        #[method_id(reduceMetricsOption)]
        pub unsafe fn reduceMetricsOption() -> Id<NSUserInterfaceCompressionOptions, Shared>;
        #[method_id(breakEqualWidthsOption)]
        pub unsafe fn breakEqualWidthsOption() -> Id<NSUserInterfaceCompressionOptions, Shared>;
        #[method_id(standardOptions)]
        pub unsafe fn standardOptions() -> Id<NSUserInterfaceCompressionOptions, Shared>;
    }
);
pub type NSUserInterfaceCompression = NSObject;
