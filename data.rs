use std::ops::Range;
use std::slice;
use libc::c_void;

use objc::Id;
use {INSObject, INSCopying, INSMutableCopying, NSRange};

pub trait INSData : INSObject {
    fn len(&self) -> usize {
        unsafe {
            msg_send![self, length]
        }
    }

    fn bytes(&self) -> &[u8] {
        let len = self.len();
        unsafe {
            let ptr: *const c_void = msg_send![self, bytes];
            slice::from_raw_parts(ptr as *const u8, len)
        }
    }

    fn with_bytes(bytes: &[u8]) -> Id<Self> {
        let cls = <Self as INSObject>::class();
        unsafe {
            let obj: *mut Self = msg_send![cls, alloc];
            let obj: *mut Self = msg_send![obj, initWithBytes:bytes.as_ptr()
                                                       length:bytes.len()];
            Id::from_retained_ptr(obj)
        }
    }
}

object_struct!(NSData);

impl INSData for NSData { }

impl INSCopying for NSData {
    type Output = NSData;
}

impl INSMutableCopying for NSData {
    type Output = NSMutableData;
}

pub trait INSMutableData : INSData {
    fn bytes_mut(&mut self) -> &mut [u8] {
        let len = self.len();
        unsafe {
            let ptr: *mut c_void = msg_send![self, bytes];
            slice::from_raw_parts_mut(ptr as *mut u8, len)
        }
    }

    fn set_len(&mut self, len: usize) {
        unsafe {
            let _: () = msg_send![self, setLength:len];
        }
    }

    fn append(&mut self, bytes: &[u8]) {
        unsafe {
            let _: () = msg_send![self, appendBytes:bytes.as_ptr()
                                             length:bytes.len()];
        }
    }

    fn replace_range(&mut self, range: Range<usize>, bytes: &[u8]) {
        let range = NSRange::from_range(range);
        unsafe {
            let _: () = msg_send![self, replaceBytesInRange:range
                                                  withBytes:bytes.as_ptr()
                                                     length:bytes.len()];
        }
    }

    fn set_bytes(&mut self, bytes: &[u8]) {
        let len = self.len();
        self.replace_range(0..len, bytes);
    }
}

object_struct!(NSMutableData);

impl INSData for NSMutableData { }

impl INSMutableData for NSMutableData { }

impl INSCopying for NSMutableData {
    type Output = NSData;
}

impl INSMutableCopying for NSMutableData {
    type Output = NSMutableData;
}

#[cfg(test)]
mod tests {
    use objc::Id;
    use super::{INSData, NSData};

    #[test]
    fn test_bytes() {
        let bytes = [3u8, 7, 16, 52, 112, 19];
        let data: Id<NSData> = INSData::with_bytes(&bytes);
        assert!(data.len() == bytes.len());
        assert!(data.bytes() == bytes.as_slice());
    }
}
