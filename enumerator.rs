use std::marker::ContravariantLifetime;
use std::mem;
use std::ptr;
use libc::c_ulong;

use objc::Id;
use objc::runtime::Object;

use INSObject;

pub struct NSEnumerator<'a, T> {
    id: Id<Object>,
    marker: ContravariantLifetime<'a>,
}

impl<'a, T> NSEnumerator<'a, T> where T: INSObject {
    pub unsafe fn from_ptr(ptr: *mut Object) -> NSEnumerator<'a, T> {
        NSEnumerator { id: Id::from_ptr(ptr), marker: ContravariantLifetime }
    }
}

impl<'a, T> Iterator for NSEnumerator<'a, T> where T: INSObject {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        unsafe {
            let obj: *mut T = msg_send![self.id, nextObject];
            obj.as_ref()
        }
    }
}

trait INSFastEnumeration: INSObject {
    type Item: INSObject;

    fn enumerator(&self) -> NSFastEnumerator<Self::Item> {
        NSFastEnumerator::<Self::Item>::new(self)
    }
}

#[repr(C)]
struct NSFastEnumerationState<T> {
    state: c_ulong,
    items_ptr: *const *const T,
    mutations_ptr: *mut c_ulong,
    extra: [c_ulong; 5],
}

const FAST_ENUM_BUF_SIZE: usize = 16;

struct NSFastEnumerator<'a, T> {
    object: &'a Object,

    ptr: *const *const T,
    end: *const *const T,

    state: NSFastEnumerationState<T>,
    buf: [*const T; FAST_ENUM_BUF_SIZE],
}

impl<'a, T> NSFastEnumerator<'a, T> where T: INSObject {
    fn new<C: INSFastEnumeration>(object: &C) -> NSFastEnumerator<C::Item> {
        NSFastEnumerator {
            object: unsafe { &*(object as *const C as *const Object) },

            ptr: ptr::null(),
            end: ptr::null(),

            state: unsafe { mem::zeroed() },
            buf: [ptr::null(); FAST_ENUM_BUF_SIZE],
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

        let count: usize = unsafe {
            msg_send![self.object, countByEnumeratingWithState:&mut self.state
                                                       objects:self.buf.as_mut_ptr()
                                                         count:self.buf.len()]
        };

        if count > 0 {
            // Check if the collection was mutated
            if let Some(mutations) = mutations {
                assert!(mutations == unsafe { *self.state.mutations_ptr },
                    "Mutation detected during enumeration of object {:?}",
                    self.object);
            }

            self.ptr = self.state.items_ptr;
            self.end = unsafe { self.ptr.offset(count as isize) };
            true
        } else {
            self.ptr = ptr::null();
            self.end = ptr::null();
            false
        }
    }
}

impl<'a, T> Iterator for NSFastEnumerator<'a, T> where T: INSObject {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
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
    use objc::Id;
    use {INSArray, INSValue, NSArray, NSValue};

    #[test]
    fn test_enumerator() {
        let vec: Vec<Id<NSValue<u32>>> = (0..4).map(INSValue::from_value).collect();
        let array: Id<NSArray<_>> = INSArray::from_vec(vec);

        let enumerator = array.object_enumerator();
        assert!(enumerator.count() == 4);

        let enumerator = array.object_enumerator();
        assert!(enumerator.enumerate().all(|(i, obj)| obj.value() == i as u32));
    }
}
