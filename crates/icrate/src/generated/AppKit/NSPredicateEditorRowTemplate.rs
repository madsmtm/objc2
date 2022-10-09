use super::__exported::NSEntityDescription;
use super::__exported::NSPredicate;
use super::__exported::NSView;
use crate::AppKit::generated::AppKitDefines::*;
use crate::CoreData::generated::NSAttributeDescription::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSComparisonPredicate::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPredicateEditorRowTemplate;
    unsafe impl ClassType for NSPredicateEditorRowTemplate {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSPredicateEditorRowTemplate {
        #[method(matchForPredicate:)]
        pub unsafe fn matchForPredicate(&self, predicate: &NSPredicate) -> c_double;
        #[method_id(templateViews)]
        pub unsafe fn templateViews(&self) -> Id<NSArray<NSView>, Shared>;
        #[method(setPredicate:)]
        pub unsafe fn setPredicate(&self, predicate: &NSPredicate);
        #[method_id(predicateWithSubpredicates:)]
        pub unsafe fn predicateWithSubpredicates(
            &self,
            subpredicates: Option<&NSArray<NSPredicate>>,
        ) -> Id<NSPredicate, Shared>;
        #[method_id(displayableSubpredicatesOfPredicate:)]
        pub unsafe fn displayableSubpredicatesOfPredicate(
            &self,
            predicate: &NSPredicate,
        ) -> Option<Id<NSArray<NSPredicate>, Shared>>;
        #[method_id(initWithLeftExpressions:rightExpressions:modifier:operators:options:)]
        pub unsafe fn initWithLeftExpressions_rightExpressions_modifier_operators_options(
            &self,
            leftExpressions: &NSArray<NSExpression>,
            rightExpressions: &NSArray<NSExpression>,
            modifier: NSComparisonPredicateModifier,
            operators: &NSArray<NSNumber>,
            options: NSUInteger,
        ) -> Id<Self, Shared>;
        #[method_id(initWithLeftExpressions:rightExpressionAttributeType:modifier:operators:options:)]
        pub unsafe fn initWithLeftExpressions_rightExpressionAttributeType_modifier_operators_options(
            &self,
            leftExpressions: &NSArray<NSExpression>,
            attributeType: NSAttributeType,
            modifier: NSComparisonPredicateModifier,
            operators: &NSArray<NSNumber>,
            options: NSUInteger,
        ) -> Id<Self, Shared>;
        #[method_id(initWithCompoundTypes:)]
        pub unsafe fn initWithCompoundTypes(
            &self,
            compoundTypes: &NSArray<NSNumber>,
        ) -> Id<Self, Shared>;
        #[method_id(leftExpressions)]
        pub unsafe fn leftExpressions(&self) -> Option<Id<NSArray<NSExpression>, Shared>>;
        #[method_id(rightExpressions)]
        pub unsafe fn rightExpressions(&self) -> Option<Id<NSArray<NSExpression>, Shared>>;
        #[method(rightExpressionAttributeType)]
        pub unsafe fn rightExpressionAttributeType(&self) -> NSAttributeType;
        #[method(modifier)]
        pub unsafe fn modifier(&self) -> NSComparisonPredicateModifier;
        #[method_id(operators)]
        pub unsafe fn operators(&self) -> Option<Id<NSArray<NSNumber>, Shared>>;
        #[method(options)]
        pub unsafe fn options(&self) -> NSUInteger;
        #[method_id(compoundTypes)]
        pub unsafe fn compoundTypes(&self) -> Option<Id<NSArray<NSNumber>, Shared>>;
        #[method_id(templatesWithAttributeKeyPaths:inEntityDescription:)]
        pub unsafe fn templatesWithAttributeKeyPaths_inEntityDescription(
            keyPaths: &NSArray<NSString>,
            entityDescription: &NSEntityDescription,
        ) -> Id<NSArray<NSPredicateEditorRowTemplate>, Shared>;
    }
);
