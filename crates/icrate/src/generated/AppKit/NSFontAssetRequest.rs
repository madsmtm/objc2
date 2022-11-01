//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

pub type NSFontAssetRequestOptions = NSUInteger;
pub const NSFontAssetRequestOptionUsesStandardUI: NSFontAssetRequestOptions = 1 << 0;

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
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(initWithFontDescriptors:options:)]
        pub unsafe fn initWithFontDescriptors_options(
            this: Option<Allocated<Self>>,
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
