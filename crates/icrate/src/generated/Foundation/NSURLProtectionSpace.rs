//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern "C" {
    pub static NSURLProtectionSpaceHTTP: &'static NSString;
}

extern "C" {
    pub static NSURLProtectionSpaceHTTPS: &'static NSString;
}

extern "C" {
    pub static NSURLProtectionSpaceFTP: &'static NSString;
}

extern "C" {
    pub static NSURLProtectionSpaceHTTPProxy: &'static NSString;
}

extern "C" {
    pub static NSURLProtectionSpaceHTTPSProxy: &'static NSString;
}

extern "C" {
    pub static NSURLProtectionSpaceFTPProxy: &'static NSString;
}

extern "C" {
    pub static NSURLProtectionSpaceSOCKSProxy: &'static NSString;
}

extern "C" {
    pub static NSURLAuthenticationMethodDefault: &'static NSString;
}

extern "C" {
    pub static NSURLAuthenticationMethodHTTPBasic: &'static NSString;
}

extern "C" {
    pub static NSURLAuthenticationMethodHTTPDigest: &'static NSString;
}

extern "C" {
    pub static NSURLAuthenticationMethodHTMLForm: &'static NSString;
}

extern "C" {
    pub static NSURLAuthenticationMethodNTLM: &'static NSString;
}

extern "C" {
    pub static NSURLAuthenticationMethodNegotiate: &'static NSString;
}

extern "C" {
    pub static NSURLAuthenticationMethodClientCertificate: &'static NSString;
}

extern "C" {
    pub static NSURLAuthenticationMethodServerTrust: &'static NSString;
}

extern_class!(
    #[derive(Debug)]
    pub struct NSURLProtectionSpace;

    unsafe impl ClassType for NSURLProtectionSpace {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSURLProtectionSpace {
        #[method_id(@__retain_semantics Init initWithHost:port:protocol:realm:authenticationMethod:)]
        pub unsafe fn initWithHost_port_protocol_realm_authenticationMethod(
            this: Option<Allocated<Self>>,
            host: &NSString,
            port: NSInteger,
            protocol: Option<&NSString>,
            realm: Option<&NSString>,
            authenticationMethod: Option<&NSString>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithProxyHost:port:type:realm:authenticationMethod:)]
        pub unsafe fn initWithProxyHost_port_type_realm_authenticationMethod(
            this: Option<Allocated<Self>>,
            host: &NSString,
            port: NSInteger,
            type_: Option<&NSString>,
            realm: Option<&NSString>,
            authenticationMethod: Option<&NSString>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other realm)]
        pub unsafe fn realm(&self) -> Option<Id<NSString, Shared>>;

        #[method(receivesCredentialSecurely)]
        pub unsafe fn receivesCredentialSecurely(&self) -> bool;

        #[method(isProxy)]
        pub unsafe fn isProxy(&self) -> bool;

        #[method_id(@__retain_semantics Other host)]
        pub unsafe fn host(&self) -> Id<NSString, Shared>;

        #[method(port)]
        pub unsafe fn port(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other proxyType)]
        pub unsafe fn proxyType(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other protocol)]
        pub unsafe fn protocol(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other authenticationMethod)]
        pub unsafe fn authenticationMethod(&self) -> Id<NSString, Shared>;
    }
);

extern_methods!(
    /// NSClientCertificateSpace
    unsafe impl NSURLProtectionSpace {
        #[method_id(@__retain_semantics Other distinguishedNames)]
        pub unsafe fn distinguishedNames(&self) -> Option<Id<NSArray<NSData>, Shared>>;
    }
);

extern_methods!(
    /// NSServerTrustValidationSpace
    unsafe impl NSURLProtectionSpace {}
);
