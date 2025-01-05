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
