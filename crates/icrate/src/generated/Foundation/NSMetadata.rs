use super::__exported::NSArray;
use super::__exported::NSDictionary;
use super::__exported::NSOperationQueue;
use super::__exported::NSPredicate;
use super::__exported::NSSortDescriptor;
use super::__exported::NSString;
use super::__exported::NSURL;
use crate::Foundation::generated::NSDate::*;
use crate::Foundation::generated::NSMetadataAttributes::*;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSMetadataQuery;
    unsafe impl ClassType for NSMetadataQuery {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSMetadataQuery {
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSMetadataQueryDelegate, Shared>>;
        # [method (setDelegate :)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSMetadataQueryDelegate>);
        #[method_id(predicate)]
        pub unsafe fn predicate(&self) -> Option<Id<NSPredicate, Shared>>;
        # [method (setPredicate :)]
        pub unsafe fn setPredicate(&self, predicate: Option<&NSPredicate>);
        #[method_id(sortDescriptors)]
        pub unsafe fn sortDescriptors(&self) -> Id<NSArray<NSSortDescriptor>, Shared>;
        # [method (setSortDescriptors :)]
        pub unsafe fn setSortDescriptors(&self, sortDescriptors: &NSArray<NSSortDescriptor>);
        #[method_id(valueListAttributes)]
        pub unsafe fn valueListAttributes(&self) -> Id<NSArray<NSString>, Shared>;
        # [method (setValueListAttributes :)]
        pub unsafe fn setValueListAttributes(&self, valueListAttributes: &NSArray<NSString>);
        #[method_id(groupingAttributes)]
        pub unsafe fn groupingAttributes(&self) -> Option<Id<NSArray<NSString>, Shared>>;
        # [method (setGroupingAttributes :)]
        pub unsafe fn setGroupingAttributes(&self, groupingAttributes: Option<&NSArray<NSString>>);
        #[method(notificationBatchingInterval)]
        pub unsafe fn notificationBatchingInterval(&self) -> NSTimeInterval;
        # [method (setNotificationBatchingInterval :)]
        pub unsafe fn setNotificationBatchingInterval(
            &self,
            notificationBatchingInterval: NSTimeInterval,
        );
        #[method_id(searchScopes)]
        pub unsafe fn searchScopes(&self) -> Id<NSArray, Shared>;
        # [method (setSearchScopes :)]
        pub unsafe fn setSearchScopes(&self, searchScopes: &NSArray);
        #[method_id(searchItems)]
        pub unsafe fn searchItems(&self) -> Option<Id<NSArray, Shared>>;
        # [method (setSearchItems :)]
        pub unsafe fn setSearchItems(&self, searchItems: Option<&NSArray>);
        #[method_id(operationQueue)]
        pub unsafe fn operationQueue(&self) -> Option<Id<NSOperationQueue, Shared>>;
        # [method (setOperationQueue :)]
        pub unsafe fn setOperationQueue(&self, operationQueue: Option<&NSOperationQueue>);
        #[method(startQuery)]
        pub unsafe fn startQuery(&self) -> bool;
        #[method(stopQuery)]
        pub unsafe fn stopQuery(&self);
        #[method(isStarted)]
        pub unsafe fn isStarted(&self) -> bool;
        #[method(isGathering)]
        pub unsafe fn isGathering(&self) -> bool;
        #[method(isStopped)]
        pub unsafe fn isStopped(&self) -> bool;
        #[method(disableUpdates)]
        pub unsafe fn disableUpdates(&self);
        #[method(enableUpdates)]
        pub unsafe fn enableUpdates(&self);
        #[method(resultCount)]
        pub unsafe fn resultCount(&self) -> NSUInteger;
        # [method_id (resultAtIndex :)]
        pub unsafe fn resultAtIndex(&self, idx: NSUInteger) -> Id<Object, Shared>;
        # [method (enumerateResultsUsingBlock :)]
        pub unsafe fn enumerateResultsUsingBlock(&self, block: TodoBlock);
        # [method (enumerateResultsWithOptions : usingBlock :)]
        pub unsafe fn enumerateResultsWithOptions_usingBlock(
            &self,
            opts: NSEnumerationOptions,
            block: TodoBlock,
        );
        #[method_id(results)]
        pub unsafe fn results(&self) -> Id<NSArray, Shared>;
        # [method (indexOfResult :)]
        pub unsafe fn indexOfResult(&self, result: &Object) -> NSUInteger;
        #[method_id(valueLists)]
        pub unsafe fn valueLists(
            &self,
        ) -> Id<NSDictionary<NSString, NSArray<NSMetadataQueryAttributeValueTuple>>, Shared>;
        #[method_id(groupedResults)]
        pub unsafe fn groupedResults(&self) -> Id<NSArray<NSMetadataQueryResultGroup>, Shared>;
        # [method_id (valueOfAttribute : forResultAtIndex :)]
        pub unsafe fn valueOfAttribute_forResultAtIndex(
            &self,
            attrName: &NSString,
            idx: NSUInteger,
        ) -> Option<Id<Object, Shared>>;
    }
);
pub type NSMetadataQueryDelegate = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSMetadataItem;
    unsafe impl ClassType for NSMetadataItem {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSMetadataItem {
        # [method_id (initWithURL :)]
        pub unsafe fn initWithURL(&self, url: &NSURL) -> Option<Id<Self, Shared>>;
        # [method_id (valueForAttribute :)]
        pub unsafe fn valueForAttribute(&self, key: &NSString) -> Option<Id<Object, Shared>>;
        # [method_id (valuesForAttributes :)]
        pub unsafe fn valuesForAttributes(
            &self,
            keys: &NSArray<NSString>,
        ) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;
        #[method_id(attributes)]
        pub unsafe fn attributes(&self) -> Id<NSArray<NSString>, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSMetadataQueryAttributeValueTuple;
    unsafe impl ClassType for NSMetadataQueryAttributeValueTuple {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSMetadataQueryAttributeValueTuple {
        #[method_id(attribute)]
        pub unsafe fn attribute(&self) -> Id<NSString, Shared>;
        #[method_id(value)]
        pub unsafe fn value(&self) -> Option<Id<Object, Shared>>;
        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSMetadataQueryResultGroup;
    unsafe impl ClassType for NSMetadataQueryResultGroup {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSMetadataQueryResultGroup {
        #[method_id(attribute)]
        pub unsafe fn attribute(&self) -> Id<NSString, Shared>;
        #[method_id(value)]
        pub unsafe fn value(&self) -> Id<Object, Shared>;
        #[method_id(subgroups)]
        pub unsafe fn subgroups(&self) -> Option<Id<NSArray<NSMetadataQueryResultGroup>, Shared>>;
        #[method(resultCount)]
        pub unsafe fn resultCount(&self) -> NSUInteger;
        # [method_id (resultAtIndex :)]
        pub unsafe fn resultAtIndex(&self, idx: NSUInteger) -> Id<Object, Shared>;
        #[method_id(results)]
        pub unsafe fn results(&self) -> Id<NSArray, Shared>;
    }
);
