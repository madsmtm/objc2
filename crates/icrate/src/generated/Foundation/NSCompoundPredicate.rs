use super::__exported::NSArray;
use crate::Foundation::generated::NSPredicate::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSCompoundPredicate;
    unsafe impl ClassType for NSCompoundPredicate {
        type Super = NSPredicate;
    }
);
extern_methods!(
    unsafe impl NSCompoundPredicate {
        # [method_id (initWithType : subpredicates :)]
        pub unsafe fn initWithType_subpredicates(
            &self,
            type_: NSCompoundPredicateType,
            subpredicates: &NSArray<NSPredicate>,
        ) -> Id<Self, Shared>;
        # [method_id (initWithCoder :)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method(compoundPredicateType)]
        pub unsafe fn compoundPredicateType(&self) -> NSCompoundPredicateType;
        #[method_id(subpredicates)]
        pub unsafe fn subpredicates(&self) -> Id<NSArray, Shared>;
        # [method_id (andPredicateWithSubpredicates :)]
        pub unsafe fn andPredicateWithSubpredicates(
            subpredicates: &NSArray<NSPredicate>,
        ) -> Id<NSCompoundPredicate, Shared>;
        # [method_id (orPredicateWithSubpredicates :)]
        pub unsafe fn orPredicateWithSubpredicates(
            subpredicates: &NSArray<NSPredicate>,
        ) -> Id<NSCompoundPredicate, Shared>;
        # [method_id (notPredicateWithSubpredicate :)]
        pub unsafe fn notPredicateWithSubpredicate(
            predicate: &NSPredicate,
        ) -> Id<NSCompoundPredicate, Shared>;
    }
);
