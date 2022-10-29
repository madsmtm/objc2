#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSBindingName = NSString;
pub type NSBindingOption = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSBindingSelectionMarker;
    unsafe impl ClassType for NSBindingSelectionMarker {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSBindingSelectionMarker {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(multipleValuesSelectionMarker)]
        pub unsafe fn multipleValuesSelectionMarker() -> Id<NSBindingSelectionMarker, Shared>;
        #[method_id(noSelectionMarker)]
        pub unsafe fn noSelectionMarker() -> Id<NSBindingSelectionMarker, Shared>;
        #[method_id(notApplicableSelectionMarker)]
        pub unsafe fn notApplicableSelectionMarker() -> Id<NSBindingSelectionMarker, Shared>;
        #[method(setDefaultPlaceholder:forMarker:onClass:withBinding:)]
        pub unsafe fn setDefaultPlaceholder_forMarker_onClass_withBinding(
            placeholder: Option<&Object>,
            marker: Option<&NSBindingSelectionMarker>,
            objectClass: &Class,
            binding: &NSBindingName,
        );
        #[method_id(defaultPlaceholderForMarker:onClass:withBinding:)]
        pub unsafe fn defaultPlaceholderForMarker_onClass_withBinding(
            marker: Option<&NSBindingSelectionMarker>,
            objectClass: &Class,
            binding: &NSBindingName,
        ) -> Option<Id<Object, Shared>>;
    }
);
pub type NSBindingInfoKey = NSString;
extern_methods!(
    #[doc = "NSKeyValueBindingCreation"]
    unsafe impl NSObject {
        #[method(exposeBinding:)]
        pub unsafe fn exposeBinding(binding: &NSBindingName);
        #[method_id(exposedBindings)]
        pub unsafe fn exposedBindings(&self) -> Id<NSArray<NSBindingName>, Shared>;
        #[method(valueClassForBinding:)]
        pub unsafe fn valueClassForBinding(&self, binding: &NSBindingName) -> Option<&Class>;
        #[method(bind:toObject:withKeyPath:options:)]
        pub unsafe fn bind_toObject_withKeyPath_options(
            &self,
            binding: &NSBindingName,
            observable: &Object,
            keyPath: &NSString,
            options: Option<&NSDictionary<NSBindingOption, Object>>,
        );
        #[method(unbind:)]
        pub unsafe fn unbind(&self, binding: &NSBindingName);
        #[method_id(infoForBinding:)]
        pub unsafe fn infoForBinding(
            &self,
            binding: &NSBindingName,
        ) -> Option<Id<NSDictionary<NSBindingInfoKey, Object>, Shared>>;
        #[method_id(optionDescriptionsForBinding:)]
        pub unsafe fn optionDescriptionsForBinding(
            &self,
            binding: &NSBindingName,
        ) -> Id<NSArray<NSAttributeDescription>, Shared>;
    }
);
extern_methods!(
    #[doc = "NSPlaceholders"]
    unsafe impl NSObject {
        #[method(setDefaultPlaceholder:forMarker:withBinding:)]
        pub unsafe fn setDefaultPlaceholder_forMarker_withBinding(
            placeholder: Option<&Object>,
            marker: Option<&Object>,
            binding: &NSBindingName,
        );
        #[method_id(defaultPlaceholderForMarker:withBinding:)]
        pub unsafe fn defaultPlaceholderForMarker_withBinding(
            marker: Option<&Object>,
            binding: &NSBindingName,
        ) -> Option<Id<Object, Shared>>;
    }
);
pub type NSEditor = NSObject;
pub type NSEditorRegistration = NSObject;
extern_methods!(
    #[doc = "NSEditor"]
    unsafe impl NSObject {
        #[method(discardEditing)]
        pub unsafe fn discardEditing(&self);
        #[method(commitEditing)]
        pub unsafe fn commitEditing(&self) -> bool;
        #[method(commitEditingWithDelegate:didCommitSelector:contextInfo:)]
        pub unsafe fn commitEditingWithDelegate_didCommitSelector_contextInfo(
            &self,
            delegate: Option<&Object>,
            didCommitSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );
        #[method(commitEditingAndReturnError:)]
        pub unsafe fn commitEditingAndReturnError(&self) -> Result<(), Id<NSError, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSEditorRegistration"]
    unsafe impl NSObject {
        #[method(objectDidBeginEditing:)]
        pub unsafe fn objectDidBeginEditing(&self, editor: &NSEditor);
        #[method(objectDidEndEditing:)]
        pub unsafe fn objectDidEndEditing(&self, editor: &NSEditor);
    }
);
extern_methods!(
    #[doc = "NSEditorAndEditorRegistrationConformance"]
    unsafe impl NSManagedObjectContext {}
);
