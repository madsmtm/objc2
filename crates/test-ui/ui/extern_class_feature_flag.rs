use objc2::runtime::NSObject;
use objc2::{extern_class, ClassType};

extern_class!(
    #[cfg(not(test))]
    #[unsafe(super(NSObject))]
    pub struct MyTestEnabled;
);

extern_class!(
    #[cfg(test)]
    #[unsafe(super(NSObject))]
    pub struct MyTestDisabled;
);

fn main() {
    let _cls = MyTestEnabled::class();

    let _cls = MyTestDisabled::class();
}
