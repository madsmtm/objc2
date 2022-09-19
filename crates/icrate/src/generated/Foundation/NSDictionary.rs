use super::__exported::NSArray;
use super::__exported::NSSet;
use super::__exported::NSString;
use super::__exported::NSURL;
use crate::Foundation::generated::NSEnumerator::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSDictionary<KeyType: Message, ObjectType: Message>;
    unsafe impl<KeyType: Message, ObjectType: Message> ClassType for NSDictionary<KeyType, ObjectType> {
        type Super = NSObject;
    }
);
impl<KeyType: Message, ObjectType: Message> NSDictionary<KeyType, ObjectType> {
    pub unsafe fn objectForKey(&self, aKey: &KeyType) -> Option<Id<ObjectType, Shared>> {
        msg_send_id![self, objectForKey: aKey]
    }
    pub unsafe fn keyEnumerator(&self) -> TodoGenerics {
        msg_send![self, keyEnumerator]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn initWithObjects_forKeys_count(
        &self,
        objects: TodoArray,
        keys: TodoArray,
        cnt: NSUInteger,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithObjects: objects, forKeys: keys, count: cnt]
    }
    pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: coder]
    }
    pub unsafe fn count(&self) -> NSUInteger {
        msg_send![self, count]
    }
}
#[doc = "NSExtendedDictionary"]
impl<KeyType: Message, ObjectType: Message> NSDictionary<KeyType, ObjectType> {
    pub unsafe fn allKeysForObject(&self, anObject: &ObjectType) -> TodoGenerics {
        msg_send![self, allKeysForObject: anObject]
    }
    pub unsafe fn descriptionWithLocale(&self, locale: Option<&Object>) -> Id<NSString, Shared> {
        msg_send_id![self, descriptionWithLocale: locale]
    }
    pub unsafe fn descriptionWithLocale_indent(
        &self,
        locale: Option<&Object>,
        level: NSUInteger,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, descriptionWithLocale: locale, indent: level]
    }
    pub unsafe fn isEqualToDictionary(&self, otherDictionary: TodoGenerics) -> bool {
        msg_send![self, isEqualToDictionary: otherDictionary]
    }
    pub unsafe fn objectEnumerator(&self) -> TodoGenerics {
        msg_send![self, objectEnumerator]
    }
    pub unsafe fn objectsForKeys_notFoundMarker(
        &self,
        keys: TodoGenerics,
        marker: &ObjectType,
    ) -> TodoGenerics {
        msg_send![self, objectsForKeys: keys, notFoundMarker: marker]
    }
    pub unsafe fn writeToURL_error(&self, url: &NSURL, error: *mut *mut NSError) -> bool {
        msg_send![self, writeToURL: url, error: error]
    }
    pub unsafe fn keysSortedByValueUsingSelector(&self, comparator: Sel) -> TodoGenerics {
        msg_send![self, keysSortedByValueUsingSelector: comparator]
    }
    pub unsafe fn getObjects_andKeys_count(
        &self,
        objects: TodoArray,
        keys: TodoArray,
        count: NSUInteger,
    ) {
        msg_send![self, getObjects: objects, andKeys: keys, count: count]
    }
    pub unsafe fn objectForKeyedSubscript(&self, key: &KeyType) -> Option<Id<ObjectType, Shared>> {
        msg_send_id![self, objectForKeyedSubscript: key]
    }
    pub unsafe fn enumerateKeysAndObjectsUsingBlock(&self, block: TodoBlock) {
        msg_send![self, enumerateKeysAndObjectsUsingBlock: block]
    }
    pub unsafe fn enumerateKeysAndObjectsWithOptions_usingBlock(
        &self,
        opts: NSEnumerationOptions,
        block: TodoBlock,
    ) {
        msg_send![
            self,
            enumerateKeysAndObjectsWithOptions: opts,
            usingBlock: block
        ]
    }
    pub unsafe fn keysSortedByValueUsingComparator(&self, cmptr: NSComparator) -> TodoGenerics {
        msg_send![self, keysSortedByValueUsingComparator: cmptr]
    }
    pub unsafe fn keysSortedByValueWithOptions_usingComparator(
        &self,
        opts: NSSortOptions,
        cmptr: NSComparator,
    ) -> TodoGenerics {
        msg_send![
            self,
            keysSortedByValueWithOptions: opts,
            usingComparator: cmptr
        ]
    }
    pub unsafe fn keysOfEntriesPassingTest(&self, predicate: TodoBlock) -> TodoGenerics {
        msg_send![self, keysOfEntriesPassingTest: predicate]
    }
    pub unsafe fn keysOfEntriesWithOptions_passingTest(
        &self,
        opts: NSEnumerationOptions,
        predicate: TodoBlock,
    ) -> TodoGenerics {
        msg_send![self, keysOfEntriesWithOptions: opts, passingTest: predicate]
    }
    pub unsafe fn allKeys(&self) -> TodoGenerics {
        msg_send![self, allKeys]
    }
    pub unsafe fn allValues(&self) -> TodoGenerics {
        msg_send![self, allValues]
    }
    pub unsafe fn description(&self) -> Id<NSString, Shared> {
        msg_send_id![self, description]
    }
    pub unsafe fn descriptionInStringsFileFormat(&self) -> Id<NSString, Shared> {
        msg_send_id![self, descriptionInStringsFileFormat]
    }
}
#[doc = "NSDeprecated"]
impl<KeyType: Message, ObjectType: Message> NSDictionary<KeyType, ObjectType> {
    pub unsafe fn getObjects_andKeys(&self, objects: TodoArray, keys: TodoArray) {
        msg_send![self, getObjects: objects, andKeys: keys]
    }
    pub unsafe fn dictionaryWithContentsOfFile(path: &NSString) -> TodoGenerics {
        msg_send![Self::class(), dictionaryWithContentsOfFile: path]
    }
    pub unsafe fn dictionaryWithContentsOfURL(url: &NSURL) -> TodoGenerics {
        msg_send![Self::class(), dictionaryWithContentsOfURL: url]
    }
    pub unsafe fn initWithContentsOfFile(&self, path: &NSString) -> TodoGenerics {
        msg_send![self, initWithContentsOfFile: path]
    }
    pub unsafe fn initWithContentsOfURL(&self, url: &NSURL) -> TodoGenerics {
        msg_send![self, initWithContentsOfURL: url]
    }
    pub unsafe fn writeToFile_atomically(&self, path: &NSString, useAuxiliaryFile: bool) -> bool {
        msg_send![self, writeToFile: path, atomically: useAuxiliaryFile]
    }
    pub unsafe fn writeToURL_atomically(&self, url: &NSURL, atomically: bool) -> bool {
        msg_send![self, writeToURL: url, atomically: atomically]
    }
}
#[doc = "NSDictionaryCreation"]
impl<KeyType: Message, ObjectType: Message> NSDictionary<KeyType, ObjectType> {
    pub unsafe fn dictionary() -> Id<Self, Shared> {
        msg_send_id![Self::class(), dictionary]
    }
    pub unsafe fn dictionaryWithObject_forKey(
        object: &ObjectType,
        key: TodoGenerics,
    ) -> Id<Self, Shared> {
        msg_send_id![Self::class(), dictionaryWithObject: object, forKey: key]
    }
    pub unsafe fn dictionaryWithObjects_forKeys_count(
        objects: TodoArray,
        keys: TodoArray,
        cnt: NSUInteger,
    ) -> Id<Self, Shared> {
        msg_send_id![
            Self::class(),
            dictionaryWithObjects: objects,
            forKeys: keys,
            count: cnt
        ]
    }
    pub unsafe fn dictionaryWithDictionary(dict: TodoGenerics) -> Id<Self, Shared> {
        msg_send_id![Self::class(), dictionaryWithDictionary: dict]
    }
    pub unsafe fn dictionaryWithObjects_forKeys(
        objects: TodoGenerics,
        keys: TodoGenerics,
    ) -> Id<Self, Shared> {
        msg_send_id![Self::class(), dictionaryWithObjects: objects, forKeys: keys]
    }
    pub unsafe fn initWithDictionary(&self, otherDictionary: TodoGenerics) -> Id<Self, Shared> {
        msg_send_id![self, initWithDictionary: otherDictionary]
    }
    pub unsafe fn initWithDictionary_copyItems(
        &self,
        otherDictionary: TodoGenerics,
        flag: bool,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithDictionary: otherDictionary, copyItems: flag]
    }
    pub unsafe fn initWithObjects_forKeys(
        &self,
        objects: TodoGenerics,
        keys: TodoGenerics,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithObjects: objects, forKeys: keys]
    }
    pub unsafe fn initWithContentsOfURL_error(
        &self,
        url: &NSURL,
        error: *mut *mut NSError,
    ) -> TodoGenerics {
        msg_send![self, initWithContentsOfURL: url, error: error]
    }
    pub unsafe fn dictionaryWithContentsOfURL_error(
        url: &NSURL,
        error: *mut *mut NSError,
    ) -> TodoGenerics {
        msg_send![
            Self::class(),
            dictionaryWithContentsOfURL: url,
            error: error
        ]
    }
}
__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSMutableDictionary<KeyType: Message, ObjectType: Message>;
    unsafe impl<KeyType: Message, ObjectType: Message> ClassType
        for NSMutableDictionary<KeyType, ObjectType>
    {
        type Super = NSDictionary;
    }
);
impl<KeyType: Message, ObjectType: Message> NSMutableDictionary<KeyType, ObjectType> {
    pub unsafe fn removeObjectForKey(&self, aKey: &KeyType) {
        msg_send![self, removeObjectForKey: aKey]
    }
    pub unsafe fn setObject_forKey(&self, anObject: &ObjectType, aKey: TodoGenerics) {
        msg_send![self, setObject: anObject, forKey: aKey]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn initWithCapacity(&self, numItems: NSUInteger) -> Id<Self, Shared> {
        msg_send_id![self, initWithCapacity: numItems]
    }
    pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: coder]
    }
}
#[doc = "NSExtendedMutableDictionary"]
impl<KeyType: Message, ObjectType: Message> NSMutableDictionary<KeyType, ObjectType> {
    pub unsafe fn addEntriesFromDictionary(&self, otherDictionary: TodoGenerics) {
        msg_send![self, addEntriesFromDictionary: otherDictionary]
    }
    pub unsafe fn removeAllObjects(&self) {
        msg_send![self, removeAllObjects]
    }
    pub unsafe fn removeObjectsForKeys(&self, keyArray: TodoGenerics) {
        msg_send![self, removeObjectsForKeys: keyArray]
    }
    pub unsafe fn setDictionary(&self, otherDictionary: TodoGenerics) {
        msg_send![self, setDictionary: otherDictionary]
    }
    pub unsafe fn setObject_forKeyedSubscript(&self, obj: Option<&ObjectType>, key: TodoGenerics) {
        msg_send![self, setObject: obj, forKeyedSubscript: key]
    }
}
#[doc = "NSMutableDictionaryCreation"]
impl<KeyType: Message, ObjectType: Message> NSMutableDictionary<KeyType, ObjectType> {
    pub unsafe fn dictionaryWithCapacity(numItems: NSUInteger) -> Id<Self, Shared> {
        msg_send_id![Self::class(), dictionaryWithCapacity: numItems]
    }
    pub unsafe fn dictionaryWithContentsOfFile(path: &NSString) -> TodoGenerics {
        msg_send![Self::class(), dictionaryWithContentsOfFile: path]
    }
    pub unsafe fn dictionaryWithContentsOfURL(url: &NSURL) -> TodoGenerics {
        msg_send![Self::class(), dictionaryWithContentsOfURL: url]
    }
    pub unsafe fn initWithContentsOfFile(&self, path: &NSString) -> TodoGenerics {
        msg_send![self, initWithContentsOfFile: path]
    }
    pub unsafe fn initWithContentsOfURL(&self, url: &NSURL) -> TodoGenerics {
        msg_send![self, initWithContentsOfURL: url]
    }
}
#[doc = "NSSharedKeySetDictionary"]
impl<KeyType: Message, ObjectType: Message> NSDictionary<KeyType, ObjectType> {
    pub unsafe fn sharedKeySetForKeys(keys: TodoGenerics) -> Id<Object, Shared> {
        msg_send_id![Self::class(), sharedKeySetForKeys: keys]
    }
}
#[doc = "NSSharedKeySetDictionary"]
impl<KeyType: Message, ObjectType: Message> NSMutableDictionary<KeyType, ObjectType> {
    pub unsafe fn dictionaryWithSharedKeySet(keyset: &Object) -> TodoGenerics {
        msg_send![Self::class(), dictionaryWithSharedKeySet: keyset]
    }
}
#[doc = "NSGenericFastEnumeraiton"]
impl<K: Message, V: Message> NSDictionary<K, V> {
    pub unsafe fn countByEnumeratingWithState_objects_count(
        &self,
        state: NonNull<NSFastEnumerationState>,
        buffer: TodoArray,
        len: NSUInteger,
    ) -> NSUInteger {
        msg_send![
            self,
            countByEnumeratingWithState: state,
            objects: buffer,
            count: len
        ]
    }
}
