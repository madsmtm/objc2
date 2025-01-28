use core::cell::UnsafeCell;
use core::marker::{PhantomData, PhantomPinned};

/// [Apple's documentation](https://developer.apple.com/documentation/iosurface/iosurfaceref?language=objc)
#[repr(C)]
pub struct IOSurfaceRef {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

#[cfg(feature = "objc2-core-foundation")]
objc2_core_foundation::cf_type!(
    #[encoding_name = "__IOSurface"]
    unsafe impl IOSurfaceRef {}
);

// Hacky fix for `IOSurfaceRef` being used without `objc2-core-foundation`
// being enabled.
#[cfg(all(not(feature = "objc2-core-foundation"), feature = "objc2"))]
unsafe impl objc2::encode::RefEncode for IOSurfaceRef {
    const ENCODING_REF: objc2::encode::Encoding =
        objc2::encode::Encoding::Pointer(&objc2::encode::Encoding::Struct("__IOSurface", &[]));
}
