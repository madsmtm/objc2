use crate::common::*;
use crate::Metal;

#[allow(dead_code)]
fn slice_to_ptr_count<T>(slice: &[T]) -> (NonNull<T>, usize) {
    let ptr: *const T = slice.as_ptr();
    let ptr: *mut T = ptr as *mut T;
    // SAFETY: Slice pointers are always non-null
    let ptr = unsafe { NonNull::new_unchecked(ptr) };
    (ptr, slice.len())
}

#[cfg(feature = "Metal_MTLRenderCommandEncoder")]
impl Metal::MTLRenderCommandEncoder {
    // TODO: Safety
    #[cfg(feature = "Metal_MTLViewport")]
    pub unsafe fn setViewports(&self, viewports: &[Metal::MTLViewport]) {
        let (ptr, count) = slice_to_ptr_count(viewports);
        unsafe { self.setViewports_count(ptr, count) }
    }
}

// TODO: Many more methods take `(NonNull<Foo>, NSUInteger)` that can be
// turned into `&[Foo]`.
