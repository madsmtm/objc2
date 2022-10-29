//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};

__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSDictionary<KeyType: Message, ObjectType: Message>;

    unsafe impl<KeyType: Message, ObjectType: Message> ClassType for NSDictionary<KeyType, ObjectType> {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl<KeyType: Message, ObjectType: Message> NSDictionary<KeyType, ObjectType> {
        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;

        #[method_id(objectForKey:)]
        pub unsafe fn objectForKey(&self, aKey: &KeyType) -> Option<Id<ObjectType, Shared>>;

        #[method_id(keyEnumerator)]
        pub unsafe fn keyEnumerator(&self) -> Id<NSEnumerator<KeyType>, Shared>;

        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;

        #[method_id(initWithObjects:forKeys:count:)]
        pub unsafe fn initWithObjects_forKeys_count(
            &self,
            objects: TodoArray,
            keys: TodoArray,
            cnt: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
    }
);

extern_methods!(
    /// NSExtendedDictionary
    unsafe impl<KeyType: Message, ObjectType: Message> NSDictionary<KeyType, ObjectType> {
        #[method_id(allKeys)]
        pub unsafe fn allKeys(&self) -> Id<NSArray<KeyType>, Shared>;

        #[method_id(allKeysForObject:)]
        pub unsafe fn allKeysForObject(
            &self,
            anObject: &ObjectType,
        ) -> Id<NSArray<KeyType>, Shared>;

        #[method_id(allValues)]
        pub unsafe fn allValues(&self) -> Id<NSArray<ObjectType>, Shared>;

        #[method_id(description)]
        pub unsafe fn description(&self) -> Id<NSString, Shared>;

        #[method_id(descriptionInStringsFileFormat)]
        pub unsafe fn descriptionInStringsFileFormat(&self) -> Id<NSString, Shared>;

        #[method_id(descriptionWithLocale:)]
        pub unsafe fn descriptionWithLocale(&self, locale: Option<&Object>)
            -> Id<NSString, Shared>;

        #[method_id(descriptionWithLocale:indent:)]
        pub unsafe fn descriptionWithLocale_indent(
            &self,
            locale: Option<&Object>,
            level: NSUInteger,
        ) -> Id<NSString, Shared>;

        #[method(isEqualToDictionary:)]
        pub unsafe fn isEqualToDictionary(
            &self,
            otherDictionary: &NSDictionary<KeyType, ObjectType>,
        ) -> bool;

        #[method_id(objectEnumerator)]
        pub unsafe fn objectEnumerator(&self) -> Id<NSEnumerator<ObjectType>, Shared>;

        #[method_id(objectsForKeys:notFoundMarker:)]
        pub unsafe fn objectsForKeys_notFoundMarker(
            &self,
            keys: &NSArray<KeyType>,
            marker: &ObjectType,
        ) -> Id<NSArray<ObjectType>, Shared>;

        #[method(writeToURL:error:)]
        pub unsafe fn writeToURL_error(&self, url: &NSURL) -> Result<(), Id<NSError, Shared>>;

        #[method_id(keysSortedByValueUsingSelector:)]
        pub unsafe fn keysSortedByValueUsingSelector(
            &self,
            comparator: Sel,
        ) -> Id<NSArray<KeyType>, Shared>;

        #[method(getObjects:andKeys:count:)]
        pub unsafe fn getObjects_andKeys_count(
            &self,
            objects: TodoArray,
            keys: TodoArray,
            count: NSUInteger,
        );

        #[method_id(objectForKeyedSubscript:)]
        pub unsafe fn objectForKeyedSubscript(
            &self,
            key: &KeyType,
        ) -> Option<Id<ObjectType, Shared>>;

        #[method(enumerateKeysAndObjectsUsingBlock:)]
        pub unsafe fn enumerateKeysAndObjectsUsingBlock(&self, block: TodoBlock);

        #[method(enumerateKeysAndObjectsWithOptions:usingBlock:)]
        pub unsafe fn enumerateKeysAndObjectsWithOptions_usingBlock(
            &self,
            opts: NSEnumerationOptions,
            block: TodoBlock,
        );

        #[method_id(keysSortedByValueUsingComparator:)]
        pub unsafe fn keysSortedByValueUsingComparator(
            &self,
            cmptr: NSComparator,
        ) -> Id<NSArray<KeyType>, Shared>;

        #[method_id(keysSortedByValueWithOptions:usingComparator:)]
        pub unsafe fn keysSortedByValueWithOptions_usingComparator(
            &self,
            opts: NSSortOptions,
            cmptr: NSComparator,
        ) -> Id<NSArray<KeyType>, Shared>;

        #[method_id(keysOfEntriesPassingTest:)]
        pub unsafe fn keysOfEntriesPassingTest(
            &self,
            predicate: TodoBlock,
        ) -> Id<NSSet<KeyType>, Shared>;

        #[method_id(keysOfEntriesWithOptions:passingTest:)]
        pub unsafe fn keysOfEntriesWithOptions_passingTest(
            &self,
            opts: NSEnumerationOptions,
            predicate: TodoBlock,
        ) -> Id<NSSet<KeyType>, Shared>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl<KeyType: Message, ObjectType: Message> NSDictionary<KeyType, ObjectType> {
        #[method(getObjects:andKeys:)]
        pub unsafe fn getObjects_andKeys(&self, objects: TodoArray, keys: TodoArray);

        #[method_id(dictionaryWithContentsOfFile:)]
        pub unsafe fn dictionaryWithContentsOfFile(
            path: &NSString,
        ) -> Option<Id<NSDictionary<KeyType, ObjectType>, Shared>>;

        #[method_id(dictionaryWithContentsOfURL:)]
        pub unsafe fn dictionaryWithContentsOfURL(
            url: &NSURL,
        ) -> Option<Id<NSDictionary<KeyType, ObjectType>, Shared>>;

        #[method_id(initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            &self,
            path: &NSString,
        ) -> Option<Id<NSDictionary<KeyType, ObjectType>, Shared>>;

        #[method_id(initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            &self,
            url: &NSURL,
        ) -> Option<Id<NSDictionary<KeyType, ObjectType>, Shared>>;

        #[method(writeToFile:atomically:)]
        pub unsafe fn writeToFile_atomically(
            &self,
            path: &NSString,
            useAuxiliaryFile: bool,
        ) -> bool;

        #[method(writeToURL:atomically:)]
        pub unsafe fn writeToURL_atomically(&self, url: &NSURL, atomically: bool) -> bool;
    }
);

