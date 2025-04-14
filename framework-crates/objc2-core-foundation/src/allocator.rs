//! Interoperability between [`CFAllocator`] and Rust's [`Allocator`] trait.
//!
//! See <https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Tasks/CustomAllocators.html>.

// TODO: Does it make sense to use CFAllocator::get_preferred_size_for_size?
// TODO: Can we mark `CFAllocator` as `Send` + `Sync`?

#[cfg(feature = "alloc")]
use alloc::sync::Arc;
use core::alloc::{AllocError, Allocator, Layout};
use core::ffi::c_void;
use core::ptr::{self, NonNull};

use crate::{CFAllocator, CFAllocatorContext, CFIndex, CFOptionFlags};

// `man malloc` states:
// > The allocated memory is aligned such that it can be used for any
// > data type, including AltiVec- and SSE-related types.
//
// For now, only assume that to be true on 64-bit platforms (as I'm unsure
// about e.g. iOS 32-bit ARM).
const MAX_SUPPORTED_ALIGN: usize = if cfg!(target_pointer_width = "64") {
    16
} else {
    align_of::<u128>()
};

#[inline]
fn get_size(size: usize) -> isize {
    // Allocations > isize::MAX are unsupported (and that's a guarantee of
    // Layout).
    debug_assert!(size <= isize::MAX as usize);
    size as isize
}

#[inline]
fn to_result(ptr: *mut c_void, size: usize) -> Result<NonNull<[u8]>, AllocError> {
    let ptr = NonNull::new(ptr.cast::<u8>()).ok_or(AllocError)?;
    Ok(NonNull::slice_from_raw_parts(ptr, size))
}

// Ideally, should be implemented for Option<&CFAllocator> too to allow
// `kCFAllocatorDefault` to work, but we can't do that because of coherence.
unsafe impl Allocator for CFAllocator {
    #[inline]
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        if MAX_SUPPORTED_ALIGN < layout.align() {
            // Could be cool if we could over-allocate and adjust the returned
            // pointer, but that'd require us to undo the adjustment on
            // deallocation, which we can't really do.
            return Err(AllocError);
        }

        let ptr = Self::allocate(Some(self), get_size(layout.size()), 0);
        to_result(ptr, layout.size())
    }

    #[inline]
    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
        // SAFETY: Upheld by caller.
        unsafe { Self::deallocate(Some(self), ptr.cast::<c_void>().as_ptr()) };
    }

    #[inline]
    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        _old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        if MAX_SUPPORTED_ALIGN < new_layout.align() {
            return Err(AllocError);
        }

        let ptr = ptr.cast::<c_void>().as_ptr();

        // SAFETY: Upheld by caller.
        let ptr = unsafe { Self::reallocate(Some(self), ptr, get_size(new_layout.size()), 0) };
        to_result(ptr, new_layout.size())
    }

    #[inline]
    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        // SAFETY: We've implemented `grow` in terms of `reallocate`, so it's
        // safe to pass the layouts we have here to that.
        // Rest is upheld by caller.
        unsafe { self.grow(ptr, old_layout, new_layout) }
    }
}

// TODO: Add this impl. Because of the functions below, you can construct
// `CFAllocator` from `Global`, but that probably doesn't matter because you
// couldn't do it in `const`, so you couldn't use it in `#[global_allocator]`
// anyhow (avoiding the recursion).
//
// We can't guarantee that allocators don't unwind, though, so we can't
// soundly implement this yet.
//
// #[cfg(feature = "std")]
// unsafe impl std::alloc::GlobalAllocator for CFAllocator {}

