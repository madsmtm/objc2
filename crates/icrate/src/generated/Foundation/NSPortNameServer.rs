#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPortNameServer;
    unsafe impl ClassType for NSPortNameServer {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSPortNameServer {
        #[method_id(systemDefaultPortNameServer)]
        pub unsafe fn systemDefaultPortNameServer() -> Id<NSPortNameServer, Shared>;
        #[method_id(portForName:)]
        pub unsafe fn portForName(&self, name: &NSString) -> Option<Id<NSPort, Shared>>;
        #[method_id(portForName:host:)]
        pub unsafe fn portForName_host(
            &self,
            name: &NSString,
            host: Option<&NSString>,
        ) -> Option<Id<NSPort, Shared>>;
        #[method(registerPort:name:)]
        pub unsafe fn registerPort_name(&self, port: &NSPort, name: &NSString) -> bool;
        #[method(removePortForName:)]
        pub unsafe fn removePortForName(&self, name: &NSString) -> bool;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSMachBootstrapServer;
    unsafe impl ClassType for NSMachBootstrapServer {
        type Super = NSPortNameServer;
    }
);
extern_methods!(
    unsafe impl NSMachBootstrapServer {
        #[method_id(sharedInstance)]
        pub unsafe fn sharedInstance() -> Id<Object, Shared>;
        #[method_id(portForName:)]
        pub unsafe fn portForName(&self, name: &NSString) -> Option<Id<NSPort, Shared>>;
        #[method_id(portForName:host:)]
        pub unsafe fn portForName_host(
            &self,
            name: &NSString,
            host: Option<&NSString>,
        ) -> Option<Id<NSPort, Shared>>;
        #[method(registerPort:name:)]
        pub unsafe fn registerPort_name(&self, port: &NSPort, name: &NSString) -> bool;
        #[method_id(servicePortWithName:)]
        pub unsafe fn servicePortWithName(&self, name: &NSString) -> Option<Id<NSPort, Shared>>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSMessagePortNameServer;
    unsafe impl ClassType for NSMessagePortNameServer {
        type Super = NSPortNameServer;
    }
);
extern_methods!(
    unsafe impl NSMessagePortNameServer {
        #[method_id(sharedInstance)]
        pub unsafe fn sharedInstance() -> Id<Object, Shared>;
        #[method_id(portForName:)]
        pub unsafe fn portForName(&self, name: &NSString) -> Option<Id<NSPort, Shared>>;
        #[method_id(portForName:host:)]
        pub unsafe fn portForName_host(
            &self,
            name: &NSString,
            host: Option<&NSString>,
        ) -> Option<Id<NSPort, Shared>>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSSocketPortNameServer;
    unsafe impl ClassType for NSSocketPortNameServer {
        type Super = NSPortNameServer;
    }
);
extern_methods!(
    unsafe impl NSSocketPortNameServer {
        #[method_id(sharedInstance)]
        pub unsafe fn sharedInstance() -> Id<Object, Shared>;
        #[method_id(portForName:)]
        pub unsafe fn portForName(&self, name: &NSString) -> Option<Id<NSPort, Shared>>;
        #[method_id(portForName:host:)]
        pub unsafe fn portForName_host(
            &self,
            name: &NSString,
            host: Option<&NSString>,
        ) -> Option<Id<NSPort, Shared>>;
        #[method(registerPort:name:)]
        pub unsafe fn registerPort_name(&self, port: &NSPort, name: &NSString) -> bool;
        #[method(removePortForName:)]
        pub unsafe fn removePortForName(&self, name: &NSString) -> bool;
        #[method_id(portForName:host:nameServerPortNumber:)]
        pub unsafe fn portForName_host_nameServerPortNumber(
            &self,
            name: &NSString,
            host: Option<&NSString>,
            portNumber: u16,
        ) -> Option<Id<NSPort, Shared>>;
        #[method(registerPort:name:nameServerPortNumber:)]
        pub unsafe fn registerPort_name_nameServerPortNumber(
            &self,
            port: &NSPort,
            name: &NSString,
            portNumber: u16,
        ) -> bool;
        #[method(defaultNameServerPortNumber)]
        pub unsafe fn defaultNameServerPortNumber(&self) -> u16;
        #[method(setDefaultNameServerPortNumber:)]
        pub unsafe fn setDefaultNameServerPortNumber(&self, defaultNameServerPortNumber: u16);
    }
);
