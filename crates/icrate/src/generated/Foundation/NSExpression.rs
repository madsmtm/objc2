//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSExpressionType {
        NSConstantValueExpressionType = 0,
        NSEvaluatedObjectExpressionType = 1,
        NSVariableExpressionType = 2,
        NSKeyPathExpressionType = 3,
        NSFunctionExpressionType = 4,
        NSUnionSetExpressionType = 5,
        NSIntersectSetExpressionType = 6,
        NSMinusSetExpressionType = 7,
        NSSubqueryExpressionType = 13,
        NSAggregateExpressionType = 14,
        NSAnyKeyExpressionType = 15,
        NSBlockExpressionType = 19,
        NSConditionalExpressionType = 20,
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSExpression;

    unsafe impl ClassType for NSExpression {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSExpression {
        #[method_id(@__retain_semantics Other expressionWithFormat:argumentArray:)]
        pub unsafe fn expressionWithFormat_argumentArray(
            expressionFormat: &NSString,
            arguments: &NSArray,
        ) -> Id<NSExpression, Shared>;

        #[method_id(@__retain_semantics Other expressionForConstantValue:)]
        pub unsafe fn expressionForConstantValue(obj: Option<&Object>) -> Id<NSExpression, Shared>;

        #[method_id(@__retain_semantics Other expressionForEvaluatedObject)]
        pub unsafe fn expressionForEvaluatedObject() -> Id<NSExpression, Shared>;

        #[method_id(@__retain_semantics Other expressionForVariable:)]
        pub unsafe fn expressionForVariable(string: &NSString) -> Id<NSExpression, Shared>;

        #[method_id(@__retain_semantics Other expressionForKeyPath:)]
        pub unsafe fn expressionForKeyPath(keyPath: &NSString) -> Id<NSExpression, Shared>;

        #[method_id(@__retain_semantics Other expressionForFunction:arguments:)]
        pub unsafe fn expressionForFunction_arguments(
            name: &NSString,
            parameters: &NSArray,
        ) -> Id<NSExpression, Shared>;

        #[method_id(@__retain_semantics Other expressionForAggregate:)]
        pub unsafe fn expressionForAggregate(
            subexpressions: &NSArray<NSExpression>,
        ) -> Id<NSExpression, Shared>;

        #[method_id(@__retain_semantics Other expressionForUnionSet:with:)]
        pub unsafe fn expressionForUnionSet_with(
            left: &NSExpression,
            right: &NSExpression,
        ) -> Id<NSExpression, Shared>;

        #[method_id(@__retain_semantics Other expressionForIntersectSet:with:)]
        pub unsafe fn expressionForIntersectSet_with(
            left: &NSExpression,
            right: &NSExpression,
        ) -> Id<NSExpression, Shared>;

        #[method_id(@__retain_semantics Other expressionForMinusSet:with:)]
        pub unsafe fn expressionForMinusSet_with(
            left: &NSExpression,
            right: &NSExpression,
        ) -> Id<NSExpression, Shared>;

        #[method_id(@__retain_semantics Other expressionForSubquery:usingIteratorVariable:predicate:)]
        pub unsafe fn expressionForSubquery_usingIteratorVariable_predicate(
            expression: &NSExpression,
            variable: &NSString,
            predicate: &NSPredicate,
        ) -> Id<NSExpression, Shared>;

        #[method_id(@__retain_semantics Other expressionForFunction:selectorName:arguments:)]
        pub unsafe fn expressionForFunction_selectorName_arguments(
            target: &NSExpression,
            name: &NSString,
            parameters: Option<&NSArray>,
        ) -> Id<NSExpression, Shared>;

        #[method_id(@__retain_semantics Other expressionForAnyKey)]
        pub unsafe fn expressionForAnyKey() -> Id<NSExpression, Shared>;

        #[method_id(@__retain_semantics Other expressionForBlock:arguments:)]
        pub unsafe fn expressionForBlock_arguments(
            block: TodoBlock,
            arguments: Option<&NSArray<NSExpression>>,
        ) -> Id<NSExpression, Shared>;

        #[method_id(@__retain_semantics Other expressionForConditional:trueExpression:falseExpression:)]
        pub unsafe fn expressionForConditional_trueExpression_falseExpression(
            predicate: &NSPredicate,
            trueExpression: &NSExpression,
            falseExpression: &NSExpression,
        ) -> Id<NSExpression, Shared>;

        #[method_id(@__retain_semantics Init initWithExpressionType:)]
        pub unsafe fn initWithExpressionType(
            this: Option<Allocated<Self>>,
            type_: NSExpressionType,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method(expressionType)]
        pub unsafe fn expressionType(&self) -> NSExpressionType;

        #[method_id(@__retain_semantics Other constantValue)]
        pub unsafe fn constantValue(&self) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other keyPath)]
        pub unsafe fn keyPath(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other function)]
        pub unsafe fn function(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other variable)]
        pub unsafe fn variable(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other operand)]
        pub unsafe fn operand(&self) -> Id<NSExpression, Shared>;

        #[method_id(@__retain_semantics Other arguments)]
        pub unsafe fn arguments(&self) -> Option<Id<NSArray<NSExpression>, Shared>>;

        #[method_id(@__retain_semantics Other collection)]
        pub unsafe fn collection(&self) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other predicate)]
        pub unsafe fn predicate(&self) -> Id<NSPredicate, Shared>;

        #[method_id(@__retain_semantics Other leftExpression)]
        pub unsafe fn leftExpression(&self) -> Id<NSExpression, Shared>;

        #[method_id(@__retain_semantics Other rightExpression)]
        pub unsafe fn rightExpression(&self) -> Id<NSExpression, Shared>;

        #[method_id(@__retain_semantics Other trueExpression)]
        pub unsafe fn trueExpression(&self) -> Id<NSExpression, Shared>;

        #[method_id(@__retain_semantics Other falseExpression)]
        pub unsafe fn falseExpression(&self) -> Id<NSExpression, Shared>;

        #[method(expressionBlock)]
        pub unsafe fn expressionBlock(&self) -> TodoBlock;

        #[method_id(@__retain_semantics Other expressionValueWithObject:context:)]
        pub unsafe fn expressionValueWithObject_context(
            &self,
            object: Option<&Object>,
            context: Option<&NSMutableDictionary>,
        ) -> Option<Id<Object, Shared>>;

        #[method(allowEvaluation)]
        pub unsafe fn allowEvaluation(&self);
    }
);
