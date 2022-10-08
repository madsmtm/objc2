use super::__exported::NSArray;
use super::__exported::NSDictionary;
use super::__exported::NSString;
use crate::Foundation::generated::NSDate::*;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSProcessInfo;
    unsafe impl ClassType for NSProcessInfo {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSProcessInfo {
        pub unsafe fn processInfo() -> Id<NSProcessInfo, Shared> {
            msg_send_id![Self::class(), processInfo]
        }
        pub unsafe fn environment(&self) -> Id<NSDictionary<NSString, NSString>, Shared> {
            msg_send_id![self, environment]
        }
        pub unsafe fn arguments(&self) -> Id<NSArray<NSString>, Shared> {
            msg_send_id![self, arguments]
        }
        pub unsafe fn hostName(&self) -> Id<NSString, Shared> {
            msg_send_id![self, hostName]
        }
        pub unsafe fn processName(&self) -> Id<NSString, Shared> {
            msg_send_id![self, processName]
        }
        pub unsafe fn setProcessName(&self, processName: &NSString) {
            msg_send![self, setProcessName: processName]
        }
        pub unsafe fn processIdentifier(&self) -> c_int {
            msg_send![self, processIdentifier]
        }
        pub unsafe fn globallyUniqueString(&self) -> Id<NSString, Shared> {
            msg_send_id![self, globallyUniqueString]
        }
        pub unsafe fn operatingSystem(&self) -> NSUInteger {
            msg_send![self, operatingSystem]
        }
        pub unsafe fn operatingSystemName(&self) -> Id<NSString, Shared> {
            msg_send_id![self, operatingSystemName]
        }
        pub unsafe fn operatingSystemVersionString(&self) -> Id<NSString, Shared> {
            msg_send_id![self, operatingSystemVersionString]
        }
        pub unsafe fn operatingSystemVersion(&self) -> NSOperatingSystemVersion {
            msg_send![self, operatingSystemVersion]
        }
        pub unsafe fn processorCount(&self) -> NSUInteger {
            msg_send![self, processorCount]
        }
        pub unsafe fn activeProcessorCount(&self) -> NSUInteger {
            msg_send![self, activeProcessorCount]
        }
        pub unsafe fn physicalMemory(&self) -> c_ulonglong {
            msg_send![self, physicalMemory]
        }
        pub unsafe fn isOperatingSystemAtLeastVersion(
            &self,
            version: NSOperatingSystemVersion,
        ) -> bool {
            msg_send![self, isOperatingSystemAtLeastVersion: version]
        }
        pub unsafe fn systemUptime(&self) -> NSTimeInterval {
            msg_send![self, systemUptime]
        }
        pub unsafe fn disableSuddenTermination(&self) {
            msg_send![self, disableSuddenTermination]
        }
        pub unsafe fn enableSuddenTermination(&self) {
            msg_send![self, enableSuddenTermination]
        }
        pub unsafe fn disableAutomaticTermination(&self, reason: &NSString) {
            msg_send![self, disableAutomaticTermination: reason]
        }
        pub unsafe fn enableAutomaticTermination(&self, reason: &NSString) {
            msg_send![self, enableAutomaticTermination: reason]
        }
        pub unsafe fn automaticTerminationSupportEnabled(&self) -> bool {
            msg_send![self, automaticTerminationSupportEnabled]
        }
        pub unsafe fn setAutomaticTerminationSupportEnabled(
            &self,
            automaticTerminationSupportEnabled: bool,
        ) {
            msg_send![
                self,
                setAutomaticTerminationSupportEnabled: automaticTerminationSupportEnabled
            ]
        }
    }
);
extern_methods!(
    #[doc = "NSProcessInfoActivity"]
    unsafe impl NSProcessInfo {
        pub unsafe fn beginActivityWithOptions_reason(
            &self,
            options: NSActivityOptions,
            reason: &NSString,
        ) -> Id<NSObject, Shared> {
            msg_send_id![self, beginActivityWithOptions: options, reason: reason]
        }
        pub unsafe fn endActivity(&self, activity: &NSObject) {
            msg_send![self, endActivity: activity]
        }
        pub unsafe fn performActivityWithOptions_reason_usingBlock(
            &self,
            options: NSActivityOptions,
            reason: &NSString,
            block: TodoBlock,
        ) {
            msg_send![
                self,
                performActivityWithOptions: options,
                reason: reason,
                usingBlock: block
            ]
        }
        pub unsafe fn performExpiringActivityWithReason_usingBlock(
            &self,
            reason: &NSString,
            block: TodoBlock,
        ) {
            msg_send![
                self,
                performExpiringActivityWithReason: reason,
                usingBlock: block
            ]
        }
    }
);
extern_methods!(
    #[doc = "NSUserInformation"]
    unsafe impl NSProcessInfo {
        pub unsafe fn userName(&self) -> Id<NSString, Shared> {
            msg_send_id![self, userName]
        }
        pub unsafe fn fullUserName(&self) -> Id<NSString, Shared> {
            msg_send_id![self, fullUserName]
        }
    }
);
extern_methods!(
    #[doc = "NSProcessInfoThermalState"]
    unsafe impl NSProcessInfo {
        pub unsafe fn thermalState(&self) -> NSProcessInfoThermalState {
            msg_send![self, thermalState]
        }
    }
);
extern_methods!(
    #[doc = "NSProcessInfoPowerState"]
    unsafe impl NSProcessInfo {
        pub unsafe fn isLowPowerModeEnabled(&self) -> bool {
            msg_send![self, isLowPowerModeEnabled]
        }
    }
);
extern_methods!(
    #[doc = "NSProcessInfoPlatform"]
    unsafe impl NSProcessInfo {
        pub unsafe fn isMacCatalystApp(&self) -> bool {
            msg_send![self, isMacCatalystApp]
        }
        pub unsafe fn isiOSAppOnMac(&self) -> bool {
            msg_send![self, isiOSAppOnMac]
        }
    }
);
