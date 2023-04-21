use core::ops::{Deref, DerefMut};

use objc2::encode::{Encoding, RefEncode};
use objc2::runtime::Object;
use objc2::{extern_class, mutability, ClassType, Message};

#[repr(transparent)]
struct MyObject(Object);

unsafe impl RefEncode for MyObject {
    const ENCODING_REF: Encoding = Encoding::Object;
}

unsafe impl Message for MyObject {}

impl Deref for MyObject {
    type Target = Object;

    fn deref(&self) -> &Object {
        &self.0
    }
}

impl DerefMut for MyObject {
    fn deref_mut(&mut self) -> &mut Object {
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
