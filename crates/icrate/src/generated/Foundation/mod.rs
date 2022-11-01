pub(crate) mod FoundationErrors;
pub(crate) mod FoundationLegacySwiftCompatibility;
pub(crate) mod NSAffineTransform;
pub(crate) mod NSAppleEventDescriptor;
pub(crate) mod NSAppleEventManager;
pub(crate) mod NSAppleScript;
pub(crate) mod NSArchiver;
pub(crate) mod NSArray;
pub(crate) mod NSAttributedString;
pub(crate) mod NSAutoreleasePool;
pub(crate) mod NSBackgroundActivityScheduler;
pub(crate) mod NSBundle;
pub(crate) mod NSByteCountFormatter;
pub(crate) mod NSByteOrder;
pub(crate) mod NSCache;
pub(crate) mod NSCalendar;
pub(crate) mod NSCalendarDate;
pub(crate) mod NSCharacterSet;
pub(crate) mod NSClassDescription;
pub(crate) mod NSCoder;
pub(crate) mod NSComparisonPredicate;
pub(crate) mod NSCompoundPredicate;
pub(crate) mod NSConnection;
pub(crate) mod NSData;
pub(crate) mod NSDate;
pub(crate) mod NSDateComponentsFormatter;
pub(crate) mod NSDateFormatter;
pub(crate) mod NSDateInterval;
pub(crate) mod NSDateIntervalFormatter;
pub(crate) mod NSDecimal;
pub(crate) mod NSDecimalNumber;
pub(crate) mod NSDictionary;
pub(crate) mod NSDistantObject;
pub(crate) mod NSDistributedLock;
pub(crate) mod NSDistributedNotificationCenter;
pub(crate) mod NSEnergyFormatter;
pub(crate) mod NSEnumerator;
pub(crate) mod NSError;
pub(crate) mod NSException;
pub(crate) mod NSExpression;
pub(crate) mod NSExtensionContext;
pub(crate) mod NSExtensionItem;
pub(crate) mod NSExtensionRequestHandling;
pub(crate) mod NSFileCoordinator;
pub(crate) mod NSFileHandle;
pub(crate) mod NSFileManager;
pub(crate) mod NSFilePresenter;
pub(crate) mod NSFileVersion;
pub(crate) mod NSFileWrapper;
pub(crate) mod NSFormatter;
pub(crate) mod NSGarbageCollector;
pub(crate) mod NSGeometry;
pub(crate) mod NSHFSFileTypes;
pub(crate) mod NSHTTPCookie;
pub(crate) mod NSHTTPCookieStorage;
pub(crate) mod NSHashTable;
pub(crate) mod NSHost;
pub(crate) mod NSISO8601DateFormatter;
pub(crate) mod NSIndexPath;
pub(crate) mod NSIndexSet;
pub(crate) mod NSInflectionRule;
pub(crate) mod NSInvocation;
pub(crate) mod NSItemProvider;
pub(crate) mod NSJSONSerialization;
pub(crate) mod NSKeyValueCoding;
pub(crate) mod NSKeyValueObserving;
pub(crate) mod NSKeyedArchiver;
pub(crate) mod NSLengthFormatter;
pub(crate) mod NSLinguisticTagger;
pub(crate) mod NSListFormatter;
pub(crate) mod NSLocale;
pub(crate) mod NSLock;
pub(crate) mod NSMapTable;
pub(crate) mod NSMassFormatter;
pub(crate) mod NSMeasurement;
pub(crate) mod NSMeasurementFormatter;
pub(crate) mod NSMetadata;
pub(crate) mod NSMetadataAttributes;
pub(crate) mod NSMethodSignature;
pub(crate) mod NSMorphology;
pub(crate) mod NSNetServices;
pub(crate) mod NSNotification;
pub(crate) mod NSNotificationQueue;
pub(crate) mod NSNull;
pub(crate) mod NSNumberFormatter;
pub(crate) mod NSObjCRuntime;
pub(crate) mod NSObject;
pub(crate) mod NSObjectScripting;
pub(crate) mod NSOperation;
pub(crate) mod NSOrderedCollectionChange;
pub(crate) mod NSOrderedCollectionDifference;
pub(crate) mod NSOrderedSet;
pub(crate) mod NSOrthography;
pub(crate) mod NSPathUtilities;
pub(crate) mod NSPersonNameComponents;
pub(crate) mod NSPersonNameComponentsFormatter;
pub(crate) mod NSPointerArray;
pub(crate) mod NSPointerFunctions;
pub(crate) mod NSPort;
pub(crate) mod NSPortCoder;
pub(crate) mod NSPortMessage;
pub(crate) mod NSPortNameServer;
pub(crate) mod NSPredicate;
pub(crate) mod NSProcessInfo;
pub(crate) mod NSProgress;
pub(crate) mod NSPropertyList;
pub(crate) mod NSProtocolChecker;
pub(crate) mod NSProxy;
pub(crate) mod NSRange;
pub(crate) mod NSRegularExpression;
pub(crate) mod NSRelativeDateTimeFormatter;
pub(crate) mod NSRunLoop;
pub(crate) mod NSScanner;
pub(crate) mod NSScriptClassDescription;
pub(crate) mod NSScriptCoercionHandler;
pub(crate) mod NSScriptCommand;
pub(crate) mod NSScriptCommandDescription;
pub(crate) mod NSScriptExecutionContext;
pub(crate) mod NSScriptKeyValueCoding;
pub(crate) mod NSScriptObjectSpecifiers;
pub(crate) mod NSScriptStandardSuiteCommands;
pub(crate) mod NSScriptSuiteRegistry;
pub(crate) mod NSScriptWhoseTests;
pub(crate) mod NSSet;
pub(crate) mod NSSortDescriptor;
pub(crate) mod NSSpellServer;
pub(crate) mod NSStream;
pub(crate) mod NSString;
pub(crate) mod NSTask;
pub(crate) mod NSTextCheckingResult;
pub(crate) mod NSThread;
pub(crate) mod NSTimeZone;
pub(crate) mod NSTimer;
pub(crate) mod NSURL;
pub(crate) mod NSURLAuthenticationChallenge;
pub(crate) mod NSURLCache;
pub(crate) mod NSURLConnection;
pub(crate) mod NSURLCredential;
pub(crate) mod NSURLCredentialStorage;
pub(crate) mod NSURLDownload;
pub(crate) mod NSURLError;
pub(crate) mod NSURLHandle;
pub(crate) mod NSURLProtectionSpace;
pub(crate) mod NSURLProtocol;
pub(crate) mod NSURLRequest;
pub(crate) mod NSURLResponse;
pub(crate) mod NSURLSession;
pub(crate) mod NSUUID;
pub(crate) mod NSUbiquitousKeyValueStore;
pub(crate) mod NSUndoManager;
pub(crate) mod NSUnit;
pub(crate) mod NSUserActivity;
pub(crate) mod NSUserDefaults;
pub(crate) mod NSUserNotification;
pub(crate) mod NSUserScriptTask;
pub(crate) mod NSValue;
pub(crate) mod NSValueTransformer;
pub(crate) mod NSXMLDTD;
pub(crate) mod NSXMLDTDNode;
pub(crate) mod NSXMLDocument;
pub(crate) mod NSXMLElement;
pub(crate) mod NSXMLNode;
pub(crate) mod NSXMLNodeOptions;
pub(crate) mod NSXMLParser;
pub(crate) mod NSXPCConnection;
pub(crate) mod NSZone;

