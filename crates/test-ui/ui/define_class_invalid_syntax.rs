use objc2::define_class;
use objc2::runtime::NSObject;

define_class!(
    #[unsafe(super(NSObject))]
    struct NoAttribute;

    impl NoAttribute {
        fn test_no_attribute() {}
    }
);

define_class!(
    #[unsafe(super(NSObject))]
    struct EmptyAttribute;

    impl EmptyAttribute {
        #[unsafe(method())]
        fn test_empty() {}
    }
);

define_class!(
    #[unsafe(super(NSObject))]
    struct DuplicateAttributes;

    impl DuplicateAttributes {
        #[unsafe(method(duplicateAttribute))]
        #[unsafe(method(duplicateAttribute))]
        fn test_duplicate_attribute() {}

        #[unsafe(method_id(duplicateAttributeDifferent))]
        #[unsafe(method(duplicateAttributeDifferent))]
        fn test_duplicate_attribute_different() {}
    }
);

define_class!(
    #[unsafe(super(NSObject))]
    struct Dealloc;

    impl Dealloc {
        #[unsafe(method(dealloc))]
        fn dealloc(&self) {}
    }
);

define_class!(
    #[unsafe(super(NSObject))]
    struct InvalidBody;

    impl InvalidBody {
        #[unsafe(method(testInvalidBody))]
        fn test_invalid_body() { a - }
    }
);

define_class!(
    #[unsafe(super(NSObject))]
    struct Pattern;

    impl Pattern {
        #[unsafe(method(testPattern:))]
        fn test_pattern((_a, _b): (u32, i32)) {}
    }
);

define_class!(
    #[unsafe(super(NSObject))]
    struct BareSelf;

    impl BareSelf {
        #[unsafe(method(testSelf))]
        fn test_self(self) {}
    }
);

define_class!(
    #[unsafe(super(NSObject))]
    struct BasicBadSyntax;

    impl BasicBadSyntax {
        #[unsafe(method(testPub))]
        pub fn test_pub() {}
    }

    impl BasicBadSyntax {
        #[unsafe(method(testConst))]
        const fn test_const() {}
    }

    impl BasicBadSyntax {
        #[unsafe(method(testAsync))]
        async fn test_async() {}
    }

    impl BasicBadSyntax {
        #[unsafe(method(testExtern))]
        extern "C" fn test_extern() {}
    }

    impl BasicBadSyntax {
        #[unsafe(method(testFnFn))]
        fn fn test_fn_fn() {}
    }

    impl BasicBadSyntax {
        #[unsafe(method(testGeneric))]
        fn test_generic<T>() {}
    }

    impl BasicBadSyntax {
        #[unsafe(method(testNoBody))]
        fn test_no_body(&self);
    }

    impl BasicBadSyntax {
        #[unsafe(method(testUnfinished))]
        fn test_unfinished()
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
    struct MissingSuper;
);

define_class!(
    #[super(NSObject)]
    struct SafeSuper;
);

fn main() {}
