use objc2::define_class;
use objc2::runtime::NSObject;

define_class!(
    #[unsafe(super(NSObject))]
    struct CustomObject;

    impl CustomObject {
        #[unsafe(method(test1))]
        fn test1() -> Vec<()> {
            unimplemented!()
        }

        #[unsafe(method(test2:))]
        fn test2(&self, _arg: Box<u32>) {
            unimplemented!()
        }

        #[unsafe(method(test3:))]
        fn test3(&self, _arg: Self) {
            unimplemented!()
        }
    }
);

fn main() {}
