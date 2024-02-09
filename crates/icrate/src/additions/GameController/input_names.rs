use crate::Foundation::NSString;

typed_extensible_enum!(
    pub type GCInputElementName = NSString;
);

typed_extensible_enum!(
    pub type GCInputButtonName = NSString;
);

typed_extensible_enum!(
    pub type GCInputAxisName = NSString;
);

typed_extensible_enum!(
    pub type GCInputSwitchName = NSString;
);

typed_extensible_enum!(
    pub type GCInputDirectionPadName = NSString;
);

extern_static!(GCInputButtonA: &'static GCInputButtonName);

extern_static!(GCInputButtonB: &'static GCInputButtonName);

extern_static!(GCInputButtonX: &'static GCInputButtonName);

extern_static!(GCInputButtonY: &'static GCInputButtonName);

extern_static!(GCInputDirectionPad: &'static GCInputDirectionPadName);

extern_static!(GCInputLeftThumbstick: &'static GCInputDirectionPadName);

extern_static!(GCInputRightThumbstick: &'static GCInputDirectionPadName);

extern_static!(GCInputLeftShoulder: &'static GCInputButtonName);

extern_static!(GCInputRightShoulder: &'static GCInputButtonName);

extern_static!(GCInputLeftTrigger: &'static GCInputButtonName);

extern_static!(GCInputRightTrigger: &'static GCInputButtonName);

extern_static!(GCInputLeftThumbstickButton: &'static GCInputButtonName);

extern_static!(GCInputRightThumbstickButton: &'static GCInputButtonName);

extern_static!(GCInputButtonHome: &'static GCInputButtonName);

extern_static!(GCInputButtonMenu: &'static GCInputButtonName);

extern_static!(GCInputButtonOptions: &'static GCInputButtonName);

extern_static!(GCInputButtonShare: &'static GCInputButtonName);

extern_static!(GCInputXboxPaddleOne: &'static GCInputButtonName);

extern_static!(GCInputXboxPaddleTwo: &'static GCInputButtonName);

extern_static!(GCInputXboxPaddleThree: &'static GCInputButtonName);

extern_static!(GCInputXboxPaddleFour: &'static GCInputButtonName);

extern_static!(GCInputDualShockTouchpadOne: &'static GCInputDirectionPadName);

extern_static!(GCInputDualShockTouchpadTwo: &'static GCInputDirectionPadName);

extern_static!(GCInputDualShockTouchpadButton: &'static GCInputButtonName);

extern_static!(GCInputSteeringWheel: &'static GCInputAxisName);

extern_static!(GCInputShifter: &'static GCInputElementName);

extern_static!(GCInputPedalAccelerator: &'static GCInputButtonName);

extern_static!(GCInputPedalBrake: &'static GCInputButtonName);

extern_static!(GCInputPedalClutch: &'static GCInputButtonName);

extern_static!(GCInputLeftPaddle: &'static GCInputButtonName);

extern_static!(GCInputRightPaddle: &'static GCInputButtonName);
