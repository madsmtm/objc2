use core::ops::{Deref, DerefMut};

use objc2::encode::{Encoding, RefEncode};
use objc2::runtime::AnyObject;
use objc2::{extern_class, mutability, ClassType, Message};

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

impl DerefMut for MyObject {
    fn deref_mut(&mut self) -> &mut AnyObject {
        &mut self.0
    }
}

extern_class!(
    pub struct MyRootClass;

    unsafe impl ClassType for MyRootClass {
        type Super = MyObject;
        type Mutability = mutability::InteriorMutable;
    }
);

fn main() {}
