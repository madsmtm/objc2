use objc2::define_class;
use objc2::rc::Retained;
use objc2::runtime::NSObject;

define_class!(
    #[unsafe(super(NSObject))]
    #[name = "CustomObject"]
    struct CustomObject;

    impl CustomObject {
        #[unsafe(method(test1))]
        fn test1() -> Retained<Self> {
            unimplemented!()
        }

        #[unsafe(method(test2))]
        fn test2() -> Vec<()> {
            unimplemented!()
        }

        #[unsafe(method(test3))]
        fn test3(&self, arg: Box<u32>) {
            unimplemented!()
        }

        #[unsafe(method(test4))]
        fn test4(&self, arg: Self) {
            unimplemented!()
        }
    }
);

fn main() {}
