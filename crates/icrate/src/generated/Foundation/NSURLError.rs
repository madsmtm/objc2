//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern "C" {
    pub static NSURLErrorDomain: &'static NSErrorDomain;
}

extern "C" {
    pub static NSURLErrorFailingURLErrorKey: &'static NSString;
}

extern "C" {
    pub static NSURLErrorFailingURLStringErrorKey: &'static NSString;
}

extern "C" {
    pub static NSErrorFailingURLStringKey: &'static NSString;
}

extern "C" {
    pub static NSURLErrorFailingURLPeerTrustErrorKey: &'static NSString;
}

extern "C" {
    pub static NSURLErrorBackgroundTaskCancelledReasonKey: &'static NSString;
}

pub const NSURLErrorCancelledReasonUserForceQuitApplication: NSInteger = 0;
pub const NSURLErrorCancelledReasonBackgroundUpdatesDisabled: NSInteger = 1;
pub const NSURLErrorCancelledReasonInsufficientSystemResources: NSInteger = 2;

extern "C" {
    pub static NSURLErrorNetworkUnavailableReasonKey: &'static NSErrorUserInfoKey;
}

pub type NSURLErrorNetworkUnavailableReason = NSInteger;
pub const NSURLErrorNetworkUnavailableReasonCellular: NSURLErrorNetworkUnavailableReason = 0;
pub const NSURLErrorNetworkUnavailableReasonExpensive: NSURLErrorNetworkUnavailableReason = 1;
pub const NSURLErrorNetworkUnavailableReasonConstrained: NSURLErrorNetworkUnavailableReason = 2;

pub const NSURLErrorUnknown: NSInteger = -1;
pub const NSURLErrorCancelled: NSInteger = -999;
pub const NSURLErrorBadURL: NSInteger = -1000;
pub const NSURLErrorTimedOut: NSInteger = -1001;
pub const NSURLErrorUnsupportedURL: NSInteger = -1002;
pub const NSURLErrorCannotFindHost: NSInteger = -1003;
pub const NSURLErrorCannotConnectToHost: NSInteger = -1004;
pub const NSURLErrorNetworkConnectionLost: NSInteger = -1005;
pub const NSURLErrorDNSLookupFailed: NSInteger = -1006;
pub const NSURLErrorHTTPTooManyRedirects: NSInteger = -1007;
pub const NSURLErrorResourceUnavailable: NSInteger = -1008;
pub const NSURLErrorNotConnectedToInternet: NSInteger = -1009;
pub const NSURLErrorRedirectToNonExistentLocation: NSInteger = -1010;
pub const NSURLErrorBadServerResponse: NSInteger = -1011;
pub const NSURLErrorUserCancelledAuthentication: NSInteger = -1012;
pub const NSURLErrorUserAuthenticationRequired: NSInteger = -1013;
pub const NSURLErrorZeroByteResource: NSInteger = -1014;
pub const NSURLErrorCannotDecodeRawData: NSInteger = -1015;
pub const NSURLErrorCannotDecodeContentData: NSInteger = -1016;
pub const NSURLErrorCannotParseResponse: NSInteger = -1017;
pub const NSURLErrorAppTransportSecurityRequiresSecureConnection: NSInteger = -1022;
pub const NSURLErrorFileDoesNotExist: NSInteger = -1100;
pub const NSURLErrorFileIsDirectory: NSInteger = -1101;
pub const NSURLErrorNoPermissionsToReadFile: NSInteger = -1102;
pub const NSURLErrorDataLengthExceedsMaximum: NSInteger = -1103;
pub const NSURLErrorFileOutsideSafeArea: NSInteger = -1104;
pub const NSURLErrorSecureConnectionFailed: NSInteger = -1200;
pub const NSURLErrorServerCertificateHasBadDate: NSInteger = -1201;
pub const NSURLErrorServerCertificateUntrusted: NSInteger = -1202;
pub const NSURLErrorServerCertificateHasUnknownRoot: NSInteger = -1203;
pub const NSURLErrorServerCertificateNotYetValid: NSInteger = -1204;
pub const NSURLErrorClientCertificateRejected: NSInteger = -1205;
pub const NSURLErrorClientCertificateRequired: NSInteger = -1206;
pub const NSURLErrorCannotLoadFromNetwork: NSInteger = -2000;
pub const NSURLErrorCannotCreateFile: NSInteger = -3000;
pub const NSURLErrorCannotOpenFile: NSInteger = -3001;
pub const NSURLErrorCannotCloseFile: NSInteger = -3002;
pub const NSURLErrorCannotWriteToFile: NSInteger = -3003;
pub const NSURLErrorCannotRemoveFile: NSInteger = -3004;
pub const NSURLErrorCannotMoveFile: NSInteger = -3005;
pub const NSURLErrorDownloadDecodingFailedMidStream: NSInteger = -3006;
pub const NSURLErrorDownloadDecodingFailedToComplete: NSInteger = -3007;
pub const NSURLErrorInternationalRoamingOff: NSInteger = -1018;
pub const NSURLErrorCallIsActive: NSInteger = -1019;
pub const NSURLErrorDataNotAllowed: NSInteger = -1020;
pub const NSURLErrorRequestBodyStreamExhausted: NSInteger = -1021;
pub const NSURLErrorBackgroundSessionRequiresSharedContainer: NSInteger = -995;
pub const NSURLErrorBackgroundSessionInUseByAnotherProcess: NSInteger = -996;
pub const NSURLErrorBackgroundSessionWasDisconnected: NSInteger = -997;
