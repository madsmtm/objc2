use super::__exported::NSManagedObjectContext;
use super::__exported::NSManagedObjectModel;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSDocument::*;
use crate::Foundation::generated::NSDictionary::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPersistentDocument;
    unsafe impl ClassType for NSPersistentDocument {
        type Super = NSDocument;
    }
);
extern_methods!(
    unsafe impl NSPersistentDocument {
        #[method_id(managedObjectContext)]
        pub unsafe fn managedObjectContext(&self) -> Option<Id<NSManagedObjectContext, Shared>>;
        #[method(setManagedObjectContext:)]
        pub unsafe fn setManagedObjectContext(
            &self,
            managedObjectContext: Option<&NSManagedObjectContext>,
        );
        #[method_id(managedObjectModel)]
        pub unsafe fn managedObjectModel(&self) -> Option<Id<NSManagedObjectModel, Shared>>;
        #[method(configurePersistentStoreCoordinatorForURL:ofType:modelConfiguration:storeOptions:error:)]
        pub unsafe fn configurePersistentStoreCoordinatorForURL_ofType_modelConfiguration_storeOptions_error(
            &self,
            url: &NSURL,
            fileType: &NSString,
            configuration: Option<&NSString>,
            storeOptions: Option<&NSDictionary<NSString, Object>>,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method_id(persistentStoreTypeForFileType:)]
        pub unsafe fn persistentStoreTypeForFileType(
            &self,
            fileType: &NSString,
        ) -> Id<NSString, Shared>;
        #[method(writeToURL:ofType:forSaveOperation:originalContentsURL:error:)]
        pub unsafe fn writeToURL_ofType_forSaveOperation_originalContentsURL_error(
            &self,
            absoluteURL: &NSURL,
            typeName: &NSString,
            saveOperation: NSSaveOperationType,
            absoluteOriginalContentsURL: Option<&NSURL>,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method(readFromURL:ofType:error:)]
        pub unsafe fn readFromURL_ofType_error(
            &self,
            absoluteURL: &NSURL,
            typeName: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method(revertToContentsOfURL:ofType:error:)]
        pub unsafe fn revertToContentsOfURL_ofType_error(
            &self,
            inAbsoluteURL: &NSURL,
            inTypeName: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSPersistentDocument {
        #[method(configurePersistentStoreCoordinatorForURL:ofType:error:)]
        pub unsafe fn configurePersistentStoreCoordinatorForURL_ofType_error(
            &self,
            url: Option<&NSURL>,
            fileType: Option<&NSString>,
        ) -> Result<(), Id<NSError, Shared>>;
    }
);
