use objc2::rc::{Allocated, Id, Ownership, Shared};
use objc2::{extern_methods, Message};

use crate::Foundation::{
    NSArray, NSDictionary, NSError, NSMutableArray, NSMutableDictionary, NSString, NSURL,
};

extern_methods!(
    /// NSArrayCreation
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSArray<ObjectType, ObjectTypeOwnership>
    {
        #[method_id(initWithContentsOfURL:error:_)]
        pub unsafe fn initWithContentsOfURL_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSArray<ObjectType, ObjectTypeOwnership>
    {
        #[method_id(initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Option<Allocated<Self>>,
            path: &NSString,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            url: &NSURL,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_methods!(
    /// NSMutableArrayCreation
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSMutableArray<ObjectType, ObjectTypeOwnership>
    {
        #[method_id(initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Option<Allocated<Self>>,
            path: &NSString,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            url: &NSURL,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl<
            KeyType: Message,
            ObjectType: Message,
            KeyTypeOwnership: Ownership,
            ObjectTypeOwnership: Ownership,
        > NSDictionary<KeyType, ObjectType, KeyTypeOwnership, ObjectTypeOwnership>
    {
        #[method_id(initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Option<Allocated<Self>>,
            path: &NSString,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            url: &NSURL,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_methods!(
    /// NSMutableDictionaryCreation
    unsafe impl<
            KeyType: Message,
            ObjectType: Message,
            KeyTypeOwnership: Ownership,
            ObjectTypeOwnership: Ownership,
        > NSMutableDictionary<KeyType, ObjectType, KeyTypeOwnership, ObjectTypeOwnership>
    {
        #[method_id(initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Option<Allocated<Self>>,
            path: &NSString,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            url: &NSURL,
        ) -> Option<Id<Self, Shared>>;
    }
);
