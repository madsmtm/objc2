#![allow(non_snake_case)]

use block2::ConcreteBlock;
use icrate::{
    ns_string,
    objc2::{rc::Id, ClassType},
    Foundation::{
        NSData, NSNumber, NSObject, NSString, NSURLErrorBackgroundTaskCancelledReasonKey,
        NSURLErrorDomain, NSURLErrorFailingURLErrorKey, NSURLErrorFailingURLStringErrorKey,
        NSURLErrorNetworkUnavailableReasonKey, NSURLResponse, NSURLSessionDataTask, NSURL,
    },
};
use objc2::{extern_class, extern_methods, Cases, Codes, UserInfo};

mod ns_url_error_reasons {
    use super::*;

    #[objc2::typed_extensible_enum(type = isize)] // or #[objc(typed_extensible_enum, type = isize)]
    pub enum BackgroundTaskCancelledReason {
        UserForceQuitApplication = 0,
        BackgroundUpdatesDisabled = 1,
        InsufficientSystemResources = 2,
    }
    impl BackgroundTaskCancelledReason {
        #[allow(unused)]
        pub(super) fn from_number(number: &NSNumber) -> Self {
            Self(number.as_isize())
        }
    }

    #[objc2::typed_extensible_enum(type = isize)] // or #[objc(typed_extensible_enum, type = isize)]
    pub enum NetworkUnavailableReason {
        Cellular = 0,
        Expensive = 1,
        Constrained = 2,
    }
    impl NetworkUnavailableReason {
        #[allow(unused)]
        pub(super) fn from_number(number: &NSNumber) -> Self {
            Self(number.as_isize())
        }
    }
}

#[objc2::error_enum( // or #[objc(error_enum,
    domain = unsafe { NSURLErrorDomain },
    // NOTE: Here we specify (typed) getters for the associated `userInfo` dict.
    user_info = [
        /// # SAFETY (type)
        /// https://developer.apple.com/documentation/foundation/nsurlerrorfailingurlerrorkey?language=objc
        { key = NSURLErrorFailingURLErrorKey, unsafe type = Id<NSURL> },
        /// # SAFETY (type)
        /// https://developer.apple.com/documentation/foundation/nsurlerrorfailingurlstringerrorkey?language=objc
        { key = NSURLErrorFailingURLStringErrorKey, unsafe type = Id<NSString> },
        /// # SAFETY (type)
        /// https://developer.apple.com/documentation/foundation/nsurlerrorbackgroundtaskcancelledreasonkey?language=objc
        { key = NSURLErrorBackgroundTaskCancelledReasonKey, unsafe type = ns_url_error_reasons::BackgroundTaskCancelledReason },
        /// # SAFETY (type)
        /// https://developer.apple.com/documentation/foundation/nsurlerrornetworkunavailablereasonkey?language=objc'
        ///
        /// # SAFETY (from)
        /// 1. Casting dictionary `Object` to `NSNumber` is safe (type)
        /// 2. `NSNumber` can safely be converted to `NetworkUnavailableReason`
        ///
        /// # SAFETY (into)
        /// `NetworkUnavailableReason` can safely be converted to `NSObject` via deref (through
        /// converting to `NSNumber` first). This happens implicitly in the `&*arg` that
        /// occurs during the call to insert the value in the dictionary.
        {
            key = NSURLErrorNetworkUnavailableReasonKey,
            unsafe type = ns_url_error_reasons::NetworkUnavailableReason,
            unsafe from = |n: Option<Id<NSNumber>>| n.map(TryInto::try_into).and_then(Result::ok),
            unsafe into = std::convert::identity,
            // NOTE: `from` and `into` aren't necessary in this case since they can be computed
            // automatically but this is what they look like for demonstration purposes.
        },
    ]
)]
pub enum NSURLError {
    Unknown = -1,
    Cancelled = -999,
    BadURL = -1000,
    TimedOut = -1001,
    UnsupportedURL = -1002,
    CannotFindHost = -1003,
    CannotConnectToHost = -1004,
    NetworkConnectionLost = -1005,
    DNSLookupFailed = -1006,
    HTTPTooManyRedirects = -1007,
    ResourceUnavailable = -1008,
    NotConnectedToInternet = -1009,
    RedirectToNonExistentLocation = -1010,
    BadServerResponse = -1011,
    UserCancelledAuthentication = -1012,
    UserAuthenticationRequired = -1013,
    ZeroByteResource = -1014,
    CannotDecodeRawData = -1015,
    CannotDecodeContentData = -1016,
    CannotParseResponse = -1017,
    AppTransportSecurityRequiresSecureConnection = -1022,
    FileDoesNotExist = -1100,
    FileIsDirectory = -1101,
    NoPermissionsToReadFile = -1102,
    DataLengthExceedsMaximum = -1103,
    FileOutsideSafeArea = -1104,
    SecureConnectionFailed = -1200,
    ServerCertificateHasBadDate = -1201,
    ServerCertificateUntrusted = -1202,
    ServerCertificateHasUnknownRoot = -1203,
    ServerCertificateNotYetValid = -1204,
    ClientCertificateRejected = -1205,
    ClientCertificateRequired = -1206,
    CannotLoadFromNetwork = -2000,
    CannotCreateFile = -3000,
    CannotOpenFile = -3001,
    CannotCloseFile = -3002,
    CannotWriteToFile = -3003,
    CannotRemoveFile = -3004,
    CannotMoveFile = -3005,
    DownloadDecodingFailedMidStream = -3006,
    DownloadDecodingFailedToComplete = -3007,
    InternationalRoamingOff = -1018,
    CallIsActive = -1019,
    DataNotAllowed = -1020,
    RequestBodyStreamExhausted = -1021,
    BackgroundSessionRequiresSharedContainer = -995,
    BackgroundSessionInUseByAnotherProcess = -996,
    BackgroundSessionWasDisconnected = -997,
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSURLSession")]
    pub struct NSURLSession;

