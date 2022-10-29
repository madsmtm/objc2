#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSRunningApplication;
    unsafe impl ClassType for NSRunningApplication {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSRunningApplication {
        #[method(isTerminated)]
        pub unsafe fn isTerminated(&self) -> bool;
        #[method(isFinishedLaunching)]
        pub unsafe fn isFinishedLaunching(&self) -> bool;
        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;
        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;
        #[method(ownsMenuBar)]
        pub unsafe fn ownsMenuBar(&self) -> bool;
        #[method(activationPolicy)]
        pub unsafe fn activationPolicy(&self) -> NSApplicationActivationPolicy;
        #[method_id(localizedName)]
        pub unsafe fn localizedName(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(bundleIdentifier)]
        pub unsafe fn bundleIdentifier(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(bundleURL)]
        pub unsafe fn bundleURL(&self) -> Option<Id<NSURL, Shared>>;
        #[method_id(executableURL)]
        pub unsafe fn executableURL(&self) -> Option<Id<NSURL, Shared>>;
        #[method(processIdentifier)]
        pub unsafe fn processIdentifier(&self) -> pid_t;
        #[method_id(launchDate)]
        pub unsafe fn launchDate(&self) -> Option<Id<NSDate, Shared>>;
        #[method_id(icon)]
        pub unsafe fn icon(&self) -> Option<Id<NSImage, Shared>>;
        #[method(executableArchitecture)]
        pub unsafe fn executableArchitecture(&self) -> NSInteger;
        #[method(hide)]
        pub unsafe fn hide(&self) -> bool;
        #[method(unhide)]
        pub unsafe fn unhide(&self) -> bool;
        #[method(activateWithOptions:)]
        pub unsafe fn activateWithOptions(&self, options: NSApplicationActivationOptions) -> bool;
        #[method(terminate)]
        pub unsafe fn terminate(&self) -> bool;
        #[method(forceTerminate)]
        pub unsafe fn forceTerminate(&self) -> bool;
        #[method_id(runningApplicationsWithBundleIdentifier:)]
        pub unsafe fn runningApplicationsWithBundleIdentifier(
            bundleIdentifier: &NSString,
        ) -> Id<NSArray<NSRunningApplication>, Shared>;
        #[method_id(runningApplicationWithProcessIdentifier:)]
        pub unsafe fn runningApplicationWithProcessIdentifier(
            pid: pid_t,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(currentApplication)]
        pub unsafe fn currentApplication() -> Id<NSRunningApplication, Shared>;
        #[method(terminateAutomaticallyTerminableApplications)]
        pub unsafe fn terminateAutomaticallyTerminableApplications();
    }
);
extern_methods!(
    #[doc = "NSWorkspaceRunningApplications"]
    unsafe impl NSWorkspace {
        #[method_id(runningApplications)]
        pub unsafe fn runningApplications(&self) -> Id<NSArray<NSRunningApplication>, Shared>;
    }
);