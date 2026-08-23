use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods};

extern_class!(
    #[unsafe(super(NSObject))]
    struct Foo;
);

impl Foo {
    extern_methods!(
        #[unsafe(method(hasBody:))]
        fn has_body() {
            unimplemented!()
        }
    );

    extern_methods!(
        #[unsafe(method(withPattern:))]
        fn with_pattern((_a, _b): (u32, i32)) {}
    );

    extern_methods!(
        #[unsafe(method(bareSelf))]
        fn bare_self(self);
    );

    extern_methods!(
        #[unsafe(method(isConst))]
        const fn is_const();
    );

    extern_methods!(
        #[unsafe(method(isAsync))]
        async fn is_async();
    );

    extern_methods!(
        #[unsafe(method(hasExtern))]
        extern "C" fn has_extern();
    );

    extern_methods!(
        #[unsafe(method(hasGeneric))]
        fn has_generic<T>();
    );

    extern_methods!(
        #[unsafe(method(unfinished))]
        fn unfinished()
    );

    extern_methods!(
        #![doc = "inner_attribute"]
    );

    extern_methods!(
        type TypeAlias = Self;
    );

    extern_methods!(
        const CONSTANT: () = ();
    );
}

fn main() {}
