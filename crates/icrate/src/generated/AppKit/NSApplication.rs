//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

extern "C" {
    static NSAppKitVersionNumber: NSAppKitVersion;
}

static NSAppKitVersionNumber10_0: NSAppKitVersion = 577;

static NSAppKitVersionNumber10_1: NSAppKitVersion = 620;

static NSAppKitVersionNumber10_2: NSAppKitVersion = 663;

static NSAppKitVersionNumber10_2_3: NSAppKitVersion = 663.6;

static NSAppKitVersionNumber10_3: NSAppKitVersion = 743;

static NSAppKitVersionNumber10_3_2: NSAppKitVersion = 743.14;

static NSAppKitVersionNumber10_3_3: NSAppKitVersion = 743.2;

static NSAppKitVersionNumber10_3_5: NSAppKitVersion = 743.24;

static NSAppKitVersionNumber10_3_7: NSAppKitVersion = 743.33;

static NSAppKitVersionNumber10_3_9: NSAppKitVersion = 743.36;

static NSAppKitVersionNumber10_4: NSAppKitVersion = 824;

static NSAppKitVersionNumber10_4_1: NSAppKitVersion = 824.1;

static NSAppKitVersionNumber10_4_3: NSAppKitVersion = 824.23;

static NSAppKitVersionNumber10_4_4: NSAppKitVersion = 824.33;

static NSAppKitVersionNumber10_4_7: NSAppKitVersion = 824.41;

static NSAppKitVersionNumber10_5: NSAppKitVersion = 949;

static NSAppKitVersionNumber10_5_2: NSAppKitVersion = 949.27;

static NSAppKitVersionNumber10_5_3: NSAppKitVersion = 949.33;

static NSAppKitVersionNumber10_6: NSAppKitVersion = 1038;

static NSAppKitVersionNumber10_7: NSAppKitVersion = 1138;

static NSAppKitVersionNumber10_7_2: NSAppKitVersion = 1138.23;

static NSAppKitVersionNumber10_7_3: NSAppKitVersion = 1138.32;

static NSAppKitVersionNumber10_7_4: NSAppKitVersion = 1138.47;

static NSAppKitVersionNumber10_8: NSAppKitVersion = 1187;

static NSAppKitVersionNumber10_9: NSAppKitVersion = 1265;

static NSAppKitVersionNumber10_10: NSAppKitVersion = 1343;

static NSAppKitVersionNumber10_10_2: NSAppKitVersion = 1344;

static NSAppKitVersionNumber10_10_3: NSAppKitVersion = 1347;

static NSAppKitVersionNumber10_10_4: NSAppKitVersion = 1348;

static NSAppKitVersionNumber10_10_5: NSAppKitVersion = 1348;

static NSAppKitVersionNumber10_10_Max: NSAppKitVersion = 1349;

static NSAppKitVersionNumber10_11: NSAppKitVersion = 1404;

static NSAppKitVersionNumber10_11_1: NSAppKitVersion = 1404.13;

static NSAppKitVersionNumber10_11_2: NSAppKitVersion = 1404.34;

static NSAppKitVersionNumber10_11_3: NSAppKitVersion = 1404.34;

static NSAppKitVersionNumber10_12: NSAppKitVersion = 1504;

static NSAppKitVersionNumber10_12_1: NSAppKitVersion = 1504.60;

static NSAppKitVersionNumber10_12_2: NSAppKitVersion = 1504.76;

static NSAppKitVersionNumber10_13: NSAppKitVersion = 1561;

static NSAppKitVersionNumber10_13_1: NSAppKitVersion = 1561.1;

static NSAppKitVersionNumber10_13_2: NSAppKitVersion = 1561.2;

static NSAppKitVersionNumber10_13_4: NSAppKitVersion = 1561.4;

static NSAppKitVersionNumber10_14: NSAppKitVersion = 1671;

static NSAppKitVersionNumber10_14_1: NSAppKitVersion = 1671.1;

static NSAppKitVersionNumber10_14_2: NSAppKitVersion = 1671.2;

