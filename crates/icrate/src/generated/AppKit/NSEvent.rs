#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSEvent;
    unsafe impl ClassType for NSEvent {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSEvent {
        #[method(type)]
        pub unsafe fn type_(&self) -> NSEventType;
        #[method(modifierFlags)]
        pub unsafe fn modifierFlags(&self) -> NSEventModifierFlags;
        #[method(timestamp)]
        pub unsafe fn timestamp(&self) -> NSTimeInterval;
        #[method_id(window)]
        pub unsafe fn window(&self) -> Option<Id<NSWindow, Shared>>;
        #[method(windowNumber)]
        pub unsafe fn windowNumber(&self) -> NSInteger;
        #[method_id(context)]
        pub unsafe fn context(&self) -> Option<Id<NSGraphicsContext, Shared>>;
        #[method(clickCount)]
        pub unsafe fn clickCount(&self) -> NSInteger;
        #[method(buttonNumber)]
        pub unsafe fn buttonNumber(&self) -> NSInteger;
        #[method(eventNumber)]
        pub unsafe fn eventNumber(&self) -> NSInteger;
        #[method(pressure)]
        pub unsafe fn pressure(&self) -> c_float;
        #[method(locationInWindow)]
        pub unsafe fn locationInWindow(&self) -> NSPoint;
        #[method(deltaX)]
        pub unsafe fn deltaX(&self) -> CGFloat;
        #[method(deltaY)]
        pub unsafe fn deltaY(&self) -> CGFloat;
        #[method(deltaZ)]
        pub unsafe fn deltaZ(&self) -> CGFloat;
        #[method(hasPreciseScrollingDeltas)]
        pub unsafe fn hasPreciseScrollingDeltas(&self) -> bool;
        #[method(scrollingDeltaX)]
        pub unsafe fn scrollingDeltaX(&self) -> CGFloat;
        #[method(scrollingDeltaY)]
        pub unsafe fn scrollingDeltaY(&self) -> CGFloat;
        #[method(momentumPhase)]
        pub unsafe fn momentumPhase(&self) -> NSEventPhase;
        #[method(isDirectionInvertedFromDevice)]
        pub unsafe fn isDirectionInvertedFromDevice(&self) -> bool;
        #[method_id(characters)]
        pub unsafe fn characters(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(charactersIgnoringModifiers)]
        pub unsafe fn charactersIgnoringModifiers(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(charactersByApplyingModifiers:)]
        pub unsafe fn charactersByApplyingModifiers(
            &self,
            modifiers: NSEventModifierFlags,
        ) -> Option<Id<NSString, Shared>>;
        #[method(isARepeat)]
        pub unsafe fn isARepeat(&self) -> bool;
        #[method(keyCode)]
        pub unsafe fn keyCode(&self) -> c_ushort;
        #[method(trackingNumber)]
        pub unsafe fn trackingNumber(&self) -> NSInteger;
        #[method(userData)]
        pub unsafe fn userData(&self) -> *mut c_void;
        #[method_id(trackingArea)]
        pub unsafe fn trackingArea(&self) -> Option<Id<NSTrackingArea, Shared>>;
        #[method(subtype)]
        pub unsafe fn subtype(&self) -> NSEventSubtype;
        #[method(data1)]
        pub unsafe fn data1(&self) -> NSInteger;
        #[method(data2)]
        pub unsafe fn data2(&self) -> NSInteger;
        #[method(eventRef)]
        pub unsafe fn eventRef(&self) -> *mut c_void;
        #[method_id(eventWithEventRef:)]
        pub unsafe fn eventWithEventRef(eventRef: NonNull<c_void>) -> Option<Id<NSEvent, Shared>>;
        #[method(CGEvent)]
        pub unsafe fn CGEvent(&self) -> CGEventRef;
        #[method_id(eventWithCGEvent:)]
        pub unsafe fn eventWithCGEvent(cgEvent: CGEventRef) -> Option<Id<NSEvent, Shared>>;
        #[method(isMouseCoalescingEnabled)]
        pub unsafe fn isMouseCoalescingEnabled() -> bool;
        #[method(setMouseCoalescingEnabled:)]
        pub unsafe fn setMouseCoalescingEnabled(mouseCoalescingEnabled: bool);
        #[method(magnification)]
        pub unsafe fn magnification(&self) -> CGFloat;
        #[method(deviceID)]
        pub unsafe fn deviceID(&self) -> NSUInteger;
        #[method(rotation)]
        pub unsafe fn rotation(&self) -> c_float;
        #[method(absoluteX)]
        pub unsafe fn absoluteX(&self) -> NSInteger;
        #[method(absoluteY)]
        pub unsafe fn absoluteY(&self) -> NSInteger;
        #[method(absoluteZ)]
        pub unsafe fn absoluteZ(&self) -> NSInteger;
        #[method(buttonMask)]
        pub unsafe fn buttonMask(&self) -> NSEventButtonMask;
        #[method(tilt)]
        pub unsafe fn tilt(&self) -> NSPoint;
        #[method(tangentialPressure)]
        pub unsafe fn tangentialPressure(&self) -> c_float;
        #[method_id(vendorDefined)]
        pub unsafe fn vendorDefined(&self) -> Id<Object, Shared>;
        #[method(vendorID)]
        pub unsafe fn vendorID(&self) -> NSUInteger;
        #[method(tabletID)]
        pub unsafe fn tabletID(&self) -> NSUInteger;
        #[method(pointingDeviceID)]
        pub unsafe fn pointingDeviceID(&self) -> NSUInteger;
        #[method(systemTabletID)]
        pub unsafe fn systemTabletID(&self) -> NSUInteger;
        #[method(vendorPointingDeviceType)]
        pub unsafe fn vendorPointingDeviceType(&self) -> NSUInteger;
        #[method(pointingDeviceSerialNumber)]
        pub unsafe fn pointingDeviceSerialNumber(&self) -> NSUInteger;
        #[method(uniqueID)]
        pub unsafe fn uniqueID(&self) -> c_ulonglong;
        #[method(capabilityMask)]
        pub unsafe fn capabilityMask(&self) -> NSUInteger;
        #[method(pointingDeviceType)]
        pub unsafe fn pointingDeviceType(&self) -> NSPointingDeviceType;
        #[method(isEnteringProximity)]
        pub unsafe fn isEnteringProximity(&self) -> bool;
        #[method_id(touchesMatchingPhase:inView:)]
        pub unsafe fn touchesMatchingPhase_inView(
            &self,
            phase: NSTouchPhase,
            view: Option<&NSView>,
        ) -> Id<NSSet<NSTouch>, Shared>;
        #[method_id(allTouches)]
        pub unsafe fn allTouches(&self) -> Id<NSSet<NSTouch>, Shared>;
        #[method_id(touchesForView:)]
        pub unsafe fn touchesForView(&self, view: &NSView) -> Id<NSSet<NSTouch>, Shared>;
        #[method_id(coalescedTouchesForTouch:)]
        pub unsafe fn coalescedTouchesForTouch(
            &self,
            touch: &NSTouch,
        ) -> Id<NSArray<NSTouch>, Shared>;
        #[method(phase)]
        pub unsafe fn phase(&self) -> NSEventPhase;
        #[method(stage)]
        pub unsafe fn stage(&self) -> NSInteger;
        #[method(stageTransition)]
        pub unsafe fn stageTransition(&self) -> CGFloat;
        #[method(associatedEventsMask)]
        pub unsafe fn associatedEventsMask(&self) -> NSEventMask;
        #[method(pressureBehavior)]
        pub unsafe fn pressureBehavior(&self) -> NSPressureBehavior;
        #[method(isSwipeTrackingFromScrollEventsEnabled)]
        pub unsafe fn isSwipeTrackingFromScrollEventsEnabled() -> bool;
        #[method(trackSwipeEventWithOptions:dampenAmountThresholdMin:max:usingHandler:)]
        pub unsafe fn trackSwipeEventWithOptions_dampenAmountThresholdMin_max_usingHandler(
            &self,
            options: NSEventSwipeTrackingOptions,
            minDampenThreshold: CGFloat,
            maxDampenThreshold: CGFloat,
            trackingHandler: TodoBlock,
        );
        #[method(startPeriodicEventsAfterDelay:withPeriod:)]
        pub unsafe fn startPeriodicEventsAfterDelay_withPeriod(
            delay: NSTimeInterval,
            period: NSTimeInterval,
        );
        #[method(stopPeriodicEvents)]
        pub unsafe fn stopPeriodicEvents();
        #[method_id(mouseEventWithType:location:modifierFlags:timestamp:windowNumber:context:eventNumber:clickCount:pressure:)]
        pub unsafe fn mouseEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_clickCount_pressure(
            type_: NSEventType,
            location: NSPoint,
            flags: NSEventModifierFlags,
            time: NSTimeInterval,
            wNum: NSInteger,
            unusedPassNil: Option<&NSGraphicsContext>,
            eNum: NSInteger,
            cNum: NSInteger,
            pressure: c_float,
        ) -> Option<Id<NSEvent, Shared>>;
        #[method_id(keyEventWithType:location:modifierFlags:timestamp:windowNumber:context:characters:charactersIgnoringModifiers:isARepeat:keyCode:)]
        pub unsafe fn keyEventWithType_location_modifierFlags_timestamp_windowNumber_context_characters_charactersIgnoringModifiers_isARepeat_keyCode(
            type_: NSEventType,
            location: NSPoint,
            flags: NSEventModifierFlags,
            time: NSTimeInterval,
            wNum: NSInteger,
            unusedPassNil: Option<&NSGraphicsContext>,
            keys: &NSString,
            ukeys: &NSString,
            flag: bool,
            code: c_ushort,
        ) -> Option<Id<NSEvent, Shared>>;
        #[method_id(enterExitEventWithType:location:modifierFlags:timestamp:windowNumber:context:eventNumber:trackingNumber:userData:)]
        pub unsafe fn enterExitEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_trackingNumber_userData(
            type_: NSEventType,
            location: NSPoint,
            flags: NSEventModifierFlags,
            time: NSTimeInterval,
            wNum: NSInteger,
            unusedPassNil: Option<&NSGraphicsContext>,
            eNum: NSInteger,
            tNum: NSInteger,
            data: *mut c_void,
        ) -> Option<Id<NSEvent, Shared>>;
        #[method_id(otherEventWithType:location:modifierFlags:timestamp:windowNumber:context:subtype:data1:data2:)]
        pub unsafe fn otherEventWithType_location_modifierFlags_timestamp_windowNumber_context_subtype_data1_data2(
            type_: NSEventType,
            location: NSPoint,
            flags: NSEventModifierFlags,
            time: NSTimeInterval,
            wNum: NSInteger,
            unusedPassNil: Option<&NSGraphicsContext>,
            subtype: c_short,
            d1: NSInteger,
            d2: NSInteger,
        ) -> Option<Id<NSEvent, Shared>>;
        #[method(mouseLocation)]
        pub unsafe fn mouseLocation() -> NSPoint;
        #[method(modifierFlags)]
        pub unsafe fn modifierFlags() -> NSEventModifierFlags;
        #[method(pressedMouseButtons)]
        pub unsafe fn pressedMouseButtons() -> NSUInteger;
        #[method(doubleClickInterval)]
        pub unsafe fn doubleClickInterval() -> NSTimeInterval;
        #[method(keyRepeatDelay)]
        pub unsafe fn keyRepeatDelay() -> NSTimeInterval;
        #[method(keyRepeatInterval)]
        pub unsafe fn keyRepeatInterval() -> NSTimeInterval;
        #[method_id(addGlobalMonitorForEventsMatchingMask:handler:)]
        pub unsafe fn addGlobalMonitorForEventsMatchingMask_handler(
            mask: NSEventMask,
            block: TodoBlock,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(addLocalMonitorForEventsMatchingMask:handler:)]
        pub unsafe fn addLocalMonitorForEventsMatchingMask_handler(
            mask: NSEventMask,
            block: TodoBlock,
        ) -> Option<Id<Object, Shared>>;
        #[method(removeMonitor:)]
        pub unsafe fn removeMonitor(eventMonitor: &Object);
    }
);
