use super::__exported::NSData;
use super::__exported::NSMutableArray;
use super::__exported::NSMutableData;
use super::__exported::NSMutableDictionary;
use super::__exported::NSString;
use crate::Foundation::generated::NSCoder::*;
use crate::Foundation::generated::NSException::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSArchiver;
    unsafe impl ClassType for NSArchiver {
        type Super = NSCoder;
    }
);
extern_methods!(
    unsafe impl NSArchiver {
        #[method_id(initForWritingWithMutableData:)]
        pub unsafe fn initForWritingWithMutableData(
            &self,
            mdata: &NSMutableData,
        ) -> Id<Self, Shared>;
        #[method_id(archiverData)]
        pub unsafe fn archiverData(&self) -> Id<NSMutableData, Shared>;
        #[method(encodeRootObject:)]
        pub unsafe fn encodeRootObject(&self, rootObject: &Object);
        #[method(encodeConditionalObject:)]
        pub unsafe fn encodeConditionalObject(&self, object: Option<&Object>);
        #[method_id(archivedDataWithRootObject:)]
        pub unsafe fn archivedDataWithRootObject(rootObject: &Object) -> Id<NSData, Shared>;
        #[method(archiveRootObject:toFile:)]
        pub unsafe fn archiveRootObject_toFile(rootObject: &Object, path: &NSString) -> bool;
        #[method(encodeClassName:intoClassName:)]
        pub unsafe fn encodeClassName_intoClassName(
            &self,
            trueName: &NSString,
            inArchiveName: &NSString,
        );
        #[method_id(classNameEncodedForTrueClassName:)]
        pub unsafe fn classNameEncodedForTrueClassName(
            &self,
            trueName: &NSString,
        ) -> Option<Id<NSString, Shared>>;
        #[method(replaceObject:withObject:)]
        pub unsafe fn replaceObject_withObject(&self, object: &Object, newObject: &Object);
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUnarchiver;
    unsafe impl ClassType for NSUnarchiver {
        type Super = NSCoder;
    }
);
extern_methods!(
    unsafe impl NSUnarchiver {
        #[method_id(initForReadingWithData:)]
        pub unsafe fn initForReadingWithData(&self, data: &NSData) -> Option<Id<Self, Shared>>;
        #[method(setObjectZone:)]
        pub unsafe fn setObjectZone(&self, zone: *mut NSZone);
        #[method(objectZone)]
        pub unsafe fn objectZone(&self) -> *mut NSZone;
        #[method(isAtEnd)]
        pub unsafe fn isAtEnd(&self) -> bool;
        #[method(systemVersion)]
        pub unsafe fn systemVersion(&self) -> c_uint;
        #[method_id(unarchiveObjectWithData:)]
        pub unsafe fn unarchiveObjectWithData(data: &NSData) -> Option<Id<Object, Shared>>;
        #[method_id(unarchiveObjectWithFile:)]
        pub unsafe fn unarchiveObjectWithFile(path: &NSString) -> Option<Id<Object, Shared>>;
        #[method(decodeClassName:asClassName:)]
        pub unsafe fn decodeClassName_asClassName(inArchiveName: &NSString, trueName: &NSString);
        #[method(decodeClassName:asClassName:)]
        pub unsafe fn decodeClassName_asClassName(
            &self,
            inArchiveName: &NSString,
            trueName: &NSString,
        );
        #[method_id(classNameDecodedForArchiveClassName:)]
        pub unsafe fn classNameDecodedForArchiveClassName(
            inArchiveName: &NSString,
        ) -> Id<NSString, Shared>;
        #[method_id(classNameDecodedForArchiveClassName:)]
        pub unsafe fn classNameDecodedForArchiveClassName(
            &self,
            inArchiveName: &NSString,
        ) -> Id<NSString, Shared>;
        #[method(replaceObject:withObject:)]
        pub unsafe fn replaceObject_withObject(&self, object: &Object, newObject: &Object);
    }
);
extern_methods!(
    #[doc = "NSArchiverCallback"]
    unsafe impl NSObject {
        #[method(classForArchiver)]
        pub unsafe fn classForArchiver(&self) -> Option<&Class>;
        #[method_id(replacementObjectForArchiver:)]
        pub unsafe fn replacementObjectForArchiver(
            &self,
            archiver: &NSArchiver,
        ) -> Option<Id<Object, Shared>>;
    }
);
