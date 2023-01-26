//! The negative cases of protocol.rs `impl_traits`
use objc2::{ClassType, declare_class, extern_protocol, ProtocolObject, ProtocolType};
use objc2::runtime::{NSObjectProtocol, NSObject};

extern_protocol!(
    unsafe trait Foo {}

    unsafe impl ProtocolType for dyn Foo {}
);

extern_protocol!(
    unsafe trait Bar: NSObjectProtocol {}

    unsafe impl ProtocolType for dyn Bar {}
);

extern_protocol!(
    unsafe trait FooBar: Foo + Bar {}

    unsafe impl ProtocolType for dyn FooBar {}
);

extern_protocol!(
    unsafe trait FooFooBar: Foo + FooBar {}

    unsafe impl ProtocolType for dyn FooFooBar {}
);

declare_class!(
    struct DummyClass;

    unsafe impl ClassType for DummyClass {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for DummyClass {}
unsafe impl Foo for DummyClass {}
unsafe impl Bar for DummyClass {}
unsafe impl FooBar for DummyClass {}
// unsafe impl FooFooBar for DummyClass {}

fn main() {
    fn impl_nsobject<T: NSObjectProtocol>() {}
    fn impl_foo<T: Foo>() {}
    fn impl_bar<T: Bar>() {}
    fn impl_foobar<T: FooBar>() {}
    fn impl_foofoobar<T: FooFooBar>() {}

    impl_nsobject::<NSObject>();
    impl_nsobject::<ProtocolObject<NSObject>>();
    impl_nsobject::<ProtocolObject<dyn Foo>>();
    impl_nsobject::<ProtocolObject<dyn Bar>>();
    impl_nsobject::<ProtocolObject<dyn FooBar>>();
    impl_nsobject::<ProtocolObject<dyn FooFooBar>>();
    impl_nsobject::<DummyClass>();

    impl_foo::<ProtocolObject<dyn Foo>>();
    impl_foo::<ProtocolObject<dyn Bar>>();
    impl_foo::<ProtocolObject<dyn FooBar>>();
    impl_foo::<ProtocolObject<dyn FooFooBar>>();
    impl_foo::<DummyClass>();

    impl_bar::<ProtocolObject<dyn Foo>>();
    impl_bar::<ProtocolObject<dyn Bar>>();
    impl_bar::<ProtocolObject<dyn FooBar>>();
    impl_bar::<ProtocolObject<dyn FooFooBar>>();
    impl_bar::<DummyClass>();

    impl_foobar::<ProtocolObject<dyn Foo>>();
    impl_foobar::<ProtocolObject<dyn Bar>>();
    impl_foobar::<ProtocolObject<dyn FooBar>>();
    impl_foobar::<ProtocolObject<dyn FooFooBar>>();
    impl_foobar::<DummyClass>();

    impl_foofoobar::<ProtocolObject<dyn Foo>>();
    impl_foofoobar::<ProtocolObject<dyn Bar>>();
    impl_foofoobar::<ProtocolObject<dyn FooBar>>();
    impl_foofoobar::<ProtocolObject<dyn FooFooBar>>();
    impl_foofoobar::<DummyClass>();
}
