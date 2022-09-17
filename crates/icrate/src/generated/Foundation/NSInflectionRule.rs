extern_class!(
    #[derive(Debug)]
    struct NSInflectionRule;
    unsafe impl ClassType for NSInflectionRule {
        type Super = NSObject;
    }
);
impl NSInflectionRule {
    pub unsafe fn init(&self) -> Id<Object, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn automaticRule() -> Id<NSInflectionRule, Shared> {
        msg_send_id![Self::class(), automaticRule]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSInflectionRuleExplicit;
    unsafe impl ClassType for NSInflectionRuleExplicit {
        type Super = NSInflectionRule;
    }
);
impl NSInflectionRuleExplicit {
    pub unsafe fn initWithMorphology(&self, morphology: &NSMorphology) -> Id<Self, Shared> {
        msg_send_id![self, initWithMorphology: morphology]
    }
    pub unsafe fn morphology(&self) -> Id<NSMorphology, Shared> {
        msg_send_id![self, morphology]
    }
}
#[doc = "NSInflectionAvailability"]
impl NSInflectionRule {
    pub unsafe fn canInflectLanguage(language: &NSString) -> bool {
        msg_send![Self::class(), canInflectLanguage: language]
    }
    pub unsafe fn canInflectPreferredLocalization() -> bool {
        msg_send![Self::class(), canInflectPreferredLocalization]
    }
}
