use alloc::boxed::Box;
use core::alloc::AllocError;
use core::alloc::Allocator;
use core::alloc::Layout;
use core::mem::MaybeUninit;
use core::ptr::slice_from_raw_parts_mut;
use core::ptr::NonNull;

use crate::ffi;
use crate::runtime::Class;
use crate::runtime::Object;

unsafe impl Allocator for &'static Class {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr: *mut Object = unsafe { msg_send![*self, alloc] };
        let size = unsafe { ffi::class_getInstanceSize(self.as_ptr()) };
        std::println!("Allocate {layout:?}, size: {size}");
        let ptr = slice_from_raw_parts_mut(ptr.cast::<u8>(), size);
        if let Some(ptr) = NonNull::new(ptr) {
            Ok(ptr)
        } else {
            Err(AllocError)
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        std::println!("Deallocate {layout:?}");
        let obj: *mut ffi::objc_object = ptr.cast().as_ptr();
        unsafe { ffi::objc_release(obj) };
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        std::println!("Drop called");
    }
}

impl Object {
    /// TODO
    pub fn new1() -> Box<Self, &'static Class> {
        let obj: Box<MaybeUninit<Object>, _> = Box::new_uninit_in(class!(NSObject));
        let (obj, cls) = Box::into_raw_with_allocator(obj);
        let obj: *mut Object = unsafe { msg_send![obj as *mut Object, init] };
        unsafe { Box::from_raw_in(obj, cls) }
    }

    /// TODO
    pub fn new2() -> Box<Self, &'static Class> {
        let ptr: *mut Object = unsafe { msg_send![class!(NSObject), new] };
        unsafe { Box::from_raw_in(ptr, class!(NSObject)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alloc() {
        let obj = Object::new1();
        std::println!("{:?}", obj);

        let obj = Object::new2();
        std::println!("{:?}", obj);
    }
}
