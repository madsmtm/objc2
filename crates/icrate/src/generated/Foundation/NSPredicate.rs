use super::__exported::NSDictionary;
use super::__exported::NSString;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSOrderedSet::*;
use crate::Foundation::generated::NSSet::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPredicate;
    unsafe impl ClassType for NSPredicate {
        type Super = NSObject;
    }
);
impl NSPredicate {
    pub unsafe fn predicateWithFormat_argumentArray(
        predicateFormat: &NSString,
        arguments: Option<&NSArray>,
    ) -> Id<NSPredicate, Shared> {
        msg_send_id![
            Self::class(),
            predicateWithFormat: predicateFormat,
            argumentArray: arguments
        ]
    }
    pub unsafe fn predicateWithFormat_arguments(
        predicateFormat: &NSString,
        argList: va_list,
    ) -> Id<NSPredicate, Shared> {
        msg_send_id![
            Self::class(),
            predicateWithFormat: predicateFormat,
            arguments: argList
        ]
    }
    pub unsafe fn predicateFromMetadataQueryString(
        queryString: &NSString,
    ) -> Option<Id<NSPredicate, Shared>> {
        msg_send_id![Self::class(), predicateFromMetadataQueryString: queryString]
    }
    pub unsafe fn predicateWithValue(value: bool) -> Id<NSPredicate, Shared> {
        msg_send_id![Self::class(), predicateWithValue: value]
    }
    pub unsafe fn predicateWithBlock(block: TodoBlock) -> Id<NSPredicate, Shared> {
        msg_send_id![Self::class(), predicateWithBlock: block]
    }
    pub unsafe fn predicateFormat(&self) -> Id<NSString, Shared> {
        msg_send_id![self, predicateFormat]
    }
    pub unsafe fn predicateWithSubstitutionVariables(
        &self,
        variables: &NSDictionary<NSString, Object>,
    ) -> Id<Self, Shared> {
        msg_send_id![self, predicateWithSubstitutionVariables: variables]
    }
    pub unsafe fn evaluateWithObject(&self, object: Option<&Object>) -> bool {
        msg_send![self, evaluateWithObject: object]
    }
    pub unsafe fn evaluateWithObject_substitutionVariables(
        &self,
        object: Option<&Object>,
        bindings: Option<&NSDictionary<NSString, Object>>,
    ) -> bool {
        msg_send![
            self,
            evaluateWithObject: object,
            substitutionVariables: bindings
        ]
    }
    pub unsafe fn allowEvaluation(&self) {
        msg_send![self, allowEvaluation]
    }
}
#[doc = "NSPredicateSupport"]
impl<ObjectType: Message> NSArray<ObjectType> {
    pub unsafe fn filteredArrayUsingPredicate(
        &self,
        predicate: &NSPredicate,
    ) -> Id<NSArray<ObjectType>, Shared> {
        msg_send_id![self, filteredArrayUsingPredicate: predicate]
    }
}
#[doc = "NSPredicateSupport"]
impl<ObjectType: Message> NSMutableArray<ObjectType> {
    pub unsafe fn filterUsingPredicate(&self, predicate: &NSPredicate) {
        msg_send![self, filterUsingPredicate: predicate]
    }
}
#[doc = "NSPredicateSupport"]
impl<ObjectType: Message> NSSet<ObjectType> {
    pub unsafe fn filteredSetUsingPredicate(
        &self,
        predicate: &NSPredicate,
    ) -> Id<NSSet<ObjectType>, Shared> {
        msg_send_id![self, filteredSetUsingPredicate: predicate]
    }
}
#[doc = "NSPredicateSupport"]
impl<ObjectType: Message> NSMutableSet<ObjectType> {
    pub unsafe fn filterUsingPredicate(&self, predicate: &NSPredicate) {
        msg_send![self, filterUsingPredicate: predicate]
    }
}
#[doc = "NSPredicateSupport"]
impl<ObjectType: Message> NSOrderedSet<ObjectType> {
    pub unsafe fn filteredOrderedSetUsingPredicate(
        &self,
        p: &NSPredicate,
    ) -> Id<NSOrderedSet<ObjectType>, Shared> {
        msg_send_id![self, filteredOrderedSetUsingPredicate: p]
    }
}
#[doc = "NSPredicateSupport"]
impl<ObjectType: Message> NSMutableOrderedSet<ObjectType> {
    pub unsafe fn filterUsingPredicate(&self, p: &NSPredicate) {
        msg_send![self, filterUsingPredicate: p]
    }
}
