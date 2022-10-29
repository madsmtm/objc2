#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSSwitch;
    unsafe impl ClassType for NSSwitch {
        type Super = NSControl;
    }
);
extern_methods!(
    unsafe impl NSSwitch {
        #[method(state)]
        pub unsafe fn state(&self) -> NSControlStateValue;
        #[method(setState:)]
        pub unsafe fn setState(&self, state: NSControlStateValue);
    }
);