static NSAppKitVersionNumber10_14_3: NSAppKitVersion = 1671.3;

static NSAppKitVersionNumber10_14_4: NSAppKitVersion = 1671.4;

static NSAppKitVersionNumber10_14_5: NSAppKitVersion = 1671.5;

static NSAppKitVersionNumber10_15: NSAppKitVersion = 1894;

static NSAppKitVersionNumber10_15_1: NSAppKitVersion = 1894.1;

static NSAppKitVersionNumber10_15_2: NSAppKitVersion = 1894.2;

static NSAppKitVersionNumber10_15_3: NSAppKitVersion = 1894.3;

static NSAppKitVersionNumber10_15_4: NSAppKitVersion = 1894.4;

static NSAppKitVersionNumber10_15_5: NSAppKitVersion = 1894.5;

static NSAppKitVersionNumber10_15_6: NSAppKitVersion = 1894.6;

static NSAppKitVersionNumber11_0: NSAppKitVersion = 2022;

static NSAppKitVersionNumber11_1: NSAppKitVersion = 2022.2;

static NSAppKitVersionNumber11_2: NSAppKitVersion = 2022.3;

static NSAppKitVersionNumber11_3: NSAppKitVersion = 2022.4;

static NSAppKitVersionNumber11_4: NSAppKitVersion = 2022.5;

extern "C" {
    static NSModalPanelRunLoopMode: &'static NSRunLoopMode;
}

extern "C" {
    static NSEventTrackingRunLoopMode: &'static NSRunLoopMode;
}

pub type NSModalResponse = NSInteger;

static NSModalResponseStop: NSModalResponse = -1000;

static NSModalResponseAbort: NSModalResponse = -1001;

static NSModalResponseContinue: NSModalResponse = -1002;

pub const NSUpdateWindowsRunLoopOrdering: i32 = 500000;

pub type NSApplicationPresentationOptions = NSUInteger;
pub const NSApplicationPresentationDefault: NSApplicationPresentationOptions = 0;
pub const NSApplicationPresentationAutoHideDock: NSApplicationPresentationOptions = 1 << 0;
pub const NSApplicationPresentationHideDock: NSApplicationPresentationOptions = 1 << 1;
pub const NSApplicationPresentationAutoHideMenuBar: NSApplicationPresentationOptions = 1 << 2;
pub const NSApplicationPresentationHideMenuBar: NSApplicationPresentationOptions = 1 << 3;
pub const NSApplicationPresentationDisableAppleMenu: NSApplicationPresentationOptions = 1 << 4;
pub const NSApplicationPresentationDisableProcessSwitching: NSApplicationPresentationOptions =
    1 << 5;
pub const NSApplicationPresentationDisableForceQuit: NSApplicationPresentationOptions = 1 << 6;
pub const NSApplicationPresentationDisableSessionTermination: NSApplicationPresentationOptions =
    1 << 7;
pub const NSApplicationPresentationDisableHideApplication: NSApplicationPresentationOptions =
    1 << 8;
pub const NSApplicationPresentationDisableMenuBarTransparency: NSApplicationPresentationOptions =
    1 << 9;
pub const NSApplicationPresentationFullScreen: NSApplicationPresentationOptions = 1 << 10;
pub const NSApplicationPresentationAutoHideToolbar: NSApplicationPresentationOptions = 1 << 11;
pub const NSApplicationPresentationDisableCursorLocationAssistance:
    NSApplicationPresentationOptions = 1 << 12;

pub type NSApplicationOcclusionState = NSUInteger;
pub const NSApplicationOcclusionStateVisible: NSApplicationOcclusionState = 1 << 1;

pub type NSWindowListOptions = NSInteger;
pub const NSWindowListOrderedFrontToBack: NSWindowListOptions = 1 << 0;

extern "C" {
    static NSApp: Option<&'static NSApplication>;
}

pub type NSRequestUserAttentionType = NSUInteger;
pub const NSCriticalRequest: NSRequestUserAttentionType = 0;
pub const NSInformationalRequest: NSRequestUserAttentionType = 10;

