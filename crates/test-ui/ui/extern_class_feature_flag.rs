use objc2::{extern_class, ClassType};
use objc2::runtime::NSObject;

extern_class!(
    #[cfg(not(feature = "mytest"))]
    pub struct MyTestEnabled;

    #[cfg(not(feature = "mytest"))]
    unsafe impl ClassType for MyTestEnabled {
        type Super = NSObject;
    }
);

extern_class!(
    #[cfg(feature = "mytest")]
    pub struct MyTestDisabled;

    #[cfg(feature = "mytest")]
    unsafe impl ClassType for MyTestDisabled {
        type Super = NSObject;
    }
);

fn main() {
    let _cls = MyTestEnabled::class();

    let _cls = MyTestDisabled::class();
}
