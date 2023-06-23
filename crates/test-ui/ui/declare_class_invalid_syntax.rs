use std::marker::PhantomData;

use objc2::declare::IvarEncode;
use objc2::rc::Id;
use objc2::runtime::NSObject;
use objc2::{declare_class, mutability, ClassType};

declare_class!(
    struct InvalidMethodDeclarations;

    unsafe impl ClassType for InvalidMethodDeclarations {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "InvalidMethodDeclarations";
    }

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
        fn test_method_id_bad_selector1() -> Id<Self> {
            unimplemented!()
        }

        #[method_id(retain)]
        fn test_method_id_bad_selector2() -> Id<Self> {
            unimplemented!()
        }

        #[method_id(release)]
        fn test_method_id_bad_selector3() -> Id<Self> {
            unimplemented!()
        }

        #[method_id(autorelease)]
        fn test_method_id_bad_selector4() -> Id<Self> {
            unimplemented!()
        }

        #[method_id(dealloc)]
        fn test_method_id_bad_selector5() -> Id<Self> {
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
);

declare_class!(
    struct InvalidField {
        field: i32,
    }

    unsafe impl ClassType for InvalidField {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "InvalidField";
    }
);

declare_class!(
    struct UnnecessaryIvarModule;

    mod ivars;

    unsafe impl ClassType for UnnecessaryIvarModule {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "UnnecessaryIvarModule";
    }
);

declare_class!(
    struct UnnecessaryIvarModuleWithFields {
        p: PhantomData<i32>,
    }

    mod ivars;

    unsafe impl ClassType for UnnecessaryIvarModuleWithFields {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "UnnecessaryIvarModuleWithFields";
    }
);

declare_class!(
    struct MissingIvarModule {
        field: IvarEncode<i32, "_field">,
    }

    unsafe impl ClassType for MissingIvarModule {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "MissingIvarModule";
    }
);

declare_class!(
    struct MissingMutability;

    unsafe impl ClassType for MissingMutability {
        type Super = NSObject;
    }
);

fn main() {}