mod __exported {
    pub use super::FoundationErrors::{
        NSBundleErrorMaximum, NSBundleErrorMinimum,
        NSBundleOnDemandResourceExceededMaximumSizeError, NSBundleOnDemandResourceInvalidTagError,
        NSBundleOnDemandResourceOutOfSpaceError, NSCloudSharingConflictError,
        NSCloudSharingErrorMaximum, NSCloudSharingErrorMinimum, NSCloudSharingNetworkFailureError,
        NSCloudSharingNoPermissionError, NSCloudSharingOtherError,
        NSCloudSharingQuotaExceededError, NSCloudSharingTooManyParticipantsError,
        NSCoderErrorMaximum, NSCoderErrorMinimum, NSCoderInvalidValueError,
        NSCoderReadCorruptError, NSCoderValueNotFoundError, NSCompressionErrorMaximum,
        NSCompressionErrorMinimum, NSCompressionFailedError, NSDecompressionFailedError,
        NSExecutableArchitectureMismatchError, NSExecutableErrorMaximum, NSExecutableErrorMinimum,
        NSExecutableLinkError, NSExecutableLoadError, NSExecutableNotLoadableError,
        NSExecutableRuntimeMismatchError, NSFeatureUnsupportedError, NSFileErrorMaximum,
        NSFileErrorMinimum, NSFileLockingError, NSFileManagerUnmountBusyError,
        NSFileManagerUnmountUnknownError, NSFileNoSuchFileError, NSFileReadCorruptFileError,
        NSFileReadInapplicableStringEncodingError, NSFileReadInvalidFileNameError,
        NSFileReadNoPermissionError, NSFileReadNoSuchFileError, NSFileReadTooLargeError,
        NSFileReadUnknownError, NSFileReadUnknownStringEncodingError,
        NSFileReadUnsupportedSchemeError, NSFileWriteFileExistsError,
        NSFileWriteInapplicableStringEncodingError, NSFileWriteInvalidFileNameError,
        NSFileWriteNoPermissionError, NSFileWriteOutOfSpaceError, NSFileWriteUnknownError,
        NSFileWriteUnsupportedSchemeError, NSFileWriteVolumeReadOnlyError, NSFormattingError,
        NSFormattingErrorMaximum, NSFormattingErrorMinimum, NSKeyValueValidationError,
        NSPropertyListErrorMaximum, NSPropertyListErrorMinimum, NSPropertyListReadCorruptError,
        NSPropertyListReadStreamError, NSPropertyListReadUnknownVersionError,
        NSPropertyListWriteInvalidError, NSPropertyListWriteStreamError,
        NSUbiquitousFileErrorMaximum, NSUbiquitousFileErrorMinimum,
        NSUbiquitousFileNotUploadedDueToQuotaError, NSUbiquitousFileUbiquityServerNotAvailable,
        NSUbiquitousFileUnavailableError, NSUserActivityConnectionUnavailableError,
        NSUserActivityErrorMaximum, NSUserActivityErrorMinimum, NSUserActivityHandoffFailedError,
        NSUserActivityHandoffUserInfoTooLargeError, NSUserActivityRemoteApplicationTimedOutError,
        NSUserCancelledError, NSValidationErrorMaximum, NSValidationErrorMinimum,
        NSXPCConnectionErrorMaximum, NSXPCConnectionErrorMinimum, NSXPCConnectionInterrupted,
        NSXPCConnectionInvalid, NSXPCConnectionReplyInvalid,
    };
    pub use super::NSAffineTransform::NSAffineTransform;
    pub use super::NSAppleEventDescriptor::{
        NSAppleEventDescriptor, NSAppleEventSendAlwaysInteract, NSAppleEventSendCanInteract,
        NSAppleEventSendCanSwitchLayer, NSAppleEventSendDefaultOptions,
        NSAppleEventSendDontAnnotate, NSAppleEventSendDontExecute, NSAppleEventSendDontRecord,
        NSAppleEventSendNeverInteract, NSAppleEventSendNoReply, NSAppleEventSendOptions,
        NSAppleEventSendQueueReply, NSAppleEventSendWaitForReply,
    };
    pub use super::NSAppleEventManager::{
        NSAppleEventManager, NSAppleEventManagerWillProcessFirstEventNotification,
        NSAppleEventTimeOutDefault, NSAppleEventTimeOutNone,
    };
    pub use super::NSAppleScript::{
        NSAppleScript, NSAppleScriptErrorAppName, NSAppleScriptErrorBriefMessage,
        NSAppleScriptErrorMessage, NSAppleScriptErrorNumber, NSAppleScriptErrorRange,
    };
    pub use super::NSArchiver::{NSArchiver, NSUnarchiver};
    pub use super::NSArray::{
        NSArray, NSBinarySearchingFirstEqual, NSBinarySearchingInsertionIndex,
        NSBinarySearchingLastEqual, NSBinarySearchingOptions, NSMutableArray,
    };
    pub use super::NSAttributedString::{
        NSAlternateDescriptionAttributeName, NSAttributedString,
        NSAttributedStringEnumerationLongestEffectiveRangeNotRequired,
        NSAttributedStringEnumerationOptions, NSAttributedStringEnumerationReverse,
        NSAttributedStringFormattingApplyReplacementIndexAttribute,
        NSAttributedStringFormattingInsertArgumentAttributesWithoutMerging,
        NSAttributedStringFormattingOptions, NSAttributedStringKey,
        NSAttributedStringMarkdownInterpretedSyntax,
        NSAttributedStringMarkdownInterpretedSyntaxFull,
        NSAttributedStringMarkdownInterpretedSyntaxInlineOnly,
        NSAttributedStringMarkdownInterpretedSyntaxInlineOnlyPreservingWhitespace,
        NSAttributedStringMarkdownParsingFailurePolicy,
        NSAttributedStringMarkdownParsingFailureReturnError,
        NSAttributedStringMarkdownParsingFailureReturnPartiallyParsedIfPossible,
        NSAttributedStringMarkdownParsingOptions, NSImageURLAttributeName,
        NSInflectionAlternativeAttributeName, NSInflectionRuleAttributeName,
        NSInlinePresentationIntent, NSInlinePresentationIntentAttributeName,
        NSInlinePresentationIntentBlockHTML, NSInlinePresentationIntentCode,
        NSInlinePresentationIntentEmphasized, NSInlinePresentationIntentInlineHTML,
        NSInlinePresentationIntentLineBreak, NSInlinePresentationIntentSoftBreak,
        NSInlinePresentationIntentStrikethrough, NSInlinePresentationIntentStronglyEmphasized,
        NSLanguageIdentifierAttributeName, NSMorphologyAttributeName, NSMutableAttributedString,
        NSPresentationIntent, NSPresentationIntentAttributeName, NSPresentationIntentKind,
        NSPresentationIntentKindBlockQuote, NSPresentationIntentKindCodeBlock,
        NSPresentationIntentKindHeader, NSPresentationIntentKindListItem,
        NSPresentationIntentKindOrderedList, NSPresentationIntentKindParagraph,
        NSPresentationIntentKindTable, NSPresentationIntentKindTableCell,
        NSPresentationIntentKindTableHeaderRow, NSPresentationIntentKindTableRow,
        NSPresentationIntentKindThematicBreak, NSPresentationIntentKindUnorderedList,
        NSPresentationIntentTableColumnAlignment, NSPresentationIntentTableColumnAlignmentCenter,
        NSPresentationIntentTableColumnAlignmentLeft,
        NSPresentationIntentTableColumnAlignmentRight, NSReplacementIndexAttributeName,
    };
    pub use super::NSAutoreleasePool::NSAutoreleasePool;
    pub use super::NSBackgroundActivityScheduler::{
        NSBackgroundActivityResult, NSBackgroundActivityResultDeferred,
        NSBackgroundActivityResultFinished, NSBackgroundActivityScheduler,
    };
    pub use super::NSBundle::{
        NSBundle, NSBundleDidLoadNotification, NSBundleExecutableArchitectureARM64,
        NSBundleExecutableArchitectureI386, NSBundleExecutableArchitecturePPC,
        NSBundleExecutableArchitecturePPC64, NSBundleExecutableArchitectureX86_64,
        NSBundleResourceRequest, NSBundleResourceRequestLoadingPriorityUrgent,
        NSBundleResourceRequestLowDiskSpaceNotification, NSLoadedClasses,
    };
    pub use super::NSByteCountFormatter::{
        NSByteCountFormatter, NSByteCountFormatterCountStyle, NSByteCountFormatterCountStyleBinary,
        NSByteCountFormatterCountStyleDecimal, NSByteCountFormatterCountStyleFile,
        NSByteCountFormatterCountStyleMemory, NSByteCountFormatterUnits,
        NSByteCountFormatterUseAll, NSByteCountFormatterUseBytes, NSByteCountFormatterUseDefault,
        NSByteCountFormatterUseEB, NSByteCountFormatterUseGB, NSByteCountFormatterUseKB,
        NSByteCountFormatterUseMB, NSByteCountFormatterUsePB, NSByteCountFormatterUseTB,
        NSByteCountFormatterUseYBOrHigher, NSByteCountFormatterUseZB,
    };
    pub use super::NSByteOrder::{NS_BigEndian, NS_LittleEndian, NS_UnknownByteOrder};
    pub use super::NSCache::{NSCache, NSCacheDelegate};
    pub use super::NSCalendar::{
        NSCalendar, NSCalendarCalendarUnit, NSCalendarDayChangedNotification, NSCalendarIdentifier,
        NSCalendarIdentifierBuddhist, NSCalendarIdentifierChinese, NSCalendarIdentifierCoptic,
        NSCalendarIdentifierEthiopicAmeteAlem, NSCalendarIdentifierEthiopicAmeteMihret,
        NSCalendarIdentifierGregorian, NSCalendarIdentifierHebrew, NSCalendarIdentifierISO8601,
        NSCalendarIdentifierIndian, NSCalendarIdentifierIslamic, NSCalendarIdentifierIslamicCivil,
        NSCalendarIdentifierIslamicTabular, NSCalendarIdentifierIslamicUmmAlQura,
        NSCalendarIdentifierJapanese, NSCalendarIdentifierPersian,
        NSCalendarIdentifierRepublicOfChina, NSCalendarMatchFirst, NSCalendarMatchLast,
        NSCalendarMatchNextTime, NSCalendarMatchNextTimePreservingSmallerUnits,
        NSCalendarMatchPreviousTimePreservingSmallerUnits, NSCalendarMatchStrictly,
        NSCalendarOptions, NSCalendarSearchBackwards, NSCalendarUnit, NSCalendarUnitCalendar,
        NSCalendarUnitDay, NSCalendarUnitEra, NSCalendarUnitHour, NSCalendarUnitMinute,
        NSCalendarUnitMonth, NSCalendarUnitNanosecond, NSCalendarUnitQuarter, NSCalendarUnitSecond,
        NSCalendarUnitTimeZone, NSCalendarUnitWeekOfMonth, NSCalendarUnitWeekOfYear,
        NSCalendarUnitWeekday, NSCalendarUnitWeekdayOrdinal, NSCalendarUnitYear,
        NSCalendarUnitYearForWeekOfYear, NSCalendarWrapComponents, NSDateComponentUndefined,
        NSDateComponents, NSDayCalendarUnit, NSEraCalendarUnit, NSHourCalendarUnit,
        NSMinuteCalendarUnit, NSMonthCalendarUnit, NSQuarterCalendarUnit, NSSecondCalendarUnit,
        NSTimeZoneCalendarUnit, NSUndefinedDateComponent, NSWeekCalendarUnit,
        NSWeekOfMonthCalendarUnit, NSWeekOfYearCalendarUnit, NSWeekdayCalendarUnit,
        NSWeekdayOrdinalCalendarUnit, NSWrapCalendarComponents, NSYearCalendarUnit,
        NSYearForWeekOfYearCalendarUnit,
    };
    pub use super::NSCalendarDate::NSCalendarDate;
    pub use super::NSCharacterSet::{
        NSCharacterSet, NSMutableCharacterSet, NSOpenStepUnicodeReservedBase,
    };
    pub use super::NSClassDescription::{
        NSClassDescription, NSClassDescriptionNeededForClassNotification,
    };
    pub use super::NSCoder::{
        NSCoder, NSDecodingFailurePolicy, NSDecodingFailurePolicyRaiseException,
        NSDecodingFailurePolicySetErrorAndReturn,
    };
    pub use super::NSComparisonPredicate::{
        NSAllPredicateModifier, NSAnyPredicateModifier, NSBeginsWithPredicateOperatorType,
        NSBetweenPredicateOperatorType, NSCaseInsensitivePredicateOption, NSComparisonPredicate,
        NSComparisonPredicateModifier, NSComparisonPredicateOptions,
        NSContainsPredicateOperatorType, NSCustomSelectorPredicateOperatorType,
        NSDiacriticInsensitivePredicateOption, NSDirectPredicateModifier,
        NSEndsWithPredicateOperatorType, NSEqualToPredicateOperatorType,
        NSGreaterThanOrEqualToPredicateOperatorType, NSGreaterThanPredicateOperatorType,
        NSInPredicateOperatorType, NSLessThanOrEqualToPredicateOperatorType,
        NSLessThanPredicateOperatorType, NSLikePredicateOperatorType,
        NSMatchesPredicateOperatorType, NSNormalizedPredicateOption,
        NSNotEqualToPredicateOperatorType, NSPredicateOperatorType,
    };
    pub use super::NSCompoundPredicate::{
        NSAndPredicateType, NSCompoundPredicate, NSCompoundPredicateType, NSNotPredicateType,
        NSOrPredicateType,
    };
    pub use super::NSConnection::{
        NSConnection, NSConnectionDelegate, NSConnectionDidDieNotification,
        NSConnectionDidInitializeNotification, NSConnectionReplyMode, NSDistantObjectRequest,
        NSFailedAuthenticationException,
    };
    pub use super::NSData::{
        NSAtomicWrite, NSData, NSDataBase64DecodingIgnoreUnknownCharacters,
        NSDataBase64DecodingOptions, NSDataBase64Encoding64CharacterLineLength,
        NSDataBase64Encoding76CharacterLineLength, NSDataBase64EncodingEndLineWithCarriageReturn,
        NSDataBase64EncodingEndLineWithLineFeed, NSDataBase64EncodingOptions,
        NSDataCompressionAlgorithm, NSDataCompressionAlgorithmLZ4, NSDataCompressionAlgorithmLZFSE,
        NSDataCompressionAlgorithmLZMA, NSDataCompressionAlgorithmZlib, NSDataReadingMapped,
        NSDataReadingMappedAlways, NSDataReadingMappedIfSafe, NSDataReadingOptions,
        NSDataReadingUncached, NSDataSearchAnchored, NSDataSearchBackwards, NSDataSearchOptions,
        NSDataWritingAtomic, NSDataWritingFileProtectionComplete,
        NSDataWritingFileProtectionCompleteUnlessOpen,
        NSDataWritingFileProtectionCompleteUntilFirstUserAuthentication,
        NSDataWritingFileProtectionMask, NSDataWritingFileProtectionNone, NSDataWritingOptions,
        NSDataWritingWithoutOverwriting, NSMappedRead, NSMutableData, NSPurgeableData,
        NSUncachedRead,
    };
    pub use super::NSDate::{NSDate, NSSystemClockDidChangeNotification};
    pub use super::NSDateComponentsFormatter::{
        NSDateComponentsFormatter, NSDateComponentsFormatterUnitsStyle,
        NSDateComponentsFormatterUnitsStyleAbbreviated, NSDateComponentsFormatterUnitsStyleBrief,
        NSDateComponentsFormatterUnitsStyleFull, NSDateComponentsFormatterUnitsStylePositional,
        NSDateComponentsFormatterUnitsStyleShort, NSDateComponentsFormatterUnitsStyleSpellOut,
        NSDateComponentsFormatterZeroFormattingBehavior,
        NSDateComponentsFormatterZeroFormattingBehaviorDefault,
        NSDateComponentsFormatterZeroFormattingBehaviorDropAll,
        NSDateComponentsFormatterZeroFormattingBehaviorDropLeading,
        NSDateComponentsFormatterZeroFormattingBehaviorDropMiddle,
        NSDateComponentsFormatterZeroFormattingBehaviorDropTrailing,
        NSDateComponentsFormatterZeroFormattingBehaviorNone,
        NSDateComponentsFormatterZeroFormattingBehaviorPad,
    };
    pub use super::NSDateFormatter::{
        NSDateFormatter, NSDateFormatterBehavior, NSDateFormatterBehavior10_0,
        NSDateFormatterBehavior10_4, NSDateFormatterBehaviorDefault, NSDateFormatterFullStyle,
        NSDateFormatterLongStyle, NSDateFormatterMediumStyle, NSDateFormatterNoStyle,
        NSDateFormatterShortStyle, NSDateFormatterStyle,
    };
    pub use super::NSDateInterval::NSDateInterval;
    pub use super::NSDateIntervalFormatter::{
        NSDateIntervalFormatter, NSDateIntervalFormatterFullStyle,
        NSDateIntervalFormatterLongStyle, NSDateIntervalFormatterMediumStyle,
        NSDateIntervalFormatterNoStyle, NSDateIntervalFormatterShortStyle,
        NSDateIntervalFormatterStyle,
    };
    pub use super::NSDecimal::{
        NSCalculationDivideByZero, NSCalculationError, NSCalculationLossOfPrecision,
        NSCalculationNoError, NSCalculationOverflow, NSCalculationUnderflow, NSRoundBankers,
        NSRoundDown, NSRoundPlain, NSRoundUp, NSRoundingMode,
    };
    pub use super::NSDecimalNumber::{
        NSDecimalNumber, NSDecimalNumberBehaviors, NSDecimalNumberDivideByZeroException,
        NSDecimalNumberExactnessException, NSDecimalNumberHandler,
        NSDecimalNumberOverflowException, NSDecimalNumberUnderflowException,
    };
    pub use super::NSDictionary::{NSDictionary, NSMutableDictionary};
    pub use super::NSDistantObject::NSDistantObject;
    pub use super::NSDistributedLock::NSDistributedLock;
    pub use super::NSDistributedNotificationCenter::{
        NSDistributedNotificationCenter, NSDistributedNotificationCenterType,
        NSDistributedNotificationDeliverImmediately, NSDistributedNotificationOptions,
        NSDistributedNotificationPostToAllSessions, NSLocalNotificationCenterType,
        NSNotificationDeliverImmediately, NSNotificationPostToAllSessions,
        NSNotificationSuspensionBehavior, NSNotificationSuspensionBehaviorCoalesce,
        NSNotificationSuspensionBehaviorDeliverImmediately, NSNotificationSuspensionBehaviorDrop,
        NSNotificationSuspensionBehaviorHold,
    };
    pub use super::NSEnergyFormatter::{
        NSEnergyFormatter, NSEnergyFormatterUnit, NSEnergyFormatterUnitCalorie,
        NSEnergyFormatterUnitJoule, NSEnergyFormatterUnitKilocalorie,
        NSEnergyFormatterUnitKilojoule,
    };
    pub use super::NSEnumerator::{NSEnumerator, NSFastEnumeration};
    pub use super::NSError::{
        NSCocoaErrorDomain, NSDebugDescriptionErrorKey, NSError, NSErrorDomain, NSErrorUserInfoKey,
        NSFilePathErrorKey, NSHelpAnchorErrorKey, NSLocalizedDescriptionKey,
        NSLocalizedFailureErrorKey, NSLocalizedFailureReasonErrorKey,
        NSLocalizedRecoveryOptionsErrorKey, NSLocalizedRecoverySuggestionErrorKey,
        NSMachErrorDomain, NSMultipleUnderlyingErrorsKey, NSOSStatusErrorDomain,
        NSPOSIXErrorDomain, NSRecoveryAttempterErrorKey, NSStringEncodingErrorKey, NSURLErrorKey,
        NSUnderlyingErrorKey,
    };
    pub use super::NSException::{
        NSAssertionHandler, NSAssertionHandlerKey, NSDestinationInvalidException, NSException,
        NSGenericException, NSInconsistentArchiveException, NSInternalInconsistencyException,
        NSInvalidArgumentException, NSInvalidReceivePortException, NSInvalidSendPortException,
        NSMallocException, NSObjectInaccessibleException, NSObjectNotAvailableException,
        NSOldStyleException, NSPortReceiveException, NSPortSendException, NSPortTimeoutException,
        NSRangeException,
    };
    pub use super::NSExpression::{
        NSAggregateExpressionType, NSAnyKeyExpressionType, NSBlockExpressionType,
        NSConditionalExpressionType, NSConstantValueExpressionType,
        NSEvaluatedObjectExpressionType, NSExpression, NSExpressionType, NSFunctionExpressionType,
        NSIntersectSetExpressionType, NSKeyPathExpressionType, NSMinusSetExpressionType,
        NSSubqueryExpressionType, NSUnionSetExpressionType, NSVariableExpressionType,
    };
    pub use super::NSExtensionContext::{
        NSExtensionContext, NSExtensionHostDidBecomeActiveNotification,
        NSExtensionHostDidEnterBackgroundNotification,
        NSExtensionHostWillEnterForegroundNotification,
        NSExtensionHostWillResignActiveNotification, NSExtensionItemsAndErrorsKey,
    };
    pub use super::NSExtensionItem::{
        NSExtensionItem, NSExtensionItemAttachmentsKey, NSExtensionItemAttributedContentTextKey,
        NSExtensionItemAttributedTitleKey,
    };
    pub use super::NSExtensionRequestHandling::NSExtensionRequestHandling;
    pub use super::NSFileCoordinator::{
        NSFileAccessIntent, NSFileCoordinator, NSFileCoordinatorReadingForUploading,
        NSFileCoordinatorReadingImmediatelyAvailableMetadataOnly, NSFileCoordinatorReadingOptions,
        NSFileCoordinatorReadingResolvesSymbolicLink, NSFileCoordinatorReadingWithoutChanges,
        NSFileCoordinatorWritingContentIndependentMetadataOnly,
        NSFileCoordinatorWritingForDeleting, NSFileCoordinatorWritingForMerging,
        NSFileCoordinatorWritingForMoving, NSFileCoordinatorWritingForReplacing,
        NSFileCoordinatorWritingOptions,
    };
    pub use super::NSFileHandle::{
        NSFileHandle, NSFileHandleConnectionAcceptedNotification,
        NSFileHandleDataAvailableNotification, NSFileHandleNotificationDataItem,
        NSFileHandleNotificationFileHandleItem, NSFileHandleNotificationMonitorModes,
        NSFileHandleOperationException, NSFileHandleReadCompletionNotification,
        NSFileHandleReadToEndOfFileCompletionNotification, NSPipe,
    };
    pub use super::NSFileManager::{
        NSDirectoryEnumerationIncludesDirectoriesPostOrder, NSDirectoryEnumerationOptions,
        NSDirectoryEnumerationProducesRelativePathURLs, NSDirectoryEnumerationSkipsHiddenFiles,
        NSDirectoryEnumerationSkipsPackageDescendants,
        NSDirectoryEnumerationSkipsSubdirectoryDescendants, NSDirectoryEnumerator,
        NSFileAppendOnly, NSFileAttributeKey, NSFileAttributeType, NSFileBusy, NSFileCreationDate,
        NSFileDeviceIdentifier, NSFileExtensionHidden, NSFileGroupOwnerAccountID,
        NSFileGroupOwnerAccountName, NSFileHFSCreatorCode, NSFileHFSTypeCode, NSFileImmutable,
        NSFileManager, NSFileManagerDelegate, NSFileManagerItemReplacementOptions,
        NSFileManagerItemReplacementUsingNewMetadataOnly,
        NSFileManagerItemReplacementWithoutDeletingBackupItem,
        NSFileManagerUnmountAllPartitionsAndEjectDisk,
        NSFileManagerUnmountDissentingProcessIdentifierErrorKey, NSFileManagerUnmountOptions,
        NSFileManagerUnmountWithoutUI, NSFileModificationDate, NSFileOwnerAccountID,
        NSFileOwnerAccountName, NSFilePosixPermissions, NSFileProtectionComplete,
        NSFileProtectionCompleteUnlessOpen, NSFileProtectionCompleteUntilFirstUserAuthentication,
        NSFileProtectionKey, NSFileProtectionNone, NSFileProtectionType, NSFileProviderService,
        NSFileProviderServiceName, NSFileReferenceCount, NSFileSize, NSFileSystemFileNumber,
        NSFileSystemFreeNodes, NSFileSystemFreeSize, NSFileSystemNodes, NSFileSystemNumber,
        NSFileSystemSize, NSFileType, NSFileTypeBlockSpecial, NSFileTypeCharacterSpecial,
        NSFileTypeDirectory, NSFileTypeRegular, NSFileTypeSocket, NSFileTypeSymbolicLink,
        NSFileTypeUnknown, NSURLRelationship, NSURLRelationshipContains, NSURLRelationshipOther,
        NSURLRelationshipSame, NSUbiquityIdentityDidChangeNotification, NSVolumeEnumerationOptions,
        NSVolumeEnumerationProduceFileReferenceURLs, NSVolumeEnumerationSkipHiddenVolumes,
    };
    pub use super::NSFilePresenter::NSFilePresenter;
    pub use super::NSFileVersion::{
        NSFileVersion, NSFileVersionAddingByMoving, NSFileVersionAddingOptions,
        NSFileVersionReplacingByMoving, NSFileVersionReplacingOptions,
    };
    pub use super::NSFileWrapper::{
        NSFileWrapper, NSFileWrapperReadingImmediate, NSFileWrapperReadingOptions,
        NSFileWrapperReadingWithoutMapping, NSFileWrapperWritingAtomic,
        NSFileWrapperWritingOptions, NSFileWrapperWritingWithNameUpdating,
    };
    pub use super::NSFormatter::{
        NSFormatter, NSFormattingContext, NSFormattingContextBeginningOfSentence,
        NSFormattingContextDynamic, NSFormattingContextListItem,
        NSFormattingContextMiddleOfSentence, NSFormattingContextStandalone,
        NSFormattingContextUnknown, NSFormattingUnitStyle, NSFormattingUnitStyleLong,
        NSFormattingUnitStyleMedium, NSFormattingUnitStyleShort,
    };
    pub use super::NSGarbageCollector::NSGarbageCollector;
    pub use super::NSGeometry::{
        NSAlignAllEdgesInward, NSAlignAllEdgesNearest, NSAlignAllEdgesOutward, NSAlignHeightInward,
        NSAlignHeightNearest, NSAlignHeightOutward, NSAlignMaxXInward, NSAlignMaxXNearest,
        NSAlignMaxXOutward, NSAlignMaxYInward, NSAlignMaxYNearest, NSAlignMaxYOutward,
        NSAlignMinXInward, NSAlignMinXNearest, NSAlignMinXOutward, NSAlignMinYInward,
        NSAlignMinYNearest, NSAlignMinYOutward, NSAlignRectFlipped, NSAlignWidthInward,
        NSAlignWidthNearest, NSAlignWidthOutward, NSAlignmentOptions, NSEdgeInsetsZero, NSMaxXEdge,
        NSMaxYEdge, NSMinXEdge, NSMinYEdge, NSPoint, NSRect, NSRectEdge, NSRectEdgeMaxX,
        NSRectEdgeMaxY, NSRectEdgeMinX, NSRectEdgeMinY, NSSize, NSZeroPoint, NSZeroRect,
        NSZeroSize,
    };
    pub use super::NSHTTPCookie::{
        NSHTTPCookie, NSHTTPCookieComment, NSHTTPCookieCommentURL, NSHTTPCookieDiscard,
        NSHTTPCookieDomain, NSHTTPCookieExpires, NSHTTPCookieMaximumAge, NSHTTPCookieName,
        NSHTTPCookieOriginURL, NSHTTPCookiePath, NSHTTPCookiePort, NSHTTPCookiePropertyKey,
        NSHTTPCookieSameSiteLax, NSHTTPCookieSameSitePolicy, NSHTTPCookieSameSiteStrict,
        NSHTTPCookieSecure, NSHTTPCookieStringPolicy, NSHTTPCookieValue, NSHTTPCookieVersion,
    };
    pub use super::NSHTTPCookieStorage::{
        NSHTTPCookieAcceptPolicy, NSHTTPCookieAcceptPolicyAlways, NSHTTPCookieAcceptPolicyNever,
        NSHTTPCookieAcceptPolicyOnlyFromMainDocumentDomain,
        NSHTTPCookieManagerAcceptPolicyChangedNotification,
        NSHTTPCookieManagerCookiesChangedNotification, NSHTTPCookieStorage,
    };
    pub use super::NSHashTable::{
        NSHashTable, NSHashTableCopyIn, NSHashTableObjectPointerPersonality, NSHashTableOptions,
        NSHashTableStrongMemory, NSHashTableWeakMemory, NSHashTableZeroingWeakMemory,
        NSIntHashCallBacks, NSIntegerHashCallBacks, NSNonOwnedPointerHashCallBacks,
        NSNonRetainedObjectHashCallBacks, NSObjectHashCallBacks,
        NSOwnedObjectIdentityHashCallBacks, NSOwnedPointerHashCallBacks,
        NSPointerToStructHashCallBacks,
    };
    pub use super::NSHost::NSHost;
    pub use super::NSISO8601DateFormatter::{
        NSISO8601DateFormatOptions, NSISO8601DateFormatWithColonSeparatorInTime,
        NSISO8601DateFormatWithColonSeparatorInTimeZone,
        NSISO8601DateFormatWithDashSeparatorInDate, NSISO8601DateFormatWithDay,
        NSISO8601DateFormatWithFractionalSeconds, NSISO8601DateFormatWithFullDate,
        NSISO8601DateFormatWithFullTime, NSISO8601DateFormatWithInternetDateTime,
        NSISO8601DateFormatWithMonth, NSISO8601DateFormatWithSpaceBetweenDateAndTime,
        NSISO8601DateFormatWithTime, NSISO8601DateFormatWithTimeZone,
        NSISO8601DateFormatWithWeekOfYear, NSISO8601DateFormatWithYear, NSISO8601DateFormatter,
    };
    pub use super::NSIndexPath::NSIndexPath;
    pub use super::NSIndexSet::{NSIndexSet, NSMutableIndexSet};
    pub use super::NSInflectionRule::{NSInflectionRule, NSInflectionRuleExplicit};
    pub use super::NSInvocation::NSInvocation;
    pub use super::NSItemProvider::{
        NSExtensionJavaScriptFinalizeArgumentKey, NSExtensionJavaScriptPreprocessingResultsKey,
        NSItemProvider, NSItemProviderErrorCode, NSItemProviderErrorDomain,
        NSItemProviderFileOptionOpenInPlace, NSItemProviderFileOptions,
        NSItemProviderItemUnavailableError, NSItemProviderPreferredImageSizeKey,
        NSItemProviderReading, NSItemProviderRepresentationVisibility,
        NSItemProviderRepresentationVisibilityAll, NSItemProviderRepresentationVisibilityGroup,
        NSItemProviderRepresentationVisibilityOwnProcess,
        NSItemProviderRepresentationVisibilityTeam, NSItemProviderUnavailableCoercionError,
        NSItemProviderUnexpectedValueClassError, NSItemProviderUnknownError, NSItemProviderWriting,
    };
    pub use super::NSJSONSerialization::{
        NSJSONReadingAllowFragments, NSJSONReadingFragmentsAllowed, NSJSONReadingJSON5Allowed,
        NSJSONReadingMutableContainers, NSJSONReadingMutableLeaves, NSJSONReadingOptions,
        NSJSONReadingTopLevelDictionaryAssumed, NSJSONSerialization, NSJSONWritingFragmentsAllowed,
        NSJSONWritingOptions, NSJSONWritingPrettyPrinted, NSJSONWritingSortedKeys,
        NSJSONWritingWithoutEscapingSlashes,
    };
    pub use super::NSKeyValueCoding::{
        NSAverageKeyValueOperator, NSCountKeyValueOperator,
        NSDistinctUnionOfArraysKeyValueOperator, NSDistinctUnionOfObjectsKeyValueOperator,
        NSDistinctUnionOfSetsKeyValueOperator, NSKeyValueOperator, NSMaximumKeyValueOperator,
        NSMinimumKeyValueOperator, NSSumKeyValueOperator, NSUndefinedKeyException,
        NSUnionOfArraysKeyValueOperator, NSUnionOfObjectsKeyValueOperator,
        NSUnionOfSetsKeyValueOperator,
    };
    pub use super::NSKeyValueObserving::{
        NSKeyValueChange, NSKeyValueChangeIndexesKey, NSKeyValueChangeInsertion,
        NSKeyValueChangeKey, NSKeyValueChangeKindKey, NSKeyValueChangeNewKey,
        NSKeyValueChangeNotificationIsPriorKey, NSKeyValueChangeOldKey, NSKeyValueChangeRemoval,
        NSKeyValueChangeReplacement, NSKeyValueChangeSetting, NSKeyValueIntersectSetMutation,
        NSKeyValueMinusSetMutation, NSKeyValueObservingOptionInitial, NSKeyValueObservingOptionNew,
        NSKeyValueObservingOptionOld, NSKeyValueObservingOptionPrior, NSKeyValueObservingOptions,
        NSKeyValueSetMutationKind, NSKeyValueSetSetMutation, NSKeyValueUnionSetMutation,
    };
    pub use super::NSKeyedArchiver::{
        NSInvalidArchiveOperationException, NSInvalidUnarchiveOperationException,
        NSKeyedArchiveRootObjectKey, NSKeyedArchiver, NSKeyedArchiverDelegate, NSKeyedUnarchiver,
        NSKeyedUnarchiverDelegate,
    };
    pub use super::NSLengthFormatter::{
        NSLengthFormatter, NSLengthFormatterUnit, NSLengthFormatterUnitCentimeter,
        NSLengthFormatterUnitFoot, NSLengthFormatterUnitInch, NSLengthFormatterUnitKilometer,
        NSLengthFormatterUnitMeter, NSLengthFormatterUnitMile, NSLengthFormatterUnitMillimeter,
        NSLengthFormatterUnitYard,
    };
    pub use super::NSLinguisticTagger::{
        NSLinguisticTag, NSLinguisticTagAdjective, NSLinguisticTagAdverb,
        NSLinguisticTagClassifier, NSLinguisticTagCloseParenthesis, NSLinguisticTagCloseQuote,
        NSLinguisticTagConjunction, NSLinguisticTagDash, NSLinguisticTagDeterminer,
        NSLinguisticTagIdiom, NSLinguisticTagInterjection, NSLinguisticTagNoun,
        NSLinguisticTagNumber, NSLinguisticTagOpenParenthesis, NSLinguisticTagOpenQuote,
        NSLinguisticTagOrganizationName, NSLinguisticTagOther, NSLinguisticTagOtherPunctuation,
        NSLinguisticTagOtherWhitespace, NSLinguisticTagOtherWord, NSLinguisticTagParagraphBreak,
        NSLinguisticTagParticle, NSLinguisticTagPersonalName, NSLinguisticTagPlaceName,
        NSLinguisticTagPreposition, NSLinguisticTagPronoun, NSLinguisticTagPunctuation,
        NSLinguisticTagScheme, NSLinguisticTagSchemeLanguage, NSLinguisticTagSchemeLemma,
        NSLinguisticTagSchemeLexicalClass, NSLinguisticTagSchemeNameType,
        NSLinguisticTagSchemeNameTypeOrLexicalClass, NSLinguisticTagSchemeScript,
        NSLinguisticTagSchemeTokenType, NSLinguisticTagSentenceTerminator, NSLinguisticTagVerb,
        NSLinguisticTagWhitespace, NSLinguisticTagWord, NSLinguisticTagWordJoiner,
        NSLinguisticTagger, NSLinguisticTaggerJoinNames, NSLinguisticTaggerOmitOther,
        NSLinguisticTaggerOmitPunctuation, NSLinguisticTaggerOmitWhitespace,
        NSLinguisticTaggerOmitWords, NSLinguisticTaggerOptions, NSLinguisticTaggerUnit,
        NSLinguisticTaggerUnitDocument, NSLinguisticTaggerUnitParagraph,
        NSLinguisticTaggerUnitSentence, NSLinguisticTaggerUnitWord,
    };
    pub use super::NSListFormatter::NSListFormatter;
    pub use super::NSLocale::{
        NSBuddhistCalendar, NSChineseCalendar, NSCurrentLocaleDidChangeNotification,
        NSGregorianCalendar, NSHebrewCalendar, NSISO8601Calendar, NSIndianCalendar,
        NSIslamicCalendar, NSIslamicCivilCalendar, NSJapaneseCalendar, NSLocale,
        NSLocaleAlternateQuotationBeginDelimiterKey, NSLocaleAlternateQuotationEndDelimiterKey,
        NSLocaleCalendar, NSLocaleCollationIdentifier, NSLocaleCollatorIdentifier,
        NSLocaleCountryCode, NSLocaleCurrencyCode, NSLocaleCurrencySymbol,
        NSLocaleDecimalSeparator, NSLocaleExemplarCharacterSet, NSLocaleGroupingSeparator,
        NSLocaleIdentifier, NSLocaleKey, NSLocaleLanguageCode, NSLocaleLanguageDirection,
        NSLocaleLanguageDirectionBottomToTop, NSLocaleLanguageDirectionLeftToRight,
        NSLocaleLanguageDirectionRightToLeft, NSLocaleLanguageDirectionTopToBottom,
        NSLocaleLanguageDirectionUnknown, NSLocaleMeasurementSystem,
        NSLocaleQuotationBeginDelimiterKey, NSLocaleQuotationEndDelimiterKey, NSLocaleScriptCode,
        NSLocaleUsesMetricSystem, NSLocaleVariantCode, NSPersianCalendar,
        NSRepublicOfChinaCalendar,
    };
    pub use super::NSLock::{NSCondition, NSConditionLock, NSLock, NSLocking, NSRecursiveLock};
    pub use super::NSMapTable::{
        NSIntMapKeyCallBacks, NSIntMapValueCallBacks, NSIntegerMapKeyCallBacks,
        NSIntegerMapValueCallBacks, NSMapTable, NSMapTableCopyIn,
        NSMapTableObjectPointerPersonality, NSMapTableOptions, NSMapTableStrongMemory,
        NSMapTableWeakMemory, NSMapTableZeroingWeakMemory, NSNonOwnedPointerMapKeyCallBacks,
        NSNonOwnedPointerMapValueCallBacks, NSNonOwnedPointerOrNullMapKeyCallBacks,
        NSNonRetainedObjectMapKeyCallBacks, NSNonRetainedObjectMapValueCallBacks,
        NSObjectMapKeyCallBacks, NSObjectMapValueCallBacks, NSOwnedPointerMapKeyCallBacks,
        NSOwnedPointerMapValueCallBacks,
    };
    pub use super::NSMassFormatter::{
        NSMassFormatter, NSMassFormatterUnit, NSMassFormatterUnitGram, NSMassFormatterUnitKilogram,
        NSMassFormatterUnitOunce, NSMassFormatterUnitPound, NSMassFormatterUnitStone,
    };
    pub use super::NSMeasurement::NSMeasurement;
    pub use super::NSMeasurementFormatter::{
        NSMeasurementFormatter, NSMeasurementFormatterUnitOptions,
        NSMeasurementFormatterUnitOptionsNaturalScale,
        NSMeasurementFormatterUnitOptionsProvidedUnit,
        NSMeasurementFormatterUnitOptionsTemperatureWithoutUnit,
    };
    pub use super::NSMetadata::{
        NSMetadataItem, NSMetadataQuery, NSMetadataQueryAccessibleUbiquitousExternalDocumentsScope,
        NSMetadataQueryAttributeValueTuple, NSMetadataQueryDelegate,
        NSMetadataQueryDidFinishGatheringNotification,
        NSMetadataQueryDidStartGatheringNotification, NSMetadataQueryDidUpdateNotification,
        NSMetadataQueryGatheringProgressNotification, NSMetadataQueryIndexedLocalComputerScope,
        NSMetadataQueryIndexedNetworkScope, NSMetadataQueryLocalComputerScope,
        NSMetadataQueryNetworkScope, NSMetadataQueryResultContentRelevanceAttribute,
        NSMetadataQueryResultGroup, NSMetadataQueryUbiquitousDataScope,
        NSMetadataQueryUbiquitousDocumentsScope, NSMetadataQueryUpdateAddedItemsKey,
        NSMetadataQueryUpdateChangedItemsKey, NSMetadataQueryUpdateRemovedItemsKey,
        NSMetadataQueryUserHomeScope,
    };
    pub use super::NSMetadataAttributes::{
        NSMetadataItemAcquisitionMakeKey, NSMetadataItemAcquisitionModelKey,
        NSMetadataItemAlbumKey, NSMetadataItemAltitudeKey, NSMetadataItemApertureKey,
        NSMetadataItemAppleLoopDescriptorsKey, NSMetadataItemAppleLoopsKeyFilterTypeKey,
        NSMetadataItemAppleLoopsLoopModeKey, NSMetadataItemAppleLoopsRootKeyKey,
        NSMetadataItemApplicationCategoriesKey, NSMetadataItemAttributeChangeDateKey,
        NSMetadataItemAudiencesKey, NSMetadataItemAudioBitRateKey,
        NSMetadataItemAudioChannelCountKey, NSMetadataItemAudioEncodingApplicationKey,
        NSMetadataItemAudioSampleRateKey, NSMetadataItemAudioTrackNumberKey,
        NSMetadataItemAuthorAddressesKey, NSMetadataItemAuthorEmailAddressesKey,
        NSMetadataItemAuthorsKey, NSMetadataItemBitsPerSampleKey,
        NSMetadataItemCFBundleIdentifierKey, NSMetadataItemCameraOwnerKey, NSMetadataItemCityKey,
        NSMetadataItemCodecsKey, NSMetadataItemColorSpaceKey, NSMetadataItemCommentKey,
        NSMetadataItemComposerKey, NSMetadataItemContactKeywordsKey,
        NSMetadataItemContentCreationDateKey, NSMetadataItemContentModificationDateKey,
        NSMetadataItemContentTypeKey, NSMetadataItemContentTypeTreeKey,
        NSMetadataItemContributorsKey, NSMetadataItemCopyrightKey, NSMetadataItemCountryKey,
        NSMetadataItemCoverageKey, NSMetadataItemCreatorKey, NSMetadataItemDateAddedKey,
        NSMetadataItemDeliveryTypeKey, NSMetadataItemDescriptionKey, NSMetadataItemDirectorKey,
        NSMetadataItemDisplayNameKey, NSMetadataItemDownloadedDateKey, NSMetadataItemDueDateKey,
        NSMetadataItemDurationSecondsKey, NSMetadataItemEXIFGPSVersionKey,
        NSMetadataItemEXIFVersionKey, NSMetadataItemEditorsKey, NSMetadataItemEmailAddressesKey,
        NSMetadataItemEncodingApplicationsKey, NSMetadataItemExecutableArchitecturesKey,
        NSMetadataItemExecutablePlatformKey, NSMetadataItemExposureModeKey,
        NSMetadataItemExposureProgramKey, NSMetadataItemExposureTimeSecondsKey,
        NSMetadataItemExposureTimeStringKey, NSMetadataItemFNumberKey,
        NSMetadataItemFSContentChangeDateKey, NSMetadataItemFSCreationDateKey,
        NSMetadataItemFSNameKey, NSMetadataItemFSSizeKey, NSMetadataItemFinderCommentKey,
        NSMetadataItemFlashOnOffKey, NSMetadataItemFocalLength35mmKey,
        NSMetadataItemFocalLengthKey, NSMetadataItemFontsKey, NSMetadataItemGPSAreaInformationKey,
        NSMetadataItemGPSDOPKey, NSMetadataItemGPSDateStampKey, NSMetadataItemGPSDestBearingKey,
        NSMetadataItemGPSDestDistanceKey, NSMetadataItemGPSDestLatitudeKey,
        NSMetadataItemGPSDestLongitudeKey, NSMetadataItemGPSDifferentalKey,
        NSMetadataItemGPSMapDatumKey, NSMetadataItemGPSMeasureModeKey,
        NSMetadataItemGPSProcessingMethodKey, NSMetadataItemGPSStatusKey,
        NSMetadataItemGPSTrackKey, NSMetadataItemGenreKey, NSMetadataItemHasAlphaChannelKey,
        NSMetadataItemHeadlineKey, NSMetadataItemISOSpeedKey, NSMetadataItemIdentifierKey,
        NSMetadataItemImageDirectionKey, NSMetadataItemInformationKey,
        NSMetadataItemInstantMessageAddressesKey, NSMetadataItemInstructionsKey,
        NSMetadataItemIsApplicationManagedKey, NSMetadataItemIsGeneralMIDISequenceKey,
        NSMetadataItemIsLikelyJunkKey, NSMetadataItemIsUbiquitousKey,
        NSMetadataItemKeySignatureKey, NSMetadataItemKeywordsKey, NSMetadataItemKindKey,
        NSMetadataItemLanguagesKey, NSMetadataItemLastUsedDateKey, NSMetadataItemLatitudeKey,
        NSMetadataItemLayerNamesKey, NSMetadataItemLensModelKey, NSMetadataItemLongitudeKey,
        NSMetadataItemLyricistKey, NSMetadataItemMaxApertureKey, NSMetadataItemMediaTypesKey,
        NSMetadataItemMeteringModeKey, NSMetadataItemMusicalGenreKey,
        NSMetadataItemMusicalInstrumentCategoryKey, NSMetadataItemMusicalInstrumentNameKey,
        NSMetadataItemNamedLocationKey, NSMetadataItemNumberOfPagesKey,
        NSMetadataItemOrganizationsKey, NSMetadataItemOrientationKey,
        NSMetadataItemOriginalFormatKey, NSMetadataItemOriginalSourceKey,
        NSMetadataItemPageHeightKey, NSMetadataItemPageWidthKey, NSMetadataItemParticipantsKey,
        NSMetadataItemPathKey, NSMetadataItemPerformersKey, NSMetadataItemPhoneNumbersKey,
        NSMetadataItemPixelCountKey, NSMetadataItemPixelHeightKey, NSMetadataItemPixelWidthKey,
        NSMetadataItemProducerKey, NSMetadataItemProfileNameKey, NSMetadataItemProjectsKey,
        NSMetadataItemPublishersKey, NSMetadataItemRecipientAddressesKey,
        NSMetadataItemRecipientEmailAddressesKey, NSMetadataItemRecipientsKey,
        NSMetadataItemRecordingDateKey, NSMetadataItemRecordingYearKey,
        NSMetadataItemRedEyeOnOffKey, NSMetadataItemResolutionHeightDPIKey,
        NSMetadataItemResolutionWidthDPIKey, NSMetadataItemRightsKey,
        NSMetadataItemSecurityMethodKey, NSMetadataItemSpeedKey, NSMetadataItemStarRatingKey,
        NSMetadataItemStateOrProvinceKey, NSMetadataItemStreamableKey, NSMetadataItemSubjectKey,
        NSMetadataItemTempoKey, NSMetadataItemTextContentKey, NSMetadataItemThemeKey,
        NSMetadataItemTimeSignatureKey, NSMetadataItemTimestampKey, NSMetadataItemTitleKey,
        NSMetadataItemTotalBitRateKey, NSMetadataItemURLKey, NSMetadataItemVersionKey,
        NSMetadataItemVideoBitRateKey, NSMetadataItemWhereFromsKey, NSMetadataItemWhiteBalanceKey,
        NSMetadataUbiquitousItemContainerDisplayNameKey,
        NSMetadataUbiquitousItemDownloadRequestedKey, NSMetadataUbiquitousItemDownloadingErrorKey,
        NSMetadataUbiquitousItemDownloadingStatusCurrent,
        NSMetadataUbiquitousItemDownloadingStatusDownloaded,
        NSMetadataUbiquitousItemDownloadingStatusKey,
        NSMetadataUbiquitousItemDownloadingStatusNotDownloaded,
        NSMetadataUbiquitousItemHasUnresolvedConflictsKey, NSMetadataUbiquitousItemIsDownloadedKey,
        NSMetadataUbiquitousItemIsDownloadingKey, NSMetadataUbiquitousItemIsExternalDocumentKey,
        NSMetadataUbiquitousItemIsSharedKey, NSMetadataUbiquitousItemIsUploadedKey,
        NSMetadataUbiquitousItemIsUploadingKey, NSMetadataUbiquitousItemPercentDownloadedKey,
        NSMetadataUbiquitousItemPercentUploadedKey, NSMetadataUbiquitousItemURLInLocalContainerKey,
        NSMetadataUbiquitousItemUploadingErrorKey,
        NSMetadataUbiquitousSharedItemCurrentUserPermissionsKey,
        NSMetadataUbiquitousSharedItemCurrentUserRoleKey,
        NSMetadataUbiquitousSharedItemMostRecentEditorNameComponentsKey,
        NSMetadataUbiquitousSharedItemOwnerNameComponentsKey,
        NSMetadataUbiquitousSharedItemPermissionsReadOnly,
        NSMetadataUbiquitousSharedItemPermissionsReadWrite,
        NSMetadataUbiquitousSharedItemRoleOwner, NSMetadataUbiquitousSharedItemRoleParticipant,
    };
    pub use super::NSMethodSignature::NSMethodSignature;
    pub use super::NSMorphology::{
        NSGrammaticalGender, NSGrammaticalGenderFeminine, NSGrammaticalGenderMasculine,
        NSGrammaticalGenderNeuter, NSGrammaticalGenderNotSet, NSGrammaticalNumber,
        NSGrammaticalNumberNotSet, NSGrammaticalNumberPlural, NSGrammaticalNumberPluralFew,
        NSGrammaticalNumberPluralMany, NSGrammaticalNumberPluralTwo, NSGrammaticalNumberSingular,
        NSGrammaticalNumberZero, NSGrammaticalPartOfSpeech, NSGrammaticalPartOfSpeechAbbreviation,
        NSGrammaticalPartOfSpeechAdjective, NSGrammaticalPartOfSpeechAdposition,
        NSGrammaticalPartOfSpeechAdverb, NSGrammaticalPartOfSpeechConjunction,
        NSGrammaticalPartOfSpeechDeterminer, NSGrammaticalPartOfSpeechInterjection,
        NSGrammaticalPartOfSpeechLetter, NSGrammaticalPartOfSpeechNotSet,
        NSGrammaticalPartOfSpeechNoun, NSGrammaticalPartOfSpeechNumeral,
        NSGrammaticalPartOfSpeechParticle, NSGrammaticalPartOfSpeechPreposition,
        NSGrammaticalPartOfSpeechPronoun, NSGrammaticalPartOfSpeechVerb, NSMorphology,
        NSMorphologyCustomPronoun,
    };
    pub use super::NSNetServices::{
        NSNetService, NSNetServiceBrowser, NSNetServiceBrowserDelegate, NSNetServiceDelegate,
        NSNetServiceListenForConnections, NSNetServiceNoAutoRename, NSNetServiceOptions,
        NSNetServicesActivityInProgress, NSNetServicesBadArgumentError,
        NSNetServicesCancelledError, NSNetServicesCollisionError, NSNetServicesError,
        NSNetServicesErrorCode, NSNetServicesErrorDomain, NSNetServicesInvalidError,
        NSNetServicesMissingRequiredConfigurationError, NSNetServicesNotFoundError,
        NSNetServicesTimeoutError, NSNetServicesUnknownError,
    };
    pub use super::NSNotification::{NSNotification, NSNotificationCenter, NSNotificationName};
    pub use super::NSNotificationQueue::{
        NSNotificationCoalescing, NSNotificationCoalescingOnName, NSNotificationCoalescingOnSender,
        NSNotificationNoCoalescing, NSNotificationQueue, NSPostASAP, NSPostNow, NSPostWhenIdle,
        NSPostingStyle,
    };
    pub use super::NSNull::NSNull;
    pub use super::NSNumberFormatter::{
        NSNumberFormatter, NSNumberFormatterBehavior, NSNumberFormatterBehavior10_0,
        NSNumberFormatterBehavior10_4, NSNumberFormatterBehaviorDefault,
        NSNumberFormatterCurrencyAccountingStyle, NSNumberFormatterCurrencyISOCodeStyle,
        NSNumberFormatterCurrencyPluralStyle, NSNumberFormatterCurrencyStyle,
        NSNumberFormatterDecimalStyle, NSNumberFormatterNoStyle, NSNumberFormatterOrdinalStyle,
        NSNumberFormatterPadAfterPrefix, NSNumberFormatterPadAfterSuffix,
        NSNumberFormatterPadBeforePrefix, NSNumberFormatterPadBeforeSuffix,
        NSNumberFormatterPadPosition, NSNumberFormatterPercentStyle, NSNumberFormatterRoundCeiling,
        NSNumberFormatterRoundDown, NSNumberFormatterRoundFloor, NSNumberFormatterRoundHalfDown,
        NSNumberFormatterRoundHalfEven, NSNumberFormatterRoundHalfUp, NSNumberFormatterRoundUp,
        NSNumberFormatterRoundingMode, NSNumberFormatterScientificStyle,
        NSNumberFormatterSpellOutStyle, NSNumberFormatterStyle,
    };
    pub use super::NSObjCRuntime::{
        NSComparisonResult, NSEnumerationConcurrent, NSEnumerationOptions, NSEnumerationReverse,
        NSExceptionName, NSFoundationVersionNumber, NSNotFound, NSOrderedAscending,
        NSOrderedDescending, NSOrderedSame, NSQualityOfService, NSQualityOfServiceBackground,
        NSQualityOfServiceDefault, NSQualityOfServiceUserInitiated,
        NSQualityOfServiceUserInteractive, NSQualityOfServiceUtility, NSRunLoopMode,
        NSSortConcurrent, NSSortOptions, NSSortStable,
    };
    pub use super::NSObject::{
        NSCoding, NSCopying, NSDiscardableContent, NSMutableCopying, NSSecureCoding,
    };
    pub use super::NSOperation::{
        NSBlockOperation, NSInvocationOperation, NSInvocationOperationCancelledException,
        NSInvocationOperationVoidResultException, NSOperation, NSOperationQueue,
        NSOperationQueueDefaultMaxConcurrentOperationCount, NSOperationQueuePriority,
        NSOperationQueuePriorityHigh, NSOperationQueuePriorityLow, NSOperationQueuePriorityNormal,
        NSOperationQueuePriorityVeryHigh, NSOperationQueuePriorityVeryLow,
    };
    pub use super::NSOrderedCollectionChange::{
        NSCollectionChangeInsert, NSCollectionChangeRemove, NSCollectionChangeType,
        NSOrderedCollectionChange,
    };
    pub use super::NSOrderedCollectionDifference::{
        NSOrderedCollectionDifference, NSOrderedCollectionDifferenceCalculationInferMoves,
        NSOrderedCollectionDifferenceCalculationOmitInsertedObjects,
        NSOrderedCollectionDifferenceCalculationOmitRemovedObjects,
        NSOrderedCollectionDifferenceCalculationOptions,
    };
    pub use super::NSOrderedSet::{NSMutableOrderedSet, NSOrderedSet};
    pub use super::NSOrthography::NSOrthography;
    pub use super::NSPathUtilities::{
        NSAdminApplicationDirectory, NSAllApplicationsDirectory, NSAllDomainsMask,
        NSAllLibrariesDirectory, NSApplicationDirectory, NSApplicationScriptsDirectory,
        NSApplicationSupportDirectory, NSAutosavedInformationDirectory, NSCachesDirectory,
        NSCoreServiceDirectory, NSDemoApplicationDirectory, NSDesktopDirectory,
        NSDeveloperApplicationDirectory, NSDeveloperDirectory, NSDocumentDirectory,
        NSDocumentationDirectory, NSDownloadsDirectory, NSInputMethodsDirectory,
        NSItemReplacementDirectory, NSLibraryDirectory, NSLocalDomainMask, NSMoviesDirectory,
        NSMusicDirectory, NSNetworkDomainMask, NSPicturesDirectory, NSPreferencePanesDirectory,
        NSPrinterDescriptionDirectory, NSSearchPathDirectory, NSSearchPathDomainMask,
        NSSharedPublicDirectory, NSSystemDomainMask, NSTrashDirectory, NSUserDirectory,
        NSUserDomainMask,
    };
    pub use super::NSPersonNameComponents::NSPersonNameComponents;
    pub use super::NSPersonNameComponentsFormatter::{
        NSPersonNameComponentDelimiter, NSPersonNameComponentFamilyName,
        NSPersonNameComponentGivenName, NSPersonNameComponentKey, NSPersonNameComponentMiddleName,
        NSPersonNameComponentNickname, NSPersonNameComponentPrefix, NSPersonNameComponentSuffix,
        NSPersonNameComponentsFormatter, NSPersonNameComponentsFormatterOptions,
        NSPersonNameComponentsFormatterPhonetic, NSPersonNameComponentsFormatterStyle,
        NSPersonNameComponentsFormatterStyleAbbreviated,
        NSPersonNameComponentsFormatterStyleDefault, NSPersonNameComponentsFormatterStyleLong,
        NSPersonNameComponentsFormatterStyleMedium, NSPersonNameComponentsFormatterStyleShort,
    };
    pub use super::NSPointerArray::NSPointerArray;
    pub use super::NSPointerFunctions::{
        NSPointerFunctions, NSPointerFunctionsCStringPersonality, NSPointerFunctionsCopyIn,
        NSPointerFunctionsIntegerPersonality, NSPointerFunctionsMachVirtualMemory,
        NSPointerFunctionsMallocMemory, NSPointerFunctionsObjectPersonality,
        NSPointerFunctionsObjectPointerPersonality, NSPointerFunctionsOpaqueMemory,
        NSPointerFunctionsOpaquePersonality, NSPointerFunctionsOptions,
        NSPointerFunctionsStrongMemory, NSPointerFunctionsStructPersonality,
        NSPointerFunctionsWeakMemory, NSPointerFunctionsZeroingWeakMemory,
    };
    pub use super::NSPort::{
        NSMachPort, NSMachPortDeallocateNone, NSMachPortDeallocateReceiveRight,
        NSMachPortDeallocateSendRight, NSMachPortDelegate, NSMachPortOptions, NSMessagePort,
        NSPort, NSPortDelegate, NSPortDidBecomeInvalidNotification, NSSocketPort,
    };
    pub use super::NSPortCoder::NSPortCoder;
    pub use super::NSPortMessage::NSPortMessage;
    pub use super::NSPortNameServer::{
        NSMachBootstrapServer, NSMessagePortNameServer, NSPortNameServer, NSSocketPortNameServer,
    };
    pub use super::NSPredicate::NSPredicate;
    pub use super::NSProcessInfo::{
        NSActivityAutomaticTerminationDisabled, NSActivityBackground,
        NSActivityIdleDisplaySleepDisabled, NSActivityIdleSystemSleepDisabled,
        NSActivityLatencyCritical, NSActivityOptions, NSActivitySuddenTerminationDisabled,
        NSActivityUserInitiated, NSActivityUserInitiatedAllowingIdleSystemSleep,
        NSHPUXOperatingSystem, NSMACHOperatingSystem, NSOSF1OperatingSystem, NSProcessInfo,
        NSProcessInfoPowerStateDidChangeNotification, NSProcessInfoThermalState,
        NSProcessInfoThermalStateCritical, NSProcessInfoThermalStateDidChangeNotification,
        NSProcessInfoThermalStateFair, NSProcessInfoThermalStateNominal,
        NSProcessInfoThermalStateSerious, NSSolarisOperatingSystem, NSSunOSOperatingSystem,
        NSWindows95OperatingSystem, NSWindowsNTOperatingSystem,
    };
    pub use super::NSProgress::{
        NSProgress, NSProgressEstimatedTimeRemainingKey, NSProgressFileAnimationImageKey,
        NSProgressFileAnimationImageOriginalRectKey, NSProgressFileCompletedCountKey,
        NSProgressFileIconKey, NSProgressFileOperationKind, NSProgressFileOperationKindCopying,
        NSProgressFileOperationKindDecompressingAfterDownloading,
        NSProgressFileOperationKindDownloading, NSProgressFileOperationKindDuplicating,
        NSProgressFileOperationKindKey, NSProgressFileOperationKindReceiving,
        NSProgressFileOperationKindUploading, NSProgressFileTotalCountKey, NSProgressFileURLKey,
        NSProgressKind, NSProgressKindFile, NSProgressReporting, NSProgressThroughputKey,
        NSProgressUserInfoKey,
    };
    pub use super::NSPropertyList::{
        NSPropertyListBinaryFormat_v1_0, NSPropertyListFormat, NSPropertyListImmutable,
        NSPropertyListMutabilityOptions, NSPropertyListMutableContainers,
        NSPropertyListMutableContainersAndLeaves, NSPropertyListOpenStepFormat,
        NSPropertyListReadOptions, NSPropertyListSerialization, NSPropertyListWriteOptions,
        NSPropertyListXMLFormat_v1_0,
    };
    pub use super::NSProtocolChecker::NSProtocolChecker;
    pub use super::NSRegularExpression::{
        NSDataDetector, NSMatchingAnchored, NSMatchingCompleted, NSMatchingFlags, NSMatchingHitEnd,
        NSMatchingInternalError, NSMatchingOptions, NSMatchingProgress, NSMatchingReportCompletion,
        NSMatchingReportProgress, NSMatchingRequiredEnd, NSMatchingWithTransparentBounds,
        NSMatchingWithoutAnchoringBounds, NSRegularExpression,
        NSRegularExpressionAllowCommentsAndWhitespace, NSRegularExpressionAnchorsMatchLines,
        NSRegularExpressionCaseInsensitive, NSRegularExpressionDotMatchesLineSeparators,
        NSRegularExpressionIgnoreMetacharacters, NSRegularExpressionOptions,
        NSRegularExpressionUseUnicodeWordBoundaries, NSRegularExpressionUseUnixLineSeparators,
    };
    pub use super::NSRelativeDateTimeFormatter::{
        NSRelativeDateTimeFormatter, NSRelativeDateTimeFormatterStyle,
        NSRelativeDateTimeFormatterStyleNamed, NSRelativeDateTimeFormatterStyleNumeric,
        NSRelativeDateTimeFormatterUnitsStyle, NSRelativeDateTimeFormatterUnitsStyleAbbreviated,
        NSRelativeDateTimeFormatterUnitsStyleFull, NSRelativeDateTimeFormatterUnitsStyleShort,
        NSRelativeDateTimeFormatterUnitsStyleSpellOut,
    };
    pub use super::NSRunLoop::{NSDefaultRunLoopMode, NSRunLoop, NSRunLoopCommonModes};
    pub use super::NSScanner::NSScanner;
    pub use super::NSScriptClassDescription::NSScriptClassDescription;
    pub use super::NSScriptCoercionHandler::NSScriptCoercionHandler;
    pub use super::NSScriptCommand::{
        NSArgumentEvaluationScriptError, NSArgumentsWrongScriptError,
        NSCannotCreateScriptCommandError, NSInternalScriptError,
        NSKeySpecifierEvaluationScriptError, NSNoScriptError,
        NSOperationNotSupportedForKeyScriptError, NSReceiverEvaluationScriptError,
        NSReceiversCantHandleCommandScriptError, NSRequiredArgumentsMissingScriptError,
        NSScriptCommand, NSUnknownKeyScriptError,
    };
    pub use super::NSScriptCommandDescription::NSScriptCommandDescription;
    pub use super::NSScriptExecutionContext::NSScriptExecutionContext;
    pub use super::NSScriptKeyValueCoding::NSOperationNotSupportedForKeyException;
    pub use super::NSScriptObjectSpecifiers::{
        NSContainerSpecifierError, NSEverySubelement, NSIndexSpecifier, NSIndexSubelement,
        NSInsertionPosition, NSInternalSpecifierError, NSInvalidIndexSpecifierError,
        NSMiddleSpecifier, NSMiddleSubelement, NSNameSpecifier, NSNoSpecifierError, NSNoSubelement,
        NSNoTopLevelContainersSpecifierError, NSOperationNotSupportedForKeySpecifierError,
        NSPositionAfter, NSPositionBefore, NSPositionBeginning, NSPositionEnd, NSPositionReplace,
        NSPositionalSpecifier, NSPropertySpecifier, NSRandomSpecifier, NSRandomSubelement,
        NSRangeSpecifier, NSRelativeAfter, NSRelativeBefore, NSRelativePosition,
        NSRelativeSpecifier, NSScriptObjectSpecifier, NSUniqueIDSpecifier,
        NSUnknownKeySpecifierError, NSWhoseSpecifier, NSWhoseSubelementIdentifier,
    };
    pub use super::NSScriptStandardSuiteCommands::{
        NSCloneCommand, NSCloseCommand, NSCountCommand, NSCreateCommand, NSDeleteCommand,
        NSExistsCommand, NSGetCommand, NSMoveCommand, NSQuitCommand, NSSaveOptions,
        NSSaveOptionsAsk, NSSaveOptionsNo, NSSaveOptionsYes, NSSetCommand,
    };
    pub use super::NSScriptSuiteRegistry::NSScriptSuiteRegistry;
    pub use super::NSScriptWhoseTests::{
        NSBeginsWithComparison, NSContainsComparison, NSEndsWithComparison, NSEqualToComparison,
        NSGreaterThanComparison, NSGreaterThanOrEqualToComparison, NSLessThanComparison,
        NSLessThanOrEqualToComparison, NSLogicalTest, NSScriptWhoseTest, NSSpecifierTest,
        NSTestComparisonOperation,
    };
    pub use super::NSSet::{NSCountedSet, NSMutableSet, NSSet};
    pub use super::NSSortDescriptor::NSSortDescriptor;
    pub use super::NSSpellServer::{
        NSGrammarCorrections, NSGrammarRange, NSGrammarUserDescription, NSSpellServer,
        NSSpellServerDelegate,
    };
    pub use super::NSStream::{
        NSInputStream, NSOutputStream, NSStream, NSStreamDataWrittenToMemoryStreamKey,
        NSStreamDelegate, NSStreamEvent, NSStreamEventEndEncountered, NSStreamEventErrorOccurred,
        NSStreamEventHasBytesAvailable, NSStreamEventHasSpaceAvailable, NSStreamEventNone,
        NSStreamEventOpenCompleted, NSStreamFileCurrentOffsetKey, NSStreamNetworkServiceType,
        NSStreamNetworkServiceTypeBackground, NSStreamNetworkServiceTypeCallSignaling,
        NSStreamNetworkServiceTypeValue, NSStreamNetworkServiceTypeVideo,
        NSStreamNetworkServiceTypeVoIP, NSStreamNetworkServiceTypeVoice, NSStreamPropertyKey,
        NSStreamSOCKSErrorDomain, NSStreamSOCKSProxyConfiguration,
        NSStreamSOCKSProxyConfigurationKey, NSStreamSOCKSProxyHostKey,
        NSStreamSOCKSProxyPasswordKey, NSStreamSOCKSProxyPortKey, NSStreamSOCKSProxyUserKey,
        NSStreamSOCKSProxyVersion, NSStreamSOCKSProxyVersion4, NSStreamSOCKSProxyVersion5,
        NSStreamSOCKSProxyVersionKey, NSStreamSocketSSLErrorDomain, NSStreamSocketSecurityLevel,
        NSStreamSocketSecurityLevelKey, NSStreamSocketSecurityLevelNegotiatedSSL,
        NSStreamSocketSecurityLevelNone, NSStreamSocketSecurityLevelSSLv2,
        NSStreamSocketSecurityLevelSSLv3, NSStreamSocketSecurityLevelTLSv1, NSStreamStatus,
        NSStreamStatusAtEnd, NSStreamStatusClosed, NSStreamStatusError, NSStreamStatusNotOpen,
        NSStreamStatusOpen, NSStreamStatusOpening, NSStreamStatusReading, NSStreamStatusWriting,
    };
    pub use super::NSString::{
        NSASCIIStringEncoding, NSAnchoredSearch, NSBackwardsSearch, NSCaseInsensitiveSearch,
        NSCharacterConversionException, NSConstantString, NSDiacriticInsensitiveSearch,
        NSForcedOrderingSearch, NSISO2022JPStringEncoding, NSISOLatin1StringEncoding,
        NSISOLatin2StringEncoding, NSJapaneseEUCStringEncoding, NSLiteralSearch,
        NSMacOSRomanStringEncoding, NSMutableString, NSNEXTSTEPStringEncoding,
        NSNonLossyASCIIStringEncoding, NSNumericSearch, NSParseErrorException,
        NSProprietaryStringEncoding, NSRegularExpressionSearch, NSShiftJISStringEncoding,
        NSSimpleCString, NSString, NSStringCompareOptions, NSStringEncoding,
        NSStringEncodingConversionAllowLossy, NSStringEncodingConversionExternalRepresentation,
        NSStringEncodingConversionOptions, NSStringEncodingDetectionAllowLossyKey,
        NSStringEncodingDetectionDisallowedEncodingsKey, NSStringEncodingDetectionFromWindowsKey,
        NSStringEncodingDetectionLikelyLanguageKey, NSStringEncodingDetectionLossySubstitutionKey,
        NSStringEncodingDetectionOptionsKey, NSStringEncodingDetectionSuggestedEncodingsKey,
        NSStringEncodingDetectionUseOnlySuggestedEncodingsKey, NSStringEnumerationByCaretPositions,
        NSStringEnumerationByComposedCharacterSequences, NSStringEnumerationByDeletionClusters,
        NSStringEnumerationByLines, NSStringEnumerationByParagraphs,
        NSStringEnumerationBySentences, NSStringEnumerationByWords, NSStringEnumerationLocalized,
        NSStringEnumerationOptions, NSStringEnumerationReverse,
        NSStringEnumerationSubstringNotRequired, NSStringTransform,
        NSStringTransformFullwidthToHalfwidth, NSStringTransformHiraganaToKatakana,
        NSStringTransformLatinToArabic, NSStringTransformLatinToCyrillic,
        NSStringTransformLatinToGreek, NSStringTransformLatinToHangul,
        NSStringTransformLatinToHebrew, NSStringTransformLatinToHiragana,
        NSStringTransformLatinToKatakana, NSStringTransformLatinToThai,
        NSStringTransformMandarinToLatin, NSStringTransformStripCombiningMarks,
        NSStringTransformStripDiacritics, NSStringTransformToLatin, NSStringTransformToUnicodeName,
        NSStringTransformToXMLHex, NSSymbolStringEncoding, NSUTF16BigEndianStringEncoding,
        NSUTF16LittleEndianStringEncoding, NSUTF16StringEncoding, NSUTF32BigEndianStringEncoding,
        NSUTF32LittleEndianStringEncoding, NSUTF32StringEncoding, NSUTF8StringEncoding,
        NSUnicodeStringEncoding, NSWidthInsensitiveSearch, NSWindowsCP1250StringEncoding,
        NSWindowsCP1251StringEncoding, NSWindowsCP1252StringEncoding,
        NSWindowsCP1253StringEncoding, NSWindowsCP1254StringEncoding,
    };
    pub use super::NSTask::{
        NSTask, NSTaskDidTerminateNotification, NSTaskTerminationReason,
        NSTaskTerminationReasonExit, NSTaskTerminationReasonUncaughtSignal,
    };
    pub use super::NSTextCheckingResult::{
        NSTextCheckingAirlineKey, NSTextCheckingAllCustomTypes, NSTextCheckingAllSystemTypes,
        NSTextCheckingAllTypes, NSTextCheckingCityKey, NSTextCheckingCountryKey,
        NSTextCheckingFlightKey, NSTextCheckingJobTitleKey, NSTextCheckingKey,
        NSTextCheckingNameKey, NSTextCheckingOrganizationKey, NSTextCheckingPhoneKey,
        NSTextCheckingResult, NSTextCheckingStateKey, NSTextCheckingStreetKey, NSTextCheckingType,
        NSTextCheckingTypeAddress, NSTextCheckingTypeCorrection, NSTextCheckingTypeDash,
        NSTextCheckingTypeDate, NSTextCheckingTypeGrammar, NSTextCheckingTypeLink,
        NSTextCheckingTypeOrthography, NSTextCheckingTypePhoneNumber, NSTextCheckingTypeQuote,
        NSTextCheckingTypeRegularExpression, NSTextCheckingTypeReplacement,
        NSTextCheckingTypeSpelling, NSTextCheckingTypeTransitInformation, NSTextCheckingTypes,
        NSTextCheckingZIPKey,
    };
    pub use super::NSThread::{
        NSDidBecomeSingleThreadedNotification, NSThread, NSThreadWillExitNotification,
        NSWillBecomeMultiThreadedNotification,
    };
    pub use super::NSTimeZone::{
        NSSystemTimeZoneDidChangeNotification, NSTimeZone, NSTimeZoneNameStyle,
        NSTimeZoneNameStyleDaylightSaving, NSTimeZoneNameStyleGeneric,
        NSTimeZoneNameStyleShortDaylightSaving, NSTimeZoneNameStyleShortGeneric,
        NSTimeZoneNameStyleShortStandard, NSTimeZoneNameStyleStandard,
    };
    pub use super::NSTimer::NSTimer;
    pub use super::NSURLAuthenticationChallenge::{
        NSURLAuthenticationChallenge, NSURLAuthenticationChallengeSender,
    };
    pub use super::NSURLCache::{
        NSCachedURLResponse, NSURLCache, NSURLCacheStorageAllowed,
        NSURLCacheStorageAllowedInMemoryOnly, NSURLCacheStorageNotAllowed, NSURLCacheStoragePolicy,
    };
    pub use super::NSURLConnection::{
        NSURLConnection, NSURLConnectionDataDelegate, NSURLConnectionDelegate,
        NSURLConnectionDownloadDelegate,
    };
    pub use super::NSURLCredential::{
        NSURLCredential, NSURLCredentialPersistence, NSURLCredentialPersistenceForSession,
        NSURLCredentialPersistenceNone, NSURLCredentialPersistencePermanent,
        NSURLCredentialPersistenceSynchronizable,
    };
    pub use super::NSURLCredentialStorage::{
        NSURLCredentialStorage, NSURLCredentialStorageChangedNotification,
        NSURLCredentialStorageRemoveSynchronizableCredentials,
    };
    pub use super::NSURLDownload::{NSURLDownload, NSURLDownloadDelegate};
    pub use super::NSURLError::{
        NSErrorFailingURLStringKey, NSURLErrorAppTransportSecurityRequiresSecureConnection,
        NSURLErrorBackgroundSessionInUseByAnotherProcess,
        NSURLErrorBackgroundSessionRequiresSharedContainer,
        NSURLErrorBackgroundSessionWasDisconnected, NSURLErrorBackgroundTaskCancelledReasonKey,
        NSURLErrorBadServerResponse, NSURLErrorBadURL, NSURLErrorCallIsActive, NSURLErrorCancelled,
        NSURLErrorCancelledReasonBackgroundUpdatesDisabled,
        NSURLErrorCancelledReasonInsufficientSystemResources,
        NSURLErrorCancelledReasonUserForceQuitApplication, NSURLErrorCannotCloseFile,
        NSURLErrorCannotConnectToHost, NSURLErrorCannotCreateFile,
        NSURLErrorCannotDecodeContentData, NSURLErrorCannotDecodeRawData, NSURLErrorCannotFindHost,
        NSURLErrorCannotLoadFromNetwork, NSURLErrorCannotMoveFile, NSURLErrorCannotOpenFile,
        NSURLErrorCannotParseResponse, NSURLErrorCannotRemoveFile, NSURLErrorCannotWriteToFile,
        NSURLErrorClientCertificateRejected, NSURLErrorClientCertificateRequired,
        NSURLErrorDNSLookupFailed, NSURLErrorDataLengthExceedsMaximum, NSURLErrorDataNotAllowed,
        NSURLErrorDomain, NSURLErrorDownloadDecodingFailedMidStream,
        NSURLErrorDownloadDecodingFailedToComplete, NSURLErrorFailingURLErrorKey,
        NSURLErrorFailingURLPeerTrustErrorKey, NSURLErrorFailingURLStringErrorKey,
        NSURLErrorFileDoesNotExist, NSURLErrorFileIsDirectory, NSURLErrorFileOutsideSafeArea,
        NSURLErrorHTTPTooManyRedirects, NSURLErrorInternationalRoamingOff,
        NSURLErrorNetworkConnectionLost, NSURLErrorNetworkUnavailableReason,
        NSURLErrorNetworkUnavailableReasonCellular, NSURLErrorNetworkUnavailableReasonConstrained,
        NSURLErrorNetworkUnavailableReasonExpensive, NSURLErrorNetworkUnavailableReasonKey,
        NSURLErrorNoPermissionsToReadFile, NSURLErrorNotConnectedToInternet,
        NSURLErrorRedirectToNonExistentLocation, NSURLErrorRequestBodyStreamExhausted,
        NSURLErrorResourceUnavailable, NSURLErrorSecureConnectionFailed,
        NSURLErrorServerCertificateHasBadDate, NSURLErrorServerCertificateHasUnknownRoot,
        NSURLErrorServerCertificateNotYetValid, NSURLErrorServerCertificateUntrusted,
        NSURLErrorTimedOut, NSURLErrorUnknown, NSURLErrorUnsupportedURL,
        NSURLErrorUserAuthenticationRequired, NSURLErrorUserCancelledAuthentication,
        NSURLErrorZeroByteResource,
    };
    pub use super::NSURLHandle::{
        NSFTPPropertyActiveTransferModeKey, NSFTPPropertyFTPProxy, NSFTPPropertyFileOffsetKey,
        NSFTPPropertyUserLoginKey, NSFTPPropertyUserPasswordKey, NSHTTPPropertyErrorPageDataKey,
        NSHTTPPropertyHTTPProxy, NSHTTPPropertyRedirectionHeadersKey,
        NSHTTPPropertyServerHTTPVersionKey, NSHTTPPropertyStatusCodeKey,
        NSHTTPPropertyStatusReasonKey, NSURLHandle, NSURLHandleClient, NSURLHandleLoadFailed,
        NSURLHandleLoadInProgress, NSURLHandleLoadSucceeded, NSURLHandleNotLoaded,
        NSURLHandleStatus,
    };
    pub use super::NSURLProtectionSpace::{
        NSURLAuthenticationMethodClientCertificate, NSURLAuthenticationMethodDefault,
        NSURLAuthenticationMethodHTMLForm, NSURLAuthenticationMethodHTTPBasic,
        NSURLAuthenticationMethodHTTPDigest, NSURLAuthenticationMethodNTLM,
        NSURLAuthenticationMethodNegotiate, NSURLAuthenticationMethodServerTrust,
        NSURLProtectionSpace, NSURLProtectionSpaceFTP, NSURLProtectionSpaceFTPProxy,
        NSURLProtectionSpaceHTTP, NSURLProtectionSpaceHTTPProxy, NSURLProtectionSpaceHTTPS,
        NSURLProtectionSpaceHTTPSProxy, NSURLProtectionSpaceSOCKSProxy,
    };
    pub use super::NSURLProtocol::{NSURLProtocol, NSURLProtocolClient};
    pub use super::NSURLRequest::{
        NSMutableURLRequest, NSURLNetworkServiceTypeAVStreaming, NSURLNetworkServiceTypeBackground,
        NSURLNetworkServiceTypeCallSignaling, NSURLNetworkServiceTypeDefault,
        NSURLNetworkServiceTypeResponsiveAV, NSURLNetworkServiceTypeResponsiveData,
        NSURLNetworkServiceTypeVideo, NSURLNetworkServiceTypeVoIP, NSURLNetworkServiceTypeVoice,
        NSURLRequest, NSURLRequestAttribution, NSURLRequestAttributionDeveloper,
        NSURLRequestAttributionUser, NSURLRequestCachePolicy, NSURLRequestNetworkServiceType,
        NSURLRequestReloadIgnoringCacheData, NSURLRequestReloadIgnoringLocalAndRemoteCacheData,
        NSURLRequestReloadIgnoringLocalCacheData, NSURLRequestReloadRevalidatingCacheData,
        NSURLRequestReturnCacheDataDontLoad, NSURLRequestReturnCacheDataElseLoad,
        NSURLRequestUseProtocolCachePolicy,
    };
    pub use super::NSURLResponse::{NSHTTPURLResponse, NSURLResponse};
    pub use super::NSURLSession::{
        NSURLSession, NSURLSessionAuthChallengeCancelAuthenticationChallenge,
        NSURLSessionAuthChallengeDisposition, NSURLSessionAuthChallengePerformDefaultHandling,
        NSURLSessionAuthChallengeRejectProtectionSpace, NSURLSessionAuthChallengeUseCredential,
        NSURLSessionConfiguration, NSURLSessionDataDelegate, NSURLSessionDataTask,
        NSURLSessionDelayedRequestCancel, NSURLSessionDelayedRequestContinueLoading,
        NSURLSessionDelayedRequestDisposition, NSURLSessionDelayedRequestUseNewRequest,
        NSURLSessionDelegate, NSURLSessionDownloadDelegate, NSURLSessionDownloadTask,
        NSURLSessionDownloadTaskResumeData, NSURLSessionMultipathServiceType,
        NSURLSessionMultipathServiceTypeAggregate, NSURLSessionMultipathServiceTypeHandover,
        NSURLSessionMultipathServiceTypeInteractive, NSURLSessionMultipathServiceTypeNone,
        NSURLSessionResponseAllow, NSURLSessionResponseBecomeDownload,
        NSURLSessionResponseBecomeStream, NSURLSessionResponseCancel,
        NSURLSessionResponseDisposition, NSURLSessionStreamDelegate, NSURLSessionStreamTask,
        NSURLSessionTask, NSURLSessionTaskDelegate, NSURLSessionTaskMetrics,
        NSURLSessionTaskMetricsDomainResolutionProtocol,
        NSURLSessionTaskMetricsDomainResolutionProtocolHTTPS,
        NSURLSessionTaskMetricsDomainResolutionProtocolTCP,
        NSURLSessionTaskMetricsDomainResolutionProtocolTLS,
        NSURLSessionTaskMetricsDomainResolutionProtocolUDP,
        NSURLSessionTaskMetricsDomainResolutionProtocolUnknown,
        NSURLSessionTaskMetricsResourceFetchType,
        NSURLSessionTaskMetricsResourceFetchTypeLocalCache,
        NSURLSessionTaskMetricsResourceFetchTypeNetworkLoad,
        NSURLSessionTaskMetricsResourceFetchTypeServerPush,
        NSURLSessionTaskMetricsResourceFetchTypeUnknown, NSURLSessionTaskPriorityDefault,
        NSURLSessionTaskPriorityHigh, NSURLSessionTaskPriorityLow, NSURLSessionTaskState,
        NSURLSessionTaskStateCanceling, NSURLSessionTaskStateCompleted,
        NSURLSessionTaskStateRunning, NSURLSessionTaskStateSuspended,
        NSURLSessionTaskTransactionMetrics, NSURLSessionTransferSizeUnknown,
        NSURLSessionUploadTask, NSURLSessionWebSocketCloseCode,
        NSURLSessionWebSocketCloseCodeAbnormalClosure, NSURLSessionWebSocketCloseCodeGoingAway,
        NSURLSessionWebSocketCloseCodeInternalServerError, NSURLSessionWebSocketCloseCodeInvalid,
        NSURLSessionWebSocketCloseCodeInvalidFramePayloadData,
        NSURLSessionWebSocketCloseCodeMandatoryExtensionMissing,
        NSURLSessionWebSocketCloseCodeMessageTooBig,
        NSURLSessionWebSocketCloseCodeNoStatusReceived,
        NSURLSessionWebSocketCloseCodeNormalClosure, NSURLSessionWebSocketCloseCodePolicyViolation,
        NSURLSessionWebSocketCloseCodeProtocolError,
        NSURLSessionWebSocketCloseCodeTLSHandshakeFailure,
        NSURLSessionWebSocketCloseCodeUnsupportedData, NSURLSessionWebSocketDelegate,
        NSURLSessionWebSocketMessage, NSURLSessionWebSocketMessageType,
        NSURLSessionWebSocketMessageTypeData, NSURLSessionWebSocketMessageTypeString,
        NSURLSessionWebSocketTask,
    };
    pub use super::NSUbiquitousKeyValueStore::{
        NSUbiquitousKeyValueStore, NSUbiquitousKeyValueStoreAccountChange,
        NSUbiquitousKeyValueStoreChangeReasonKey, NSUbiquitousKeyValueStoreChangedKeysKey,
        NSUbiquitousKeyValueStoreDidChangeExternallyNotification,
        NSUbiquitousKeyValueStoreInitialSyncChange, NSUbiquitousKeyValueStoreQuotaViolationChange,
        NSUbiquitousKeyValueStoreServerChange,
    };
    pub use super::NSUndoManager::{
        NSUndoCloseGroupingRunLoopOrdering, NSUndoManager, NSUndoManagerCheckpointNotification,
        NSUndoManagerDidCloseUndoGroupNotification, NSUndoManagerDidOpenUndoGroupNotification,
        NSUndoManagerDidRedoChangeNotification, NSUndoManagerDidUndoChangeNotification,
        NSUndoManagerGroupIsDiscardableKey, NSUndoManagerWillCloseUndoGroupNotification,
        NSUndoManagerWillRedoChangeNotification, NSUndoManagerWillUndoChangeNotification,
    };
    pub use super::NSUnit::{
        NSDimension, NSUnit, NSUnitAcceleration, NSUnitAngle, NSUnitArea, NSUnitConcentrationMass,
        NSUnitConverter, NSUnitConverterLinear, NSUnitDispersion, NSUnitDuration,
        NSUnitElectricCharge, NSUnitElectricCurrent, NSUnitElectricPotentialDifference,
        NSUnitElectricResistance, NSUnitEnergy, NSUnitFrequency, NSUnitFuelEfficiency,
        NSUnitIlluminance, NSUnitInformationStorage, NSUnitLength, NSUnitMass, NSUnitPower,
        NSUnitPressure, NSUnitSpeed, NSUnitTemperature, NSUnitVolume,
    };
    pub use super::NSUserActivity::{
        NSUserActivity, NSUserActivityDelegate, NSUserActivityPersistentIdentifier,
        NSUserActivityTypeBrowsingWeb,
    };
    pub use super::NSUserDefaults::{
        NSAMPMDesignation, NSArgumentDomain, NSCurrencySymbol, NSDateFormatString,
        NSDateTimeOrdering, NSDecimalDigits, NSDecimalSeparator, NSEarlierTimeDesignations,
        NSGlobalDomain, NSHourNameDesignations, NSInternationalCurrencyString,
        NSLaterTimeDesignations, NSMonthNameArray, NSNegativeCurrencyFormatString,
        NSNextDayDesignations, NSNextNextDayDesignations, NSPositiveCurrencyFormatString,
        NSPriorDayDesignations, NSRegistrationDomain, NSShortDateFormatString,
        NSShortMonthNameArray, NSShortTimeDateFormatString, NSShortWeekDayNameArray,
        NSThisDayDesignations, NSThousandsSeparator, NSTimeDateFormatString, NSTimeFormatString,
        NSUbiquitousUserDefaultsCompletedInitialSyncNotification,
        NSUbiquitousUserDefaultsDidChangeAccountsNotification,
        NSUbiquitousUserDefaultsNoCloudAccountNotification, NSUserDefaults,
        NSUserDefaultsDidChangeNotification, NSUserDefaultsSizeLimitExceededNotification,
        NSWeekDayNameArray, NSYearMonthWeekDesignations,
    };
    pub use super::NSUserNotification::{
        NSUserNotification, NSUserNotificationAction, NSUserNotificationActivationType,
        NSUserNotificationActivationTypeActionButtonClicked,
        NSUserNotificationActivationTypeAdditionalActionClicked,
        NSUserNotificationActivationTypeContentsClicked, NSUserNotificationActivationTypeNone,
        NSUserNotificationActivationTypeReplied, NSUserNotificationCenter,
        NSUserNotificationCenterDelegate, NSUserNotificationDefaultSoundName,
    };
    pub use super::NSUserScriptTask::{
        NSUserAppleScriptTask, NSUserAutomatorTask, NSUserScriptTask, NSUserUnixTask,
    };
    pub use super::NSValue::{NSNumber, NSValue};
    pub use super::NSValueTransformer::{
        NSIsNilTransformerName, NSIsNotNilTransformerName, NSKeyedUnarchiveFromDataTransformerName,
        NSNegateBooleanTransformerName, NSSecureUnarchiveFromDataTransformer,
        NSSecureUnarchiveFromDataTransformerName, NSUnarchiveFromDataTransformerName,
        NSValueTransformer, NSValueTransformerName,
    };
    pub use super::NSXMLDTDNode::{
        NSXMLAttributeCDATAKind, NSXMLAttributeEntitiesKind, NSXMLAttributeEntityKind,
        NSXMLAttributeEnumerationKind, NSXMLAttributeIDKind, NSXMLAttributeIDRefKind,
        NSXMLAttributeIDRefsKind, NSXMLAttributeNMTokenKind, NSXMLAttributeNMTokensKind,
        NSXMLAttributeNotationKind, NSXMLDTDNode, NSXMLDTDNodeKind, NSXMLElementDeclarationAnyKind,
        NSXMLElementDeclarationElementKind, NSXMLElementDeclarationEmptyKind,
        NSXMLElementDeclarationMixedKind, NSXMLElementDeclarationUndefinedKind,
        NSXMLEntityGeneralKind, NSXMLEntityParameterKind, NSXMLEntityParsedKind,
        NSXMLEntityPredefined, NSXMLEntityUnparsedKind,
    };
    pub use super::NSXMLDocument::{
        NSXMLDocument, NSXMLDocumentContentKind, NSXMLDocumentHTMLKind, NSXMLDocumentTextKind,
        NSXMLDocumentXHTMLKind, NSXMLDocumentXMLKind,
    };
    pub use super::NSXMLElement::NSXMLElement;
    pub use super::NSXMLNode::{
        NSXMLAttributeDeclarationKind, NSXMLAttributeKind, NSXMLCommentKind, NSXMLDTDKind,
        NSXMLDocumentKind, NSXMLElementDeclarationKind, NSXMLElementKind,
        NSXMLEntityDeclarationKind, NSXMLInvalidKind, NSXMLNamespaceKind, NSXMLNode, NSXMLNodeKind,
        NSXMLNotationDeclarationKind, NSXMLProcessingInstructionKind, NSXMLTextKind,
    };
    pub use super::NSXMLNodeOptions::{
        NSXMLDocumentIncludeContentTypeDeclaration, NSXMLDocumentTidyHTML, NSXMLDocumentTidyXML,
        NSXMLDocumentValidate, NSXMLDocumentXInclude, NSXMLNodeCompactEmptyElement,
        NSXMLNodeExpandEmptyElement, NSXMLNodeIsCDATA, NSXMLNodeLoadExternalEntitiesAlways,
        NSXMLNodeLoadExternalEntitiesNever, NSXMLNodeLoadExternalEntitiesSameOriginOnly,
        NSXMLNodeNeverEscapeContents, NSXMLNodeOptions, NSXMLNodeOptionsNone, NSXMLNodePreserveAll,
        NSXMLNodePreserveAttributeOrder, NSXMLNodePreserveCDATA,
        NSXMLNodePreserveCharacterReferences, NSXMLNodePreserveDTD, NSXMLNodePreserveEmptyElements,
        NSXMLNodePreserveEntities, NSXMLNodePreserveNamespaceOrder, NSXMLNodePreservePrefixes,
        NSXMLNodePreserveQuotes, NSXMLNodePreserveWhitespace, NSXMLNodePrettyPrint,
        NSXMLNodePromoteSignificantWhitespace, NSXMLNodeUseDoubleQuotes, NSXMLNodeUseSingleQuotes,
    };
    pub use super::NSXMLParser::{
        NSXMLParser, NSXMLParserAttributeHasNoValueError, NSXMLParserAttributeListNotFinishedError,
        NSXMLParserAttributeListNotStartedError, NSXMLParserAttributeNotFinishedError,
        NSXMLParserAttributeNotStartedError, NSXMLParserAttributeRedefinedError,
        NSXMLParserCDATANotFinishedError, NSXMLParserCharacterRefAtEOFError,
        NSXMLParserCharacterRefInDTDError, NSXMLParserCharacterRefInEpilogError,
        NSXMLParserCharacterRefInPrologError, NSXMLParserCommentContainsDoubleHyphenError,
        NSXMLParserCommentNotFinishedError, NSXMLParserConditionalSectionNotFinishedError,
        NSXMLParserConditionalSectionNotStartedError, NSXMLParserDOCTYPEDeclNotFinishedError,
        NSXMLParserDelegate, NSXMLParserDelegateAbortedParseError, NSXMLParserDocumentStartError,
        NSXMLParserElementContentDeclNotFinishedError,
        NSXMLParserElementContentDeclNotStartedError, NSXMLParserEmptyDocumentError,
        NSXMLParserEncodingNotSupportedError, NSXMLParserEntityBoundaryError,
        NSXMLParserEntityIsExternalError, NSXMLParserEntityIsParameterError,
        NSXMLParserEntityNotFinishedError, NSXMLParserEntityNotStartedError,
        NSXMLParserEntityRefAtEOFError, NSXMLParserEntityRefInDTDError,
        NSXMLParserEntityRefInEpilogError, NSXMLParserEntityRefInPrologError,
        NSXMLParserEntityRefLoopError, NSXMLParserEntityReferenceMissingSemiError,
        NSXMLParserEntityReferenceWithoutNameError, NSXMLParserEntityValueRequiredError,
        NSXMLParserEqualExpectedError, NSXMLParserError, NSXMLParserErrorDomain,
        NSXMLParserExternalEntityResolvingPolicy, NSXMLParserExternalStandaloneEntityError,
        NSXMLParserExternalSubsetNotFinishedError, NSXMLParserExtraContentError,
        NSXMLParserGTRequiredError, NSXMLParserInternalError, NSXMLParserInvalidCharacterError,
        NSXMLParserInvalidCharacterInEntityError, NSXMLParserInvalidCharacterRefError,
        NSXMLParserInvalidConditionalSectionError, NSXMLParserInvalidDecimalCharacterRefError,
        NSXMLParserInvalidEncodingError, NSXMLParserInvalidEncodingNameError,
        NSXMLParserInvalidHexCharacterRefError, NSXMLParserInvalidURIError,
        NSXMLParserLTRequiredError, NSXMLParserLTSlashRequiredError,
        NSXMLParserLessThanSymbolInAttributeError, NSXMLParserLiteralNotFinishedError,
        NSXMLParserLiteralNotStartedError, NSXMLParserMisplacedCDATAEndStringError,
        NSXMLParserMisplacedXMLDeclarationError, NSXMLParserMixedContentDeclNotFinishedError,
        NSXMLParserMixedContentDeclNotStartedError, NSXMLParserNAMERequiredError,
        NSXMLParserNMTOKENRequiredError, NSXMLParserNamespaceDeclarationError,
        NSXMLParserNoDTDError, NSXMLParserNotWellBalancedError,
        NSXMLParserNotationNotFinishedError, NSXMLParserNotationNotStartedError,
        NSXMLParserOutOfMemoryError, NSXMLParserPCDATARequiredError,
        NSXMLParserParsedEntityRefAtEOFError, NSXMLParserParsedEntityRefInEpilogError,
        NSXMLParserParsedEntityRefInInternalError, NSXMLParserParsedEntityRefInInternalSubsetError,
        NSXMLParserParsedEntityRefInPrologError, NSXMLParserParsedEntityRefMissingSemiError,
        NSXMLParserParsedEntityRefNoNameError, NSXMLParserPrematureDocumentEndError,
        NSXMLParserProcessingInstructionNotFinishedError,
        NSXMLParserProcessingInstructionNotStartedError, NSXMLParserPublicIdentifierRequiredError,
        NSXMLParserResolveExternalEntitiesAlways, NSXMLParserResolveExternalEntitiesNever,
        NSXMLParserResolveExternalEntitiesNoNetwork,
        NSXMLParserResolveExternalEntitiesSameOriginOnly, NSXMLParserSeparatorRequiredError,
        NSXMLParserSpaceRequiredError, NSXMLParserStandaloneValueError,
        NSXMLParserStringNotClosedError, NSXMLParserStringNotStartedError,
        NSXMLParserTagNameMismatchError, NSXMLParserURIFragmentError, NSXMLParserURIRequiredError,
        NSXMLParserUndeclaredEntityError, NSXMLParserUnfinishedTagError,
        NSXMLParserUnknownEncodingError, NSXMLParserUnparsedEntityError,
        NSXMLParserXMLDeclNotFinishedError, NSXMLParserXMLDeclNotStartedError,
    };
    pub use super::NSXPCConnection::{
        NSXPCCoder, NSXPCConnection, NSXPCConnectionOptions, NSXPCConnectionPrivileged,
        NSXPCInterface, NSXPCListener, NSXPCListenerDelegate, NSXPCListenerEndpoint,
        NSXPCProxyCreating,
    };
    pub use super::NSZone::{NSCollectorDisabledOption, NSScannedOption};
    pub use super::NSURL::{
        NSFileSecurity, NSThumbnail1024x1024SizeKey, NSURLAddedToDirectoryDateKey,
        NSURLApplicationIsScriptableKey, NSURLAttributeModificationDateKey,
        NSURLBookmarkCreationMinimalBookmark, NSURLBookmarkCreationOptions,
        NSURLBookmarkCreationPreferFileIDResolution,
        NSURLBookmarkCreationSecurityScopeAllowOnlyReadAccess,
        NSURLBookmarkCreationSuitableForBookmarkFile, NSURLBookmarkCreationWithSecurityScope,
        NSURLBookmarkCreationWithoutImplicitSecurityScope, NSURLBookmarkFileCreationOptions,
        NSURLBookmarkResolutionOptions, NSURLBookmarkResolutionWithSecurityScope,
        NSURLBookmarkResolutionWithoutImplicitStartAccessing,
        NSURLBookmarkResolutionWithoutMounting, NSURLBookmarkResolutionWithoutUI,
        NSURLCanonicalPathKey, NSURLComponents, NSURLContentAccessDateKey,
        NSURLContentModificationDateKey, NSURLContentTypeKey, NSURLCreationDateKey,
        NSURLCustomIconKey, NSURLDocumentIdentifierKey, NSURLEffectiveIconKey,
        NSURLFileAllocatedSizeKey, NSURLFileContentIdentifierKey, NSURLFileProtectionComplete,
        NSURLFileProtectionCompleteUnlessOpen,
        NSURLFileProtectionCompleteUntilFirstUserAuthentication, NSURLFileProtectionKey,
        NSURLFileProtectionNone, NSURLFileProtectionType, NSURLFileResourceIdentifierKey,
        NSURLFileResourceType, NSURLFileResourceTypeBlockSpecial,
        NSURLFileResourceTypeCharacterSpecial, NSURLFileResourceTypeDirectory,
        NSURLFileResourceTypeKey, NSURLFileResourceTypeNamedPipe, NSURLFileResourceTypeRegular,
        NSURLFileResourceTypeSocket, NSURLFileResourceTypeSymbolicLink,
        NSURLFileResourceTypeUnknown, NSURLFileScheme, NSURLFileSecurityKey, NSURLFileSizeKey,
        NSURLGenerationIdentifierKey, NSURLHasHiddenExtensionKey, NSURLIsAliasFileKey,
        NSURLIsApplicationKey, NSURLIsDirectoryKey, NSURLIsExcludedFromBackupKey,
        NSURLIsExecutableKey, NSURLIsHiddenKey, NSURLIsMountTriggerKey, NSURLIsPackageKey,
        NSURLIsPurgeableKey, NSURLIsReadableKey, NSURLIsRegularFileKey, NSURLIsSparseKey,
        NSURLIsSymbolicLinkKey, NSURLIsSystemImmutableKey, NSURLIsUbiquitousItemKey,
        NSURLIsUserImmutableKey, NSURLIsVolumeKey, NSURLIsWritableKey, NSURLKeysOfUnsetValuesKey,
        NSURLLabelColorKey, NSURLLabelNumberKey, NSURLLinkCountKey, NSURLLocalizedLabelKey,
        NSURLLocalizedNameKey, NSURLLocalizedTypeDescriptionKey, NSURLMayHaveExtendedAttributesKey,
        NSURLMayShareFileContentKey, NSURLNameKey, NSURLParentDirectoryURLKey, NSURLPathKey,
        NSURLPreferredIOBlockSizeKey, NSURLQuarantinePropertiesKey, NSURLQueryItem,
        NSURLResourceKey, NSURLTagNamesKey, NSURLThumbnailDictionaryItem,
        NSURLThumbnailDictionaryKey, NSURLThumbnailKey, NSURLTotalFileAllocatedSizeKey,
        NSURLTotalFileSizeKey, NSURLTypeIdentifierKey, NSURLUbiquitousItemContainerDisplayNameKey,
        NSURLUbiquitousItemDownloadRequestedKey, NSURLUbiquitousItemDownloadingErrorKey,
        NSURLUbiquitousItemDownloadingStatus, NSURLUbiquitousItemDownloadingStatusCurrent,
        NSURLUbiquitousItemDownloadingStatusDownloaded, NSURLUbiquitousItemDownloadingStatusKey,
        NSURLUbiquitousItemDownloadingStatusNotDownloaded,
        NSURLUbiquitousItemHasUnresolvedConflictsKey, NSURLUbiquitousItemIsDownloadedKey,
        NSURLUbiquitousItemIsDownloadingKey, NSURLUbiquitousItemIsExcludedFromSyncKey,
        NSURLUbiquitousItemIsSharedKey, NSURLUbiquitousItemIsUploadedKey,
        NSURLUbiquitousItemIsUploadingKey, NSURLUbiquitousItemPercentDownloadedKey,
        NSURLUbiquitousItemPercentUploadedKey, NSURLUbiquitousItemUploadingErrorKey,
        NSURLUbiquitousSharedItemCurrentUserPermissionsKey,
        NSURLUbiquitousSharedItemCurrentUserRoleKey,
        NSURLUbiquitousSharedItemMostRecentEditorNameComponentsKey,
        NSURLUbiquitousSharedItemOwnerNameComponentsKey, NSURLUbiquitousSharedItemPermissions,
        NSURLUbiquitousSharedItemPermissionsReadOnly,
        NSURLUbiquitousSharedItemPermissionsReadWrite, NSURLUbiquitousSharedItemRole,
        NSURLUbiquitousSharedItemRoleOwner, NSURLUbiquitousSharedItemRoleParticipant,
        NSURLVolumeAvailableCapacityForImportantUsageKey,
        NSURLVolumeAvailableCapacityForOpportunisticUsageKey, NSURLVolumeAvailableCapacityKey,
        NSURLVolumeCreationDateKey, NSURLVolumeIdentifierKey, NSURLVolumeIsAutomountedKey,
        NSURLVolumeIsBrowsableKey, NSURLVolumeIsEjectableKey, NSURLVolumeIsEncryptedKey,
        NSURLVolumeIsInternalKey, NSURLVolumeIsJournalingKey, NSURLVolumeIsLocalKey,
        NSURLVolumeIsReadOnlyKey, NSURLVolumeIsRemovableKey, NSURLVolumeIsRootFileSystemKey,
        NSURLVolumeLocalizedFormatDescriptionKey, NSURLVolumeLocalizedNameKey,
        NSURLVolumeMaximumFileSizeKey, NSURLVolumeNameKey, NSURLVolumeResourceCountKey,
        NSURLVolumeSupportsAccessPermissionsKey, NSURLVolumeSupportsAdvisoryFileLockingKey,
        NSURLVolumeSupportsCasePreservedNamesKey, NSURLVolumeSupportsCaseSensitiveNamesKey,
        NSURLVolumeSupportsCompressionKey, NSURLVolumeSupportsExclusiveRenamingKey,
        NSURLVolumeSupportsExtendedSecurityKey, NSURLVolumeSupportsFileCloningKey,
        NSURLVolumeSupportsFileProtectionKey, NSURLVolumeSupportsHardLinksKey,
        NSURLVolumeSupportsImmutableFilesKey, NSURLVolumeSupportsJournalingKey,
        NSURLVolumeSupportsPersistentIDsKey, NSURLVolumeSupportsRenamingKey,
        NSURLVolumeSupportsRootDirectoryDatesKey, NSURLVolumeSupportsSparseFilesKey,
        NSURLVolumeSupportsSwapRenamingKey, NSURLVolumeSupportsSymbolicLinksKey,
        NSURLVolumeSupportsVolumeSizesKey, NSURLVolumeSupportsZeroRunsKey,
        NSURLVolumeTotalCapacityKey, NSURLVolumeURLForRemountingKey, NSURLVolumeURLKey,
        NSURLVolumeUUIDStringKey, NSURL,
    };
    pub use super::NSUUID::NSUUID;
    pub use super::NSXMLDTD::NSXMLDTD;
}
