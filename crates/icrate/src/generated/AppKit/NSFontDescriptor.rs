use crate::AppKit::generated::AppKitDefines::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSDictionary::*;
use crate::Foundation::generated::NSGeometry::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSSet::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSFontSymbolicTraits = u32;
use super::__exported::NSAffineTransform;
pub type NSFontDescriptorAttributeName = NSString;
pub type NSFontDescriptorTraitKey = NSString;
pub type NSFontDescriptorVariationKey = NSString;
pub type NSFontDescriptorFeatureKey = NSString;
pub type NSFontWeight = CGFloat;
pub type NSFontDescriptorSystemDesign = NSString;
pub type NSFontTextStyle = NSString;
pub type NSFontTextStyleOptionKey = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSFontDescriptor;
    unsafe impl ClassType for NSFontDescriptor {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSFontDescriptor {
        #[method_id(postscriptName)]
        pub unsafe fn postscriptName(&self) -> Option<Id<NSString, Shared>>;
        #[method(pointSize)]
        pub unsafe fn pointSize(&self) -> CGFloat;
        #[method_id(matrix)]
        pub unsafe fn matrix(&self) -> Option<Id<NSAffineTransform, Shared>>;
        #[method(symbolicTraits)]
        pub unsafe fn symbolicTraits(&self) -> NSFontDescriptorSymbolicTraits;
        #[method(requiresFontAssetRequest)]
        pub unsafe fn requiresFontAssetRequest(&self) -> bool;
        #[method_id(objectForKey:)]
        pub unsafe fn objectForKey(
            &self,
            attribute: &NSFontDescriptorAttributeName,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(fontAttributes)]
        pub unsafe fn fontAttributes(
            &self,
        ) -> Id<NSDictionary<NSFontDescriptorAttributeName, Object>, Shared>;
        #[method_id(fontDescriptorWithFontAttributes:)]
        pub unsafe fn fontDescriptorWithFontAttributes(
            attributes: Option<&NSDictionary<NSFontDescriptorAttributeName, Object>>,
        ) -> Id<NSFontDescriptor, Shared>;
        #[method_id(fontDescriptorWithName:size:)]
        pub unsafe fn fontDescriptorWithName_size(
            fontName: &NSString,
            size: CGFloat,
        ) -> Id<NSFontDescriptor, Shared>;
        #[method_id(fontDescriptorWithName:matrix:)]
        pub unsafe fn fontDescriptorWithName_matrix(
            fontName: &NSString,
            matrix: &NSAffineTransform,
        ) -> Id<NSFontDescriptor, Shared>;
        #[method_id(initWithFontAttributes:)]
        pub unsafe fn initWithFontAttributes(
            &self,
            attributes: Option<&NSDictionary<NSFontDescriptorAttributeName, Object>>,
        ) -> Id<Self, Shared>;
        #[method_id(matchingFontDescriptorsWithMandatoryKeys:)]
        pub unsafe fn matchingFontDescriptorsWithMandatoryKeys(
            &self,
            mandatoryKeys: Option<&NSSet<NSFontDescriptorAttributeName>>,
        ) -> Id<NSArray<NSFontDescriptor>, Shared>;
        #[method_id(matchingFontDescriptorWithMandatoryKeys:)]
        pub unsafe fn matchingFontDescriptorWithMandatoryKeys(
            &self,
            mandatoryKeys: Option<&NSSet<NSFontDescriptorAttributeName>>,
        ) -> Option<Id<NSFontDescriptor, Shared>>;
        #[method_id(fontDescriptorByAddingAttributes:)]
        pub unsafe fn fontDescriptorByAddingAttributes(
            &self,
            attributes: &NSDictionary<NSFontDescriptorAttributeName, Object>,
        ) -> Id<NSFontDescriptor, Shared>;
        #[method_id(fontDescriptorWithSymbolicTraits:)]
        pub unsafe fn fontDescriptorWithSymbolicTraits(
            &self,
            symbolicTraits: NSFontDescriptorSymbolicTraits,
        ) -> Id<NSFontDescriptor, Shared>;
        #[method_id(fontDescriptorWithSize:)]
        pub unsafe fn fontDescriptorWithSize(
            &self,
            newPointSize: CGFloat,
        ) -> Id<NSFontDescriptor, Shared>;
        #[method_id(fontDescriptorWithMatrix:)]
        pub unsafe fn fontDescriptorWithMatrix(
            &self,
            matrix: &NSAffineTransform,
        ) -> Id<NSFontDescriptor, Shared>;
        #[method_id(fontDescriptorWithFace:)]
        pub unsafe fn fontDescriptorWithFace(
            &self,
            newFace: &NSString,
        ) -> Id<NSFontDescriptor, Shared>;
        #[method_id(fontDescriptorWithFamily:)]
        pub unsafe fn fontDescriptorWithFamily(
            &self,
            newFamily: &NSString,
        ) -> Id<NSFontDescriptor, Shared>;
        #[method_id(fontDescriptorWithDesign:)]
        pub unsafe fn fontDescriptorWithDesign(
            &self,
            design: &NSFontDescriptorSystemDesign,
        ) -> Option<Id<Self, Shared>>;
    }
);
pub type NSFontFamilyClass = u32;
extern_methods!(
    #[doc = "NSFontDescriptor_TextStyles"]
    unsafe impl NSFontDescriptor {
        #[method_id(preferredFontDescriptorForTextStyle:options:)]
        pub unsafe fn preferredFontDescriptorForTextStyle_options(
            style: &NSFontTextStyle,
            options: &NSDictionary<NSFontTextStyleOptionKey, Object>,
        ) -> Id<NSFontDescriptor, Shared>;
    }
);
