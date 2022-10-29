#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSProcessInfo;
    unsafe impl ClassType for NSProcessInfo {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSProcessInfo {
        #[method_id(processInfo)]
        pub unsafe fn processInfo() -> Id<NSProcessInfo, Shared>;
        #[method_id(environment)]
        pub unsafe fn environment(&self) -> Id<NSDictionary<NSString, NSString>, Shared>;
        #[method_id(arguments)]
        pub unsafe fn arguments(&self) -> Id<NSArray<NSString>, Shared>;
        #[method_id(hostName)]
        pub unsafe fn hostName(&self) -> Id<NSString, Shared>;
        #[method_id(processName)]
        pub unsafe fn processName(&self) -> Id<NSString, Shared>;
        #[method(setProcessName:)]
        pub unsafe fn setProcessName(&self, processName: &NSString);
        #[method(processIdentifier)]
        pub unsafe fn processIdentifier(&self) -> c_int;
        #[method_id(globallyUniqueString)]
        pub unsafe fn globallyUniqueString(&self) -> Id<NSString, Shared>;
        #[method(operatingSystem)]
        pub unsafe fn operatingSystem(&self) -> NSUInteger;
        #[method_id(operatingSystemName)]
        pub unsafe fn operatingSystemName(&self) -> Id<NSString, Shared>;
        #[method_id(operatingSystemVersionString)]
        pub unsafe fn operatingSystemVersionString(&self) -> Id<NSString, Shared>;
        #[method(operatingSystemVersion)]
        pub unsafe fn operatingSystemVersion(&self) -> NSOperatingSystemVersion;
        #[method(processorCount)]
        pub unsafe fn processorCount(&self) -> NSUInteger;
        #[method(activeProcessorCount)]
        pub unsafe fn activeProcessorCount(&self) -> NSUInteger;
        #[method(physicalMemory)]
        pub unsafe fn physicalMemory(&self) -> c_ulonglong;
        #[method(isOperatingSystemAtLeastVersion:)]
        pub unsafe fn isOperatingSystemAtLeastVersion(
            &self,
            version: NSOperatingSystemVersion,
        ) -> bool;
        #[method(systemUptime)]
        pub unsafe fn systemUptime(&self) -> NSTimeInterval;
        #[method(disableSuddenTermination)]
        pub unsafe fn disableSuddenTermination(&self);
        #[method(enableSuddenTermination)]
        pub unsafe fn enableSuddenTermination(&self);
        #[method(disableAutomaticTermination:)]
        pub unsafe fn disableAutomaticTermination(&self, reason: &NSString);
        #[method(enableAutomaticTermination:)]
        pub unsafe fn enableAutomaticTermination(&self, reason: &NSString);
        #[method(automaticTerminationSupportEnabled)]
        pub unsafe fn automaticTerminationSupportEnabled(&self) -> bool;
        #[method(setAutomaticTerminationSupportEnabled:)]
        pub unsafe fn setAutomaticTerminationSupportEnabled(
            &self,
            automaticTerminationSupportEnabled: bool,
        );
    }
);
extern_methods!(
    #[doc = "NSProcessInfoActivity"]
    unsafe impl NSProcessInfo {
        #[method_id(beginActivityWithOptions:reason:)]
        pub unsafe fn beginActivityWithOptions_reason(
            &self,
            options: NSActivityOptions,
            reason: &NSString,
        ) -> Id<NSObject, Shared>;
        #[method(endActivity:)]
        pub unsafe fn endActivity(&self, activity: &NSObject);
        #[method(performActivityWithOptions:reason:usingBlock:)]
        pub unsafe fn performActivityWithOptions_reason_usingBlock(
            &self,
            options: NSActivityOptions,
            reason: &NSString,
            block: TodoBlock,
        );
        #[method(performExpiringActivityWithReason:usingBlock:)]
        pub unsafe fn performExpiringActivityWithReason_usingBlock(
            &self,
            reason: &NSString,
            block: TodoBlock,
        );
    }
);
extern_methods!(
    #[doc = "NSUserInformation"]
    unsafe impl NSProcessInfo {
        #[method_id(userName)]
        pub unsafe fn userName(&self) -> Id<NSString, Shared>;
        #[method_id(fullUserName)]
        pub unsafe fn fullUserName(&self) -> Id<NSString, Shared>;
    }
);
extern_methods!(
    #[doc = "NSProcessInfoThermalState"]
    unsafe impl NSProcessInfo {
        #[method(thermalState)]
        pub unsafe fn thermalState(&self) -> NSProcessInfoThermalState;
    }
);
extern_methods!(
    #[doc = "NSProcessInfoPowerState"]
    unsafe impl NSProcessInfo {
        #[method(isLowPowerModeEnabled)]
        pub unsafe fn isLowPowerModeEnabled(&self) -> bool;
    }
);
extern_methods!(
    #[doc = "NSProcessInfoPlatform"]
    unsafe impl NSProcessInfo {
        #[method(isMacCatalystApp)]
        pub unsafe fn isMacCatalystApp(&self) -> bool;
        #[method(isiOSAppOnMac)]
        pub unsafe fn isiOSAppOnMac(&self) -> bool;
    }
);
