#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSGestureRecognizer;
    unsafe impl ClassType for NSGestureRecognizer {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSGestureRecognizer {
        #[method_id(initWithTarget:action:)]
        pub unsafe fn initWithTarget_action(
            &self,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(target)]
        pub unsafe fn target(&self) -> Option<Id<Object, Shared>>;
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);
        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;
        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);
        #[method(state)]
        pub unsafe fn state(&self) -> NSGestureRecognizerState;
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSGestureRecognizerDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSGestureRecognizerDelegate>);
        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;
        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);
        #[method_id(view)]
        pub unsafe fn view(&self) -> Option<Id<NSView, Shared>>;
        #[method_id(pressureConfiguration)]
        pub unsafe fn pressureConfiguration(&self) -> Id<NSPressureConfiguration, Shared>;
        #[method(setPressureConfiguration:)]
        pub unsafe fn setPressureConfiguration(
            &self,
            pressureConfiguration: &NSPressureConfiguration,
        );
        #[method(delaysPrimaryMouseButtonEvents)]
        pub unsafe fn delaysPrimaryMouseButtonEvents(&self) -> bool;
        #[method(setDelaysPrimaryMouseButtonEvents:)]
        pub unsafe fn setDelaysPrimaryMouseButtonEvents(
            &self,
            delaysPrimaryMouseButtonEvents: bool,
        );
        #[method(delaysSecondaryMouseButtonEvents)]
        pub unsafe fn delaysSecondaryMouseButtonEvents(&self) -> bool;
        #[method(setDelaysSecondaryMouseButtonEvents:)]
        pub unsafe fn setDelaysSecondaryMouseButtonEvents(
            &self,
            delaysSecondaryMouseButtonEvents: bool,
        );
        #[method(delaysOtherMouseButtonEvents)]
        pub unsafe fn delaysOtherMouseButtonEvents(&self) -> bool;
        #[method(setDelaysOtherMouseButtonEvents:)]
        pub unsafe fn setDelaysOtherMouseButtonEvents(&self, delaysOtherMouseButtonEvents: bool);
        #[method(delaysKeyEvents)]
        pub unsafe fn delaysKeyEvents(&self) -> bool;
        #[method(setDelaysKeyEvents:)]
        pub unsafe fn setDelaysKeyEvents(&self, delaysKeyEvents: bool);
        #[method(delaysMagnificationEvents)]
        pub unsafe fn delaysMagnificationEvents(&self) -> bool;
        #[method(setDelaysMagnificationEvents:)]
        pub unsafe fn setDelaysMagnificationEvents(&self, delaysMagnificationEvents: bool);
        #[method(delaysRotationEvents)]
        pub unsafe fn delaysRotationEvents(&self) -> bool;
        #[method(setDelaysRotationEvents:)]
        pub unsafe fn setDelaysRotationEvents(&self, delaysRotationEvents: bool);
        #[method(locationInView:)]
        pub unsafe fn locationInView(&self, view: Option<&NSView>) -> NSPoint;
    }
);
extern_methods!(
    #[doc = "NSTouchBar"]
    unsafe impl NSGestureRecognizer {
        #[method(allowedTouchTypes)]
        pub unsafe fn allowedTouchTypes(&self) -> NSTouchTypeMask;
        #[method(setAllowedTouchTypes:)]
        pub unsafe fn setAllowedTouchTypes(&self, allowedTouchTypes: NSTouchTypeMask);
    }
);
pub type NSGestureRecognizerDelegate = NSObject;
extern_methods!(
    #[doc = "NSSubclassUse"]
    unsafe impl NSGestureRecognizer {
        #[method(state)]
        pub unsafe fn state(&self) -> NSGestureRecognizerState;
        #[method(setState:)]
        pub unsafe fn setState(&self, state: NSGestureRecognizerState);
        #[method(reset)]
        pub unsafe fn reset(&self);
        #[method(canPreventGestureRecognizer:)]
        pub unsafe fn canPreventGestureRecognizer(
            &self,
            preventedGestureRecognizer: &NSGestureRecognizer,
        ) -> bool;
        #[method(canBePreventedByGestureRecognizer:)]
        pub unsafe fn canBePreventedByGestureRecognizer(
            &self,
            preventingGestureRecognizer: &NSGestureRecognizer,
        ) -> bool;
        #[method(shouldRequireFailureOfGestureRecognizer:)]
        pub unsafe fn shouldRequireFailureOfGestureRecognizer(
            &self,
            otherGestureRecognizer: &NSGestureRecognizer,
        ) -> bool;
        #[method(shouldBeRequiredToFailByGestureRecognizer:)]
        pub unsafe fn shouldBeRequiredToFailByGestureRecognizer(
            &self,
            otherGestureRecognizer: &NSGestureRecognizer,
        ) -> bool;
        #[method(mouseDown:)]
        pub unsafe fn mouseDown(&self, event: &NSEvent);
        #[method(rightMouseDown:)]
        pub unsafe fn rightMouseDown(&self, event: &NSEvent);
        #[method(otherMouseDown:)]
        pub unsafe fn otherMouseDown(&self, event: &NSEvent);
        #[method(mouseUp:)]
        pub unsafe fn mouseUp(&self, event: &NSEvent);
        #[method(rightMouseUp:)]
        pub unsafe fn rightMouseUp(&self, event: &NSEvent);
        #[method(otherMouseUp:)]
        pub unsafe fn otherMouseUp(&self, event: &NSEvent);
        #[method(mouseDragged:)]
        pub unsafe fn mouseDragged(&self, event: &NSEvent);
        #[method(rightMouseDragged:)]
        pub unsafe fn rightMouseDragged(&self, event: &NSEvent);
        #[method(otherMouseDragged:)]
        pub unsafe fn otherMouseDragged(&self, event: &NSEvent);
        #[method(keyDown:)]
        pub unsafe fn keyDown(&self, event: &NSEvent);
        #[method(keyUp:)]
        pub unsafe fn keyUp(&self, event: &NSEvent);
        #[method(flagsChanged:)]
        pub unsafe fn flagsChanged(&self, event: &NSEvent);
        #[method(tabletPoint:)]
        pub unsafe fn tabletPoint(&self, event: &NSEvent);
        #[method(magnifyWithEvent:)]
        pub unsafe fn magnifyWithEvent(&self, event: &NSEvent);
        #[method(rotateWithEvent:)]
        pub unsafe fn rotateWithEvent(&self, event: &NSEvent);
        #[method(pressureChangeWithEvent:)]
        pub unsafe fn pressureChangeWithEvent(&self, event: &NSEvent);
        #[method(touchesBeganWithEvent:)]
        pub unsafe fn touchesBeganWithEvent(&self, event: &NSEvent);
        #[method(touchesMovedWithEvent:)]
        pub unsafe fn touchesMovedWithEvent(&self, event: &NSEvent);
        #[method(touchesEndedWithEvent:)]
        pub unsafe fn touchesEndedWithEvent(&self, event: &NSEvent);
        #[method(touchesCancelledWithEvent:)]
        pub unsafe fn touchesCancelledWithEvent(&self, event: &NSEvent);
    }
);
