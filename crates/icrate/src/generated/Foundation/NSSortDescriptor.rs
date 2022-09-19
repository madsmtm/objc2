use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSOrderedSet::*;
use crate::Foundation::generated::NSSet::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSSortDescriptor;
    unsafe impl ClassType for NSSortDescriptor {
        type Super = NSObject;
    }
);
impl NSSortDescriptor {
    pub unsafe fn sortDescriptorWithKey_ascending(
        key: Option<&NSString>,
        ascending: bool,
    ) -> Id<Self, Shared> {
        msg_send_id![
            Self::class(),
            sortDescriptorWithKey: key,
            ascending: ascending
        ]
    }
    pub unsafe fn sortDescriptorWithKey_ascending_selector(
        key: Option<&NSString>,
        ascending: bool,
        selector: Option<Sel>,
    ) -> Id<Self, Shared> {
        msg_send_id![
            Self::class(),
            sortDescriptorWithKey: key,
            ascending: ascending,
            selector: selector
        ]
    }
    pub unsafe fn initWithKey_ascending(
        &self,
        key: Option<&NSString>,
        ascending: bool,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithKey: key, ascending: ascending]
    }
    pub unsafe fn initWithKey_ascending_selector(
        &self,
        key: Option<&NSString>,
        ascending: bool,
        selector: Option<Sel>,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithKey: key,
            ascending: ascending,
            selector: selector
        ]
    }
    pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: coder]
    }
    pub unsafe fn allowEvaluation(&self) {
        msg_send![self, allowEvaluation]
    }
    pub unsafe fn sortDescriptorWithKey_ascending_comparator(
        key: Option<&NSString>,
        ascending: bool,
        cmptr: NSComparator,
    ) -> Id<Self, Shared> {
        msg_send_id![
            Self::class(),
            sortDescriptorWithKey: key,
            ascending: ascending,
            comparator: cmptr
        ]
    }
    pub unsafe fn initWithKey_ascending_comparator(
        &self,
        key: Option<&NSString>,
        ascending: bool,
        cmptr: NSComparator,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithKey: key,
            ascending: ascending,
            comparator: cmptr
        ]
    }
    pub unsafe fn compareObject_toObject(
        &self,
        object1: &Object,
        object2: &Object,
    ) -> NSComparisonResult {
        msg_send![self, compareObject: object1, toObject: object2]
    }
    pub unsafe fn key(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, key]
    }
    pub unsafe fn ascending(&self) -> bool {
        msg_send![self, ascending]
    }
    pub unsafe fn selector(&self) -> Option<Sel> {
        msg_send![self, selector]
    }
    pub unsafe fn comparator(&self) -> NSComparator {
        msg_send![self, comparator]
    }
    pub unsafe fn reversedSortDescriptor(&self) -> Id<Object, Shared> {
        msg_send_id![self, reversedSortDescriptor]
    }
}
#[doc = "NSSortDescriptorSorting"]
impl<ObjectType: Message> NSSet<ObjectType> {
    pub unsafe fn sortedArrayUsingDescriptors(
        &self,
        sortDescriptors: &NSArray<NSSortDescriptor>,
    ) -> Id<NSArray<ObjectType>, Shared> {
        msg_send_id![self, sortedArrayUsingDescriptors: sortDescriptors]
    }
}
#[doc = "NSSortDescriptorSorting"]
impl<ObjectType: Message> NSArray<ObjectType> {
    pub unsafe fn sortedArrayUsingDescriptors(
        &self,
        sortDescriptors: &NSArray<NSSortDescriptor>,
    ) -> Id<NSArray<ObjectType>, Shared> {
        msg_send_id![self, sortedArrayUsingDescriptors: sortDescriptors]
    }
}
#[doc = "NSSortDescriptorSorting"]
impl<ObjectType: Message> NSMutableArray<ObjectType> {
    pub unsafe fn sortUsingDescriptors(&self, sortDescriptors: &NSArray<NSSortDescriptor>) {
        msg_send![self, sortUsingDescriptors: sortDescriptors]
    }
}
#[doc = "NSKeyValueSorting"]
impl<ObjectType: Message> NSOrderedSet<ObjectType> {
    pub unsafe fn sortedArrayUsingDescriptors(
        &self,
        sortDescriptors: &NSArray<NSSortDescriptor>,
    ) -> Id<NSArray<ObjectType>, Shared> {
        msg_send_id![self, sortedArrayUsingDescriptors: sortDescriptors]
    }
}
#[doc = "NSKeyValueSorting"]
impl<ObjectType: Message> NSMutableOrderedSet<ObjectType> {
    pub unsafe fn sortUsingDescriptors(&self, sortDescriptors: &NSArray<NSSortDescriptor>) {
        msg_send![self, sortUsingDescriptors: sortDescriptors]
    }
}