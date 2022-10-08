use super::__exported::NSArray;
use super::__exported::NSData;
use super::__exported::NSString;
use super::__exported::NSURLProtectionSpaceInternal;
use crate::Foundation::generated::NSObject::*;
use crate::Security::generated::Security::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSURLProtectionSpace;
    unsafe impl ClassType for NSURLProtectionSpace {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSURLProtectionSpace {
        # [method_id (initWithHost : port : protocol : realm : authenticationMethod :)]
        pub unsafe fn initWithHost_port_protocol_realm_authenticationMethod(
            &self,
            host: &NSString,
            port: NSInteger,
            protocol: Option<&NSString>,
            realm: Option<&NSString>,
            authenticationMethod: Option<&NSString>,
        ) -> Id<Self, Shared>;
        # [method_id (initWithProxyHost : port : type : realm : authenticationMethod :)]
        pub unsafe fn initWithProxyHost_port_type_realm_authenticationMethod(
            &self,
            host: &NSString,
            port: NSInteger,
            type_: Option<&NSString>,
            realm: Option<&NSString>,
            authenticationMethod: Option<&NSString>,
        ) -> Id<Self, Shared>;
        #[method_id(realm)]
        pub unsafe fn realm(&self) -> Option<Id<NSString, Shared>>;
        #[method(receivesCredentialSecurely)]
        pub unsafe fn receivesCredentialSecurely(&self) -> bool;
        #[method(isProxy)]
        pub unsafe fn isProxy(&self) -> bool;
        #[method_id(host)]
        pub unsafe fn host(&self) -> Id<NSString, Shared>;
        #[method(port)]
        pub unsafe fn port(&self) -> NSInteger;
        #[method_id(proxyType)]
        pub unsafe fn proxyType(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(protocol)]
        pub unsafe fn protocol(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(authenticationMethod)]
        pub unsafe fn authenticationMethod(&self) -> Id<NSString, Shared>;
    }
);
extern_methods!(
    #[doc = "NSClientCertificateSpace"]
    unsafe impl NSURLProtectionSpace {
        #[method_id(distinguishedNames)]
        pub unsafe fn distinguishedNames(&self) -> Option<Id<NSArray<NSData>, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSServerTrustValidationSpace"]
    unsafe impl NSURLProtectionSpace {
        #[method(serverTrust)]
        pub unsafe fn serverTrust(&self) -> SecTrustRef;
    }
);
