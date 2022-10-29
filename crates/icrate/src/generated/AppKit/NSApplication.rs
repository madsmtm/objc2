#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSModalResponse = NSInteger;
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
    #[doc = "NSAppearanceCustomization"]
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
    #[doc = "NSEvent"]
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
    #[doc = "NSResponder"]
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
    #[doc = "NSWindowsMenu"]
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
    #[doc = "NSFullKeyboardAccess"]
    unsafe impl NSApplication {
        #[method(isFullKeyboardAccessEnabled)]
        pub unsafe fn isFullKeyboardAccessEnabled(&self) -> bool;
    }
);
pub type NSApplicationDelegate = NSObject;
extern_methods!(
    #[doc = "NSServicesMenu"]
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
    #[doc = "NSServicesHandling"]
    unsafe impl NSApplication {
        #[method_id(servicesProvider)]
        pub unsafe fn servicesProvider(&self) -> Option<Id<Object, Shared>>;
        #[method(setServicesProvider:)]
        pub unsafe fn setServicesProvider(&self, servicesProvider: Option<&Object>);
    }
);
pub type NSAboutPanelOptionKey = NSString;
extern_methods!(
    #[doc = "NSStandardAboutPanel"]
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
    #[doc = "NSApplicationLayoutDirection"]
    unsafe impl NSApplication {
        #[method(userInterfaceLayoutDirection)]
        pub unsafe fn userInterfaceLayoutDirection(&self) -> NSUserInterfaceLayoutDirection;
    }
);
extern_methods!(
    #[doc = "NSRestorableUserInterface"]
    unsafe impl NSApplication {
        #[method(disableRelaunchOnLogin)]
        pub unsafe fn disableRelaunchOnLogin(&self);
        #[method(enableRelaunchOnLogin)]
        pub unsafe fn enableRelaunchOnLogin(&self);
    }
);
extern_methods!(
    #[doc = "NSRemoteNotifications"]
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
extern_methods!(
    #[doc = "NSDeprecated"]
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