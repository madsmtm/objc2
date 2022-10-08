use super::__exported::NSArray;
use super::__exported::NSMutableDictionary;
use super::__exported::NSPredicate;
use super::__exported::NSString;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSExpression;
    unsafe impl ClassType for NSExpression {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSExpression {
        # [method_id (expressionWithFormat : argumentArray :)]
        pub unsafe fn expressionWithFormat_argumentArray(
            expressionFormat: &NSString,
            arguments: &NSArray,
        ) -> Id<NSExpression, Shared>;
        # [method_id (expressionWithFormat : arguments :)]
        pub unsafe fn expressionWithFormat_arguments(
            expressionFormat: &NSString,
            argList: va_list,
        ) -> Id<NSExpression, Shared>;
        # [method_id (expressionForConstantValue :)]
        pub unsafe fn expressionForConstantValue(obj: Option<&Object>) -> Id<NSExpression, Shared>;
        #[method_id(expressionForEvaluatedObject)]
        pub unsafe fn expressionForEvaluatedObject() -> Id<NSExpression, Shared>;
        # [method_id (expressionForVariable :)]
        pub unsafe fn expressionForVariable(string: &NSString) -> Id<NSExpression, Shared>;
        # [method_id (expressionForKeyPath :)]
        pub unsafe fn expressionForKeyPath(keyPath: &NSString) -> Id<NSExpression, Shared>;
        # [method_id (expressionForFunction : arguments :)]
        pub unsafe fn expressionForFunction_arguments(
            name: &NSString,
            parameters: &NSArray,
        ) -> Id<NSExpression, Shared>;
        # [method_id (expressionForAggregate :)]
        pub unsafe fn expressionForAggregate(
            subexpressions: &NSArray<NSExpression>,
        ) -> Id<NSExpression, Shared>;
        # [method_id (expressionForUnionSet : with :)]
        pub unsafe fn expressionForUnionSet_with(
            left: &NSExpression,
            right: &NSExpression,
        ) -> Id<NSExpression, Shared>;
        # [method_id (expressionForIntersectSet : with :)]
        pub unsafe fn expressionForIntersectSet_with(
            left: &NSExpression,
            right: &NSExpression,
        ) -> Id<NSExpression, Shared>;
        # [method_id (expressionForMinusSet : with :)]
        pub unsafe fn expressionForMinusSet_with(
            left: &NSExpression,
            right: &NSExpression,
        ) -> Id<NSExpression, Shared>;
        # [method_id (expressionForSubquery : usingIteratorVariable : predicate :)]
        pub unsafe fn expressionForSubquery_usingIteratorVariable_predicate(
            expression: &NSExpression,
            variable: &NSString,
            predicate: &NSPredicate,
        ) -> Id<NSExpression, Shared>;
        # [method_id (expressionForFunction : selectorName : arguments :)]
        pub unsafe fn expressionForFunction_selectorName_arguments(
            target: &NSExpression,
            name: &NSString,
            parameters: Option<&NSArray>,
        ) -> Id<NSExpression, Shared>;
        #[method_id(expressionForAnyKey)]
        pub unsafe fn expressionForAnyKey() -> Id<NSExpression, Shared>;
        # [method_id (expressionForBlock : arguments :)]
        pub unsafe fn expressionForBlock_arguments(
            block: TodoBlock,
            arguments: Option<&NSArray<NSExpression>>,
        ) -> Id<NSExpression, Shared>;
        # [method_id (expressionForConditional : trueExpression : falseExpression :)]
        pub unsafe fn expressionForConditional_trueExpression_falseExpression(
            predicate: &NSPredicate,
            trueExpression: &NSExpression,
            falseExpression: &NSExpression,
        ) -> Id<NSExpression, Shared>;
        # [method_id (initWithExpressionType :)]
        pub unsafe fn initWithExpressionType(&self, type_: NSExpressionType) -> Id<Self, Shared>;
        # [method_id (initWithCoder :)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method(expressionType)]
        pub unsafe fn expressionType(&self) -> NSExpressionType;
        #[method_id(constantValue)]
        pub unsafe fn constantValue(&self) -> Option<Id<Object, Shared>>;
        #[method_id(keyPath)]
        pub unsafe fn keyPath(&self) -> Id<NSString, Shared>;
        #[method_id(function)]
        pub unsafe fn function(&self) -> Id<NSString, Shared>;
        #[method_id(variable)]
        pub unsafe fn variable(&self) -> Id<NSString, Shared>;
        #[method_id(operand)]
        pub unsafe fn operand(&self) -> Id<NSExpression, Shared>;
        #[method_id(arguments)]
        pub unsafe fn arguments(&self) -> Option<Id<NSArray<NSExpression>, Shared>>;
        #[method_id(collection)]
        pub unsafe fn collection(&self) -> Id<Object, Shared>;
        #[method_id(predicate)]
        pub unsafe fn predicate(&self) -> Id<NSPredicate, Shared>;
        #[method_id(leftExpression)]
        pub unsafe fn leftExpression(&self) -> Id<NSExpression, Shared>;
        #[method_id(rightExpression)]
        pub unsafe fn rightExpression(&self) -> Id<NSExpression, Shared>;
        #[method_id(trueExpression)]
        pub unsafe fn trueExpression(&self) -> Id<NSExpression, Shared>;
        #[method_id(falseExpression)]
        pub unsafe fn falseExpression(&self) -> Id<NSExpression, Shared>;
        #[method(expressionBlock)]
        pub unsafe fn expressionBlock(&self) -> TodoBlock;
        # [method_id (expressionValueWithObject : context :)]
        pub unsafe fn expressionValueWithObject_context(
            &self,
            object: Option<&Object>,
            context: Option<&NSMutableDictionary>,
        ) -> Option<Id<Object, Shared>>;
        #[method(allowEvaluation)]
        pub unsafe fn allowEvaluation(&self);
    }
);