extern_methods!(
    /// NSDictionaryCreation
    unsafe impl<KeyType: Message, ObjectType: Message> NSDictionary<KeyType, ObjectType> {
        #[method_id(dictionary)]
        pub unsafe fn dictionary() -> Id<Self, Shared>;

        #[method_id(dictionaryWithObject:forKey:)]
        pub unsafe fn dictionaryWithObject_forKey(
            object: &ObjectType,
            key: &NSCopying,
        ) -> Id<Self, Shared>;

        #[method_id(dictionaryWithObjects:forKeys:count:)]
        pub unsafe fn dictionaryWithObjects_forKeys_count(
            objects: TodoArray,
            keys: TodoArray,
            cnt: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(dictionaryWithDictionary:)]
        pub unsafe fn dictionaryWithDictionary(
            dict: &NSDictionary<KeyType, ObjectType>,
        ) -> Id<Self, Shared>;

        #[method_id(dictionaryWithObjects:forKeys:)]
        pub unsafe fn dictionaryWithObjects_forKeys(
            objects: &NSArray<ObjectType>,
            keys: &NSArray<NSCopying>,
        ) -> Id<Self, Shared>;

        #[method_id(initWithDictionary:)]
        pub unsafe fn initWithDictionary(
            &self,
            otherDictionary: &NSDictionary<KeyType, ObjectType>,
        ) -> Id<Self, Shared>;

        #[method_id(initWithDictionary:copyItems:)]
        pub unsafe fn initWithDictionary_copyItems(
            &self,
            otherDictionary: &NSDictionary<KeyType, ObjectType>,
            flag: bool,
        ) -> Id<Self, Shared>;

        #[method_id(initWithObjects:forKeys:)]
        pub unsafe fn initWithObjects_forKeys(
            &self,
            objects: &NSArray<ObjectType>,
            keys: &NSArray<NSCopying>,
        ) -> Id<Self, Shared>;

        #[method_id(initWithContentsOfURL:error:)]
        pub unsafe fn initWithContentsOfURL_error(
            &self,
            url: &NSURL,
        ) -> Result<Id<NSDictionary<NSString, ObjectType>, Shared>, Id<NSError, Shared>>;

        #[method_id(dictionaryWithContentsOfURL:error:)]
        pub unsafe fn dictionaryWithContentsOfURL_error(
            url: &NSURL,
        ) -> Result<Id<NSDictionary<NSString, ObjectType>, Shared>, Id<NSError, Shared>>;
    }
);