/// Methods to construct [`CFAllocator`] from a Rust [`Allocator`].
///
/// These must be `Send` + `Sync`, since allocators can be placed into objects
/// and later used after that object has been passed to another thread.
impl CFAllocator {
    #[inline]
    #[cfg(feature = "alloc")]
    pub fn from_alloc<A: Allocator + Send + Sync + 'static>(
        in_allocator: Option<&CFAllocator>,
        rust_allocator: Arc<A>,
    ) -> Option<crate::CFRetained<Self>> {
        unsafe extern "C-unwind" fn retain<A>(info: *const c_void) -> *const c_void {
            // SAFETY: The pointer was passed to `CFAllocatorContext.info` below.
            unsafe { Arc::increment_strong_count(info.cast::<A>()) };
            info
        }
        unsafe extern "C-unwind" fn release<A>(info: *const c_void) {
            // SAFETY: The pointer was passed to `CFAllocatorContext.info` below.
            unsafe { Arc::decrement_strong_count(info.cast::<A>()) };
        }

        let mut context = CFAllocatorContext {
            version: 0,
            // Will be retained upon creation.
            info: Arc::as_ptr(&rust_allocator) as *mut c_void,
            retain: Some(retain::<A>),
            release: Some(release::<A>),
            copyDescription: None,

            allocate: Some(allocate::<A>),
            reallocate: Some(reallocate::<A>),
            deallocate: Some(deallocate::<A>),
            preferredSize: None,
        };

        // SAFETY: The pointers above are correct.
        unsafe { Self::new(in_allocator, &mut context) }
    }

    #[inline]
    pub fn from_static_alloc<A: Allocator + Send + Sync + 'static>(
        in_allocator: Option<&CFAllocator>,
        rust_allocator: &'static A,
    ) -> Option<crate::CFRetained<Self>> {
        let mut context = CFAllocatorContext {
            version: 0,
            // `'static` means we don't need retain/release callbacks.
            info: rust_allocator as *const A as *mut c_void,
            retain: None,
            release: None,
            copyDescription: None,

            allocate: Some(allocate::<A>),
            reallocate: Some(reallocate::<A>),
            deallocate: Some(deallocate::<A>),
            preferredSize: None,
        };

        // SAFETY: The pointers above are correct.
        unsafe { Self::new(in_allocator, &mut context) }
    }
}

unsafe extern "C-unwind" fn allocate<A: Allocator>(
    size: CFIndex,
    _hint: CFOptionFlags,
    info: *mut c_void,
) -> *mut c_void {
    // SAFETY: The allocator was passed above in `CFAllocatorContext.info`.
    let allocator = unsafe { &*info.cast::<A>() };

    debug_assert!(0 <= size);
    // Allocate a bit more space for storing size of allocation.
    let allocation_size = size as usize + MAX_SUPPORTED_ALIGN;

    let Ok(layout) = Layout::from_size_align(allocation_size, MAX_SUPPORTED_ALIGN) else {
        // Size too large.
        return ptr::null_mut();
    };

    let Ok(ptr) = allocator.allocate(layout) else {
        // Allocation failed.
        return ptr::null_mut();
    };
    let ptr = ptr.as_ptr();

    assert!(size_of::<usize>() <= MAX_SUPPORTED_ALIGN);
    assert!(align_of::<usize>() <= MAX_SUPPORTED_ALIGN);
    // SAFETY:
    // - We have exclusive access to the location, because we allocated enough
    //   space for the `usize` (`MAX_SUPPORTED_ALIGN` is larger than that).
    // - The pointer is aligned for at least `usize`.
    // - We know the value will not be overwritten, because the user doesn't
    //   know that the allocation was larger.
    unsafe { ptr.cast::<usize>().write(allocation_size) };

    // Offset away the size information.
    //
    // The pointer is still going to be aligned to `MAX_SUPPORTED_ALIGN`.
    unsafe { ptr.byte_add(MAX_SUPPORTED_ALIGN) }.cast::<c_void>()
}

