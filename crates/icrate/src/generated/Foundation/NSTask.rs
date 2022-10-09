use super::__exported::NSArray;
use super::__exported::NSDictionary;
use super::__exported::NSString;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTask;
    unsafe impl ClassType for NSTask {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTask {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(executableURL)]
        pub unsafe fn executableURL(&self) -> Option<Id<NSURL, Shared>>;
        #[method(setExecutableURL:)]
        pub unsafe fn setExecutableURL(&self, executableURL: Option<&NSURL>);
        #[method_id(arguments)]
        pub unsafe fn arguments(&self) -> Option<Id<NSArray<NSString>, Shared>>;
        #[method(setArguments:)]
        pub unsafe fn setArguments(&self, arguments: Option<&NSArray<NSString>>);
        #[method_id(environment)]
        pub unsafe fn environment(&self) -> Option<Id<NSDictionary<NSString, NSString>, Shared>>;
        #[method(setEnvironment:)]
        pub unsafe fn setEnvironment(&self, environment: Option<&NSDictionary<NSString, NSString>>);
        #[method_id(currentDirectoryURL)]
        pub unsafe fn currentDirectoryURL(&self) -> Option<Id<NSURL, Shared>>;
        #[method(setCurrentDirectoryURL:)]
        pub unsafe fn setCurrentDirectoryURL(&self, currentDirectoryURL: Option<&NSURL>);
        #[method_id(standardInput)]
        pub unsafe fn standardInput(&self) -> Option<Id<Object, Shared>>;
        #[method(setStandardInput:)]
        pub unsafe fn setStandardInput(&self, standardInput: Option<&Object>);
        #[method_id(standardOutput)]
        pub unsafe fn standardOutput(&self) -> Option<Id<Object, Shared>>;
        #[method(setStandardOutput:)]
        pub unsafe fn setStandardOutput(&self, standardOutput: Option<&Object>);
        #[method_id(standardError)]
        pub unsafe fn standardError(&self) -> Option<Id<Object, Shared>>;
        #[method(setStandardError:)]
        pub unsafe fn setStandardError(&self, standardError: Option<&Object>);
        #[method(launchAndReturnError:)]
        pub unsafe fn launchAndReturnError(&self) -> Result<(), Id<NSError, Shared>>;
        #[method(interrupt)]
        pub unsafe fn interrupt(&self);
        #[method(terminate)]
        pub unsafe fn terminate(&self);
        #[method(suspend)]
        pub unsafe fn suspend(&self) -> bool;
        #[method(resume)]
        pub unsafe fn resume(&self) -> bool;
        #[method(processIdentifier)]
        pub unsafe fn processIdentifier(&self) -> c_int;
        #[method(isRunning)]
        pub unsafe fn isRunning(&self) -> bool;
        #[method(terminationStatus)]
        pub unsafe fn terminationStatus(&self) -> c_int;
        #[method(terminationReason)]
        pub unsafe fn terminationReason(&self) -> NSTaskTerminationReason;
        #[method(terminationHandler)]
        pub unsafe fn terminationHandler(&self) -> TodoBlock;
        #[method(setTerminationHandler:)]
        pub unsafe fn setTerminationHandler(&self, terminationHandler: TodoBlock);
        #[method(qualityOfService)]
        pub unsafe fn qualityOfService(&self) -> NSQualityOfService;
        #[method(setQualityOfService:)]
        pub unsafe fn setQualityOfService(&self, qualityOfService: NSQualityOfService);
    }
);
extern_methods!(
    #[doc = "NSTaskConveniences"]
    unsafe impl NSTask {
        #[method_id(launchedTaskWithExecutableURL:arguments:error:terminationHandler:)]
        pub unsafe fn launchedTaskWithExecutableURL_arguments_error_terminationHandler(
            url: &NSURL,
            arguments: &NSArray<NSString>,
            error: *mut *mut NSError,
            terminationHandler: TodoBlock,
        ) -> Option<Id<NSTask, Shared>>;
        #[method(waitUntilExit)]
        pub unsafe fn waitUntilExit(&self);
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSTask {
        #[method_id(launchPath)]
        pub unsafe fn launchPath(&self) -> Option<Id<NSString, Shared>>;
        #[method(setLaunchPath:)]
        pub unsafe fn setLaunchPath(&self, launchPath: Option<&NSString>);
        #[method_id(currentDirectoryPath)]
        pub unsafe fn currentDirectoryPath(&self) -> Id<NSString, Shared>;
        #[method(setCurrentDirectoryPath:)]
        pub unsafe fn setCurrentDirectoryPath(&self, currentDirectoryPath: &NSString);
        #[method(launch)]
        pub unsafe fn launch(&self);
        #[method_id(launchedTaskWithLaunchPath:arguments:)]
        pub unsafe fn launchedTaskWithLaunchPath_arguments(
            path: &NSString,
            arguments: &NSArray<NSString>,
        ) -> Id<NSTask, Shared>;
    }
);