pub type NSApplicationDelegateReply = NSUInteger;
pub const NSApplicationDelegateReplySuccess: NSApplicationDelegateReply = 0;
pub const NSApplicationDelegateReplyCancel: NSApplicationDelegateReply = 1;
pub const NSApplicationDelegateReplyFailure: NSApplicationDelegateReply = 2;

extern_class!(
    #[derive(Debug)]
    pub struct NSApplication;

    unsafe impl ClassType for NSApplication {
        type Super = NSResponder;
    }
);

extern_methods!(
    unsafe impl NSApplication {
        #[method_id(sharedApplication)]
        pub unsafe fn sharedApplication() -> Id<NSApplication, Shared>;

        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSApplicationDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSApplicationDelegate>);

        #[method(hide:)]
        pub unsafe fn hide(&self, sender: Option<&Object>);

        #[method(unhide:)]
        pub unsafe fn unhide(&self, sender: Option<&Object>);

        #[method(unhideWithoutActivation)]
        pub unsafe fn unhideWithoutActivation(&self);

        #[method_id(windowWithWindowNumber:)]
        pub unsafe fn windowWithWindowNumber(
            &self,
            windowNum: NSInteger,
        ) -> Option<Id<NSWindow, Shared>>;

        #[method_id(mainWindow)]
        pub unsafe fn mainWindow(&self) -> Option<Id<NSWindow, Shared>>;

        #[method_id(keyWindow)]
        pub unsafe fn keyWindow(&self) -> Option<Id<NSWindow, Shared>>;

        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;

        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        #[method(isRunning)]
        pub unsafe fn isRunning(&self) -> bool;

        #[method(deactivate)]
        pub unsafe fn deactivate(&self);

        #[method(activateIgnoringOtherApps:)]
        pub unsafe fn activateIgnoringOtherApps(&self, flag: bool);

        #[method(hideOtherApplications:)]
        pub unsafe fn hideOtherApplications(&self, sender: Option<&Object>);

        #[method(unhideAllApplications:)]
        pub unsafe fn unhideAllApplications(&self, sender: Option<&Object>);

        #[method(finishLaunching)]
        pub unsafe fn finishLaunching(&self);

        #[method(run)]
        pub unsafe fn run(&self);

        #[method(runModalForWindow:)]
        pub unsafe fn runModalForWindow(&self, window: &NSWindow) -> NSModalResponse;

        #[method(stop:)]
        pub unsafe fn stop(&self, sender: Option<&Object>);

        #[method(stopModal)]
        pub unsafe fn stopModal(&self);

        #[method(stopModalWithCode:)]
        pub unsafe fn stopModalWithCode(&self, returnCode: NSModalResponse);

        #[method(abortModal)]
        pub unsafe fn abortModal(&self);

        #[method_id(modalWindow)]
        pub unsafe fn modalWindow(&self) -> Option<Id<NSWindow, Shared>>;

        #[method(beginModalSessionForWindow:)]
        pub unsafe fn beginModalSessionForWindow(&self, window: &NSWindow) -> NSModalSession;

        #[method(runModalSession:)]
        pub unsafe fn runModalSession(&self, session: NSModalSession) -> NSModalResponse;

        #[method(endModalSession:)]
        pub unsafe fn endModalSession(&self, session: NSModalSession);

        #[method(terminate:)]
        pub unsafe fn terminate(&self, sender: Option<&Object>);

        #[method(requestUserAttention:)]
        pub unsafe fn requestUserAttention(
            &self,
            requestType: NSRequestUserAttentionType,
        ) -> NSInteger;

        #[method(cancelUserAttentionRequest:)]
        pub unsafe fn cancelUserAttentionRequest(&self, request: NSInteger);

        #[method(enumerateWindowsWithOptions:usingBlock:)]
        pub unsafe fn enumerateWindowsWithOptions_usingBlock(
            &self,
            options: NSWindowListOptions,
            block: TodoBlock,
        );

        #[method(preventWindowOrdering)]
        pub unsafe fn preventWindowOrdering(&self);

        #[method_id(windows)]
        pub unsafe fn windows(&self) -> Id<NSArray<NSWindow>, Shared>;

        #[method(setWindowsNeedUpdate:)]
        pub unsafe fn setWindowsNeedUpdate(&self, needUpdate: bool);

        #[method(updateWindows)]
        pub unsafe fn updateWindows(&self);

        #[method_id(mainMenu)]
        pub unsafe fn mainMenu(&self) -> Option<Id<NSMenu, Shared>>;

        #[method(setMainMenu:)]
        pub unsafe fn setMainMenu(&self, mainMenu: Option<&NSMenu>);

        #[method_id(helpMenu)]
        pub unsafe fn helpMenu(&self) -> Option<Id<NSMenu, Shared>>;

        #[method(setHelpMenu:)]
        pub unsafe fn setHelpMenu(&self, helpMenu: Option<&NSMenu>);

        #[method_id(applicationIconImage)]
        pub unsafe fn applicationIconImage(&self) -> Option<Id<NSImage, Shared>>;

        #[method(setApplicationIconImage:)]
        pub unsafe fn setApplicationIconImage(&self, applicationIconImage: Option<&NSImage>);

        #[method(activationPolicy)]
        pub unsafe fn activationPolicy(&self) -> NSApplicationActivationPolicy;

        #[method(setActivationPolicy:)]
        pub unsafe fn setActivationPolicy(
            &self,
            activationPolicy: NSApplicationActivationPolicy,
        ) -> bool;

        #[method_id(dockTile)]
        pub unsafe fn dockTile(&self) -> Id<NSDockTile, Shared>;

        #[method(reportException:)]
        pub unsafe fn reportException(&self, exception: &NSException);

        #[method(detachDrawingThread:toTarget:withObject:)]
        pub unsafe fn detachDrawingThread_toTarget_withObject(
            selector: Sel,
            target: &Object,
            argument: Option<&Object>,
        );

        #[method(replyToApplicationShouldTerminate:)]
        pub unsafe fn replyToApplicationShouldTerminate(&self, shouldTerminate: bool);

        #[method(replyToOpenOrPrint:)]
        pub unsafe fn replyToOpenOrPrint(&self, reply: NSApplicationDelegateReply);

        #[method(orderFrontCharacterPalette:)]
        pub unsafe fn orderFrontCharacterPalette(&self, sender: Option<&Object>);

        #[method(presentationOptions)]
        pub unsafe fn presentationOptions(&self) -> NSApplicationPresentationOptions;

        #[method(setPresentationOptions:)]
        pub unsafe fn setPresentationOptions(
            &self,
            presentationOptions: NSApplicationPresentationOptions,
        );

        #[method(currentSystemPresentationOptions)]
        pub unsafe fn currentSystemPresentationOptions(&self) -> NSApplicationPresentationOptions;

        #[method(occlusionState)]
        pub unsafe fn occlusionState(&self) -> NSApplicationOcclusionState;

        #[method(isProtectedDataAvailable)]
        pub unsafe fn isProtectedDataAvailable(&self) -> bool;
    }
);

