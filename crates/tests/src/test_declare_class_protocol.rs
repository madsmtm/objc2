#![cfg(feature = "Foundation_all")]
use icrate::Foundation::NSCopying;
use objc2::mutability::Immutable;
use objc2::rc::Id;
use objc2::runtime::{NSObject, NSZone};
use objc2::{declare_class, ClassType, DeclaredClass, ProtocolType};

#[test]
#[should_panic = "could not create new class TestDeclareClassDuplicate. Perhaps a class with that name already exists?"]
fn test_declare_class_duplicate() {
    declare_class!(
        struct Custom1;

        unsafe impl ClassType for Custom1 {
            type Super = NSObject;
            type Mutability = Immutable;
            const NAME: &'static str = "TestDeclareClassDuplicate";
        }

        impl DeclaredClass for Custom1 {}
    );

    declare_class!(
        struct Custom2;

        unsafe impl ClassType for Custom2 {
            type Super = NSObject;
            type Mutability = Immutable;
            const NAME: &'static str = "TestDeclareClassDuplicate";
        }

        impl DeclaredClass for Custom2 {}
    );

    let _cls = Custom1::class();
    // Should panic
    let _cls = Custom2::class();
}

#[test]
fn test_declare_class_protocol() {
    declare_class!(
        struct Custom;

        unsafe impl ClassType for Custom {
            type Super = NSObject;
            type Mutability = Immutable;
            const NAME: &'static str = "TestDeclareClassProtocolNotFound";
        }

        impl DeclaredClass for Custom {}

        unsafe impl NSCopying for Custom {
            #[method_id(copyWithZone:)]
            fn copy_with_zone(&self, _zone: *const NSZone) -> Id<Self> {
                unimplemented!()
            }
        }
    );

    let cls = Custom::class();
    assert!(cls.conforms_to(<dyn NSCopying>::protocol().unwrap()));
}

#[test]
#[cfg_attr(
    debug_assertions,
    should_panic = "declared invalid method -[TestDeclareClassInvalidMethod description]: expected return to have type code '@', but found 'v'"
)]
fn test_declare_class_invalid_method() {
    declare_class!(
        struct Custom;

        unsafe impl ClassType for Custom {
            type Super = NSObject;
            type Mutability = Immutable;
            const NAME: &'static str = "TestDeclareClassInvalidMethod";
        }

        impl DeclaredClass for Custom {}

        unsafe impl Custom {
            // Override `description` with a bad return type
            #[method(description)]
            fn description(&self) {}
        }
    );

    let _cls = Custom::class();
}

#[test]
#[cfg_attr(
    all(debug_assertions, feature = "verify"),
    should_panic = "must implement required protocol method -[NSCopying copyWithZone:]"
)]
fn test_declare_class_missing_protocol_method() {
    declare_class!(
        struct Custom;

        unsafe impl ClassType for Custom {
            type Super = NSObject;
            type Mutability = Immutable;
            const NAME: &'static str = "TestDeclareClassMissingProtocolMethod";
        }

        impl DeclaredClass for Custom {}

        unsafe impl NSCopying for Custom {
            // Missing required method
        }
    );

    let _cls = Custom::class();
}

#[test]
// #[cfg_attr(all(debug_assertions, feature = "verify"), should_panic = "...")]
fn test_declare_class_invalid_protocol_method() {
    declare_class!(
        struct Custom;

        unsafe impl ClassType for Custom {
            type Super = NSObject;
            type Mutability = Immutable;
            const NAME: &'static str = "TestDeclareClassInvalidProtocolMethod";
        }

        impl DeclaredClass for Custom {}

        unsafe impl NSCopying for Custom {
            // Override with a bad return type
            #[method(copyWithZone:)]
            fn copy_with_zone(&self, _zone: *const NSZone) -> u8 {
                42
            }
        }
    );

    let _cls = Custom::class();
}

#[test]
#[cfg_attr(
    all(debug_assertions, feature = "verify"),
    should_panic = "failed overriding protocol method -[NSCopying someOtherMethod]: method not found"
)]
fn test_declare_class_extra_protocol_method() {
    declare_class!(
        struct Custom;

        unsafe impl ClassType for Custom {
            type Super = NSObject;
            type Mutability = Immutable;
            const NAME: &'static str = "TestDeclareClassExtraProtocolMethod";
        }

        impl DeclaredClass for Custom {}

        unsafe impl NSCopying for Custom {
            #[method_id(copyWithZone:)]
            fn copy_with_zone(&self, _zone: *const NSZone) -> Id<Self> {
                unimplemented!()
            }

            // This doesn't exist on the protocol
            #[method(someOtherMethod)]
            fn some_other_method(&self) {}
        }
    );

    let _cls = Custom::class();
}
