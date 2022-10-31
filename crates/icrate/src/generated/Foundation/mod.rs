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
    pub use super::NSAppleEventManager::NSAppleEventManager;
    pub use super::NSAppleScript::NSAppleScript;
    pub use super::NSArchiver::{NSArchiver, NSUnarchiver};
    pub use super::NSArray::{
        NSArray, NSBinarySearchingFirstEqual, NSBinarySearchingInsertionIndex,
        NSBinarySearchingLastEqual, NSBinarySearchingOptions, NSMutableArray,
    };
    pub use super::NSAttributedString::{
        NSAttributedString, NSAttributedStringEnumerationLongestEffectiveRangeNotRequired,
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
        NSAttributedStringMarkdownParsingOptions, NSInlinePresentationIntent,
        NSInlinePresentationIntentBlockHTML, NSInlinePresentationIntentCode,
        NSInlinePresentationIntentEmphasized, NSInlinePresentationIntentInlineHTML,
        NSInlinePresentationIntentLineBreak, NSInlinePresentationIntentSoftBreak,
        NSInlinePresentationIntentStrikethrough, NSInlinePresentationIntentStronglyEmphasized,
        NSMutableAttributedString, NSPresentationIntent, NSPresentationIntentKind,
        NSPresentationIntentKindBlockQuote, NSPresentationIntentKindCodeBlock,
        NSPresentationIntentKindHeader, NSPresentationIntentKindListItem,
        NSPresentationIntentKindOrderedList, NSPresentationIntentKindParagraph,
        NSPresentationIntentKindTable, NSPresentationIntentKindTableCell,
        NSPresentationIntentKindTableHeaderRow, NSPresentationIntentKindTableRow,
        NSPresentationIntentKindThematicBreak, NSPresentationIntentKindUnorderedList,
        NSPresentationIntentTableColumnAlignment, NSPresentationIntentTableColumnAlignmentCenter,
        NSPresentationIntentTableColumnAlignmentLeft,
        NSPresentationIntentTableColumnAlignmentRight,
    };
    pub use super::NSAutoreleasePool::NSAutoreleasePool;
    pub use super::NSBackgroundActivityScheduler::{
        NSBackgroundActivityResult, NSBackgroundActivityResultDeferred,
        NSBackgroundActivityResultFinished, NSBackgroundActivityScheduler,
    };
    pub use super::NSBundle::{
        NSBundle, NSBundleExecutableArchitectureARM64, NSBundleExecutableArchitectureI386,
        NSBundleExecutableArchitecturePPC, NSBundleExecutableArchitecturePPC64,
        NSBundleExecutableArchitectureX86_64, NSBundleResourceRequest,
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
        NSCalendar, NSCalendarCalendarUnit, NSCalendarIdentifier, NSCalendarMatchFirst,
        NSCalendarMatchLast, NSCalendarMatchNextTime,
        NSCalendarMatchNextTimePreservingSmallerUnits,
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
    pub use super::NSClassDescription::NSClassDescription;
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
    pub use super::NSConnection::{NSConnection, NSConnectionDelegate, NSDistantObjectRequest};
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
    pub use super::NSDate::NSDate;
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
        NSDecimalNumber, NSDecimalNumberBehaviors, NSDecimalNumberHandler,
    };
    pub use super::NSDictionary::{NSDictionary, NSMutableDictionary};
    pub use super::NSDistantObject::NSDistantObject;
    pub use super::NSDistributedLock::NSDistributedLock;
    pub use super::NSDistributedNotificationCenter::{
        NSDistributedNotificationCenter, NSDistributedNotificationCenterType,
        NSDistributedNotificationDeliverImmediately, NSDistributedNotificationOptions,
        NSDistributedNotificationPostToAllSessions, NSNotificationSuspensionBehavior,
        NSNotificationSuspensionBehaviorCoalesce,
        NSNotificationSuspensionBehaviorDeliverImmediately, NSNotificationSuspensionBehaviorDrop,
        NSNotificationSuspensionBehaviorHold,
    };
    pub use super::NSEnergyFormatter::{
        NSEnergyFormatter, NSEnergyFormatterUnit, NSEnergyFormatterUnitCalorie,
        NSEnergyFormatterUnitJoule, NSEnergyFormatterUnitKilocalorie,
        NSEnergyFormatterUnitKilojoule,
    };
    pub use super::NSEnumerator::{NSEnumerator, NSFastEnumeration};
    pub use super::NSError::{NSError, NSErrorDomain, NSErrorUserInfoKey};
    pub use super::NSException::{NSAssertionHandler, NSException};
    pub use super::NSExpression::{
        NSAggregateExpressionType, NSAnyKeyExpressionType, NSBlockExpressionType,
        NSConditionalExpressionType, NSConstantValueExpressionType,
        NSEvaluatedObjectExpressionType, NSExpression, NSExpressionType, NSFunctionExpressionType,
        NSIntersectSetExpressionType, NSKeyPathExpressionType, NSMinusSetExpressionType,
        NSSubqueryExpressionType, NSUnionSetExpressionType, NSVariableExpressionType,
    };
    pub use super::NSExtensionContext::NSExtensionContext;
    pub use super::NSExtensionItem::NSExtensionItem;
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
    pub use super::NSFileHandle::{NSFileHandle, NSPipe};
    pub use super::NSFileManager::{
        NSDirectoryEnumerationIncludesDirectoriesPostOrder, NSDirectoryEnumerationOptions,
        NSDirectoryEnumerationProducesRelativePathURLs, NSDirectoryEnumerationSkipsHiddenFiles,
        NSDirectoryEnumerationSkipsPackageDescendants,
        NSDirectoryEnumerationSkipsSubdirectoryDescendants, NSDirectoryEnumerator,
        NSFileAttributeKey, NSFileAttributeType, NSFileManager, NSFileManagerDelegate,
        NSFileManagerItemReplacementOptions, NSFileManagerItemReplacementUsingNewMetadataOnly,
        NSFileManagerItemReplacementWithoutDeletingBackupItem,
        NSFileManagerUnmountAllPartitionsAndEjectDisk, NSFileManagerUnmountOptions,
        NSFileManagerUnmountWithoutUI, NSFileProtectionType, NSFileProviderService,
        NSFileProviderServiceName, NSURLRelationship, NSURLRelationshipContains,
        NSURLRelationshipOther, NSURLRelationshipSame, NSVolumeEnumerationOptions,
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
        NSAlignWidthNearest, NSAlignWidthOutward, NSAlignmentOptions, NSMaxXEdge, NSMaxYEdge,
        NSMinXEdge, NSMinYEdge, NSPoint, NSRect, NSRectEdge, NSRectEdgeMaxX, NSRectEdgeMaxY,
        NSRectEdgeMinX, NSRectEdgeMinY, NSSize,
    };
    pub use super::NSHTTPCookie::{
        NSHTTPCookie, NSHTTPCookiePropertyKey, NSHTTPCookieStringPolicy,
    };
    pub use super::NSHTTPCookieStorage::{
        NSHTTPCookieAcceptPolicy, NSHTTPCookieAcceptPolicyAlways, NSHTTPCookieAcceptPolicyNever,
        NSHTTPCookieAcceptPolicyOnlyFromMainDocumentDomain, NSHTTPCookieStorage,
    };
    pub use super::NSHashTable::{NSHashTable, NSHashTableOptions};
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
        NSItemProvider, NSItemProviderErrorCode, NSItemProviderFileOptionOpenInPlace,
        NSItemProviderFileOptions, NSItemProviderItemUnavailableError, NSItemProviderReading,
        NSItemProviderRepresentationVisibility, NSItemProviderRepresentationVisibilityAll,
        NSItemProviderRepresentationVisibilityGroup,
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
    pub use super::NSKeyValueCoding::NSKeyValueOperator;
    pub use super::NSKeyValueObserving::{
        NSKeyValueChange, NSKeyValueChangeInsertion, NSKeyValueChangeKey, NSKeyValueChangeRemoval,
        NSKeyValueChangeReplacement, NSKeyValueChangeSetting, NSKeyValueIntersectSetMutation,
        NSKeyValueMinusSetMutation, NSKeyValueObservingOptionInitial, NSKeyValueObservingOptionNew,
        NSKeyValueObservingOptionOld, NSKeyValueObservingOptionPrior, NSKeyValueObservingOptions,
        NSKeyValueSetMutationKind, NSKeyValueSetSetMutation, NSKeyValueUnionSetMutation,
    };
    pub use super::NSKeyedArchiver::{
        NSKeyedArchiver, NSKeyedArchiverDelegate, NSKeyedUnarchiver, NSKeyedUnarchiverDelegate,
    };
    pub use super::NSLengthFormatter::{
        NSLengthFormatter, NSLengthFormatterUnit, NSLengthFormatterUnitCentimeter,
        NSLengthFormatterUnitFoot, NSLengthFormatterUnitInch, NSLengthFormatterUnitKilometer,
        NSLengthFormatterUnitMeter, NSLengthFormatterUnitMile, NSLengthFormatterUnitMillimeter,
        NSLengthFormatterUnitYard,
    };
    pub use super::NSLinguisticTagger::{
        NSLinguisticTag, NSLinguisticTagScheme, NSLinguisticTagger, NSLinguisticTaggerJoinNames,
        NSLinguisticTaggerOmitOther, NSLinguisticTaggerOmitPunctuation,
        NSLinguisticTaggerOmitWhitespace, NSLinguisticTaggerOmitWords, NSLinguisticTaggerOptions,
        NSLinguisticTaggerUnit, NSLinguisticTaggerUnitDocument, NSLinguisticTaggerUnitParagraph,
        NSLinguisticTaggerUnitSentence, NSLinguisticTaggerUnitWord,
    };
    pub use super::NSListFormatter::NSListFormatter;
    pub use super::NSLocale::{
        NSLocale, NSLocaleKey, NSLocaleLanguageDirection, NSLocaleLanguageDirectionBottomToTop,
        NSLocaleLanguageDirectionLeftToRight, NSLocaleLanguageDirectionRightToLeft,
        NSLocaleLanguageDirectionTopToBottom, NSLocaleLanguageDirectionUnknown,
    };
    pub use super::NSLock::{NSCondition, NSConditionLock, NSLock, NSLocking, NSRecursiveLock};
    pub use super::NSMapTable::{NSMapTable, NSMapTableOptions};
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
        NSMetadataItem, NSMetadataQuery, NSMetadataQueryAttributeValueTuple,
        NSMetadataQueryDelegate, NSMetadataQueryResultGroup,
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
        NSNetServicesInvalidError, NSNetServicesMissingRequiredConfigurationError,
        NSNetServicesNotFoundError, NSNetServicesTimeoutError, NSNetServicesUnknownError,
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
        NSExceptionName, NSOrderedAscending, NSOrderedDescending, NSOrderedSame,
        NSQualityOfService, NSQualityOfServiceBackground, NSQualityOfServiceDefault,
        NSQualityOfServiceUserInitiated, NSQualityOfServiceUserInteractive,
        NSQualityOfServiceUtility, NSRunLoopMode, NSSortConcurrent, NSSortOptions, NSSortStable,
    };
    pub use super::NSObject::{
        NSCoding, NSCopying, NSDiscardableContent, NSMutableCopying, NSSecureCoding,
    };
    pub use super::NSOperation::{
        NSBlockOperation, NSInvocationOperation, NSOperation, NSOperationQueue,
        NSOperationQueuePriority, NSOperationQueuePriorityHigh, NSOperationQueuePriorityLow,
        NSOperationQueuePriorityNormal, NSOperationQueuePriorityVeryHigh,
        NSOperationQueuePriorityVeryLow,
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
        NSPort, NSPortDelegate, NSSocketPort,
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
        NSProcessInfoThermalState, NSProcessInfoThermalStateCritical,
        NSProcessInfoThermalStateFair, NSProcessInfoThermalStateNominal,
        NSProcessInfoThermalStateSerious, NSSolarisOperatingSystem, NSSunOSOperatingSystem,
        NSWindows95OperatingSystem, NSWindowsNTOperatingSystem,
    };
    pub use super::NSProgress::{
        NSProgress, NSProgressFileOperationKind, NSProgressKind, NSProgressReporting,
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
    pub use super::NSProxy::NSProxy;
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
    pub use super::NSRunLoop::NSRunLoop;
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
    pub use super::NSSpellServer::{NSSpellServer, NSSpellServerDelegate};
    pub use super::NSStream::{
        NSInputStream, NSOutputStream, NSStream, NSStreamDelegate, NSStreamEvent,
        NSStreamEventEndEncountered, NSStreamEventErrorOccurred, NSStreamEventHasBytesAvailable,
        NSStreamEventHasSpaceAvailable, NSStreamEventNone, NSStreamEventOpenCompleted,
        NSStreamNetworkServiceTypeValue, NSStreamPropertyKey, NSStreamSOCKSProxyConfiguration,
        NSStreamSOCKSProxyVersion, NSStreamSocketSecurityLevel, NSStreamStatus,
        NSStreamStatusAtEnd, NSStreamStatusClosed, NSStreamStatusError, NSStreamStatusNotOpen,
        NSStreamStatusOpen, NSStreamStatusOpening, NSStreamStatusReading, NSStreamStatusWriting,
    };
    pub use super::NSString::{
        NSASCIIStringEncoding, NSAnchoredSearch, NSBackwardsSearch, NSCaseInsensitiveSearch,
        NSConstantString, NSDiacriticInsensitiveSearch, NSForcedOrderingSearch,
        NSISO2022JPStringEncoding, NSISOLatin1StringEncoding, NSISOLatin2StringEncoding,
        NSJapaneseEUCStringEncoding, NSLiteralSearch, NSMacOSRomanStringEncoding, NSMutableString,
        NSNEXTSTEPStringEncoding, NSNonLossyASCIIStringEncoding, NSNumericSearch,
        NSProprietaryStringEncoding, NSRegularExpressionSearch, NSShiftJISStringEncoding,
        NSSimpleCString, NSString, NSStringCompareOptions, NSStringEncoding,
        NSStringEncodingConversionAllowLossy, NSStringEncodingConversionExternalRepresentation,
        NSStringEncodingConversionOptions, NSStringEncodingDetectionOptionsKey,
        NSStringEnumerationByCaretPositions, NSStringEnumerationByComposedCharacterSequences,
        NSStringEnumerationByDeletionClusters, NSStringEnumerationByLines,
        NSStringEnumerationByParagraphs, NSStringEnumerationBySentences,
        NSStringEnumerationByWords, NSStringEnumerationLocalized, NSStringEnumerationOptions,
        NSStringEnumerationReverse, NSStringEnumerationSubstringNotRequired, NSStringTransform,
        NSSymbolStringEncoding, NSUTF16BigEndianStringEncoding, NSUTF16LittleEndianStringEncoding,
        NSUTF16StringEncoding, NSUTF32BigEndianStringEncoding, NSUTF32LittleEndianStringEncoding,
        NSUTF32StringEncoding, NSUTF8StringEncoding, NSUnicodeStringEncoding,
        NSWidthInsensitiveSearch, NSWindowsCP1250StringEncoding, NSWindowsCP1251StringEncoding,
        NSWindowsCP1252StringEncoding, NSWindowsCP1253StringEncoding,
        NSWindowsCP1254StringEncoding,
    };
    pub use super::NSTask::{
        NSTask, NSTaskTerminationReason, NSTaskTerminationReasonExit,
        NSTaskTerminationReasonUncaughtSignal,
    };
    pub use super::NSTextCheckingResult::{
        NSTextCheckingAllCustomTypes, NSTextCheckingAllSystemTypes, NSTextCheckingAllTypes,
        NSTextCheckingKey, NSTextCheckingResult, NSTextCheckingType, NSTextCheckingTypeAddress,
        NSTextCheckingTypeCorrection, NSTextCheckingTypeDash, NSTextCheckingTypeDate,
        NSTextCheckingTypeGrammar, NSTextCheckingTypeLink, NSTextCheckingTypeOrthography,
        NSTextCheckingTypePhoneNumber, NSTextCheckingTypeQuote,
        NSTextCheckingTypeRegularExpression, NSTextCheckingTypeReplacement,
        NSTextCheckingTypeSpelling, NSTextCheckingTypeTransitInformation, NSTextCheckingTypes,
    };
    pub use super::NSThread::NSThread;
    pub use super::NSTimeZone::{
        NSTimeZone, NSTimeZoneNameStyle, NSTimeZoneNameStyleDaylightSaving,
        NSTimeZoneNameStyleGeneric, NSTimeZoneNameStyleShortDaylightSaving,
        NSTimeZoneNameStyleShortGeneric, NSTimeZoneNameStyleShortStandard,
        NSTimeZoneNameStyleStandard,
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
    pub use super::NSURLCredentialStorage::NSURLCredentialStorage;
    pub use super::NSURLDownload::{NSURLDownload, NSURLDownloadDelegate};
    pub use super::NSURLError::{
        NSURLErrorAppTransportSecurityRequiresSecureConnection,
        NSURLErrorBackgroundSessionInUseByAnotherProcess,
        NSURLErrorBackgroundSessionRequiresSharedContainer,
        NSURLErrorBackgroundSessionWasDisconnected, NSURLErrorBadServerResponse, NSURLErrorBadURL,
        NSURLErrorCallIsActive, NSURLErrorCancelled,
        NSURLErrorCancelledReasonBackgroundUpdatesDisabled,
        NSURLErrorCancelledReasonInsufficientSystemResources,
        NSURLErrorCancelledReasonUserForceQuitApplication, NSURLErrorCannotCloseFile,
        NSURLErrorCannotConnectToHost, NSURLErrorCannotCreateFile,
        NSURLErrorCannotDecodeContentData, NSURLErrorCannotDecodeRawData, NSURLErrorCannotFindHost,
        NSURLErrorCannotLoadFromNetwork, NSURLErrorCannotMoveFile, NSURLErrorCannotOpenFile,
        NSURLErrorCannotParseResponse, NSURLErrorCannotRemoveFile, NSURLErrorCannotWriteToFile,
        NSURLErrorClientCertificateRejected, NSURLErrorClientCertificateRequired,
        NSURLErrorDNSLookupFailed, NSURLErrorDataLengthExceedsMaximum, NSURLErrorDataNotAllowed,
        NSURLErrorDownloadDecodingFailedMidStream, NSURLErrorDownloadDecodingFailedToComplete,
        NSURLErrorFileDoesNotExist, NSURLErrorFileIsDirectory, NSURLErrorFileOutsideSafeArea,
        NSURLErrorHTTPTooManyRedirects, NSURLErrorInternationalRoamingOff,
        NSURLErrorNetworkConnectionLost, NSURLErrorNetworkUnavailableReason,
        NSURLErrorNetworkUnavailableReasonCellular, NSURLErrorNetworkUnavailableReasonConstrained,
        NSURLErrorNetworkUnavailableReasonExpensive, NSURLErrorNoPermissionsToReadFile,
        NSURLErrorNotConnectedToInternet, NSURLErrorRedirectToNonExistentLocation,
        NSURLErrorRequestBodyStreamExhausted, NSURLErrorResourceUnavailable,
        NSURLErrorSecureConnectionFailed, NSURLErrorServerCertificateHasBadDate,
        NSURLErrorServerCertificateHasUnknownRoot, NSURLErrorServerCertificateNotYetValid,
        NSURLErrorServerCertificateUntrusted, NSURLErrorTimedOut, NSURLErrorUnknown,
        NSURLErrorUnsupportedURL, NSURLErrorUserAuthenticationRequired,
        NSURLErrorUserCancelledAuthentication, NSURLErrorZeroByteResource,
    };
    pub use super::NSURLHandle::{
        NSURLHandle, NSURLHandleClient, NSURLHandleLoadFailed, NSURLHandleLoadInProgress,
        NSURLHandleLoadSucceeded, NSURLHandleNotLoaded, NSURLHandleStatus,
    };
    pub use super::NSURLProtectionSpace::NSURLProtectionSpace;
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
        NSURLSessionMultipathServiceType, NSURLSessionMultipathServiceTypeAggregate,
        NSURLSessionMultipathServiceTypeHandover, NSURLSessionMultipathServiceTypeInteractive,
        NSURLSessionMultipathServiceTypeNone, NSURLSessionResponseAllow,
        NSURLSessionResponseBecomeDownload, NSURLSessionResponseBecomeStream,
        NSURLSessionResponseCancel, NSURLSessionResponseDisposition, NSURLSessionStreamDelegate,
        NSURLSessionStreamTask, NSURLSessionTask, NSURLSessionTaskDelegate,
        NSURLSessionTaskMetrics, NSURLSessionTaskMetricsDomainResolutionProtocol,
        NSURLSessionTaskMetricsDomainResolutionProtocolHTTPS,
        NSURLSessionTaskMetricsDomainResolutionProtocolTCP,
        NSURLSessionTaskMetricsDomainResolutionProtocolTLS,
        NSURLSessionTaskMetricsDomainResolutionProtocolUDP,
        NSURLSessionTaskMetricsDomainResolutionProtocolUnknown,
        NSURLSessionTaskMetricsResourceFetchType,
        NSURLSessionTaskMetricsResourceFetchTypeLocalCache,
        NSURLSessionTaskMetricsResourceFetchTypeNetworkLoad,
        NSURLSessionTaskMetricsResourceFetchTypeServerPush,
        NSURLSessionTaskMetricsResourceFetchTypeUnknown, NSURLSessionTaskState,
        NSURLSessionTaskStateCanceling, NSURLSessionTaskStateCompleted,
        NSURLSessionTaskStateRunning, NSURLSessionTaskStateSuspended,
        NSURLSessionTaskTransactionMetrics, NSURLSessionUploadTask, NSURLSessionWebSocketCloseCode,
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
        NSUbiquitousKeyValueStoreInitialSyncChange, NSUbiquitousKeyValueStoreQuotaViolationChange,
        NSUbiquitousKeyValueStoreServerChange,
    };
    pub use super::NSUndoManager::NSUndoManager;
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
    };
    pub use super::NSUserDefaults::NSUserDefaults;
    pub use super::NSUserNotification::{
        NSUserNotification, NSUserNotificationAction, NSUserNotificationActivationType,
        NSUserNotificationActivationTypeActionButtonClicked,
        NSUserNotificationActivationTypeAdditionalActionClicked,
        NSUserNotificationActivationTypeContentsClicked, NSUserNotificationActivationTypeNone,
        NSUserNotificationActivationTypeReplied, NSUserNotificationCenter,
        NSUserNotificationCenterDelegate,
    };
    pub use super::NSUserScriptTask::{
        NSUserAppleScriptTask, NSUserAutomatorTask, NSUserScriptTask, NSUserUnixTask,
    };
    pub use super::NSValue::{NSNumber, NSValue};
    pub use super::NSValueTransformer::{
        NSSecureUnarchiveFromDataTransformer, NSValueTransformer, NSValueTransformerName,
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
        NSXMLParserEqualExpectedError, NSXMLParserError, NSXMLParserExternalEntityResolvingPolicy,
        NSXMLParserExternalStandaloneEntityError, NSXMLParserExternalSubsetNotFinishedError,
        NSXMLParserExtraContentError, NSXMLParserGTRequiredError, NSXMLParserInternalError,
        NSXMLParserInvalidCharacterError, NSXMLParserInvalidCharacterInEntityError,
        NSXMLParserInvalidCharacterRefError, NSXMLParserInvalidConditionalSectionError,
        NSXMLParserInvalidDecimalCharacterRefError, NSXMLParserInvalidEncodingError,
        NSXMLParserInvalidEncodingNameError, NSXMLParserInvalidHexCharacterRefError,
        NSXMLParserInvalidURIError, NSXMLParserLTRequiredError, NSXMLParserLTSlashRequiredError,
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
        NSFileSecurity, NSURLBookmarkCreationMinimalBookmark, NSURLBookmarkCreationOptions,
        NSURLBookmarkCreationPreferFileIDResolution,
        NSURLBookmarkCreationSecurityScopeAllowOnlyReadAccess,
        NSURLBookmarkCreationSuitableForBookmarkFile, NSURLBookmarkCreationWithSecurityScope,
        NSURLBookmarkCreationWithoutImplicitSecurityScope, NSURLBookmarkFileCreationOptions,
        NSURLBookmarkResolutionOptions, NSURLBookmarkResolutionWithSecurityScope,
        NSURLBookmarkResolutionWithoutImplicitStartAccessing,
        NSURLBookmarkResolutionWithoutMounting, NSURLBookmarkResolutionWithoutUI, NSURLComponents,
        NSURLFileProtectionType, NSURLFileResourceType, NSURLQueryItem, NSURLResourceKey,
        NSURLThumbnailDictionaryItem, NSURLUbiquitousItemDownloadingStatus,
        NSURLUbiquitousSharedItemPermissions, NSURLUbiquitousSharedItemRole, NSURL,
    };
    pub use super::NSUUID::NSUUID;
    pub use super::NSXMLDTD::NSXMLDTD;
}
