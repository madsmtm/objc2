use super::__exported::NSExpression;
use super::__exported::NSPredicateOperator;
use crate::Foundation::generated::NSPredicate::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSComparisonPredicate;
    unsafe impl ClassType for NSComparisonPredicate {
        type Super = NSPredicate;
    }
);
extern_methods!(
    unsafe impl NSComparisonPredicate {
        #[method_id(predicateWithLeftExpression:rightExpression:modifier:type:options:)]
        pub unsafe fn predicateWithLeftExpression_rightExpression_modifier_type_options(
            lhs: &NSExpression,
            rhs: &NSExpression,
            modifier: NSComparisonPredicateModifier,
            type_: NSPredicateOperatorType,
            options: NSComparisonPredicateOptions,
        ) -> Id<NSComparisonPredicate, Shared>;
        #[method_id(predicateWithLeftExpression:rightExpression:customSelector:)]
        pub unsafe fn predicateWithLeftExpression_rightExpression_customSelector(
            lhs: &NSExpression,
            rhs: &NSExpression,
            selector: Sel,
        ) -> Id<NSComparisonPredicate, Shared>;
        #[method_id(initWithLeftExpression:rightExpression:modifier:type:options:)]
        pub unsafe fn initWithLeftExpression_rightExpression_modifier_type_options(
            &self,
            lhs: &NSExpression,
            rhs: &NSExpression,
            modifier: NSComparisonPredicateModifier,
            type_: NSPredicateOperatorType,
            options: NSComparisonPredicateOptions,
        ) -> Id<Self, Shared>;
        #[method_id(initWithLeftExpression:rightExpression:customSelector:)]
        pub unsafe fn initWithLeftExpression_rightExpression_customSelector(
            &self,
            lhs: &NSExpression,
            rhs: &NSExpression,
            selector: Sel,
        ) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method(predicateOperatorType)]
        pub unsafe fn predicateOperatorType(&self) -> NSPredicateOperatorType;
        #[method(comparisonPredicateModifier)]
        pub unsafe fn comparisonPredicateModifier(&self) -> NSComparisonPredicateModifier;
        #[method_id(leftExpression)]
        pub unsafe fn leftExpression(&self) -> Id<NSExpression, Shared>;
        #[method_id(rightExpression)]
        pub unsafe fn rightExpression(&self) -> Id<NSExpression, Shared>;
        #[method(customSelector)]
        pub unsafe fn customSelector(&self) -> Option<Sel>;
        #[method(options)]
        pub unsafe fn options(&self) -> NSComparisonPredicateOptions;
    }
);
