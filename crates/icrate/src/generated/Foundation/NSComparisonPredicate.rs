//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};

pub type NSComparisonPredicateOptions = NSUInteger;
pub const NSCaseInsensitivePredicateOption: NSComparisonPredicateOptions = 1;
pub const NSDiacriticInsensitivePredicateOption: NSComparisonPredicateOptions = 2;
pub const NSNormalizedPredicateOption: NSComparisonPredicateOptions = 4;

pub type NSComparisonPredicateModifier = NSUInteger;
pub const NSDirectPredicateModifier: NSComparisonPredicateModifier = 0;
pub const NSAllPredicateModifier: NSComparisonPredicateModifier = 1;
pub const NSAnyPredicateModifier: NSComparisonPredicateModifier = 2;

pub type NSPredicateOperatorType = NSUInteger;
pub const NSLessThanPredicateOperatorType: NSPredicateOperatorType = 0;
pub const NSLessThanOrEqualToPredicateOperatorType: NSPredicateOperatorType = 1;
pub const NSGreaterThanPredicateOperatorType: NSPredicateOperatorType = 2;
pub const NSGreaterThanOrEqualToPredicateOperatorType: NSPredicateOperatorType = 3;
pub const NSEqualToPredicateOperatorType: NSPredicateOperatorType = 4;
pub const NSNotEqualToPredicateOperatorType: NSPredicateOperatorType = 5;
pub const NSMatchesPredicateOperatorType: NSPredicateOperatorType = 6;
pub const NSLikePredicateOperatorType: NSPredicateOperatorType = 7;
pub const NSBeginsWithPredicateOperatorType: NSPredicateOperatorType = 8;
pub const NSEndsWithPredicateOperatorType: NSPredicateOperatorType = 9;
pub const NSInPredicateOperatorType: NSPredicateOperatorType = 10;
pub const NSCustomSelectorPredicateOperatorType: NSPredicateOperatorType = 11;
pub const NSContainsPredicateOperatorType: NSPredicateOperatorType = 99;
pub const NSBetweenPredicateOperatorType: NSPredicateOperatorType = 100;

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