extern_methods!(
    /// NSAppearanceCustomization
    unsafe impl NSApplication {
        #[method_id(appearance)]
        pub unsafe fn appearance(&self) -> Option<Id<NSAppearance, Shared>>;

        #[method(setAppearance:)]
        pub unsafe fn setAppearance(&self, appearance: Option<&NSAppearance>);

        #[method_id(effectiveAppearance)]
        pub unsafe fn effectiveAppearance(&self) -> Id<NSAppearance, Shared>;
    }
);

extern_methods!(
    /// NSEvent
    unsafe impl NSApplication {
        #[method(sendEvent:)]
        pub unsafe fn sendEvent(&self, event: &NSEvent);

        #[method(postEvent:atStart:)]
        pub unsafe fn postEvent_atStart(&self, event: &NSEvent, flag: bool);

        #[method_id(currentEvent)]
        pub unsafe fn currentEvent(&self) -> Option<Id<NSEvent, Shared>>;

        #[method_id(nextEventMatchingMask:untilDate:inMode:dequeue:)]
        pub unsafe fn nextEventMatchingMask_untilDate_inMode_dequeue(
            &self,
            mask: NSEventMask,
            expiration: Option<&NSDate>,
            mode: &NSRunLoopMode,
            deqFlag: bool,
        ) -> Option<Id<NSEvent, Shared>>;

        #[method(discardEventsMatchingMask:beforeEvent:)]
        pub unsafe fn discardEventsMatchingMask_beforeEvent(
            &self,
            mask: NSEventMask,
            lastEvent: Option<&NSEvent>,
        );
    }
);

