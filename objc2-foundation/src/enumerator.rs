use core::marker::PhantomData;
use core::mem;
use core::mem::size_of;
use core::ptr;
use core::ptr::NonNull;
use core::slice;
use std::os::raw::c_ulong;

use objc2::rc::{Id, Owned};
use objc2::runtime::Object;
use objc2::{msg_send, Encode, Encoding, RefEncode};

use super::INSObject;

pub struct NSEnumerator<'a, T: INSObject + ?Sized> {
    id: Id<Object, Owned>,
    item: PhantomData<&'a T>,
}

impl<'a, T: INSObject + ?Sized> NSEnumerator<'a, T> {
    /// TODO
    ///
    /// # Safety
    ///
    /// The object pointer must be a valid `NSEnumerator` with `Owned`
    /// ownership.
    pub unsafe fn from_ptr(ptr: *mut Object) -> Self {
        let ptr = NonNull::new(ptr).unwrap();
        Self {
            id: unsafe { Id::retain(ptr) },
            item: PhantomData,
        }
    }
}

impl<'a, T: INSObject + ?Sized> Iterator for NSEnumerator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        unsafe { msg_send![self.id, nextObject] }
    }
}

pub unsafe trait INSFastEnumeration: INSObject {
    type Item: INSObject + ?Sized;

    fn enumerator(&self) -> NSFastEnumerator<'_, Self> {
        NSFastEnumerator::new(self)
    }
}

#[repr(C)]
struct NSFastEnumerationState<T: INSObject + ?Sized> {
    state: c_ulong, // TODO: Verify this is actually always 64 bit
    items_ptr: *const *const T,
    mutations_ptr: *mut c_ulong,
    extra: [c_ulong; 5],
}

/// Ideally the encoding of `long` would just be the same as `int` when it's
/// 32 bits wide and the same as `long long` when it's 64 bits wide; then
/// `c_long::ENCODING` would just work.
///
/// Unfortunately, `long` and `unsigned long` have a different encoding than
/// `int` when it's 32 bits wide; the 'l'/'L' encoding.
const U_LONG_ENCODING: Encoding<'static> = {
    // We could also just have used:
    // #[cfg(any(target_pointer_width = "32", windows))]
    //
    // But this way we exactly match what `clang` does:
    // https://github.com/llvm/llvm-project/blob/release/13.x/clang/lib/AST/ASTContext.cpp#L7245
    if size_of::<c_ulong>() == 4 {
        // @encode(unsigned long) = 'L'
        Encoding::ULong
    } else {
        // @encode(unsigned long) = 'Q'
        Encoding::ULongLong
    }
};

unsafe impl<T: INSObject + ?Sized> Encode for NSFastEnumerationState<T> {
    const ENCODING: Encoding<'static> = Encoding::Struct(
        "?",
        &[
            U_LONG_ENCODING,
            Encoding::Pointer(&Encoding::Object), // <*const *const T>::ENCODING
            Encoding::Pointer(&U_LONG_ENCODING),
            Encoding::Array(5, &U_LONG_ENCODING),
        ],
    );
}

unsafe impl<T: INSObject + ?Sized> RefEncode for NSFastEnumerationState<T> {
    const ENCODING_REF: Encoding<'static> = Encoding::Pointer(&Self::ENCODING);
}

fn enumerate<'a, 'b: 'a, C: INSFastEnumeration + ?Sized>(
    object: &'b C,
    state: &mut NSFastEnumerationState<C::Item>,
    buf: &'a mut [*const C::Item],
) -> Option<&'a [*const C::Item]> {
    let count: usize = unsafe {
        // Reborrow state so that we don't move it
        let state = &mut *state;
        msg_send![
            object,
            countByEnumeratingWithState: state,
            objects: buf.as_mut_ptr(),
            count: buf.len(),
        ]
    };

    if count > 0 {
        unsafe { Some(slice::from_raw_parts(state.items_ptr, count)) }
    } else {
        None
    }
}

const FAST_ENUM_BUF_SIZE: usize = 16;

pub struct NSFastEnumerator<'a, C: 'a + INSFastEnumeration + ?Sized> {
    object: &'a C,

    ptr: *const *const C::Item,
    end: *const *const C::Item,

    state: NSFastEnumerationState<C::Item>,
    buf: [*const C::Item; FAST_ENUM_BUF_SIZE],
}

impl<'a, C: INSFastEnumeration + ?Sized> NSFastEnumerator<'a, C> {
    fn new(object: &'a C) -> Self {
        // SAFETY: C::Item is ?Sized, but it should always be a thin pointer.
        // This is just a way to get a null pointer because we can't apply the
        // correct ptr::Thin bound yet.
        let null_ptr = unsafe { mem::zeroed::<*const C::Item>() };
        Self {
            object,

            ptr: ptr::null(),
            end: ptr::null(),

            state: unsafe { mem::zeroed() },
            // SAFETY:
            buf: [null_ptr; FAST_ENUM_BUF_SIZE],
        }
    }

    fn update_buf(&mut self) -> bool {
        // If this isn't our first time enumerating, record the previous value
        // from the mutations pointer.
        let mutations = if !self.ptr.is_null() {
            Some(unsafe { *self.state.mutations_ptr })
        } else {
            None
        };

        let next_buf = enumerate(self.object, &mut self.state, &mut self.buf);
        if let Some(buf) = next_buf {
            // Check if the collection was mutated
            if let Some(mutations) = mutations {
                assert_eq!(
                    mutations,
                    unsafe { *self.state.mutations_ptr },
                    "Mutation detected during enumeration of object {:p}",
                    self.object
                );
            }

            self.ptr = buf.as_ptr();
            self.end = unsafe { self.ptr.add(buf.len()) };
            true
        } else {
            self.ptr = ptr::null();
            self.end = ptr::null();
            false
        }
    }
}

impl<'a, C: INSFastEnumeration + ?Sized> Iterator for NSFastEnumerator<'a, C> {
    type Item = &'a C::Item;

    fn next(&mut self) -> Option<&'a C::Item> {
        if self.ptr == self.end && !self.update_buf() {
            None
        } else {
            unsafe {
                let obj = *self.ptr;
                self.ptr = self.ptr.offset(1);
                Some(&*obj)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::INSFastEnumeration;
    use crate::{INSArray, INSValue, NSArray, NSValue};

    #[test]
    fn test_enumerator() {
        let vec = (0u32..4).map(NSValue::new).collect();
        let array = NSArray::from_vec(vec);

        let enumerator = array.iter();
        assert_eq!(enumerator.count(), 4);

        let enumerator = array.iter();
        assert!(enumerator.enumerate().all(|(i, obj)| obj.get() == i as u32));
    }

    #[test]
    fn test_fast_enumerator() {
        let vec = (0u32..4).map(NSValue::new).collect();
        let array = NSArray::from_vec(vec);

        let enumerator = array.enumerator();
        assert_eq!(enumerator.count(), 4);

        let enumerator = array.enumerator();
        assert!(enumerator.enumerate().all(|(i, obj)| obj.get() == i as u32));
    }
}
