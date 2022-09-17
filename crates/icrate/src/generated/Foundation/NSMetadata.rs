#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    struct NSMetadataQuery;
    unsafe impl ClassType for NSMetadataQuery {
        type Super = NSObject;
    }
);
impl NSMetadataQuery {
    pub unsafe fn startQuery(&self) -> bool {
        msg_send![self, startQuery]
    }
    pub unsafe fn stopQuery(&self) {
        msg_send![self, stopQuery]
    }
    pub unsafe fn disableUpdates(&self) {
        msg_send![self, disableUpdates]
    }
    pub unsafe fn enableUpdates(&self) {
        msg_send![self, enableUpdates]
    }
    pub unsafe fn resultAtIndex(&self, idx: NSUInteger) -> Id<Object, Shared> {
        msg_send_id![self, resultAtIndex: idx]
    }
    pub unsafe fn enumerateResultsUsingBlock(&self, block: TodoBlock) {
        msg_send![self, enumerateResultsUsingBlock: block]
    }
    pub unsafe fn enumerateResultsWithOptions_usingBlock(
        &self,
        opts: NSEnumerationOptions,
        block: TodoBlock,
    ) {
        msg_send![self, enumerateResultsWithOptions: opts, usingBlock: block]
    }
    pub unsafe fn indexOfResult(&self, result: &Object) -> NSUInteger {
        msg_send![self, indexOfResult: result]
    }
    pub unsafe fn valueOfAttribute_forResultAtIndex(
        &self,
        attrName: &NSString,
        idx: NSUInteger,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![self, valueOfAttribute: attrName, forResultAtIndex: idx]
    }
    pub unsafe fn delegate(&self) -> TodoGenerics {
        msg_send![self, delegate]
    }
    pub unsafe fn setDelegate(&self, delegate: TodoGenerics) {
        msg_send![self, setDelegate: delegate]
    }
    pub unsafe fn predicate(&self) -> Option<Id<NSPredicate, Shared>> {
        msg_send_id![self, predicate]
    }
    pub unsafe fn setPredicate(&self, predicate: Option<&NSPredicate>) {
        msg_send![self, setPredicate: predicate]
    }
    pub unsafe fn sortDescriptors(&self) -> TodoGenerics {
        msg_send![self, sortDescriptors]
    }
    pub unsafe fn setSortDescriptors(&self, sortDescriptors: TodoGenerics) {
        msg_send![self, setSortDescriptors: sortDescriptors]
    }
    pub unsafe fn valueListAttributes(&self) -> TodoGenerics {
        msg_send![self, valueListAttributes]
    }
    pub unsafe fn setValueListAttributes(&self, valueListAttributes: TodoGenerics) {
        msg_send![self, setValueListAttributes: valueListAttributes]
    }
    pub unsafe fn groupingAttributes(&self) -> TodoGenerics {
        msg_send![self, groupingAttributes]
    }
    pub unsafe fn setGroupingAttributes(&self, groupingAttributes: TodoGenerics) {
        msg_send![self, setGroupingAttributes: groupingAttributes]
    }
    pub unsafe fn notificationBatchingInterval(&self) -> NSTimeInterval {
        msg_send![self, notificationBatchingInterval]
    }
    pub unsafe fn setNotificationBatchingInterval(
        &self,
        notificationBatchingInterval: NSTimeInterval,
    ) {
        msg_send![
            self,
            setNotificationBatchingInterval: notificationBatchingInterval
        ]
    }
    pub unsafe fn searchScopes(&self) -> Id<NSArray, Shared> {
        msg_send_id![self, searchScopes]
    }
    pub unsafe fn setSearchScopes(&self, searchScopes: &NSArray) {
        msg_send![self, setSearchScopes: searchScopes]
    }
    pub unsafe fn searchItems(&self) -> Option<Id<NSArray, Shared>> {
        msg_send_id![self, searchItems]
    }
    pub unsafe fn setSearchItems(&self, searchItems: Option<&NSArray>) {
        msg_send![self, setSearchItems: searchItems]
    }
    pub unsafe fn operationQueue(&self) -> Option<Id<NSOperationQueue, Shared>> {
        msg_send_id![self, operationQueue]
    }
    pub unsafe fn setOperationQueue(&self, operationQueue: Option<&NSOperationQueue>) {
        msg_send![self, setOperationQueue: operationQueue]
    }
    pub unsafe fn isStarted(&self) -> bool {
        msg_send![self, isStarted]
    }
    pub unsafe fn isGathering(&self) -> bool {
        msg_send![self, isGathering]
    }
    pub unsafe fn isStopped(&self) -> bool {
        msg_send![self, isStopped]
    }
    pub unsafe fn resultCount(&self) -> NSUInteger {
        msg_send![self, resultCount]
    }
    pub unsafe fn results(&self) -> Id<NSArray, Shared> {
        msg_send_id![self, results]
    }
    pub unsafe fn valueLists(&self) -> TodoGenerics {
        msg_send![self, valueLists]
    }
    pub unsafe fn groupedResults(&self) -> TodoGenerics {
        msg_send![self, groupedResults]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSMetadataItem;
    unsafe impl ClassType for NSMetadataItem {
        type Super = NSObject;
    }
);
impl NSMetadataItem {
    pub unsafe fn initWithURL(&self, url: &NSURL) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithURL: url]
    }
    pub unsafe fn valueForAttribute(&self, key: &NSString) -> Option<Id<Object, Shared>> {
        msg_send_id![self, valueForAttribute: key]
    }
    pub unsafe fn valuesForAttributes(&self, keys: TodoGenerics) -> TodoGenerics {
        msg_send![self, valuesForAttributes: keys]
    }
    pub unsafe fn attributes(&self) -> TodoGenerics {
        msg_send![self, attributes]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSMetadataQueryAttributeValueTuple;
    unsafe impl ClassType for NSMetadataQueryAttributeValueTuple {
        type Super = NSObject;
    }
);
impl NSMetadataQueryAttributeValueTuple {
    pub unsafe fn attribute(&self) -> Id<NSString, Shared> {
        msg_send_id![self, attribute]
    }
    pub unsafe fn value(&self) -> Option<Id<Object, Shared>> {
        msg_send_id![self, value]
    }
    pub unsafe fn count(&self) -> NSUInteger {
        msg_send![self, count]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSMetadataQueryResultGroup;
    unsafe impl ClassType for NSMetadataQueryResultGroup {
        type Super = NSObject;
    }
);
impl NSMetadataQueryResultGroup {
    pub unsafe fn resultAtIndex(&self, idx: NSUInteger) -> Id<Object, Shared> {
        msg_send_id![self, resultAtIndex: idx]
    }
    pub unsafe fn attribute(&self) -> Id<NSString, Shared> {
        msg_send_id![self, attribute]
    }
    pub unsafe fn value(&self) -> Id<Object, Shared> {
        msg_send_id![self, value]
    }
    pub unsafe fn subgroups(&self) -> TodoGenerics {
        msg_send![self, subgroups]
    }
    pub unsafe fn resultCount(&self) -> NSUInteger {
        msg_send![self, resultCount]
    }
    pub unsafe fn results(&self) -> Id<NSArray, Shared> {
        msg_send_id![self, results]
    }
}
