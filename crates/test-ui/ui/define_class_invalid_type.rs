use objc2::define_class;
use objc2::rc::Retained;
use objc2::runtime::NSObject;

define_class!(
    #[unsafe(super(NSObject))]
    #[name = "CustomObject"]
    struct CustomObject;

    unsafe impl CustomObject {
        #[method(test1)]
        fn test1() -> Retained<Self> {
            unimplemented!()
        }

        #[method(test2)]
        fn test2() -> Vec<()> {
            unimplemented!()
        }

        #[method(test3)]
        fn test3(&self, arg: Box<u32>) {
            unimplemented!()
        }

        #[method(test4)]
        fn test4(&self, arg: Self) {
            unimplemented!()
        }
    }
);

fn main() {}
