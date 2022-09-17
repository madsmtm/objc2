use super::__exported::NSArray;
use super::__exported::NSData;
use super::__exported::NSDictionary;
use super::__exported::NSMutableDictionary;
use super::__exported::NSString;
use super::__exported::NSURL;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSUserDefaults;
    unsafe impl ClassType for NSUserDefaults {
        type Super = NSObject;
    }
);
impl NSUserDefaults {
    pub unsafe fn resetStandardUserDefaults() {
        msg_send![Self::class(), resetStandardUserDefaults]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn initWithSuiteName(
        &self,
        suitename: Option<&NSString>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithSuiteName: suitename]
    }
    pub unsafe fn initWithUser(&self, username: &NSString) -> Option<Id<Object, Shared>> {
        msg_send_id![self, initWithUser: username]
    }
    pub unsafe fn objectForKey(&self, defaultName: &NSString) -> Option<Id<Object, Shared>> {
        msg_send_id![self, objectForKey: defaultName]
    }
    pub unsafe fn setObject_forKey(&self, value: Option<&Object>, defaultName: &NSString) {
        msg_send![self, setObject: value, forKey: defaultName]
    }
    pub unsafe fn removeObjectForKey(&self, defaultName: &NSString) {
        msg_send![self, removeObjectForKey: defaultName]
    }
    pub unsafe fn stringForKey(&self, defaultName: &NSString) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, stringForKey: defaultName]
    }
    pub unsafe fn arrayForKey(&self, defaultName: &NSString) -> Option<Id<NSArray, Shared>> {
        msg_send_id![self, arrayForKey: defaultName]
    }
    pub unsafe fn dictionaryForKey(&self, defaultName: &NSString) -> TodoGenerics {
        msg_send![self, dictionaryForKey: defaultName]
    }
    pub unsafe fn dataForKey(&self, defaultName: &NSString) -> Option<Id<NSData, Shared>> {
        msg_send_id![self, dataForKey: defaultName]
    }
    pub unsafe fn stringArrayForKey(&self, defaultName: &NSString) -> TodoGenerics {
        msg_send![self, stringArrayForKey: defaultName]
    }
    pub unsafe fn integerForKey(&self, defaultName: &NSString) -> NSInteger {
        msg_send![self, integerForKey: defaultName]
    }
    pub unsafe fn floatForKey(&self, defaultName: &NSString) -> c_float {
        msg_send![self, floatForKey: defaultName]
    }
    pub unsafe fn doubleForKey(&self, defaultName: &NSString) -> c_double {
        msg_send![self, doubleForKey: defaultName]
    }
    pub unsafe fn boolForKey(&self, defaultName: &NSString) -> bool {
        msg_send![self, boolForKey: defaultName]
    }
    pub unsafe fn URLForKey(&self, defaultName: &NSString) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, URLForKey: defaultName]
    }
    pub unsafe fn setInteger_forKey(&self, value: NSInteger, defaultName: &NSString) {
        msg_send![self, setInteger: value, forKey: defaultName]
    }
    pub unsafe fn setFloat_forKey(&self, value: c_float, defaultName: &NSString) {
        msg_send![self, setFloat: value, forKey: defaultName]
    }
    pub unsafe fn setDouble_forKey(&self, value: c_double, defaultName: &NSString) {
        msg_send![self, setDouble: value, forKey: defaultName]
    }
    pub unsafe fn setBool_forKey(&self, value: bool, defaultName: &NSString) {
        msg_send![self, setBool: value, forKey: defaultName]
    }
    pub unsafe fn setURL_forKey(&self, url: Option<&NSURL>, defaultName: &NSString) {
        msg_send![self, setURL: url, forKey: defaultName]
    }
    pub unsafe fn registerDefaults(&self, registrationDictionary: TodoGenerics) {
        msg_send![self, registerDefaults: registrationDictionary]
    }
    pub unsafe fn addSuiteNamed(&self, suiteName: &NSString) {
        msg_send![self, addSuiteNamed: suiteName]
    }
    pub unsafe fn removeSuiteNamed(&self, suiteName: &NSString) {
        msg_send![self, removeSuiteNamed: suiteName]
    }
    pub unsafe fn dictionaryRepresentation(&self) -> TodoGenerics {
        msg_send![self, dictionaryRepresentation]
    }
    pub unsafe fn volatileDomainForName(&self, domainName: &NSString) -> TodoGenerics {
        msg_send![self, volatileDomainForName: domainName]
    }
    pub unsafe fn setVolatileDomain_forName(&self, domain: TodoGenerics, domainName: &NSString) {
        msg_send![self, setVolatileDomain: domain, forName: domainName]
    }
    pub unsafe fn removeVolatileDomainForName(&self, domainName: &NSString) {
        msg_send![self, removeVolatileDomainForName: domainName]
    }
    pub unsafe fn persistentDomainNames(&self) -> Id<NSArray, Shared> {
        msg_send_id![self, persistentDomainNames]
    }
    pub unsafe fn persistentDomainForName(&self, domainName: &NSString) -> TodoGenerics {
        msg_send![self, persistentDomainForName: domainName]
    }
    pub unsafe fn setPersistentDomain_forName(&self, domain: TodoGenerics, domainName: &NSString) {
        msg_send![self, setPersistentDomain: domain, forName: domainName]
    }
    pub unsafe fn removePersistentDomainForName(&self, domainName: &NSString) {
        msg_send![self, removePersistentDomainForName: domainName]
    }
    pub unsafe fn synchronize(&self) -> bool {
        msg_send![self, synchronize]
    }
    pub unsafe fn objectIsForcedForKey(&self, key: &NSString) -> bool {
        msg_send![self, objectIsForcedForKey: key]
    }
    pub unsafe fn objectIsForcedForKey_inDomain(&self, key: &NSString, domain: &NSString) -> bool {
        msg_send![self, objectIsForcedForKey: key, inDomain: domain]
    }
    pub unsafe fn standardUserDefaults() -> Id<NSUserDefaults, Shared> {
        msg_send_id![Self::class(), standardUserDefaults]
    }
    pub unsafe fn volatileDomainNames(&self) -> TodoGenerics {
        msg_send![self, volatileDomainNames]
    }
}
