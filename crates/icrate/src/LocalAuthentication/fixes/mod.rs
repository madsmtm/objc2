use crate::Foundation::NSInteger;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum LACredentialType {
        LACredentialTypeApplicationPassword = 0,
        LACredentialTypeSmartCardPIN = -3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum LAError {
        LAErrorAuthenticationFailed = -1,
        LAErrorUserCancel = -2,
        LAErrorUserFallback = -3,
        LAErrorSystemCancel = -4,
        LAErrorPasscodeNotSet = -5,
        #[deprecated = "use LAErrorBiometryNotAvailable"]
        LAErrorTouchIDNotAvailable = -6,
        #[deprecated = "use LAErrorBiometryNotEnrolled"]
        LAErrorTouchIDNotEnrolled = -7,
        #[deprecated = "use LAErrorBiometryLockout"]
        LAErrorTouchIDLockout = -8,
        LAErrorAppCancel = -9,
        LAErrorInvalidContext = -10,
        LAErrorBiometryNotAvailable = -6,
        LAErrorBiometryNotEnrolled = -7,
        LAErrorBiometryLockout = -8,
        LAErrorNotInteractive = -1004,
        LAErrorWatchNotAvailable = -11,
        LAErrorBiometryNotPaired = -12,
        LAErrorBiometryDisconnected = -13,
        LAErrorInvalidDimensions = -14,
    }
);
