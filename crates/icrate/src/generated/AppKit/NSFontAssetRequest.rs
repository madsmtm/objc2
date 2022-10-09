use super::__exported::NSFontDescriptor;
use crate::AppKit::generated::AppKitDefines::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSProgress::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSFontAssetRequest;
    unsafe impl ClassType for NSFontAssetRequest {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSFontAssetRequest {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithFontDescriptors:options:)]
        pub unsafe fn initWithFontDescriptors_options(
            &self,
            fontDescriptors: &NSArray<NSFontDescriptor>,
            options: NSFontAssetRequestOptions,
        ) -> Id<Self, Shared>;
        #[method_id(downloadedFontDescriptors)]
        pub unsafe fn downloadedFontDescriptors(&self) -> Id<NSArray<NSFontDescriptor>, Shared>;
        #[method_id(progress)]
        pub unsafe fn progress(&self) -> Id<NSProgress, Shared>;
        #[method(downloadFontAssetsWithCompletionHandler:)]
        pub unsafe fn downloadFontAssetsWithCompletionHandler(&self, completionHandler: TodoBlock);
    }
);
