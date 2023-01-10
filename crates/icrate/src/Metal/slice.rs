use crate::common::*;
use crate::Metal;

fn slice_to_ptr_count<T>(slice: &[T]) -> (NonNull<T>, usize) {
    let ptr: *const T = slice.as_ptr();
    let ptr: *mut T = ptr as *mut T;
    // SAFETY: Slice pointers are always non-null
    let ptr = unsafe { NonNull::new_unchecked(ptr) };
    (ptr, slice.len())
}

impl Metal::MTLRenderCommandEncoder {
    // TODO: Safety
    pub unsafe fn setViewports(&self, viewports: &[Metal::MTLViewport]) {
        let (ptr, count) = slice_to_ptr_count(viewports);
        unsafe { self.setViewports_count(ptr, count) }
    }
}

// TODO: Many more methods take `(NonNull<Foo>, NSUInteger)` that can be
// turned into `&[Foo]`.
