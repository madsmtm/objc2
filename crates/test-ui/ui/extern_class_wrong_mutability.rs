use objc2::runtime::NSObject;
use objc2::{extern_class, mutability, ClassType};

extern_class!(
    pub struct MyMainThreadClass;

    unsafe impl ClassType for MyMainThreadClass {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
    }
);

extern_class!(
    pub struct MyAnyThreadClass;

    unsafe impl ClassType for MyAnyThreadClass {
        type Super = MyMainThreadClass;
        type Mutability = mutability::InteriorMutable;
    }
);

extern_class!(
    pub struct MyImmutableClass1;

    unsafe impl ClassType for MyImmutableClass1 {
        type Super = NSObject;
        type Mutability = mutability::ImmutableWithMutableSubclass<MyMutableClass1>;
    }
);

extern_class!(
    pub struct MyMutableClass1;

    unsafe impl ClassType for MyMutableClass1 {
        type Super = MyImmutableClass1;
        type Mutability = mutability::MutableWithImmutableSuperclass<NSObject>;
    }
);

extern_class!(
    pub struct MyImmutableClass2;

    unsafe impl ClassType for MyImmutableClass2 {
        type Super = NSObject;
        type Mutability = mutability::ImmutableWithMutableSubclass<NSObject>;
    }
);

extern_class!(
    pub struct MyMutableClass2;

    unsafe impl ClassType for MyMutableClass2 {
        type Super = MyImmutableClass2;
        type Mutability = mutability::MutableWithImmutableSuperclass<MyImmutableClass2>;
    }
);

fn main() {}
