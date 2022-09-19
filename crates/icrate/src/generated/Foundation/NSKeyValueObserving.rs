use super::__exported::NSIndexSet;
use super::__exported::NSString;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSDictionary::*;
use crate::Foundation::generated::NSOrderedSet::*;
use crate::Foundation::generated::NSSet::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
pub type NSKeyValueChangeKey = NSString;
#[doc = "NSKeyValueObserving"]
impl NSObject {
    pub unsafe fn observeValueForKeyPath_ofObject_change_context(
        &self,
        keyPath: Option<&NSString>,
        object: Option<&Object>,
        change: TodoGenerics,
        context: *mut c_void,
    ) {
        msg_send![
            self,
            observeValueForKeyPath: keyPath,
            ofObject: object,
            change: change,
            context: context
        ]
    }
}
#[doc = "NSKeyValueObserverRegistration"]
impl NSObject {
    pub unsafe fn addObserver_forKeyPath_options_context(
        &self,
        observer: &NSObject,
        keyPath: &NSString,
        options: NSKeyValueObservingOptions,
        context: *mut c_void,
    ) {
        msg_send![
            self,
            addObserver: observer,
            forKeyPath: keyPath,
            options: options,
            context: context
        ]
    }
    pub unsafe fn removeObserver_forKeyPath_context(
        &self,
        observer: &NSObject,
        keyPath: &NSString,
        context: *mut c_void,
    ) {
        msg_send![
            self,
            removeObserver: observer,
            forKeyPath: keyPath,
            context: context
        ]
    }
    pub unsafe fn removeObserver_forKeyPath(&self, observer: &NSObject, keyPath: &NSString) {
        msg_send![self, removeObserver: observer, forKeyPath: keyPath]
    }
}
#[doc = "NSKeyValueObserverRegistration"]
impl NSArray {
    pub unsafe fn addObserver_toObjectsAtIndexes_forKeyPath_options_context(
        &self,
        observer: &NSObject,
        indexes: &NSIndexSet,
        keyPath: &NSString,
        options: NSKeyValueObservingOptions,
        context: *mut c_void,
    ) {
        msg_send![
            self,
            addObserver: observer,
            toObjectsAtIndexes: indexes,
            forKeyPath: keyPath,
            options: options,
            context: context
        ]
    }
    pub unsafe fn removeObserver_fromObjectsAtIndexes_forKeyPath_context(
        &self,
        observer: &NSObject,
        indexes: &NSIndexSet,
        keyPath: &NSString,
        context: *mut c_void,
    ) {
        msg_send![
            self,
            removeObserver: observer,
            fromObjectsAtIndexes: indexes,
            forKeyPath: keyPath,
            context: context
        ]
    }
    pub unsafe fn removeObserver_fromObjectsAtIndexes_forKeyPath(
        &self,
        observer: &NSObject,
        indexes: &NSIndexSet,
        keyPath: &NSString,
    ) {
        msg_send![
            self,
            removeObserver: observer,
            fromObjectsAtIndexes: indexes,
            forKeyPath: keyPath
        ]
    }
    pub unsafe fn addObserver_forKeyPath_options_context(
        &self,
        observer: &NSObject,
        keyPath: &NSString,
        options: NSKeyValueObservingOptions,
        context: *mut c_void,
    ) {
        msg_send![
            self,
            addObserver: observer,
            forKeyPath: keyPath,
            options: options,
            context: context
        ]
    }
    pub unsafe fn removeObserver_forKeyPath_context(
        &self,
        observer: &NSObject,
        keyPath: &NSString,
        context: *mut c_void,
    ) {
        msg_send![
            self,
            removeObserver: observer,
            forKeyPath: keyPath,
            context: context
        ]
    }
    pub unsafe fn removeObserver_forKeyPath(&self, observer: &NSObject, keyPath: &NSString) {
        msg_send![self, removeObserver: observer, forKeyPath: keyPath]
    }
}
#[doc = "NSKeyValueObserverRegistration"]
impl NSOrderedSet {
    pub unsafe fn addObserver_forKeyPath_options_context(
        &self,
        observer: &NSObject,
        keyPath: &NSString,
        options: NSKeyValueObservingOptions,
        context: *mut c_void,
    ) {
        msg_send![
            self,
            addObserver: observer,
            forKeyPath: keyPath,
            options: options,
            context: context
        ]
    }
    pub unsafe fn removeObserver_forKeyPath_context(
        &self,
        observer: &NSObject,
        keyPath: &NSString,
        context: *mut c_void,
    ) {
        msg_send![
            self,
            removeObserver: observer,
            forKeyPath: keyPath,
            context: context
        ]
    }
    pub unsafe fn removeObserver_forKeyPath(&self, observer: &NSObject, keyPath: &NSString) {
        msg_send![self, removeObserver: observer, forKeyPath: keyPath]
    }
}
#[doc = "NSKeyValueObserverRegistration"]
impl NSSet {
    pub unsafe fn addObserver_forKeyPath_options_context(
        &self,
        observer: &NSObject,
        keyPath: &NSString,
        options: NSKeyValueObservingOptions,
        context: *mut c_void,
    ) {
        msg_send![
            self,
            addObserver: observer,
            forKeyPath: keyPath,
            options: options,
            context: context
        ]
    }
    pub unsafe fn removeObserver_forKeyPath_context(
        &self,
        observer: &NSObject,
        keyPath: &NSString,
        context: *mut c_void,
    ) {
        msg_send![
            self,
            removeObserver: observer,
            forKeyPath: keyPath,
            context: context
        ]
    }
    pub unsafe fn removeObserver_forKeyPath(&self, observer: &NSObject, keyPath: &NSString) {
        msg_send![self, removeObserver: observer, forKeyPath: keyPath]
    }
}
#[doc = "NSKeyValueObserverNotification"]
impl NSObject {
    pub unsafe fn willChangeValueForKey(&self, key: &NSString) {
        msg_send![self, willChangeValueForKey: key]
    }
    pub unsafe fn didChangeValueForKey(&self, key: &NSString) {
        msg_send![self, didChangeValueForKey: key]
    }
    pub unsafe fn willChange_valuesAtIndexes_forKey(
        &self,
        changeKind: NSKeyValueChange,
        indexes: &NSIndexSet,
        key: &NSString,
    ) {
        msg_send![
            self,
            willChange: changeKind,
            valuesAtIndexes: indexes,
            forKey: key
        ]
    }
    pub unsafe fn didChange_valuesAtIndexes_forKey(
        &self,
        changeKind: NSKeyValueChange,
        indexes: &NSIndexSet,
        key: &NSString,
    ) {
        msg_send![
            self,
            didChange: changeKind,
            valuesAtIndexes: indexes,
            forKey: key
        ]
    }
    pub unsafe fn willChangeValueForKey_withSetMutation_usingObjects(
        &self,
        key: &NSString,
        mutationKind: NSKeyValueSetMutationKind,
        objects: &NSSet,
    ) {
        msg_send![
            self,
            willChangeValueForKey: key,
            withSetMutation: mutationKind,
            usingObjects: objects
        ]
    }
    pub unsafe fn didChangeValueForKey_withSetMutation_usingObjects(
        &self,
        key: &NSString,
        mutationKind: NSKeyValueSetMutationKind,
        objects: &NSSet,
    ) {
        msg_send![
            self,
            didChangeValueForKey: key,
            withSetMutation: mutationKind,
            usingObjects: objects
        ]
    }
}
#[doc = "NSKeyValueObservingCustomization"]
impl NSObject {
    pub unsafe fn keyPathsForValuesAffectingValueForKey(key: &NSString) -> TodoGenerics {
        msg_send![Self::class(), keyPathsForValuesAffectingValueForKey: key]
    }
    pub unsafe fn automaticallyNotifiesObserversForKey(key: &NSString) -> bool {
        msg_send![Self::class(), automaticallyNotifiesObserversForKey: key]
    }
    pub unsafe fn observationInfo(&self) -> *mut c_void {
        msg_send![self, observationInfo]
    }
    pub unsafe fn setObservationInfo(&self, observationInfo: *mut c_void) {
        msg_send![self, setObservationInfo: observationInfo]
    }
}
#[doc = "NSDeprecatedKeyValueObservingCustomization"]
impl NSObject {
    pub unsafe fn setKeys_triggerChangeNotificationsForDependentKey(
        keys: &NSArray,
        dependentKey: &NSString,
    ) {
        msg_send![
            Self::class(),
            setKeys: keys,
            triggerChangeNotificationsForDependentKey: dependentKey
        ]
    }
}
