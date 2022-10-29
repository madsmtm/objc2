#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_methods!(
    #[doc = "NSAccessibility"]
    unsafe impl NSObject {
        #[method_id(accessibilityAttributeNames)]
        pub unsafe fn accessibilityAttributeNames(
            &self,
        ) -> Id<NSArray<NSAccessibilityAttributeName>, Shared>;
        #[method_id(accessibilityAttributeValue:)]
        pub unsafe fn accessibilityAttributeValue(
            &self,
            attribute: &NSAccessibilityAttributeName,
        ) -> Option<Id<Object, Shared>>;
        #[method(accessibilityIsAttributeSettable:)]
        pub unsafe fn accessibilityIsAttributeSettable(
            &self,
            attribute: &NSAccessibilityAttributeName,
        ) -> bool;
        #[method(accessibilitySetValue:forAttribute:)]
        pub unsafe fn accessibilitySetValue_forAttribute(
            &self,
            value: Option<&Object>,
            attribute: &NSAccessibilityAttributeName,
        );
        #[method_id(accessibilityParameterizedAttributeNames)]
        pub unsafe fn accessibilityParameterizedAttributeNames(
            &self,
        ) -> Id<NSArray<NSAccessibilityParameterizedAttributeName>, Shared>;
        #[method_id(accessibilityAttributeValue:forParameter:)]
        pub unsafe fn accessibilityAttributeValue_forParameter(
            &self,
            attribute: &NSAccessibilityParameterizedAttributeName,
            parameter: Option<&Object>,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(accessibilityActionNames)]
        pub unsafe fn accessibilityActionNames(
            &self,
        ) -> Id<NSArray<NSAccessibilityActionName>, Shared>;
        #[method_id(accessibilityActionDescription:)]
        pub unsafe fn accessibilityActionDescription(
            &self,
            action: &NSAccessibilityActionName,
        ) -> Option<Id<NSString, Shared>>;
        #[method(accessibilityPerformAction:)]
        pub unsafe fn accessibilityPerformAction(&self, action: &NSAccessibilityActionName);
        #[method(accessibilityIsIgnored)]
        pub unsafe fn accessibilityIsIgnored(&self) -> bool;
        #[method_id(accessibilityHitTest:)]
        pub unsafe fn accessibilityHitTest(&self, point: NSPoint) -> Option<Id<Object, Shared>>;
        #[method_id(accessibilityFocusedUIElement)]
        pub unsafe fn accessibilityFocusedUIElement(&self) -> Option<Id<Object, Shared>>;
        #[method(accessibilityIndexOfChild:)]
        pub unsafe fn accessibilityIndexOfChild(&self, child: &Object) -> NSUInteger;
        #[method(accessibilityArrayAttributeCount:)]
        pub unsafe fn accessibilityArrayAttributeCount(
            &self,
            attribute: &NSAccessibilityAttributeName,
        ) -> NSUInteger;
        #[method_id(accessibilityArrayAttributeValues:index:maxCount:)]
        pub unsafe fn accessibilityArrayAttributeValues_index_maxCount(
            &self,
            attribute: &NSAccessibilityAttributeName,
            index: NSUInteger,
            maxCount: NSUInteger,
        ) -> Id<NSArray, Shared>;
        #[method(accessibilityNotifiesWhenDestroyed)]
        pub unsafe fn accessibilityNotifiesWhenDestroyed(&self) -> bool;
    }
);
extern_methods!(
    #[doc = "NSWorkspaceAccessibilityDisplay"]
    unsafe impl NSWorkspace {
        #[method(accessibilityDisplayShouldIncreaseContrast)]
        pub unsafe fn accessibilityDisplayShouldIncreaseContrast(&self) -> bool;
        #[method(accessibilityDisplayShouldDifferentiateWithoutColor)]
        pub unsafe fn accessibilityDisplayShouldDifferentiateWithoutColor(&self) -> bool;
        #[method(accessibilityDisplayShouldReduceTransparency)]
        pub unsafe fn accessibilityDisplayShouldReduceTransparency(&self) -> bool;
        #[method(accessibilityDisplayShouldReduceMotion)]
        pub unsafe fn accessibilityDisplayShouldReduceMotion(&self) -> bool;
        #[method(accessibilityDisplayShouldInvertColors)]
        pub unsafe fn accessibilityDisplayShouldInvertColors(&self) -> bool;
    }
);
extern_methods!(
    #[doc = "NSWorkspaceAccessibility"]
    unsafe impl NSWorkspace {
        #[method(isVoiceOverEnabled)]
        pub unsafe fn isVoiceOverEnabled(&self) -> bool;
        #[method(isSwitchControlEnabled)]
        pub unsafe fn isSwitchControlEnabled(&self) -> bool;
    }
);
extern_methods!(
    #[doc = "NSAccessibilityAdditions"]
    unsafe impl NSObject {
        #[method(accessibilitySetOverrideValue:forAttribute:)]
        pub unsafe fn accessibilitySetOverrideValue_forAttribute(
            &self,
            value: Option<&Object>,
            attribute: &NSAccessibilityAttributeName,
        ) -> bool;
    }
);
