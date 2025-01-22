use objc2::extern_methods;

extern_methods!(
    #[unsafe(method(foo))]
    fn class_method();
);

extern_methods!(
    #[unsafe(method(foo))]
    fn instance_method(&self);
);

fn main() {}
