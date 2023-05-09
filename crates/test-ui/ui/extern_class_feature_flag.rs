use objc2::runtime::NSObject;
use objc2::{extern_class, mutability, ClassType};

extern_class!(
    #[cfg(not(feature = "mytest"))]
    pub struct MyTestEnabled;

    #[cfg(not(feature = "mytest"))]
    unsafe impl ClassType for MyTestEnabled {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

extern_class!(
    #[cfg(feature = "mytest")]
    pub struct MyTestDisabled;

    #[cfg(feature = "mytest")]
    unsafe impl ClassType for MyTestDisabled {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

fn main() {
    let _cls = MyTestEnabled::class();

    let _cls = MyTestDisabled::class();
}
