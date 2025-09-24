use core::ptr::NonNull;

use super::NWRetained;

#[doc(alias = "nw_object_t")]
pub unsafe trait NWObject {
    /// Increment the reference count of the object.
    ///
    /// This extends the duration in which the object is alive by detaching it
    /// from the lifetime information carried by the reference.
    #[doc(alias = "nw_retain")]
    fn retain(&self) -> NWRetained<Self> {
        let ptr: NonNull<Self> = NonNull::from(self);
        // SAFETY:
        // - The pointer is valid since it came from `&self`.
        // - The lifetime of the pointer itself is extended, but any lifetime
        //   that the object may carry is still kept within the type itself.
        unsafe { NWRetained::retain(ptr) }
    }
}
