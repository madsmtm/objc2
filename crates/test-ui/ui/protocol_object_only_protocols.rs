//! Ensure that `ProtocolObject` cannot be incorrectly constructed.
use icrate::Foundation::NSCopying;
use objc2::runtime::{NSObject, NSObjectProtocol, ProtocolObject};

trait Foo {
    fn foo(&self) {}
}

impl<T: ?Sized> Foo for T {}

fn main() {
    let obj = NSObject::new();
    let _: &ProtocolObject<NSObject> = ProtocolObject::from_ref(&*obj);
    let _: &ProtocolObject<dyn Send> = ProtocolObject::from_ref(&*obj);
    let _: &ProtocolObject<dyn Foo> = ProtocolObject::from_ref(&*obj);

    // `NSObject` is neither `Send` nor `Sync`.
    let _: &ProtocolObject<dyn NSObjectProtocol + Send> = ProtocolObject::from_ref(&*obj);
    let _: &ProtocolObject<dyn NSObjectProtocol + Sync> = ProtocolObject::from_ref(&*obj);
    let _: &ProtocolObject<dyn NSObjectProtocol + Send + Sync> = ProtocolObject::from_ref(&*obj);

    // `NSObject` is not `NSCopying`.
    let _: &ProtocolObject<dyn NSCopying> = ProtocolObject::from_ref(&*obj);

    // `dyn NSCopying + Send` does not implement `ImplementedBy` (yet).
    let _: &ProtocolObject<dyn NSCopying + Send> = ProtocolObject::from_ref(&*obj);
}
