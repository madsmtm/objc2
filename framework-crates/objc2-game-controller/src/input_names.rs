use objc2_foundation::NSString;

// NS_TYPED_EXTENSIBLE_ENUM
pub type GCInputElementName = NSString;

// NS_TYPED_EXTENSIBLE_ENUM
pub type GCInputButtonName = NSString;

// NS_TYPED_EXTENSIBLE_ENUM
pub type GCInputAxisName = NSString;

// NS_TYPED_EXTENSIBLE_ENUM
pub type GCInputSwitchName = NSString;

// NS_TYPED_EXTENSIBLE_ENUM
pub type GCInputDirectionPadName = NSString;

extern "C" {
    pub static GCInputButtonA: &'static GCInputButtonName;
    pub static GCInputButtonB: &'static GCInputButtonName;
    pub static GCInputButtonX: &'static GCInputButtonName;
    pub static GCInputButtonY: &'static GCInputButtonName;
    pub static GCInputDirectionPad: &'static GCInputDirectionPadName;
    pub static GCInputLeftThumbstick: &'static GCInputDirectionPadName;
    pub static GCInputRightThumbstick: &'static GCInputDirectionPadName;
    pub static GCInputLeftShoulder: &'static GCInputButtonName;
    pub static GCInputRightShoulder: &'static GCInputButtonName;
    pub static GCInputLeftTrigger: &'static GCInputButtonName;
    pub static GCInputRightTrigger: &'static GCInputButtonName;
    pub static GCInputLeftThumbstickButton: &'static GCInputButtonName;
    pub static GCInputRightThumbstickButton: &'static GCInputButtonName;
    pub static GCInputButtonHome: &'static GCInputButtonName;
    pub static GCInputButtonMenu: &'static GCInputButtonName;
    pub static GCInputButtonOptions: &'static GCInputButtonName;
    pub static GCInputButtonShare: &'static GCInputButtonName;
    pub static GCInputXboxPaddleOne: &'static GCInputButtonName;
    pub static GCInputXboxPaddleTwo: &'static GCInputButtonName;
    pub static GCInputXboxPaddleThree: &'static GCInputButtonName;
    pub static GCInputXboxPaddleFour: &'static GCInputButtonName;
    pub static GCInputDualShockTouchpadOne: &'static GCInputDirectionPadName;
    pub static GCInputDualShockTouchpadTwo: &'static GCInputDirectionPadName;
    pub static GCInputDualShockTouchpadButton: &'static GCInputButtonName;
    pub static GCInputSteeringWheel: &'static GCInputAxisName;
    pub static GCInputShifter: &'static GCInputElementName;
    pub static GCInputPedalAccelerator: &'static GCInputButtonName;
    pub static GCInputPedalBrake: &'static GCInputButtonName;
    pub static GCInputPedalClutch: &'static GCInputButtonName;
    pub static GCInputLeftPaddle: &'static GCInputButtonName;
    pub static GCInputRightPaddle: &'static GCInputButtonName;
}
