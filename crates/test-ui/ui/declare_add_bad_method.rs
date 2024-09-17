use objc2::declare::ClassBuilder;
use objc2::rc::{Allocated, Retained};
use objc2::runtime::{NSObject, Sel};
use objc2::{sel, ClassType};

fn main() {
    let builder = ClassBuilder::new(c"DeclareClassTest", NSObject::class()).unwrap();

    // Test receiver
    unsafe {
        fn foo(_obj: NSObject, _sel: Sel) {}
        builder.add_method(sel!(foo), foo as fn(_, _));
    }
    unsafe {
        fn foo(_obj: Allocated<NSObject>, _sel: Sel) {}
        builder.add_method(sel!(foo), foo as fn(_, _));
    }

    // Test return type
    unsafe {
        fn foo(_obj: &NSObject, _sel: Sel) -> bool {
            unimplemented!()
        }
        builder.add_method(sel!(foo), foo as fn(_, _) -> _);
    }
    unsafe {
        fn foo(_obj: &NSObject, _sel: Sel) -> Retained<NSObject> {
            unimplemented!()
        }
        builder.add_method(sel!(foo), foo as fn(_, _) -> _);
    }

    // Test arguments
    unsafe {
        fn foo(_obj: &NSObject, _sel: Sel, _item: bool) {}
        builder.add_method(sel!(foo:), foo as fn(_, _, _));
    }
}
