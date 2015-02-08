use std::marker::ContravariantLifetime;
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
        NSFastEnumerator {
            object: unsafe { &*(self as *const Self as *const Object) },
        }
    }
}

#[repr(C)]
struct NSFastEnumerationState {
    state: c_ulong,
    items_ptr: *mut *const Object,
    mutations_ptr: *mut c_ulong,
    extra: [c_ulong; 5],
}

struct NSFastEnumerator<'a, T> {
    object: &'a Object,
}