    #[cfg(feature = "Foundation_NSURLSession")]
    unsafe impl ClassType for NSURLSession {
        type Super = NSObject;
    }
);

extern_methods!(
    /// NSURLSessionAsynchronousConvenience
    #[cfg(feature = "Foundation_NSURLSession")]
    unsafe impl NSURLSession {
        #[method_id(@__retain_semantics Other sharedSession)]
        pub unsafe fn sharedSession() -> Id<NSURLSession>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL",
            feature = "Foundation_NSURLResponse",
            feature = "Foundation_NSURLSessionDataTask"
        ))]
        #[method_id(@__retain_semantics Other dataTaskWithURL:completionHandler:)]
        pub unsafe fn dataTaskWithURL_completionHandlerRefined(
            &self,
            url: &NSURL,
            // NOTE: we refine `NSError` to `NSURLError` here
            completion_handler: &block2::Block<
                (*mut NSData, *mut NSURLResponse, *mut NSURLError),
                (),
            >,
        ) -> Id<NSURLSessionDataTask>;
    }
);

fn example_catch() {
    let (tx, rx) = std::sync::mpsc::channel::<Option<NSURLErrorCodes>>();
    std::thread::spawn(move || {
        let session = unsafe { NSURLSession::sharedSession() };
        let task = {
            let url_string = ns_string!("foo://www.google.com");
            let url = unsafe { NSURL::URLWithString(url_string) }.expect("URL should parse");
            let completion_handler = {
                // NOTE: here we use a refined error type (via `ns_error_enum`) for the completion handler
                let block = ConcreteBlock::new(move |_data, _response, error: *mut NSURLError| {
                    // Get the refined error
                    let error = unsafe { error.as_ref() }.expect("error should be present");
                    // Verify that the refined error has the expected domain
                    assert_eq!(&*error.domain(), unsafe { NSURLErrorDomain });
                    // Use the refined error to pattern match on the error codes
                    #[allow(clippy::single_match)]
                    match error.code().cases() {
                        Some(code) => match code {
                            Cases::<NSURLError>::UnsupportedURL => {
                                // NOTE: here we use the generated `user_info` getter to check the failing URL
                                let failing_url_string =
                                    error.failing_url_string().expect("value should exist");
                                assert_eq!(&*failing_url_string, url_string);

                                // NOTE: we can also pattern match on the typed `user_info` data
                                #[allow(unused)]
                                let UserInfo::<NSURLError> {
                                    failing_url,
                                    failing_url_string,
                                    background_task_cancelled_reason,
                                    network_unavailable_reason,
                                } = error.user_info();
                                let failing_url_string =
                                    error.failing_url_string().expect("value should exist");
                                assert_eq!(&*failing_url_string, url_string);

                                // Send back the error code
                                tx.send(Some(error.code())).expect("channel should send");
                            }
                            // Rust Analyzer can actually fill in all these cases but we only care about the above case for this example
                            _ => {}
                        },
                        None => {
                            println!("unknown code from error: {:#?}", error.as_super());
                        }
                    }
                });
                block.copy()
            };
            unsafe { session.dataTaskWithURL_completionHandlerRefined(&url, &completion_handler) }
        };
        unsafe { task.resume() };
    });
    // Wait for the completion handler to finish with a possible error code
    let code = rx.recv().expect("channel should receive");
    // Verify that the error code is the one we expected
    assert!(code == Some(Codes::<NSURLError>::UNSUPPORTED_URL));
}

fn example_throw() {
    use ns_url_error_reasons::BackgroundTaskCancelledReason;
    let reason = Some(BackgroundTaskCancelledReason::BACKGROUND_UPDATES_DISABLED);
    let code = Codes::<NSURLError>::UNSUPPORTED_URL;
    let user_info = {
        UserInfo::<NSURLError> {
            background_task_cancelled_reason: reason.clone(),
            ..Default::default()
        }
    };
    // Construct an error with `code` and `user_info`
    let error = NSURLError::new_with_user_info(code, user_info);
    // Confirm that the `user_info` we get back contains the data we expect
    let UserInfo::<NSURLError> {
        background_task_cancelled_reason,
        ..
    } = error.user_info();
    assert_eq!(reason, background_task_cancelled_reason);
}

fn main() {
    example_catch();
    example_throw();
}
