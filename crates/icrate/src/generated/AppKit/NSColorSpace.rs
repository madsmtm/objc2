use super::__exported::NSData;
use crate::AppKit::generated::AppKitDefines::*;
use crate::ApplicationServices::generated::ApplicationServices::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSColorSpace;
    unsafe impl ClassType for NSColorSpace {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSColorSpace {
        #[method_id(initWithICCProfileData:)]
        pub unsafe fn initWithICCProfileData(&self, iccData: &NSData) -> Option<Id<Self, Shared>>;
        #[method_id(ICCProfileData)]
        pub unsafe fn ICCProfileData(&self) -> Option<Id<NSData, Shared>>;
        #[method_id(initWithColorSyncProfile:)]
        pub unsafe fn initWithColorSyncProfile(
            &self,
            prof: NonNull<c_void>,
        ) -> Option<Id<Self, Shared>>;
        #[method(colorSyncProfile)]
        pub unsafe fn colorSyncProfile(&self) -> *mut c_void;
        #[method_id(initWithCGColorSpace:)]
        pub unsafe fn initWithCGColorSpace(
            &self,
            cgColorSpace: CGColorSpaceRef,
        ) -> Option<Id<Self, Shared>>;
        #[method(CGColorSpace)]
        pub unsafe fn CGColorSpace(&self) -> CGColorSpaceRef;
        #[method(numberOfColorComponents)]
        pub unsafe fn numberOfColorComponents(&self) -> NSInteger;
        #[method(colorSpaceModel)]
        pub unsafe fn colorSpaceModel(&self) -> NSColorSpaceModel;
        #[method_id(localizedName)]
        pub unsafe fn localizedName(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(sRGBColorSpace)]
        pub unsafe fn sRGBColorSpace() -> Id<NSColorSpace, Shared>;
        #[method_id(genericGamma22GrayColorSpace)]
        pub unsafe fn genericGamma22GrayColorSpace() -> Id<NSColorSpace, Shared>;
        #[method_id(extendedSRGBColorSpace)]
        pub unsafe fn extendedSRGBColorSpace() -> Id<NSColorSpace, Shared>;
        #[method_id(extendedGenericGamma22GrayColorSpace)]
        pub unsafe fn extendedGenericGamma22GrayColorSpace() -> Id<NSColorSpace, Shared>;
        #[method_id(displayP3ColorSpace)]
        pub unsafe fn displayP3ColorSpace() -> Id<NSColorSpace, Shared>;
        #[method_id(adobeRGB1998ColorSpace)]
        pub unsafe fn adobeRGB1998ColorSpace() -> Id<NSColorSpace, Shared>;
        #[method_id(genericRGBColorSpace)]
        pub unsafe fn genericRGBColorSpace() -> Id<NSColorSpace, Shared>;
        #[method_id(genericGrayColorSpace)]
        pub unsafe fn genericGrayColorSpace() -> Id<NSColorSpace, Shared>;
        #[method_id(genericCMYKColorSpace)]
        pub unsafe fn genericCMYKColorSpace() -> Id<NSColorSpace, Shared>;
        #[method_id(deviceRGBColorSpace)]
        pub unsafe fn deviceRGBColorSpace() -> Id<NSColorSpace, Shared>;
        #[method_id(deviceGrayColorSpace)]
        pub unsafe fn deviceGrayColorSpace() -> Id<NSColorSpace, Shared>;
        #[method_id(deviceCMYKColorSpace)]
        pub unsafe fn deviceCMYKColorSpace() -> Id<NSColorSpace, Shared>;
        #[method_id(availableColorSpacesWithModel:)]
        pub unsafe fn availableColorSpacesWithModel(
            model: NSColorSpaceModel,
        ) -> Id<NSArray<NSColorSpace>, Shared>;
    }
);
