use super::NSArray;
use crate::Foundation::generated::NSPredicate::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSCompoundPredicate;
    unsafe impl ClassType for NSCompoundPredicate {
        type Super = NSPredicate;
    }
);
impl NSCompoundPredicate {
    pub unsafe fn initWithType_subpredicates(
        &self,
        type_: NSCompoundPredicateType,
        subpredicates: TodoGenerics,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithType: type_, subpredicates: subpredicates]
    }
    pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: coder]
    }
    pub unsafe fn andPredicateWithSubpredicates(
        subpredicates: TodoGenerics,
    ) -> Id<NSCompoundPredicate, Shared> {
        msg_send_id![Self::class(), andPredicateWithSubpredicates: subpredicates]
    }
    pub unsafe fn orPredicateWithSubpredicates(
        subpredicates: TodoGenerics,
    ) -> Id<NSCompoundPredicate, Shared> {
        msg_send_id![Self::class(), orPredicateWithSubpredicates: subpredicates]
    }
    pub unsafe fn notPredicateWithSubpredicate(
        predicate: &NSPredicate,
    ) -> Id<NSCompoundPredicate, Shared> {
        msg_send_id![Self::class(), notPredicateWithSubpredicate: predicate]
    }
    pub unsafe fn compoundPredicateType(&self) -> NSCompoundPredicateType {
        msg_send![self, compoundPredicateType]
    }
    pub unsafe fn subpredicates(&self) -> Id<NSArray, Shared> {
        msg_send_id![self, subpredicates]
    }
}