unsafe extern "C-unwind" fn deallocate<A: Allocator>(ptr: *mut c_void, info: *mut c_void) {
    // SAFETY: The allocator was passed above in `CFAllocatorContext.info`.
    let allocator = unsafe { &*info.cast::<A>() };

    // CFAllocatorDeallocateCallBack documents that the pointer must not be NULL.
    let ptr = NonNull::new(ptr.cast::<u8>()).expect("must not provide NULL pointer");

    // SAFETY: We offset the pointer when allocating it.
    let ptr = unsafe { ptr.byte_sub(MAX_SUPPORTED_ALIGN) };

    // SAFETY: The allocation size was written at the start of the pointer.
    let allocation_size = unsafe { ptr.cast::<usize>().read() };
    let layout = Layout::from_size_align(allocation_size, MAX_SUPPORTED_ALIGN).unwrap();

    // SAFETY: We allocated the pointer above with the layout above.
    unsafe { allocator.deallocate(ptr, layout) };
}

unsafe extern "C-unwind" fn reallocate<A: Allocator>(
    ptr: *mut c_void,
    new_size: CFIndex,
    _hint: CFOptionFlags,
    info: *mut c_void,
) -> *mut c_void {
    // SAFETY: The allocator was passed above in `CFAllocatorContext.info`.
    let allocator = unsafe { &*info.cast::<A>() };

    // Roughly the same as we do in `deallocate`.
    let ptr = NonNull::new(ptr.cast::<u8>()).expect("must not provide NULL pointer");
    let ptr = unsafe { ptr.byte_sub(MAX_SUPPORTED_ALIGN) };
    let allocation_size = unsafe { ptr.cast::<usize>().read() };
    let old_layout = Layout::from_size_align(allocation_size, MAX_SUPPORTED_ALIGN).unwrap();

    // Roughly the same as we do in `allocate`.
    debug_assert!(0 <= new_size);
    let allocation_size = new_size as usize + MAX_SUPPORTED_ALIGN;
    let Ok(new_layout) = Layout::from_size_align(allocation_size, MAX_SUPPORTED_ALIGN) else {
        return ptr::null_mut();
    };
    let ptr = unsafe {
        if old_layout.size() <= new_layout.size() {
            allocator.grow(ptr, old_layout, new_layout)
        } else {
            allocator.shrink(ptr, old_layout, new_layout)
        }
    };
    let Ok(ptr) = ptr else {
        return ptr::null_mut();
    };
    let ptr = ptr.as_ptr();
    unsafe { ptr.cast::<usize>().write(allocation_size) };
    unsafe { ptr.byte_add(MAX_SUPPORTED_ALIGN) }.cast::<c_void>()
}

#[cfg(test)]
#[cfg(all(feature = "CFData", feature = "std"))]
mod tests {
    use alloc::alloc::Global;
    use std::alloc::System;

    use crate::{kCFAllocatorMalloc, kCFAllocatorSystemDefault, CFMutableData};

    use super::*;

    #[test]
    fn allocator_impl() {
        let allocators = [
            unsafe { kCFAllocatorSystemDefault }.unwrap(),
            unsafe { kCFAllocatorMalloc }.unwrap(),
        ];
        for alloc in allocators {
            let layouts = [
                Layout::new::<u8>(),
                Layout::new::<u128>(),
                Layout::new::<(u8, u32, u64, u128)>(),
            ];
            for layout in layouts {
                let ptr = alloc.allocate(layout).unwrap();
                let new_layout = Layout::from_size_align(64, layout.align()).unwrap();
                let ptr = unsafe { alloc.grow(ptr.cast::<u8>(), layout, new_layout) }.unwrap();
                unsafe { alloc.deallocate(ptr.cast::<u8>(), new_layout) };
            }

            let ptr = alloc.allocate(Layout::from_size_align(8, 128).unwrap());
            assert!(ptr.is_err());
        }
    }

    #[test]
    fn created_allocator() {
        let allocators = [
            CFAllocator::from_static_alloc(None, &Global).unwrap(),
            CFAllocator::from_alloc(None, Arc::new(System)).unwrap(),
        ];
        for alloc in allocators {
            let data = CFMutableData::new(Some(&alloc), 0).unwrap();
            for _ in 0..20 {
                let bytes = "12345";
                unsafe { data.append_bytes(bytes.as_ptr(), bytes.len() as CFIndex) };
            }
        }
    }
}
