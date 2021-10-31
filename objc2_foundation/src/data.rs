#[cfg(feature = "block")]
use alloc::vec::Vec;
use core::ops::Range;
use core::slice;
use core::{ffi::c_void, ptr::NonNull};

use super::{INSCopying, INSMutableCopying, INSObject, NSRange};
use objc2::msg_send;
use objc2::rc::{Id, Owned, Shared};

pub unsafe trait INSData: INSObject {
    fn len(&self) -> usize {
        unsafe { msg_send![self, length] }
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn bytes(&self) -> &[u8] {
        let ptr: *const c_void = unsafe { msg_send![self, bytes] };
        // The bytes pointer may be null for length zero
        if ptr.is_null() {
            &[]
        } else {
            unsafe { slice::from_raw_parts(ptr as *const u8, self.len()) }
        }
    }

    fn with_bytes(bytes: &[u8]) -> Id<Self, Self::Ownership> {
        let cls = Self::class();
        let bytes_ptr = bytes.as_ptr() as *const c_void;
        unsafe {
            let obj: *mut Self = msg_send![cls, alloc];
            let obj: *mut Self = msg_send![
                obj,
                initWithBytes: bytes_ptr,
                length: bytes.len(),
            ];
            Id::new(NonNull::new_unchecked(obj))
        }
    }

    #[cfg(feature = "block")]
    fn from_vec(bytes: Vec<u8>) -> Id<Self, Self::Ownership> {
        use objc2_block::{Block, ConcreteBlock};

        let capacity = bytes.capacity();
        let dealloc = ConcreteBlock::new(move |bytes: *mut c_void, len: usize| unsafe {
            // Recreate the Vec and let it drop
            let _ = Vec::from_raw_parts(bytes as *mut u8, len, capacity);
        });
        let dealloc = dealloc.copy();
        let dealloc: &Block<(*mut c_void, usize), ()> = &dealloc;

        let mut bytes = bytes;
        let bytes_ptr = bytes.as_mut_ptr() as *mut c_void;

        // GNUStep's NSData `initWithBytesNoCopy:length:deallocator:` has a
        // bug; it forgets to assign the input buffer and length to the
        // instance before it swizzles to NSDataWithDeallocatorBlock.
        // See https://github.com/gnustep/libs-base/pull/213
        // So we just use NSDataWithDeallocatorBlock directly.
        #[cfg(gnustep)]
        let cls = {
            let cls = Self::class();
            if cls == objc2::class!(NSData) {
                objc2::class!(NSDataWithDeallocatorBlock)
            } else {
                cls
            }
        };
        #[cfg(not(gnustep))]
        let cls = Self::class();
        unsafe {
            let obj: *mut Self = msg_send![cls, alloc];
            let obj: *mut Self = msg_send![
                obj,
                initWithBytesNoCopy: bytes_ptr,
                length: bytes.len(),
                deallocator: dealloc,
            ];
            core::mem::forget(bytes);
            Id::new(NonNull::new_unchecked(obj))
        }
    }
}

object_struct!(unsafe NSData, Shared);

unsafe impl INSData for NSData {}

unsafe impl INSCopying for NSData {
    type Output = NSData;
}

unsafe impl INSMutableCopying for NSData {
    type Output = NSMutableData;
}

pub unsafe trait INSMutableData: INSData {
    fn bytes_mut(&mut self) -> &mut [u8] {
        let ptr: *mut c_void = unsafe { msg_send![self, mutableBytes] };
        // The bytes pointer may be null for length zero
        let (ptr, len) = if ptr.is_null() {
            (0x1 as *mut u8, 0)
        } else {
            (ptr as *mut u8, self.len())
        };
        unsafe { slice::from_raw_parts_mut(ptr, len) }
    }

    fn set_len(&mut self, len: usize) {
        unsafe {
            let _: () = msg_send![self, setLength: len];
        }
    }

    fn append(&mut self, bytes: &[u8]) {
        let bytes_ptr = bytes.as_ptr() as *const c_void;
        unsafe {
            let _: () = msg_send![
                self,
                appendBytes: bytes_ptr,
                length:bytes.len(),
            ];
        }
    }

    fn replace_range(&mut self, range: Range<usize>, bytes: &[u8]) {
        let range = NSRange::from_range(range);
        let bytes_ptr = bytes.as_ptr() as *const c_void;
        unsafe {
            let _: () = msg_send![
                self,
                replaceBytesInRange:range,
                withBytes:bytes_ptr,
                length:bytes.len(),
            ];
        }
    }

    fn set_bytes(&mut self, bytes: &[u8]) {
        let len = self.len();
        self.replace_range(0..len, bytes);
    }
}

object_struct!(unsafe NSMutableData, Owned);

unsafe impl INSData for NSMutableData {}

unsafe impl INSMutableData for NSMutableData {}

unsafe impl INSCopying for NSMutableData {
    type Output = NSData;
}

unsafe impl INSMutableCopying for NSMutableData {
    type Output = NSMutableData;
}

#[cfg(test)]
mod tests {
    use super::{INSData, INSMutableData, NSData, NSMutableData};
    use crate::INSObject;
    #[cfg(feature = "block")]
    use alloc::vec;

    #[test]
    fn test_bytes() {
        let bytes = [3, 7, 16, 52, 112, 19];
        let data = NSData::with_bytes(&bytes);
        assert!(data.len() == bytes.len());
        assert!(data.bytes() == bytes);
    }

    #[test]
    fn test_no_bytes() {
        let data = NSData::new();
        assert!(Some(data.bytes()).is_some());
    }

    #[test]
    fn test_bytes_mut() {
        let mut data = NSMutableData::with_bytes(&[7, 16]);
        data.bytes_mut()[0] = 3;
        assert!(data.bytes() == [3, 16]);
    }

    #[test]
    fn test_set_len() {
        let mut data = NSMutableData::with_bytes(&[7, 16]);
        data.set_len(4);
        assert!(data.len() == 4);
        assert!(data.bytes() == [7, 16, 0, 0]);

        data.set_len(1);
        assert!(data.len() == 1);
        assert!(data.bytes() == [7]);
    }

    #[test]
    fn test_append() {
        let mut data = NSMutableData::with_bytes(&[7, 16]);
        data.append(&[3, 52]);
        assert!(data.len() == 4);
        assert!(data.bytes() == [7, 16, 3, 52]);
    }

    #[test]
    fn test_replace() {
        let mut data = NSMutableData::with_bytes(&[7, 16]);
        data.replace_range(0..0, &[3]);
        assert!(data.bytes() == [3, 7, 16]);

        data.replace_range(1..2, &[52, 13]);
        assert!(data.bytes() == [3, 52, 13, 16]);

        data.replace_range(2..4, &[6]);
        assert!(data.bytes() == [3, 52, 6]);

        data.set_bytes(&[8, 17]);
        assert!(data.bytes() == [8, 17]);
    }

    #[cfg(feature = "block")]
    #[test]
    fn test_from_vec() {
        let bytes = vec![3, 7, 16];
        let bytes_ptr = bytes.as_ptr();

        let data = NSData::from_vec(bytes);
        assert_eq!(data.bytes().as_ptr(), bytes_ptr);
    }
}
