use core::ops::Deref;

use objc2::encode::{Encoding, RefEncode};
use objc2::runtime::AnyObject;
use objc2::{extern_class, AnyThread, Message};

#[repr(transparent)]
struct MyObject(AnyObject);

unsafe impl RefEncode for MyObject {
    const ENCODING_REF: Encoding = Encoding::Object;
}

unsafe impl Message for MyObject {}

impl Deref for MyObject {
    type Target = AnyObject;

    fn deref(&self) -> &AnyObject {
        &self.0
    }
}

extern_class!(
    #[unsafe(super(MyObject))]
    #[thread_kind = AnyThread]
    pub struct MyRootClass;
);

fn main() {}