__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSMutableDictionary<KeyType: Message, ObjectType: Message>;

    unsafe impl<KeyType: Message, ObjectType: Message> ClassType
        for NSMutableDictionary<KeyType, ObjectType>
    {
        type Super = NSDictionary;
    }
);

extern_methods!(
    unsafe impl<KeyType: Message, ObjectType: Message> NSMutableDictionary<KeyType, ObjectType> {
        #[method(removeObjectForKey:)]
        pub unsafe fn removeObjectForKey(&self, aKey: &KeyType);

        #[method(setObject:forKey:)]
        pub unsafe fn setObject_forKey(&self, anObject: &ObjectType, aKey: &NSCopying);

        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;

        #[method_id(initWithCapacity:)]
        pub unsafe fn initWithCapacity(&self, numItems: NSUInteger) -> Id<Self, Shared>;

        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
    }
);

extern_methods!(
    /// NSExtendedMutableDictionary
    unsafe impl<KeyType: Message, ObjectType: Message> NSMutableDictionary<KeyType, ObjectType> {
        #[method(addEntriesFromDictionary:)]
        pub unsafe fn addEntriesFromDictionary(
            &self,
            otherDictionary: &NSDictionary<KeyType, ObjectType>,
        );

        #[method(removeAllObjects)]
        pub unsafe fn removeAllObjects(&self);

        #[method(removeObjectsForKeys:)]
        pub unsafe fn removeObjectsForKeys(&self, keyArray: &NSArray<KeyType>);

        #[method(setDictionary:)]
        pub unsafe fn setDictionary(&self, otherDictionary: &NSDictionary<KeyType, ObjectType>);

        #[method(setObject:forKeyedSubscript:)]
        pub unsafe fn setObject_forKeyedSubscript(&self, obj: Option<&ObjectType>, key: &NSCopying);
    }
);

extern_methods!(
    /// NSMutableDictionaryCreation
    unsafe impl<KeyType: Message, ObjectType: Message> NSMutableDictionary<KeyType, ObjectType> {
        #[method_id(dictionaryWithCapacity:)]
        pub unsafe fn dictionaryWithCapacity(numItems: NSUInteger) -> Id<Self, Shared>;

        #[method_id(dictionaryWithContentsOfFile:)]
        pub unsafe fn dictionaryWithContentsOfFile(
            path: &NSString,
        ) -> Option<Id<NSMutableDictionary<KeyType, ObjectType>, Shared>>;

        #[method_id(dictionaryWithContentsOfURL:)]
        pub unsafe fn dictionaryWithContentsOfURL(
            url: &NSURL,
        ) -> Option<Id<NSMutableDictionary<KeyType, ObjectType>, Shared>>;

        #[method_id(initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            &self,
            path: &NSString,
        ) -> Option<Id<NSMutableDictionary<KeyType, ObjectType>, Shared>>;

        #[method_id(initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            &self,
            url: &NSURL,
        ) -> Option<Id<NSMutableDictionary<KeyType, ObjectType>, Shared>>;
    }
);

extern_methods!(
    /// NSSharedKeySetDictionary
    unsafe impl<KeyType: Message, ObjectType: Message> NSDictionary<KeyType, ObjectType> {
        #[method_id(sharedKeySetForKeys:)]
        pub unsafe fn sharedKeySetForKeys(keys: &NSArray<NSCopying>) -> Id<Object, Shared>;
    }
);

extern_methods!(
    /// NSSharedKeySetDictionary
    unsafe impl<KeyType: Message, ObjectType: Message> NSMutableDictionary<KeyType, ObjectType> {
        #[method_id(dictionaryWithSharedKeySet:)]
        pub unsafe fn dictionaryWithSharedKeySet(
            keyset: &Object,
        ) -> Id<NSMutableDictionary<KeyType, ObjectType>, Shared>;
    }
);

extern_methods!(
    /// NSGenericFastEnumeraiton
    unsafe impl<K: Message, V: Message> NSDictionary<K, V> {
        #[method(countByEnumeratingWithState:objects:count:)]
        pub unsafe fn countByEnumeratingWithState_objects_count(
            &self,
            state: NonNull<NSFastEnumerationState>,
            buffer: TodoArray,
            len: NSUInteger,
        ) -> NSUInteger;
    }
);
