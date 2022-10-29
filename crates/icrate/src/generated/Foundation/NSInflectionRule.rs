#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSInflectionRule;
    unsafe impl ClassType for NSInflectionRule {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSInflectionRule {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Object, Shared>;
        #[method_id(automaticRule)]
        pub unsafe fn automaticRule() -> Id<NSInflectionRule, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSInflectionRuleExplicit;
    unsafe impl ClassType for NSInflectionRuleExplicit {
        type Super = NSInflectionRule;
    }
);
extern_methods!(
    unsafe impl NSInflectionRuleExplicit {
        #[method_id(initWithMorphology:)]
        pub unsafe fn initWithMorphology(&self, morphology: &NSMorphology) -> Id<Self, Shared>;
        #[method_id(morphology)]
        pub unsafe fn morphology(&self) -> Id<NSMorphology, Shared>;
    }
);
extern_methods!(
    #[doc = "NSInflectionAvailability"]
    unsafe impl NSInflectionRule {
        #[method(canInflectLanguage:)]
        pub unsafe fn canInflectLanguage(language: &NSString) -> bool;
        #[method(canInflectPreferredLocalization)]
        pub unsafe fn canInflectPreferredLocalization() -> bool;
    }
);
