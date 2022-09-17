extern_class!(
    #[derive(Debug)]
    struct NSPortNameServer;
    unsafe impl ClassType for NSPortNameServer {
        type Super = NSObject;
    }
);
impl NSPortNameServer {
    pub unsafe fn systemDefaultPortNameServer() -> Id<NSPortNameServer, Shared> {
        msg_send_id![Self::class(), systemDefaultPortNameServer]
    }
    pub unsafe fn portForName(&self, name: &NSString) -> Option<Id<NSPort, Shared>> {
        msg_send_id![self, portForName: name]
    }
    pub unsafe fn portForName_host(
        &self,
        name: &NSString,
        host: Option<&NSString>,
    ) -> Option<Id<NSPort, Shared>> {
        msg_send_id![self, portForName: name, host: host]
    }
    pub unsafe fn registerPort_name(&self, port: &NSPort, name: &NSString) -> bool {
        msg_send![self, registerPort: port, name: name]
    }
    pub unsafe fn removePortForName(&self, name: &NSString) -> bool {
        msg_send![self, removePortForName: name]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSMachBootstrapServer;
    unsafe impl ClassType for NSMachBootstrapServer {
        type Super = NSPortNameServer;
    }
);
impl NSMachBootstrapServer {
    pub unsafe fn sharedInstance() -> Id<Object, Shared> {
        msg_send_id![Self::class(), sharedInstance]
    }
    pub unsafe fn portForName(&self, name: &NSString) -> Option<Id<NSPort, Shared>> {
        msg_send_id![self, portForName: name]
    }
    pub unsafe fn portForName_host(
        &self,
        name: &NSString,
        host: Option<&NSString>,
    ) -> Option<Id<NSPort, Shared>> {
        msg_send_id![self, portForName: name, host: host]
    }
    pub unsafe fn registerPort_name(&self, port: &NSPort, name: &NSString) -> bool {
        msg_send![self, registerPort: port, name: name]
    }
    pub unsafe fn servicePortWithName(&self, name: &NSString) -> Option<Id<NSPort, Shared>> {
        msg_send_id![self, servicePortWithName: name]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSMessagePortNameServer;
    unsafe impl ClassType for NSMessagePortNameServer {
        type Super = NSPortNameServer;
    }
);
impl NSMessagePortNameServer {
    pub unsafe fn sharedInstance() -> Id<Object, Shared> {
        msg_send_id![Self::class(), sharedInstance]
    }
    pub unsafe fn portForName(&self, name: &NSString) -> Option<Id<NSPort, Shared>> {
        msg_send_id![self, portForName: name]
    }
    pub unsafe fn portForName_host(
        &self,
        name: &NSString,
        host: Option<&NSString>,
    ) -> Option<Id<NSPort, Shared>> {
        msg_send_id![self, portForName: name, host: host]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSSocketPortNameServer;
    unsafe impl ClassType for NSSocketPortNameServer {
        type Super = NSPortNameServer;
    }
);
impl NSSocketPortNameServer {
    pub unsafe fn sharedInstance() -> Id<Object, Shared> {
        msg_send_id![Self::class(), sharedInstance]
    }
    pub unsafe fn portForName(&self, name: &NSString) -> Option<Id<NSPort, Shared>> {
        msg_send_id![self, portForName: name]
    }
    pub unsafe fn portForName_host(
        &self,
        name: &NSString,
        host: Option<&NSString>,
    ) -> Option<Id<NSPort, Shared>> {
        msg_send_id![self, portForName: name, host: host]
    }
    pub unsafe fn registerPort_name(&self, port: &NSPort, name: &NSString) -> bool {
        msg_send![self, registerPort: port, name: name]
    }
    pub unsafe fn removePortForName(&self, name: &NSString) -> bool {
        msg_send![self, removePortForName: name]
    }
    pub unsafe fn portForName_host_nameServerPortNumber(
        &self,
        name: &NSString,
        host: Option<&NSString>,
        portNumber: uint16_t,
    ) -> Option<Id<NSPort, Shared>> {
        msg_send_id![
            self,
            portForName: name,
            host: host,
            nameServerPortNumber: portNumber
        ]
    }
    pub unsafe fn registerPort_name_nameServerPortNumber(
        &self,
        port: &NSPort,
        name: &NSString,
        portNumber: uint16_t,
    ) -> bool {
        msg_send![
            self,
            registerPort: port,
            name: name,
            nameServerPortNumber: portNumber
        ]
    }
    pub unsafe fn defaultNameServerPortNumber(&self) -> uint16_t {
        msg_send![self, defaultNameServerPortNumber]
    }
    pub unsafe fn setDefaultNameServerPortNumber(&self, defaultNameServerPortNumber: uint16_t) {
        msg_send![
            self,
            setDefaultNameServerPortNumber: defaultNameServerPortNumber
        ]
    }
}
