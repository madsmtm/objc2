use core::cell::UnsafeCell;
use core::marker::{PhantomData, PhantomPinned};
#[cfg(feature = "objc2")]
use objc2::cf_objc2_type;
use objc2_core_foundation::cf_type;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpdfmarkedcontentitem?language=objc)
#[doc(alias = "CGPDFMarkedContentItemRef")]
#[repr(C)]
pub struct CGPDFMarkedContentItem {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

cf_type!(
    unsafe impl CGPDFMarkedContentItem {}
);
#[cfg(feature = "objc2")]
cf_objc2_type!(
    unsafe impl RefEncode<"CGPDFMarkedContentItem"> for CGPDFMarkedContentItem {}
);

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpdfstructureelement?language=objc)
#[doc(alias = "CGPDFStructureElementRef")]
#[repr(C)]
pub struct CGPDFStructureElement {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

cf_type!(
    unsafe impl CGPDFStructureElement {}
);
#[cfg(feature = "objc2")]
cf_objc2_type!(
    unsafe impl RefEncode<"CGPDFStructureElement"> for CGPDFStructureElement {}
);
