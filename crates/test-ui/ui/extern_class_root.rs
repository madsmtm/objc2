use core::ops::Deref;

use objc2::encode::{Encoding, RefEncode};
use objc2::runtime::AnyObject;
use objc2::{extern_class, AllocAnyThread, ClassType, Message};

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
    pub struct MyRootClass;

    unsafe impl ClassType for MyRootClass {
        type Super = MyObject;
        type ThreadKind = dyn AllocAnyThread;
    }
);

fn main() {}
