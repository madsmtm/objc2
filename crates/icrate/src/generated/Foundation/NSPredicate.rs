#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPredicate;
    unsafe impl ClassType for NSPredicate {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSPredicate {
        #[method_id(predicateWithFormat:argumentArray:)]
        pub unsafe fn predicateWithFormat_argumentArray(
            predicateFormat: &NSString,
            arguments: Option<&NSArray>,
        ) -> Id<NSPredicate, Shared>;
        #[method_id(predicateWithFormat:arguments:)]
        pub unsafe fn predicateWithFormat_arguments(
            predicateFormat: &NSString,
            argList: va_list,
        ) -> Id<NSPredicate, Shared>;
        #[method_id(predicateFromMetadataQueryString:)]
        pub unsafe fn predicateFromMetadataQueryString(
            queryString: &NSString,
        ) -> Option<Id<NSPredicate, Shared>>;
        #[method_id(predicateWithValue:)]
        pub unsafe fn predicateWithValue(value: bool) -> Id<NSPredicate, Shared>;
        #[method_id(predicateWithBlock:)]
        pub unsafe fn predicateWithBlock(block: TodoBlock) -> Id<NSPredicate, Shared>;
        #[method_id(predicateFormat)]
        pub unsafe fn predicateFormat(&self) -> Id<NSString, Shared>;
        #[method_id(predicateWithSubstitutionVariables:)]
        pub unsafe fn predicateWithSubstitutionVariables(
            &self,
            variables: &NSDictionary<NSString, Object>,
        ) -> Id<Self, Shared>;
        #[method(evaluateWithObject:)]
        pub unsafe fn evaluateWithObject(&self, object: Option<&Object>) -> bool;
        #[method(evaluateWithObject:substitutionVariables:)]
        pub unsafe fn evaluateWithObject_substitutionVariables(
            &self,
            object: Option<&Object>,
            bindings: Option<&NSDictionary<NSString, Object>>,
        ) -> bool;
        #[method(allowEvaluation)]
        pub unsafe fn allowEvaluation(&self);
    }
);
extern_methods!(
    #[doc = "NSPredicateSupport"]
    unsafe impl<ObjectType: Message> NSArray<ObjectType> {
        #[method_id(filteredArrayUsingPredicate:)]
        pub unsafe fn filteredArrayUsingPredicate(
            &self,
            predicate: &NSPredicate,
        ) -> Id<NSArray<ObjectType>, Shared>;
    }
);
extern_methods!(
    #[doc = "NSPredicateSupport"]
    unsafe impl<ObjectType: Message> NSMutableArray<ObjectType> {
        #[method(filterUsingPredicate:)]
        pub unsafe fn filterUsingPredicate(&self, predicate: &NSPredicate);
    }
);
extern_methods!(
    #[doc = "NSPredicateSupport"]
    unsafe impl<ObjectType: Message> NSSet<ObjectType> {
        #[method_id(filteredSetUsingPredicate:)]
        pub unsafe fn filteredSetUsingPredicate(
            &self,
            predicate: &NSPredicate,
        ) -> Id<NSSet<ObjectType>, Shared>;
    }
);
extern_methods!(
    #[doc = "NSPredicateSupport"]
    unsafe impl<ObjectType: Message> NSMutableSet<ObjectType> {
        #[method(filterUsingPredicate:)]
        pub unsafe fn filterUsingPredicate(&self, predicate: &NSPredicate);
    }
);
extern_methods!(
    #[doc = "NSPredicateSupport"]
    unsafe impl<ObjectType: Message> NSOrderedSet<ObjectType> {
        #[method_id(filteredOrderedSetUsingPredicate:)]
        pub unsafe fn filteredOrderedSetUsingPredicate(
            &self,
            p: &NSPredicate,
        ) -> Id<NSOrderedSet<ObjectType>, Shared>;
    }
);
extern_methods!(
    #[doc = "NSPredicateSupport"]
    unsafe impl<ObjectType: Message> NSMutableOrderedSet<ObjectType> {
        #[method(filterUsingPredicate:)]
        pub unsafe fn filterUsingPredicate(&self, p: &NSPredicate);
    }
);
