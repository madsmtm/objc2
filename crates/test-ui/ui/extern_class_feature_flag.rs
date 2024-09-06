use objc2::runtime::NSObject;
use objc2::{extern_class, ClassType};

extern_class!(
    #[cfg(not(test))]
    pub struct MyTestEnabled;

    #[cfg(not(test))]
    unsafe impl ClassType for MyTestEnabled {
        type Super = NSObject;
    }
);

extern_class!(
    #[cfg(test)]
    pub struct MyTestDisabled;

    #[cfg(test)]
    unsafe impl ClassType for MyTestDisabled {
        type Super = NSObject;
    }
);

fn main() {
    let _cls = MyTestEnabled::class();

    let _cls = MyTestDisabled::class();
}
