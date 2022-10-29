#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSUserDefaults;
    unsafe impl ClassType for NSUserDefaults {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSUserDefaults {
        #[method_id(standardUserDefaults)]
        pub unsafe fn standardUserDefaults() -> Id<NSUserDefaults, Shared>;
        #[method(resetStandardUserDefaults)]
        pub unsafe fn resetStandardUserDefaults();
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithSuiteName:)]
        pub unsafe fn initWithSuiteName(
            &self,
            suitename: Option<&NSString>,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithUser:)]
        pub unsafe fn initWithUser(&self, username: &NSString) -> Option<Id<Object, Shared>>;
        #[method_id(objectForKey:)]
        pub unsafe fn objectForKey(&self, defaultName: &NSString) -> Option<Id<Object, Shared>>;
        #[method(setObject:forKey:)]
        pub unsafe fn setObject_forKey(&self, value: Option<&Object>, defaultName: &NSString);
        #[method(removeObjectForKey:)]
        pub unsafe fn removeObjectForKey(&self, defaultName: &NSString);
        #[method_id(stringForKey:)]
        pub unsafe fn stringForKey(&self, defaultName: &NSString) -> Option<Id<NSString, Shared>>;
        #[method_id(arrayForKey:)]
        pub unsafe fn arrayForKey(&self, defaultName: &NSString) -> Option<Id<NSArray, Shared>>;
        #[method_id(dictionaryForKey:)]
        pub unsafe fn dictionaryForKey(
            &self,
            defaultName: &NSString,
        ) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;
        #[method_id(dataForKey:)]
        pub unsafe fn dataForKey(&self, defaultName: &NSString) -> Option<Id<NSData, Shared>>;
        #[method_id(stringArrayForKey:)]
        pub unsafe fn stringArrayForKey(
            &self,
            defaultName: &NSString,
        ) -> Option<Id<NSArray<NSString>, Shared>>;
        #[method(integerForKey:)]
        pub unsafe fn integerForKey(&self, defaultName: &NSString) -> NSInteger;
        #[method(floatForKey:)]
        pub unsafe fn floatForKey(&self, defaultName: &NSString) -> c_float;
        #[method(doubleForKey:)]
        pub unsafe fn doubleForKey(&self, defaultName: &NSString) -> c_double;
        #[method(boolForKey:)]
        pub unsafe fn boolForKey(&self, defaultName: &NSString) -> bool;
        #[method_id(URLForKey:)]
        pub unsafe fn URLForKey(&self, defaultName: &NSString) -> Option<Id<NSURL, Shared>>;
        #[method(setInteger:forKey:)]
        pub unsafe fn setInteger_forKey(&self, value: NSInteger, defaultName: &NSString);
        #[method(setFloat:forKey:)]
        pub unsafe fn setFloat_forKey(&self, value: c_float, defaultName: &NSString);
        #[method(setDouble:forKey:)]
        pub unsafe fn setDouble_forKey(&self, value: c_double, defaultName: &NSString);
        #[method(setBool:forKey:)]
        pub unsafe fn setBool_forKey(&self, value: bool, defaultName: &NSString);
        #[method(setURL:forKey:)]
        pub unsafe fn setURL_forKey(&self, url: Option<&NSURL>, defaultName: &NSString);
        #[method(registerDefaults:)]
        pub unsafe fn registerDefaults(
            &self,
            registrationDictionary: &NSDictionary<NSString, Object>,
        );
        #[method(addSuiteNamed:)]
        pub unsafe fn addSuiteNamed(&self, suiteName: &NSString);
        #[method(removeSuiteNamed:)]
        pub unsafe fn removeSuiteNamed(&self, suiteName: &NSString);
        #[method_id(dictionaryRepresentation)]
        pub unsafe fn dictionaryRepresentation(&self)
            -> Id<NSDictionary<NSString, Object>, Shared>;
        #[method_id(volatileDomainNames)]
        pub unsafe fn volatileDomainNames(&self) -> Id<NSArray<NSString>, Shared>;
        #[method_id(volatileDomainForName:)]
        pub unsafe fn volatileDomainForName(
            &self,
            domainName: &NSString,
        ) -> Id<NSDictionary<NSString, Object>, Shared>;
        #[method(setVolatileDomain:forName:)]
        pub unsafe fn setVolatileDomain_forName(
            &self,
            domain: &NSDictionary<NSString, Object>,
            domainName: &NSString,
        );
        #[method(removeVolatileDomainForName:)]
        pub unsafe fn removeVolatileDomainForName(&self, domainName: &NSString);
        #[method_id(persistentDomainNames)]
        pub unsafe fn persistentDomainNames(&self) -> Id<NSArray, Shared>;
        #[method_id(persistentDomainForName:)]
        pub unsafe fn persistentDomainForName(
            &self,
            domainName: &NSString,
        ) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;
        #[method(setPersistentDomain:forName:)]
        pub unsafe fn setPersistentDomain_forName(
            &self,
            domain: &NSDictionary<NSString, Object>,
            domainName: &NSString,
        );
        #[method(removePersistentDomainForName:)]
        pub unsafe fn removePersistentDomainForName(&self, domainName: &NSString);
        #[method(synchronize)]
        pub unsafe fn synchronize(&self) -> bool;
        #[method(objectIsForcedForKey:)]
        pub unsafe fn objectIsForcedForKey(&self, key: &NSString) -> bool;
        #[method(objectIsForcedForKey:inDomain:)]
        pub unsafe fn objectIsForcedForKey_inDomain(
            &self,
            key: &NSString,
            domain: &NSString,
        ) -> bool;
    }
);
