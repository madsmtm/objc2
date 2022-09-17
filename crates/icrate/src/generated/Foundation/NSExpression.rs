use super::NSArray;
use super::NSMutableDictionary;
use super::NSPredicate;
use super::NSString;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSExpression;
    unsafe impl ClassType for NSExpression {
        type Super = NSObject;
    }
);
impl NSExpression {
    pub unsafe fn expressionWithFormat_argumentArray(
        expressionFormat: &NSString,
        arguments: &NSArray,
    ) -> Id<NSExpression, Shared> {
        msg_send_id![
            Self::class(),
            expressionWithFormat: expressionFormat,
            argumentArray: arguments
        ]
    }
    pub unsafe fn expressionWithFormat_arguments(
        expressionFormat: &NSString,
        argList: va_list,
    ) -> Id<NSExpression, Shared> {
        msg_send_id![
            Self::class(),
            expressionWithFormat: expressionFormat,
            arguments: argList
        ]
    }
    pub unsafe fn expressionForConstantValue(obj: Option<&Object>) -> Id<NSExpression, Shared> {
        msg_send_id![Self::class(), expressionForConstantValue: obj]
    }
    pub unsafe fn expressionForEvaluatedObject() -> Id<NSExpression, Shared> {
        msg_send_id![Self::class(), expressionForEvaluatedObject]
    }
    pub unsafe fn expressionForVariable(string: &NSString) -> Id<NSExpression, Shared> {
        msg_send_id![Self::class(), expressionForVariable: string]
    }
    pub unsafe fn expressionForKeyPath(keyPath: &NSString) -> Id<NSExpression, Shared> {
        msg_send_id![Self::class(), expressionForKeyPath: keyPath]
    }
    pub unsafe fn expressionForFunction_arguments(
        name: &NSString,
        parameters: &NSArray,
    ) -> Id<NSExpression, Shared> {
        msg_send_id![
            Self::class(),
            expressionForFunction: name,
            arguments: parameters
        ]
    }
    pub unsafe fn expressionForAggregate(subexpressions: TodoGenerics) -> Id<NSExpression, Shared> {
        msg_send_id![Self::class(), expressionForAggregate: subexpressions]
    }
    pub unsafe fn expressionForUnionSet_with(
        left: &NSExpression,
        right: &NSExpression,
    ) -> Id<NSExpression, Shared> {
        msg_send_id![Self::class(), expressionForUnionSet: left, with: right]
    }
    pub unsafe fn expressionForIntersectSet_with(
        left: &NSExpression,
        right: &NSExpression,
    ) -> Id<NSExpression, Shared> {
        msg_send_id![Self::class(), expressionForIntersectSet: left, with: right]
    }
    pub unsafe fn expressionForMinusSet_with(
        left: &NSExpression,
        right: &NSExpression,
    ) -> Id<NSExpression, Shared> {
        msg_send_id![Self::class(), expressionForMinusSet: left, with: right]
    }
    pub unsafe fn expressionForSubquery_usingIteratorVariable_predicate(
        expression: &NSExpression,
        variable: &NSString,
        predicate: &NSPredicate,
    ) -> Id<NSExpression, Shared> {
        msg_send_id![
            Self::class(),
            expressionForSubquery: expression,
            usingIteratorVariable: variable,
            predicate: predicate
        ]
    }
    pub unsafe fn expressionForFunction_selectorName_arguments(
        target: &NSExpression,
        name: &NSString,
        parameters: Option<&NSArray>,
    ) -> Id<NSExpression, Shared> {
        msg_send_id![
            Self::class(),
            expressionForFunction: target,
            selectorName: name,
            arguments: parameters
        ]
    }
    pub unsafe fn expressionForAnyKey() -> Id<NSExpression, Shared> {
        msg_send_id![Self::class(), expressionForAnyKey]
    }
    pub unsafe fn expressionForBlock_arguments(
        block: TodoBlock,
        arguments: TodoGenerics,
    ) -> Id<NSExpression, Shared> {
        msg_send_id![
            Self::class(),
            expressionForBlock: block,
            arguments: arguments
        ]
    }
    pub unsafe fn expressionForConditional_trueExpression_falseExpression(
        predicate: &NSPredicate,
        trueExpression: &NSExpression,
        falseExpression: &NSExpression,
    ) -> Id<NSExpression, Shared> {
        msg_send_id![
            Self::class(),
            expressionForConditional: predicate,
            trueExpression: trueExpression,
            falseExpression: falseExpression
        ]
    }
    pub unsafe fn initWithExpressionType(&self, type_: NSExpressionType) -> Id<Self, Shared> {
        msg_send_id![self, initWithExpressionType: type_]
    }
    pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: coder]
    }
    pub unsafe fn expressionValueWithObject_context(
        &self,
        object: Option<&Object>,
        context: Option<&NSMutableDictionary>,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![self, expressionValueWithObject: object, context: context]
    }
    pub unsafe fn allowEvaluation(&self) {
        msg_send![self, allowEvaluation]
    }
    pub unsafe fn expressionType(&self) -> NSExpressionType {
        msg_send![self, expressionType]
    }
    pub unsafe fn constantValue(&self) -> Option<Id<Object, Shared>> {
        msg_send_id![self, constantValue]
    }
    pub unsafe fn keyPath(&self) -> Id<NSString, Shared> {
        msg_send_id![self, keyPath]
    }
    pub unsafe fn function(&self) -> Id<NSString, Shared> {
        msg_send_id![self, function]
    }
    pub unsafe fn variable(&self) -> Id<NSString, Shared> {
        msg_send_id![self, variable]
    }
    pub unsafe fn operand(&self) -> Id<NSExpression, Shared> {
        msg_send_id![self, operand]
    }
    pub unsafe fn arguments(&self) -> TodoGenerics {
        msg_send![self, arguments]
    }
    pub unsafe fn collection(&self) -> Id<Object, Shared> {
        msg_send_id![self, collection]
    }
    pub unsafe fn predicate(&self) -> Id<NSPredicate, Shared> {
        msg_send_id![self, predicate]
    }
    pub unsafe fn leftExpression(&self) -> Id<NSExpression, Shared> {
        msg_send_id![self, leftExpression]
    }
    pub unsafe fn rightExpression(&self) -> Id<NSExpression, Shared> {
        msg_send_id![self, rightExpression]
    }
    pub unsafe fn trueExpression(&self) -> Id<NSExpression, Shared> {
        msg_send_id![self, trueExpression]
    }
    pub unsafe fn falseExpression(&self) -> Id<NSExpression, Shared> {
        msg_send_id![self, falseExpression]
    }
    pub unsafe fn expressionBlock(&self) -> TodoBlock {
        msg_send![self, expressionBlock]
    }
}
