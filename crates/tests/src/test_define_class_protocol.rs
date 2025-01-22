use objc2::rc::Retained;
use objc2::runtime::{NSObject, NSZone};
use objc2::{define_class, ClassType, ProtocolType};
use objc2_foundation::NSCopying;

#[test]
#[should_panic = "could not create new class TestDefineClassDuplicate. Perhaps a class with that name already exists?"]
fn test_define_class_duplicate() {
    define_class!(
        #[unsafe(super(NSObject))]
        #[name = "TestDefineClassDuplicate"]
        struct Custom1;
    );

    define_class!(
        #[unsafe(super(NSObject))]
        #[name = "TestDefineClassDuplicate"]
        struct Custom2;
    );

    let _cls = Custom1::class();
    // Should panic
    let _cls = Custom2::class();
}

#[test]
fn test_define_class_protocol() {
    define_class!(
        #[unsafe(super(NSObject))]
        #[name = "TestDefineClassProtocolNotFound"]
        struct Custom;

        unsafe impl NSCopying for Custom {
            #[unsafe(method_id(copyWithZone:))]
            fn copy_with_zone(&self, _zone: *const NSZone) -> Retained<Self> {
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
    should_panic = "defined invalid method -[TestDefineClassInvalidMethod description]: expected return to have type code '@', but found 'v'"
)]
fn test_define_class_invalid_method() {
    define_class!(
        #[unsafe(super(NSObject))]
        #[name = "TestDefineClassInvalidMethod"]
        struct Custom;

        impl Custom {
            // Override `description` with a bad return type
            #[unsafe(method(description))]
            fn description(&self) {}
        }
    );

    let _cls = Custom::class();
}

#[test]
#[cfg_attr(
    debug_assertions,
    should_panic = "must implement required protocol method -[NSCopying copyWithZone:]"
)]
fn test_define_class_missing_protocol_method() {
    define_class!(
        #[unsafe(super(NSObject))]
        #[name = "TestDefineClassMissingProtocolMethod"]
        struct Custom;

        unsafe impl NSCopying for Custom {
            // Missing required method
        }
    );

    let _cls = Custom::class();
}

#[test]
// #[cfg_attr(debug_assertions, should_panic = "...")]
fn test_define_class_invalid_protocol_method() {
    define_class!(
        #[unsafe(super(NSObject))]
        #[name = "TestDefineClassInvalidProtocolMethod"]
        struct Custom;

        unsafe impl NSCopying for Custom {
            // Override with a bad return type
            #[unsafe(method(copyWithZone:))]
            fn copy_with_zone(&self, _zone: *const NSZone) -> u8 {
                42
            }
        }
    );

    let _cls = Custom::class();
}

#[test]
#[cfg_attr(
    debug_assertions,
    should_panic = "failed overriding protocol method -[NSCopying someOtherMethod]: method not found"
)]
fn test_define_class_extra_protocol_method() {
    define_class!(
        #[unsafe(super(NSObject))]
        #[name = "TestDefineClassExtraProtocolMethod"]
        struct Custom;

        unsafe impl NSCopying for Custom {
            #[unsafe(method_id(copyWithZone:))]
            fn copy_with_zone(&self, _zone: *const NSZone) -> Retained<Self> {
                unimplemented!()
            }

            // This doesn't exist on the protocol
            #[unsafe(method(someOtherMethod))]
            fn some_other_method(&self) {}
        }
    );

    let _cls = Custom::class();
}