extern_methods!(
    /// NSResponder
    unsafe impl NSApplication {
        #[method(sendAction:to:from:)]
        pub unsafe fn sendAction_to_from(
            &self,
            action: Sel,
            target: Option<&Object>,
            sender: Option<&Object>,
        ) -> bool;

        #[method_id(targetForAction:)]
        pub unsafe fn targetForAction(&self, action: Sel) -> Option<Id<Object, Shared>>;

        #[method_id(targetForAction:to:from:)]
        pub unsafe fn targetForAction_to_from(
            &self,
            action: Sel,
            target: Option<&Object>,
            sender: Option<&Object>,
        ) -> Option<Id<Object, Shared>>;

        #[method(tryToPerform:with:)]
        pub unsafe fn tryToPerform_with(&self, action: Sel, object: Option<&Object>) -> bool;

        #[method_id(validRequestorForSendType:returnType:)]
        pub unsafe fn validRequestorForSendType_returnType(
            &self,
            sendType: Option<&NSPasteboardType>,
            returnType: Option<&NSPasteboardType>,
        ) -> Option<Id<Object, Shared>>;
    }
);

extern_methods!(
    /// NSWindowsMenu
    unsafe impl NSApplication {
        #[method_id(windowsMenu)]
        pub unsafe fn windowsMenu(&self) -> Option<Id<NSMenu, Shared>>;

        #[method(setWindowsMenu:)]
        pub unsafe fn setWindowsMenu(&self, windowsMenu: Option<&NSMenu>);

        #[method(arrangeInFront:)]
        pub unsafe fn arrangeInFront(&self, sender: Option<&Object>);

        #[method(removeWindowsItem:)]
        pub unsafe fn removeWindowsItem(&self, win: &NSWindow);

        #[method(addWindowsItem:title:filename:)]
        pub unsafe fn addWindowsItem_title_filename(
            &self,
            win: &NSWindow,
            string: &NSString,
            isFilename: bool,
        );

        #[method(changeWindowsItem:title:filename:)]
        pub unsafe fn changeWindowsItem_title_filename(
            &self,
            win: &NSWindow,
            string: &NSString,
            isFilename: bool,
        );

        #[method(updateWindowsItem:)]
        pub unsafe fn updateWindowsItem(&self, win: &NSWindow);

        #[method(miniaturizeAll:)]
        pub unsafe fn miniaturizeAll(&self, sender: Option<&Object>);
    }
);

extern_methods!(
    /// NSFullKeyboardAccess
    unsafe impl NSApplication {
        #[method(isFullKeyboardAccessEnabled)]
        pub unsafe fn isFullKeyboardAccessEnabled(&self) -> bool;
    }
);

pub type NSApplicationTerminateReply = NSUInteger;
pub const NSTerminateCancel: NSApplicationTerminateReply = 0;
pub const NSTerminateNow: NSApplicationTerminateReply = 1;
pub const NSTerminateLater: NSApplicationTerminateReply = 2;

pub type NSApplicationPrintReply = NSUInteger;
pub const NSPrintingCancelled: NSApplicationPrintReply = 0;
pub const NSPrintingSuccess: NSApplicationPrintReply = 1;
pub const NSPrintingFailure: NSApplicationPrintReply = 3;
pub const NSPrintingReplyLater: NSApplicationPrintReply = 2;

pub type NSApplicationDelegate = NSObject;

