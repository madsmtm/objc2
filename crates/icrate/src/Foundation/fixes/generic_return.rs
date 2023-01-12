use crate::common::*;
use crate::Foundation;

extern_methods!(
    /// NSArrayCreation
    #[cfg(feature = "Foundation_NSArray")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        Foundation::NSArray<ObjectType, ObjectTypeOwnership>
    {
        #[cfg(all(feature = "Foundation_NSURL", feature = "Foundation_NSError"))]
        #[method_id(initWithContentsOfURL:error:_)]
        pub unsafe fn initWithContentsOfURL_error(
            this: Option<Allocated<Self>>,
            url: &Foundation::NSURL,
        ) -> Result<Id<Self, Shared>, Id<Foundation::NSError, Shared>>;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "Foundation_NSArray")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        Foundation::NSArray<ObjectType, ObjectTypeOwnership>
    {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Option<Allocated<Self>>,
            path: &Foundation::NSString,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            url: &Foundation::NSURL,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_methods!(
    /// NSMutableArrayCreation
    #[cfg(feature = "Foundation_NSMutableArray")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        Foundation::NSMutableArray<ObjectType, ObjectTypeOwnership>
    {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Option<Allocated<Self>>,
            path: &Foundation::NSString,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            url: &Foundation::NSURL,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "Foundation_NSDictionary")]
    unsafe impl<
            KeyType: Message,
            ObjectType: Message,
            KeyTypeOwnership: Ownership,
            ObjectTypeOwnership: Ownership,
        > Foundation::NSDictionary<KeyType, ObjectType, KeyTypeOwnership, ObjectTypeOwnership>
    {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Option<Allocated<Self>>,
            path: &Foundation::NSString,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            url: &Foundation::NSURL,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_methods!(
    /// NSMutableDictionaryCreation
    #[cfg(feature = "Foundation_NSMutableDictionary")]
    unsafe impl<
            KeyType: Message,
            ObjectType: Message,
            KeyTypeOwnership: Ownership,
            ObjectTypeOwnership: Ownership,
        >
        Foundation::NSMutableDictionary<KeyType, ObjectType, KeyTypeOwnership, ObjectTypeOwnership>
    {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Option<Allocated<Self>>,
            path: &Foundation::NSString,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            url: &Foundation::NSURL,
        ) -> Option<Id<Self, Shared>>;
    }
);
