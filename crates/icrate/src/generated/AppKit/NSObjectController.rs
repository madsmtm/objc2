use super::__exported::NSError;
use super::__exported::NSFetchRequest;
use super::__exported::NSManagedObjectContext;
use super::__exported::NSPredicate;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSController::*;
use crate::AppKit::generated::NSUserInterfaceValidation::*;
use crate::Foundation::generated::NSArray::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSObjectController;
    unsafe impl ClassType for NSObjectController {
        type Super = NSController;
    }
);
extern_methods!(
    unsafe impl NSObjectController {
        #[method_id(initWithContent:)]
        pub unsafe fn initWithContent(&self, content: Option<&Object>) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(content)]
        pub unsafe fn content(&self) -> Option<Id<Object, Shared>>;
        #[method(setContent:)]
        pub unsafe fn setContent(&self, content: Option<&Object>);
        #[method_id(selection)]
        pub unsafe fn selection(&self) -> Id<Object, Shared>;
        #[method_id(selectedObjects)]
        pub unsafe fn selectedObjects(&self) -> Id<NSArray, Shared>;
        #[method(automaticallyPreparesContent)]
        pub unsafe fn automaticallyPreparesContent(&self) -> bool;
        #[method(setAutomaticallyPreparesContent:)]
        pub unsafe fn setAutomaticallyPreparesContent(&self, automaticallyPreparesContent: bool);
        #[method(prepareContent)]
        pub unsafe fn prepareContent(&self);
        #[method(objectClass)]
        pub unsafe fn objectClass(&self) -> Option<&Class>;
        #[method(setObjectClass:)]
        pub unsafe fn setObjectClass(&self, objectClass: Option<&Class>);
        #[method_id(newObject)]
        pub unsafe fn newObject(&self) -> Id<Object, Shared>;
        #[method(addObject:)]
        pub unsafe fn addObject(&self, object: &Object);
        #[method(removeObject:)]
        pub unsafe fn removeObject(&self, object: &Object);
        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;
        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);
        #[method(add:)]
        pub unsafe fn add(&self, sender: Option<&Object>);
        #[method(canAdd)]
        pub unsafe fn canAdd(&self) -> bool;
        #[method(remove:)]
        pub unsafe fn remove(&self, sender: Option<&Object>);
        #[method(canRemove)]
        pub unsafe fn canRemove(&self) -> bool;
        #[method(validateUserInterfaceItem:)]
        pub unsafe fn validateUserInterfaceItem(&self, item: &NSValidatedUserInterfaceItem)
            -> bool;
    }
);
extern_methods!(
    #[doc = "NSManagedController"]
    unsafe impl NSObjectController {
        #[method_id(managedObjectContext)]
        pub unsafe fn managedObjectContext(&self) -> Option<Id<NSManagedObjectContext, Shared>>;
        #[method(setManagedObjectContext:)]
        pub unsafe fn setManagedObjectContext(
            &self,
            managedObjectContext: Option<&NSManagedObjectContext>,
        );
        #[method_id(entityName)]
        pub unsafe fn entityName(&self) -> Option<Id<NSString, Shared>>;
        #[method(setEntityName:)]
        pub unsafe fn setEntityName(&self, entityName: Option<&NSString>);
        #[method_id(fetchPredicate)]
        pub unsafe fn fetchPredicate(&self) -> Option<Id<NSPredicate, Shared>>;
        #[method(setFetchPredicate:)]
        pub unsafe fn setFetchPredicate(&self, fetchPredicate: Option<&NSPredicate>);
        #[method(fetchWithRequest:merge:error:)]
        pub unsafe fn fetchWithRequest_merge_error(
            &self,
            fetchRequest: Option<&NSFetchRequest>,
            merge: bool,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method(fetch:)]
        pub unsafe fn fetch(&self, sender: Option<&Object>);
        #[method(usesLazyFetching)]
        pub unsafe fn usesLazyFetching(&self) -> bool;
        #[method(setUsesLazyFetching:)]
        pub unsafe fn setUsesLazyFetching(&self, usesLazyFetching: bool);
        #[method_id(defaultFetchRequest)]
        pub unsafe fn defaultFetchRequest(&self) -> Id<NSFetchRequest, Shared>;
    }
);
