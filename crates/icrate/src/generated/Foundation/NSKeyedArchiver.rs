use super::__exported::NSArray;
use super::__exported::NSData;
use super::__exported::NSMutableData;
use super::__exported::NSString;
use crate::Foundation::generated::NSCoder::*;
use crate::Foundation::generated::NSException::*;
use crate::Foundation::generated::NSGeometry::*;
use crate::Foundation::generated::NSPropertyList::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSKeyedArchiver;
    unsafe impl ClassType for NSKeyedArchiver {
        type Super = NSCoder;
    }
);
extern_methods!(
    unsafe impl NSKeyedArchiver {
        # [method_id (initRequiringSecureCoding :)]
        pub unsafe fn initRequiringSecureCoding(
            &self,
            requiresSecureCoding: bool,
        ) -> Id<Self, Shared>;
        # [method_id (archivedDataWithRootObject : requiringSecureCoding : error :)]
        pub unsafe fn archivedDataWithRootObject_requiringSecureCoding_error(
            object: &Object,
            requiresSecureCoding: bool,
        ) -> Result<Id<NSData, Shared>, Id<NSError, Shared>>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        # [method_id (initForWritingWithMutableData :)]
        pub unsafe fn initForWritingWithMutableData(
            &self,
            data: &NSMutableData,
        ) -> Id<Self, Shared>;
        # [method_id (archivedDataWithRootObject :)]
        pub unsafe fn archivedDataWithRootObject(rootObject: &Object) -> Id<NSData, Shared>;
        # [method (archiveRootObject : toFile :)]
        pub unsafe fn archiveRootObject_toFile(rootObject: &Object, path: &NSString) -> bool;
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSKeyedArchiverDelegate, Shared>>;
        # [method (setDelegate :)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSKeyedArchiverDelegate>);
        #[method(outputFormat)]
        pub unsafe fn outputFormat(&self) -> NSPropertyListFormat;
        # [method (setOutputFormat :)]
        pub unsafe fn setOutputFormat(&self, outputFormat: NSPropertyListFormat);
        #[method_id(encodedData)]
        pub unsafe fn encodedData(&self) -> Id<NSData, Shared>;
        #[method(finishEncoding)]
        pub unsafe fn finishEncoding(&self);
        # [method (setClassName : forClass :)]
        pub unsafe fn setClassName_forClass(codedName: Option<&NSString>, cls: &Class);
        # [method (setClassName : forClass :)]
        pub unsafe fn setClassName_forClass(&self, codedName: Option<&NSString>, cls: &Class);
        # [method_id (classNameForClass :)]
        pub unsafe fn classNameForClass(cls: &Class) -> Option<Id<NSString, Shared>>;
        # [method_id (classNameForClass :)]
        pub unsafe fn classNameForClass(&self, cls: &Class) -> Option<Id<NSString, Shared>>;
        # [method (encodeObject : forKey :)]
        pub unsafe fn encodeObject_forKey(&self, object: Option<&Object>, key: &NSString);
        # [method (encodeConditionalObject : forKey :)]
        pub unsafe fn encodeConditionalObject_forKey(
            &self,
            object: Option<&Object>,
            key: &NSString,
        );
        # [method (encodeBool : forKey :)]
        pub unsafe fn encodeBool_forKey(&self, value: bool, key: &NSString);
        # [method (encodeInt : forKey :)]
        pub unsafe fn encodeInt_forKey(&self, value: c_int, key: &NSString);
        # [method (encodeInt32 : forKey :)]
        pub unsafe fn encodeInt32_forKey(&self, value: i32, key: &NSString);
        # [method (encodeInt64 : forKey :)]
        pub unsafe fn encodeInt64_forKey(&self, value: int64_t, key: &NSString);
        # [method (encodeFloat : forKey :)]
        pub unsafe fn encodeFloat_forKey(&self, value: c_float, key: &NSString);
        # [method (encodeDouble : forKey :)]
        pub unsafe fn encodeDouble_forKey(&self, value: c_double, key: &NSString);
        # [method (encodeBytes : length : forKey :)]
        pub unsafe fn encodeBytes_length_forKey(
            &self,
            bytes: *mut u8,
            length: NSUInteger,
            key: &NSString,
        );
        #[method(requiresSecureCoding)]
        pub unsafe fn requiresSecureCoding(&self) -> bool;
        # [method (setRequiresSecureCoding :)]
        pub unsafe fn setRequiresSecureCoding(&self, requiresSecureCoding: bool);
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSKeyedUnarchiver;
    unsafe impl ClassType for NSKeyedUnarchiver {
        type Super = NSCoder;
    }
);
extern_methods!(
    unsafe impl NSKeyedUnarchiver {
        # [method_id (initForReadingFromData : error :)]
        pub unsafe fn initForReadingFromData_error(
            &self,
            data: &NSData,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
        # [method_id (unarchivedObjectOfClass : fromData : error :)]
        pub unsafe fn unarchivedObjectOfClass_fromData_error(
            cls: &Class,
            data: &NSData,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;
        # [method_id (unarchivedArrayOfObjectsOfClass : fromData : error :)]
        pub unsafe fn unarchivedArrayOfObjectsOfClass_fromData_error(
            cls: &Class,
            data: &NSData,
        ) -> Result<Id<NSArray, Shared>, Id<NSError, Shared>>;
        # [method_id (unarchivedDictionaryWithKeysOfClass : objectsOfClass : fromData : error :)]
        pub unsafe fn unarchivedDictionaryWithKeysOfClass_objectsOfClass_fromData_error(
            keyCls: &Class,
            valueCls: &Class,
            data: &NSData,
        ) -> Result<Id<NSDictionary, Shared>, Id<NSError, Shared>>;
        # [method_id (unarchivedObjectOfClasses : fromData : error :)]
        pub unsafe fn unarchivedObjectOfClasses_fromData_error(
            classes: &NSSet<TodoClass>,
            data: &NSData,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;
        # [method_id (unarchivedArrayOfObjectsOfClasses : fromData : error :)]
        pub unsafe fn unarchivedArrayOfObjectsOfClasses_fromData_error(
            classes: &NSSet<TodoClass>,
            data: &NSData,
        ) -> Result<Id<NSArray, Shared>, Id<NSError, Shared>>;
        # [method_id (unarchivedDictionaryWithKeysOfClasses : objectsOfClasses : fromData : error :)]
        pub unsafe fn unarchivedDictionaryWithKeysOfClasses_objectsOfClasses_fromData_error(
            keyClasses: &NSSet<TodoClass>,
            valueClasses: &NSSet<TodoClass>,
            data: &NSData,
        ) -> Result<Id<NSDictionary, Shared>, Id<NSError, Shared>>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        # [method_id (initForReadingWithData :)]
        pub unsafe fn initForReadingWithData(&self, data: &NSData) -> Id<Self, Shared>;
        # [method_id (unarchiveObjectWithData :)]
        pub unsafe fn unarchiveObjectWithData(data: &NSData) -> Option<Id<Object, Shared>>;
        # [method_id (unarchiveTopLevelObjectWithData : error :)]
        pub unsafe fn unarchiveTopLevelObjectWithData_error(
            data: &NSData,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;
        # [method_id (unarchiveObjectWithFile :)]
        pub unsafe fn unarchiveObjectWithFile(path: &NSString) -> Option<Id<Object, Shared>>;
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSKeyedUnarchiverDelegate, Shared>>;
        # [method (setDelegate :)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSKeyedUnarchiverDelegate>);
        #[method(finishDecoding)]
        pub unsafe fn finishDecoding(&self);
        # [method (setClass : forClassName :)]
        pub unsafe fn setClass_forClassName(cls: Option<&Class>, codedName: &NSString);
        # [method (setClass : forClassName :)]
        pub unsafe fn setClass_forClassName(&self, cls: Option<&Class>, codedName: &NSString);
        # [method (classForClassName :)]
        pub unsafe fn classForClassName(codedName: &NSString) -> Option<&Class>;
        # [method (classForClassName :)]
        pub unsafe fn classForClassName(&self, codedName: &NSString) -> Option<&Class>;
        # [method (containsValueForKey :)]
        pub unsafe fn containsValueForKey(&self, key: &NSString) -> bool;
        # [method_id (decodeObjectForKey :)]
        pub unsafe fn decodeObjectForKey(&self, key: &NSString) -> Option<Id<Object, Shared>>;
        # [method (decodeBoolForKey :)]
        pub unsafe fn decodeBoolForKey(&self, key: &NSString) -> bool;
        # [method (decodeIntForKey :)]
        pub unsafe fn decodeIntForKey(&self, key: &NSString) -> c_int;
        # [method (decodeInt32ForKey :)]
        pub unsafe fn decodeInt32ForKey(&self, key: &NSString) -> i32;
        # [method (decodeInt64ForKey :)]
        pub unsafe fn decodeInt64ForKey(&self, key: &NSString) -> int64_t;
        # [method (decodeFloatForKey :)]
        pub unsafe fn decodeFloatForKey(&self, key: &NSString) -> c_float;
        # [method (decodeDoubleForKey :)]
        pub unsafe fn decodeDoubleForKey(&self, key: &NSString) -> c_double;
        # [method (decodeBytesForKey : returnedLength :)]
        pub unsafe fn decodeBytesForKey_returnedLength(
            &self,
            key: &NSString,
            lengthp: *mut NSUInteger,
        ) -> *mut u8;
        #[method(requiresSecureCoding)]
        pub unsafe fn requiresSecureCoding(&self) -> bool;
        # [method (setRequiresSecureCoding :)]
        pub unsafe fn setRequiresSecureCoding(&self, requiresSecureCoding: bool);
        #[method(decodingFailurePolicy)]
        pub unsafe fn decodingFailurePolicy(&self) -> NSDecodingFailurePolicy;
        # [method (setDecodingFailurePolicy :)]
        pub unsafe fn setDecodingFailurePolicy(
            &self,
            decodingFailurePolicy: NSDecodingFailurePolicy,
        );
    }
);
pub type NSKeyedArchiverDelegate = NSObject;
pub type NSKeyedUnarchiverDelegate = NSObject;
extern_methods!(
    #[doc = "NSKeyedArchiverObjectSubstitution"]
    unsafe impl NSObject {
        #[method(classForKeyedArchiver)]
        pub unsafe fn classForKeyedArchiver(&self) -> Option<&Class>;
        # [method_id (replacementObjectForKeyedArchiver :)]
        pub unsafe fn replacementObjectForKeyedArchiver(
            &self,
            archiver: &NSKeyedArchiver,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(classFallbacksForKeyedArchiver)]
        pub unsafe fn classFallbacksForKeyedArchiver() -> Id<NSArray<NSString>, Shared>;
    }
);
extern_methods!(
    #[doc = "NSKeyedUnarchiverObjectSubstitution"]
    unsafe impl NSObject {
        #[method(classForKeyedUnarchiver)]
        pub unsafe fn classForKeyedUnarchiver() -> &Class;
    }
);