extern_methods!(
    /// NSServicesMenu
    unsafe impl NSApplication {
        #[method_id(servicesMenu)]
        pub unsafe fn servicesMenu(&self) -> Option<Id<NSMenu, Shared>>;

        #[method(setServicesMenu:)]
        pub unsafe fn setServicesMenu(&self, servicesMenu: Option<&NSMenu>);

        #[method(registerServicesMenuSendTypes:returnTypes:)]
        pub unsafe fn registerServicesMenuSendTypes_returnTypes(
            &self,
            sendTypes: &NSArray<NSPasteboardType>,
            returnTypes: &NSArray<NSPasteboardType>,
        );
    }
);

pub type NSServicesMenuRequestor = NSObject;

extern_methods!(
    /// NSServicesHandling
    unsafe impl NSApplication {
        #[method_id(servicesProvider)]
        pub unsafe fn servicesProvider(&self) -> Option<Id<Object, Shared>>;

        #[method(setServicesProvider:)]
        pub unsafe fn setServicesProvider(&self, servicesProvider: Option<&Object>);
    }
);

pub type NSAboutPanelOptionKey = NSString;

extern "C" {
    static NSAboutPanelOptionCredits: &'static NSAboutPanelOptionKey;
}

extern "C" {
    static NSAboutPanelOptionApplicationName: &'static NSAboutPanelOptionKey;
}

extern "C" {
    static NSAboutPanelOptionApplicationIcon: &'static NSAboutPanelOptionKey;
}

extern "C" {
    static NSAboutPanelOptionVersion: &'static NSAboutPanelOptionKey;
}

extern "C" {
    static NSAboutPanelOptionApplicationVersion: &'static NSAboutPanelOptionKey;
}

extern_methods!(
    /// NSStandardAboutPanel
    unsafe impl NSApplication {
        #[method(orderFrontStandardAboutPanel:)]
        pub unsafe fn orderFrontStandardAboutPanel(&self, sender: Option<&Object>);

        #[method(orderFrontStandardAboutPanelWithOptions:)]
        pub unsafe fn orderFrontStandardAboutPanelWithOptions(
            &self,
            optionsDictionary: &NSDictionary<NSAboutPanelOptionKey, Object>,
        );
    }
);

extern_methods!(
    /// NSApplicationLayoutDirection
    unsafe impl NSApplication {
        #[method(userInterfaceLayoutDirection)]
        pub unsafe fn userInterfaceLayoutDirection(&self) -> NSUserInterfaceLayoutDirection;
    }
);

extern_methods!(
    /// NSRestorableUserInterface
    unsafe impl NSApplication {
        #[method(disableRelaunchOnLogin)]
        pub unsafe fn disableRelaunchOnLogin(&self);

        #[method(enableRelaunchOnLogin)]
        pub unsafe fn enableRelaunchOnLogin(&self);
    }
);

pub type NSRemoteNotificationType = NSUInteger;
pub const NSRemoteNotificationTypeNone: NSRemoteNotificationType = 0;
pub const NSRemoteNotificationTypeBadge: NSRemoteNotificationType = 1 << 0;
pub const NSRemoteNotificationTypeSound: NSRemoteNotificationType = 1 << 1;
pub const NSRemoteNotificationTypeAlert: NSRemoteNotificationType = 1 << 2;

extern_methods!(
    /// NSRemoteNotifications
    unsafe impl NSApplication {
        #[method(registerForRemoteNotifications)]
        pub unsafe fn registerForRemoteNotifications(&self);

        #[method(unregisterForRemoteNotifications)]
        pub unsafe fn unregisterForRemoteNotifications(&self);

        #[method(isRegisteredForRemoteNotifications)]
        pub unsafe fn isRegisteredForRemoteNotifications(&self) -> bool;

        #[method(registerForRemoteNotificationTypes:)]
        pub unsafe fn registerForRemoteNotificationTypes(&self, types: NSRemoteNotificationType);

        #[method(enabledRemoteNotificationTypes)]
        pub unsafe fn enabledRemoteNotificationTypes(&self) -> NSRemoteNotificationType;
    }
);

pub type NSServiceProviderName = NSString;

