#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSKeyValueChangeKey = NSString;
extern_methods!(
    #[doc = "NSKeyValueObserving"]
    unsafe impl NSObject {
        #[method(observeValueForKeyPath:ofObject:change:context:)]
        pub unsafe fn observeValueForKeyPath_ofObject_change_context(
            &self,
            keyPath: Option<&NSString>,
            object: Option<&Object>,
            change: Option<&NSDictionary<NSKeyValueChangeKey, Object>>,
            context: *mut c_void,
        );
    }
);
extern_methods!(
    #[doc = "NSKeyValueObserverRegistration"]
    unsafe impl NSObject {
        #[method(addObserver:forKeyPath:options:context:)]
        pub unsafe fn addObserver_forKeyPath_options_context(
            &self,
            observer: &NSObject,
            keyPath: &NSString,
            options: NSKeyValueObservingOptions,
            context: *mut c_void,
        );
        #[method(removeObserver:forKeyPath:context:)]
        pub unsafe fn removeObserver_forKeyPath_context(
            &self,
            observer: &NSObject,
            keyPath: &NSString,
            context: *mut c_void,
        );
        #[method(removeObserver:forKeyPath:)]
        pub unsafe fn removeObserver_forKeyPath(&self, observer: &NSObject, keyPath: &NSString);
    }
);
extern_methods!(
    #[doc = "NSKeyValueObserverRegistration"]
    unsafe impl<ObjectType: Message> NSArray<ObjectType> {
        #[method(addObserver:toObjectsAtIndexes:forKeyPath:options:context:)]
        pub unsafe fn addObserver_toObjectsAtIndexes_forKeyPath_options_context(
            &self,
            observer: &NSObject,
            indexes: &NSIndexSet,
            keyPath: &NSString,
            options: NSKeyValueObservingOptions,
            context: *mut c_void,
        );
        #[method(removeObserver:fromObjectsAtIndexes:forKeyPath:context:)]
        pub unsafe fn removeObserver_fromObjectsAtIndexes_forKeyPath_context(
            &self,
            observer: &NSObject,
            indexes: &NSIndexSet,
            keyPath: &NSString,
            context: *mut c_void,
        );
        #[method(removeObserver:fromObjectsAtIndexes:forKeyPath:)]
        pub unsafe fn removeObserver_fromObjectsAtIndexes_forKeyPath(
            &self,
            observer: &NSObject,
            indexes: &NSIndexSet,
            keyPath: &NSString,
        );
        #[method(addObserver:forKeyPath:options:context:)]
        pub unsafe fn addObserver_forKeyPath_options_context(
            &self,
            observer: &NSObject,
            keyPath: &NSString,
            options: NSKeyValueObservingOptions,
            context: *mut c_void,
        );
        #[method(removeObserver:forKeyPath:context:)]
        pub unsafe fn removeObserver_forKeyPath_context(
            &self,
            observer: &NSObject,
            keyPath: &NSString,
            context: *mut c_void,
        );
        #[method(removeObserver:forKeyPath:)]
        pub unsafe fn removeObserver_forKeyPath(&self, observer: &NSObject, keyPath: &NSString);
    }
);
extern_methods!(
    #[doc = "NSKeyValueObserverRegistration"]
    unsafe impl<ObjectType: Message> NSOrderedSet<ObjectType> {
        #[method(addObserver:forKeyPath:options:context:)]
        pub unsafe fn addObserver_forKeyPath_options_context(
            &self,
            observer: &NSObject,
            keyPath: &NSString,
            options: NSKeyValueObservingOptions,
            context: *mut c_void,
        );
        #[method(removeObserver:forKeyPath:context:)]
        pub unsafe fn removeObserver_forKeyPath_context(
            &self,
            observer: &NSObject,
            keyPath: &NSString,
            context: *mut c_void,
        );
        #[method(removeObserver:forKeyPath:)]
        pub unsafe fn removeObserver_forKeyPath(&self, observer: &NSObject, keyPath: &NSString);
    }
);
extern_methods!(
    #[doc = "NSKeyValueObserverRegistration"]
    unsafe impl<ObjectType: Message> NSSet<ObjectType> {
        #[method(addObserver:forKeyPath:options:context:)]
        pub unsafe fn addObserver_forKeyPath_options_context(
            &self,
            observer: &NSObject,
            keyPath: &NSString,
            options: NSKeyValueObservingOptions,
            context: *mut c_void,
        );
        #[method(removeObserver:forKeyPath:context:)]
        pub unsafe fn removeObserver_forKeyPath_context(
            &self,
            observer: &NSObject,
            keyPath: &NSString,
            context: *mut c_void,
        );
        #[method(removeObserver:forKeyPath:)]
        pub unsafe fn removeObserver_forKeyPath(&self, observer: &NSObject, keyPath: &NSString);
    }
);
extern_methods!(
    #[doc = "NSKeyValueObserverNotification"]
    unsafe impl NSObject {
        #[method(willChangeValueForKey:)]
        pub unsafe fn willChangeValueForKey(&self, key: &NSString);
        #[method(didChangeValueForKey:)]
        pub unsafe fn didChangeValueForKey(&self, key: &NSString);
        #[method(willChange:valuesAtIndexes:forKey:)]
        pub unsafe fn willChange_valuesAtIndexes_forKey(
            &self,
            changeKind: NSKeyValueChange,
            indexes: &NSIndexSet,
            key: &NSString,
        );
        #[method(didChange:valuesAtIndexes:forKey:)]
        pub unsafe fn didChange_valuesAtIndexes_forKey(
            &self,
            changeKind: NSKeyValueChange,
            indexes: &NSIndexSet,
            key: &NSString,
        );
        #[method(willChangeValueForKey:withSetMutation:usingObjects:)]
        pub unsafe fn willChangeValueForKey_withSetMutation_usingObjects(
            &self,
            key: &NSString,
            mutationKind: NSKeyValueSetMutationKind,
            objects: &NSSet,
        );
        #[method(didChangeValueForKey:withSetMutation:usingObjects:)]
        pub unsafe fn didChangeValueForKey_withSetMutation_usingObjects(
            &self,
            key: &NSString,
            mutationKind: NSKeyValueSetMutationKind,
            objects: &NSSet,
        );
    }
);
extern_methods!(
    #[doc = "NSKeyValueObservingCustomization"]
    unsafe impl NSObject {
        #[method_id(keyPathsForValuesAffectingValueForKey:)]
        pub unsafe fn keyPathsForValuesAffectingValueForKey(
            key: &NSString,
        ) -> Id<NSSet<NSString>, Shared>;
        #[method(automaticallyNotifiesObserversForKey:)]
        pub unsafe fn automaticallyNotifiesObserversForKey(key: &NSString) -> bool;
        #[method(observationInfo)]
        pub unsafe fn observationInfo(&self) -> *mut c_void;
        #[method(setObservationInfo:)]
        pub unsafe fn setObservationInfo(&self, observationInfo: *mut c_void);
    }
);
extern_methods!(
    #[doc = "NSDeprecatedKeyValueObservingCustomization"]
    unsafe impl NSObject {
        #[method(setKeys:triggerChangeNotificationsForDependentKey:)]
        pub unsafe fn setKeys_triggerChangeNotificationsForDependentKey(
            keys: &NSArray,
            dependentKey: &NSString,
        );
    }
);
