use objc2::define_class;
use objc2::rc::Retained;
use objc2::runtime::NSObject;

define_class!(
    #[unsafe(super(NSObject))]
    #[name = "InvalidMethodDeclarations"]
    struct InvalidMethodDeclarations;

    impl InvalidMethodDeclarations {
        fn test_no_attribute() {
            unimplemented!()
        }

        #[unsafe(method(duplicateAttribute))]
        #[unsafe(method(duplicateAttribute))]
        fn test_duplicate_attribute() {}

        #[unsafe(method_id(duplicateAttributeDifferent))]
        #[unsafe(method(duplicateAttributeDifferent))]
        fn test_duplicate_attribute_different() {}

        #[unsafe(method_id(testMethodRetained))]
        fn test_retained_no_return() {
            unimplemented!()
        }

        #[unsafe(method(testInvalidBody))]
        fn test_invalid_body() {
            a -
        }

        #[unsafe(method(testPattern:))]
        fn test_pattern((a, b): (u32, i32)) {
            unimplemented!()
        }

        #[unsafe(method(testSelf))]
        fn test_self(self) {
            unimplemented!()
        }
    }

    impl InvalidMethodDeclarations {
        #[unsafe(method(testPub))]
        pub fn test_pub() {}
    }

    impl InvalidMethodDeclarations {
        #[unsafe(method(testConst))]
        const fn test_const() {}
    }

    impl InvalidMethodDeclarations {
        #[unsafe(method(testAsync))]
        async fn test_async() {}
    }

    impl InvalidMethodDeclarations {
        #[unsafe(method(testExtern))]
        extern "C" fn test_extern() {}
    }

    impl InvalidMethodDeclarations {
        #[unsafe(method(testFnFn))]
        fn fn test_fn_fn() {}
    }

    impl InvalidMethodDeclarations {
        #[unsafe(method(testGeneric))]
        fn test_generic<T>() {}
    }

    impl InvalidMethodDeclarations {
        #[unsafe(method(testNoBody))]
        fn test_no_body(&self);
    }

    impl InvalidMethodDeclarations {
        #[unsafe(method(testUnfinished))]
        fn test_unfinished()
    }

    impl InvalidMethodDeclarations {
        #[unsafe(method_id(alloc))]
        fn test_retained_bad_selector1() -> Retained<Self> {
            unimplemented!()
        }

        #[unsafe(method_id(retain))]
        fn test_retained_bad_selector2() -> Retained<Self> {
            unimplemented!()
        }

        #[unsafe(method_id(release))]
        fn test_retained_bad_selector3() -> Retained<Self> {
            unimplemented!()
        }

        #[unsafe(method_id(autorelease))]
        fn test_retained_bad_selector4() -> Retained<Self> {
            unimplemented!()
        }

        #[unsafe(method_id(dealloc))]
        fn test_retained_bad_selector5() -> Retained<Self> {
            unimplemented!()
        }
    }

    impl InvalidMethodDeclarations {
        #[unsafe(method(dealloc))]
        fn deallocMethod(&mut self) {}
    }

    impl InvalidMethodDeclarations {
        #![doc = "inner_attribute"]
    }

    impl InvalidMethodDeclarations {
        type TypeAlias = Self;
    }

    impl InvalidMethodDeclarations {
        const CONSTANT: () = ();
    }
);

define_class!(
    #[unsafe(super(NSObject))]
    struct MissingName;
);

define_class!(
    #[name = "MissingSuper"]
    struct MissingSuper;
);

define_class!(
    #[super(NSObject)]
    #[name = "SafeSuper"]
    struct SafeSuper;
);

define_class!(
    struct MissingBoth;
);

define_class!(
    #[unsafe(super(NSObject))]
    #[name = "HasRepr"]
    #[repr(transparent)]
    struct HasRepr;
);

fn main() {}
