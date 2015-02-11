use std::slice;
use libc::c_void;

use objc::Id;
use {INSObject, INSCopying};

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
