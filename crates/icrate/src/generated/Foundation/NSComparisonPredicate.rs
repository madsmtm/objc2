extern_class!(
    #[derive(Debug)]
    struct NSComparisonPredicate;
    unsafe impl ClassType for NSComparisonPredicate {
        type Super = NSPredicate;
    }
);
impl NSComparisonPredicate {
    pub unsafe fn predicateWithLeftExpression_rightExpression_modifier_type_options(
        lhs: &NSExpression,
        rhs: &NSExpression,
        modifier: NSComparisonPredicateModifier,
        type_: NSPredicateOperatorType,
        options: NSComparisonPredicateOptions,
    ) -> Id<NSComparisonPredicate, Shared> {
        msg_send_id ! [Self :: class () , predicateWithLeftExpression : lhs , rightExpression : rhs , modifier : modifier , type : type_ , options : options]
    }
    pub unsafe fn predicateWithLeftExpression_rightExpression_customSelector(
        lhs: &NSExpression,
        rhs: &NSExpression,
        selector: Sel,
    ) -> Id<NSComparisonPredicate, Shared> {
        msg_send_id![
            Self::class(),
            predicateWithLeftExpression: lhs,
            rightExpression: rhs,
            customSelector: selector
        ]
    }
    pub unsafe fn initWithLeftExpression_rightExpression_modifier_type_options(
        &self,
        lhs: &NSExpression,
        rhs: &NSExpression,
        modifier: NSComparisonPredicateModifier,
        type_: NSPredicateOperatorType,
        options: NSComparisonPredicateOptions,
    ) -> Id<Self, Shared> {
        msg_send_id ! [self , initWithLeftExpression : lhs , rightExpression : rhs , modifier : modifier , type : type_ , options : options]
    }
    pub unsafe fn initWithLeftExpression_rightExpression_customSelector(
        &self,
        lhs: &NSExpression,
        rhs: &NSExpression,
        selector: Sel,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithLeftExpression: lhs,
            rightExpression: rhs,
            customSelector: selector
        ]
    }
    pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: coder]
    }
    pub unsafe fn predicateOperatorType(&self) -> NSPredicateOperatorType {
        msg_send![self, predicateOperatorType]
    }
    pub unsafe fn comparisonPredicateModifier(&self) -> NSComparisonPredicateModifier {
        msg_send![self, comparisonPredicateModifier]
    }
    pub unsafe fn leftExpression(&self) -> Id<NSExpression, Shared> {
        msg_send_id![self, leftExpression]
    }
    pub unsafe fn rightExpression(&self) -> Id<NSExpression, Shared> {
        msg_send_id![self, rightExpression]
    }
    pub unsafe fn customSelector(&self) -> Option<Sel> {
        msg_send![self, customSelector]
    }
    pub unsafe fn options(&self) -> NSComparisonPredicateOptions {
        msg_send![self, options]
    }
}