extern "C" {
    static NSApplicationDidBecomeActiveNotification: &'static NSNotificationName;
}

extern "C" {
    static NSApplicationDidHideNotification: &'static NSNotificationName;
}

extern "C" {
    static NSApplicationDidFinishLaunchingNotification: &'static NSNotificationName;
}

extern "C" {
    static NSApplicationDidResignActiveNotification: &'static NSNotificationName;
}

extern "C" {
    static NSApplicationDidUnhideNotification: &'static NSNotificationName;
}

extern "C" {
    static NSApplicationDidUpdateNotification: &'static NSNotificationName;
}

extern "C" {
    static NSApplicationWillBecomeActiveNotification: &'static NSNotificationName;
}

extern "C" {
    static NSApplicationWillHideNotification: &'static NSNotificationName;
}

extern "C" {
    static NSApplicationWillFinishLaunchingNotification: &'static NSNotificationName;
}

extern "C" {
    static NSApplicationWillResignActiveNotification: &'static NSNotificationName;
}

extern "C" {
    static NSApplicationWillUnhideNotification: &'static NSNotificationName;
}

extern "C" {
    static NSApplicationWillUpdateNotification: &'static NSNotificationName;
}

extern "C" {
    static NSApplicationWillTerminateNotification: &'static NSNotificationName;
}

extern "C" {
    static NSApplicationDidChangeScreenParametersNotification: &'static NSNotificationName;
}

extern "C" {
    static NSApplicationProtectedDataWillBecomeUnavailableNotification: &'static NSNotificationName;
}

extern "C" {
    static NSApplicationProtectedDataDidBecomeAvailableNotification: &'static NSNotificationName;
}

extern "C" {
    static NSApplicationLaunchIsDefaultLaunchKey: &'static NSString;
}

extern "C" {
    static NSApplicationLaunchUserNotificationKey: &'static NSString;
}

extern "C" {
    static NSApplicationLaunchRemoteNotificationKey: &'static NSString;
}

extern "C" {
    static NSApplicationDidChangeOcclusionStateNotification: &'static NSNotificationName;
}

pub const NSRunStoppedResponse: i32 = -1000;
pub const NSRunAbortedResponse: i32 = -1001;
pub const NSRunContinuesResponse: i32 = -1002;

extern_methods!(
    /// NSDeprecated
    unsafe impl NSApplication {
        #[method(runModalForWindow:relativeToWindow:)]
        pub unsafe fn runModalForWindow_relativeToWindow(
            &self,
            window: Option<&NSWindow>,
            docWindow: Option<&NSWindow>,
        ) -> NSInteger;

        #[method(beginModalSessionForWindow:relativeToWindow:)]
        pub unsafe fn beginModalSessionForWindow_relativeToWindow(
            &self,
            window: Option<&NSWindow>,
            docWindow: Option<&NSWindow>,
        ) -> NSModalSession;

        #[method(application:printFiles:)]
        pub unsafe fn application_printFiles(
            &self,
            sender: Option<&NSApplication>,
            filenames: Option<&NSArray<NSString>>,
        );

        #[method(beginSheet:modalForWindow:modalDelegate:didEndSelector:contextInfo:)]
        pub unsafe fn beginSheet_modalForWindow_modalDelegate_didEndSelector_contextInfo(
            &self,
            sheet: &NSWindow,
            docWindow: &NSWindow,
            modalDelegate: Option<&Object>,
            didEndSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[method(endSheet:)]
        pub unsafe fn endSheet(&self, sheet: &NSWindow);

        #[method(endSheet:returnCode:)]
        pub unsafe fn endSheet_returnCode(&self, sheet: &NSWindow, returnCode: NSInteger);

        #[method_id(makeWindowsPerform:inOrder:)]
        pub unsafe fn makeWindowsPerform_inOrder(
            &self,
            selector: Sel,
            flag: bool,
        ) -> Option<Id<NSWindow, Shared>>;

        #[method_id(context)]
        pub unsafe fn context(&self) -> Option<Id<NSGraphicsContext, Shared>>;
    }
);
