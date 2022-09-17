#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    struct NSURLProtectionSpace;
    unsafe impl ClassType for NSURLProtectionSpace {
        type Super = NSObject;
    }
);
impl NSURLProtectionSpace {
    pub unsafe fn initWithHost_port_protocol_realm_authenticationMethod(
        &self,
        host: &NSString,
        port: NSInteger,
        protocol: Option<&NSString>,
        realm: Option<&NSString>,
        authenticationMethod: Option<&NSString>,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithHost: host,
            port: port,
            protocol: protocol,
            realm: realm,
            authenticationMethod: authenticationMethod
        ]
    }
    pub unsafe fn initWithProxyHost_port_type_realm_authenticationMethod(
        &self,
        host: &NSString,
        port: NSInteger,
        type_: Option<&NSString>,
        realm: Option<&NSString>,
        authenticationMethod: Option<&NSString>,
    ) -> Id<Self, Shared> {
        msg_send_id ! [self , initWithProxyHost : host , port : port , type : type_ , realm : realm , authenticationMethod : authenticationMethod]
    }
    pub unsafe fn realm(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, realm]
    }
    pub unsafe fn receivesCredentialSecurely(&self) -> bool {
        msg_send![self, receivesCredentialSecurely]
    }
    pub unsafe fn isProxy(&self) -> bool {
        msg_send![self, isProxy]
    }
    pub unsafe fn host(&self) -> Id<NSString, Shared> {
        msg_send_id![self, host]
    }
    pub unsafe fn port(&self) -> NSInteger {
        msg_send![self, port]
    }
    pub unsafe fn proxyType(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, proxyType]
    }
    pub unsafe fn protocol(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, protocol]
    }
    pub unsafe fn authenticationMethod(&self) -> Id<NSString, Shared> {
        msg_send_id![self, authenticationMethod]
    }
}
#[doc = "NSClientCertificateSpace"]
impl NSURLProtectionSpace {
    pub unsafe fn distinguishedNames(&self) -> TodoGenerics {
        msg_send![self, distinguishedNames]
    }
}
#[doc = "NSServerTrustValidationSpace"]
impl NSURLProtectionSpace {
    pub unsafe fn serverTrust(&self) -> SecTrustRef {
        msg_send![self, serverTrust]
    }
}
