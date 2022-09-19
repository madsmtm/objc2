use super::__exported::NSArray;
use super::__exported::NSDictionary;
use super::__exported::NSString;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTask;
    unsafe impl ClassType for NSTask {
        type Super = NSObject;
    }
);
impl NSTask {
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn launchAndReturnError(&self, error: *mut *mut NSError) -> bool {
        msg_send![self, launchAndReturnError: error]
    }
    pub unsafe fn interrupt(&self) {
        msg_send![self, interrupt]
    }
    pub unsafe fn terminate(&self) {
        msg_send![self, terminate]
    }
    pub unsafe fn suspend(&self) -> bool {
        msg_send![self, suspend]
    }
    pub unsafe fn resume(&self) -> bool {
        msg_send![self, resume]
    }
    pub unsafe fn executableURL(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, executableURL]
    }
    pub unsafe fn setExecutableURL(&self, executableURL: Option<&NSURL>) {
        msg_send![self, setExecutableURL: executableURL]
    }
    pub unsafe fn arguments(&self) -> Option<Id<NSArray<NSString>, Shared>> {
        msg_send_id![self, arguments]
    }
    pub unsafe fn setArguments(&self, arguments: Option<&NSArray<NSString>>) {
        msg_send![self, setArguments: arguments]
    }
    pub unsafe fn environment(&self) -> Option<Id<NSDictionary<NSString, NSString>, Shared>> {
        msg_send_id![self, environment]
    }
    pub unsafe fn setEnvironment(&self, environment: Option<&NSDictionary<NSString, NSString>>) {
        msg_send![self, setEnvironment: environment]
    }
    pub unsafe fn currentDirectoryURL(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, currentDirectoryURL]
    }
    pub unsafe fn setCurrentDirectoryURL(&self, currentDirectoryURL: Option<&NSURL>) {
        msg_send![self, setCurrentDirectoryURL: currentDirectoryURL]
    }
    pub unsafe fn standardInput(&self) -> Option<Id<Object, Shared>> {
        msg_send_id![self, standardInput]
    }
    pub unsafe fn setStandardInput(&self, standardInput: Option<&Object>) {
        msg_send![self, setStandardInput: standardInput]
    }
    pub unsafe fn standardOutput(&self) -> Option<Id<Object, Shared>> {
        msg_send_id![self, standardOutput]
    }
    pub unsafe fn setStandardOutput(&self, standardOutput: Option<&Object>) {
        msg_send![self, setStandardOutput: standardOutput]
    }
    pub unsafe fn standardError(&self) -> Option<Id<Object, Shared>> {
        msg_send_id![self, standardError]
    }
    pub unsafe fn setStandardError(&self, standardError: Option<&Object>) {
        msg_send![self, setStandardError: standardError]
    }
    pub unsafe fn processIdentifier(&self) -> c_int {
        msg_send![self, processIdentifier]
    }
    pub unsafe fn isRunning(&self) -> bool {
        msg_send![self, isRunning]
    }
    pub unsafe fn terminationStatus(&self) -> c_int {
        msg_send![self, terminationStatus]
    }
    pub unsafe fn terminationReason(&self) -> NSTaskTerminationReason {
        msg_send![self, terminationReason]
    }
    pub unsafe fn terminationHandler(&self) -> TodoBlock {
        msg_send![self, terminationHandler]
    }
    pub unsafe fn setTerminationHandler(&self, terminationHandler: TodoBlock) {
        msg_send![self, setTerminationHandler: terminationHandler]
    }
    pub unsafe fn qualityOfService(&self) -> NSQualityOfService {
        msg_send![self, qualityOfService]
    }
    pub unsafe fn setQualityOfService(&self, qualityOfService: NSQualityOfService) {
        msg_send![self, setQualityOfService: qualityOfService]
    }
}
#[doc = "NSTaskConveniences"]
impl NSTask {
    pub unsafe fn launchedTaskWithExecutableURL_arguments_error_terminationHandler(
        url: &NSURL,
        arguments: &NSArray<NSString>,
        error: *mut *mut NSError,
        terminationHandler: TodoBlock,
    ) -> Option<Id<NSTask, Shared>> {
        msg_send_id![
            Self::class(),
            launchedTaskWithExecutableURL: url,
            arguments: arguments,
            error: error,
            terminationHandler: terminationHandler
        ]
    }
    pub unsafe fn waitUntilExit(&self) {
        msg_send![self, waitUntilExit]
    }
}
#[doc = "NSDeprecated"]
impl NSTask {
    pub unsafe fn launch(&self) {
        msg_send![self, launch]
    }
    pub unsafe fn launchedTaskWithLaunchPath_arguments(
        path: &NSString,
        arguments: &NSArray<NSString>,
    ) -> Id<NSTask, Shared> {
        msg_send_id![
            Self::class(),
            launchedTaskWithLaunchPath: path,
            arguments: arguments
        ]
    }
    pub unsafe fn launchPath(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, launchPath]
    }
    pub unsafe fn setLaunchPath(&self, launchPath: Option<&NSString>) {
        msg_send![self, setLaunchPath: launchPath]
    }
    pub unsafe fn currentDirectoryPath(&self) -> Id<NSString, Shared> {
        msg_send_id![self, currentDirectoryPath]
    }
    pub unsafe fn setCurrentDirectoryPath(&self, currentDirectoryPath: &NSString) {
        msg_send![self, setCurrentDirectoryPath: currentDirectoryPath]
    }
}
