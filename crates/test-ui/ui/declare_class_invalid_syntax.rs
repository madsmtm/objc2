use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{declare_class, mutability, ClassType, DeclaredClass};

declare_class!(
    struct InvalidMethodDeclarations;

    unsafe impl ClassType for InvalidMethodDeclarations {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "InvalidMethodDeclarations";
    }

    impl DeclaredClass for InvalidMethodDeclarations {}

    unsafe impl InvalidMethodDeclarations {
        fn test_no_attribute() {
            unimplemented!()
        }

        #[method(duplicateAttribute)]
        #[method(duplicateAttribute)]
        fn test_duplicate_attribute() {}

        #[method_id(duplicateAttributeDifferent)]
        #[method(duplicateAttributeDifferent)]
        fn test_duplicate_attribute_different() {}

        #[method_id(testMethodId)]
        fn test_method_id_no_return() {
            unimplemented!()
        }

        #[method(testInvalidBody)]
        fn test_invalid_body() {
            a -
        }

        #[method(testPattern:)]
        fn test_pattern((a, b): (u32, i32)) {
            unimplemented!()
        }

        #[method(testSelf)]
        fn test_self(self) {
            unimplemented!()
        }
    }

    unsafe impl InvalidMethodDeclarations {
        #[method(testPub)]
        pub fn test_pub() {}
    }

    unsafe impl InvalidMethodDeclarations {
        #[method(testConst)]
        const fn test_const() {}
    }

    unsafe impl InvalidMethodDeclarations {
        #[method(testAsync)]
        async fn test_async() {}
    }

    unsafe impl InvalidMethodDeclarations {
        #[method(testExtern)]
        extern "C" fn test_extern() {}
    }

    unsafe impl InvalidMethodDeclarations {
        #[method(testFnFn)]
        fn fn test_fn_fn() {}
    }

    unsafe impl InvalidMethodDeclarations {
        #[method(testGeneric)]
        fn test_generic<T>() {}
    }

    unsafe impl InvalidMethodDeclarations {
        #[method(testNoBody)]
        fn test_no_body(&self);
    }

    unsafe impl InvalidMethodDeclarations {
        #[method(testUnfinished)]
        fn test_unfinished()
    }

    unsafe impl InvalidMethodDeclarations {
        #[method_id(alloc)]
        fn test_method_id_bad_selector1() -> Retained<Self> {
            unimplemented!()
        }

        #[method_id(retain)]
        fn test_method_id_bad_selector2() -> Retained<Self> {
            unimplemented!()
        }

        #[method_id(release)]
        fn test_method_id_bad_selector3() -> Retained<Self> {
            unimplemented!()
        }

        #[method_id(autorelease)]
        fn test_method_id_bad_selector4() -> Retained<Self> {
            unimplemented!()
        }

        #[method_id(dealloc)]
        fn test_method_id_bad_selector5() -> Retained<Self> {
            unimplemented!()
        }
    }

    unsafe impl InvalidMethodDeclarations {
        #[method(dealloc)]
        fn deallocMethod(&mut self) {}
    }

    unsafe impl InvalidMethodDeclarations {
        #![doc = "inner_attribute"]
    }

    unsafe impl InvalidMethodDeclarations {
        type TypeAlias = Self;
    }

    unsafe impl InvalidMethodDeclarations {
        const CONSTANT: () = ();
    }
);

declare_class!(
    struct MissingName;

    unsafe impl ClassType for MissingName {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }

    impl DeclaredClass for MissingName {}
);

declare_class!(
    struct MissingMutability;

    unsafe impl ClassType for MissingMutability {
        type Super = NSObject;
    }

    impl DeclaredClass for MissingMutability {}
);

declare_class!(
    struct MissingDeclaredClass;

    unsafe impl ClassType for MissingDeclaredClass {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "MissingDeclaredClass";
    }
);

fn main() {}
