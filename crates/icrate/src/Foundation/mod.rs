#[allow(unused_imports)]
#[path = "../generated/Foundation/mod.rs"]
pub(crate) mod generated;

pub use objc2::ffi::NSIntegerMax;
pub use objc2::foundation::{CGFloat, CGPoint, CGRect, CGSize, NSZone};
pub use objc2::ns_string;

objc2::__inner_extern_class! {
    @__inner
    pub struct (NSObject) {}

    unsafe impl () for NSObject {
        INHERITS = [objc2::runtime::Object];
    }
}

unsafe impl objc2::ClassType for NSObject {
    type Super = objc2::runtime::Object;
    const NAME: &'static str = "NSObject";

    #[inline]
    fn class() -> &'static objc2::runtime::Class {
        objc2::class!(NSObject)
    }

    fn as_super(&self) -> &Self::Super {
        &self.__inner
    }

    fn as_super_mut(&mut self) -> &mut Self::Super {
        &mut self.__inner
    }
}
impl PartialEq for NSObject {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}
impl Eq for NSObject {}
impl std::hash::Hash for NSObject {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {
        todo!()
    }
}
impl std::fmt::Debug for NSObject {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

objc2::__inner_extern_class! {
    @__inner
    pub struct (NSProxy) {}

    unsafe impl () for NSProxy {
        INHERITS = [objc2::runtime::Object];
    }
}
unsafe impl objc2::ClassType for NSProxy {
    type Super = objc2::runtime::Object;
    const NAME: &'static str = "NSProxy";

    #[inline]
    fn class() -> &'static objc2::runtime::Class {
        objc2::class!(NSProxy)
    }

    fn as_super(&self) -> &Self::Super {
        &self.__inner
    }

    fn as_super_mut(&mut self) -> &mut Self::Super {
        &mut self.__inner
    }
}
impl PartialEq for NSProxy {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}
impl Eq for NSProxy {}
impl std::hash::Hash for NSProxy {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {
        todo!()
    }
}
impl std::fmt::Debug for NSProxy {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

struct_impl!(
    pub struct NSDecimal {
        // signed   int _exponent:8;
        // unsigned int _length:4;
        // unsigned int _isNegative:1;
        // unsigned int _isCompact:1;
        // unsigned int _reserved:18;
        _inner: i32,
        _mantissa: [std::ffi::c_ushort; 8],
    }
);

// Generated

pub use self::generated::FoundationErrors::{
    NSBundleErrorMaximum, NSBundleErrorMinimum, NSBundleOnDemandResourceExceededMaximumSizeError,
    NSBundleOnDemandResourceInvalidTagError, NSBundleOnDemandResourceOutOfSpaceError,
    NSCloudSharingConflictError, NSCloudSharingErrorMaximum, NSCloudSharingErrorMinimum,
    NSCloudSharingNetworkFailureError, NSCloudSharingNoPermissionError, NSCloudSharingOtherError,
    NSCloudSharingQuotaExceededError, NSCloudSharingTooManyParticipantsError, NSCoderErrorMaximum,
    NSCoderErrorMinimum, NSCoderInvalidValueError, NSCoderReadCorruptError,
    NSCoderValueNotFoundError, NSCompressionErrorMaximum, NSCompressionErrorMinimum,
    NSCompressionFailedError, NSDecompressionFailedError, NSExecutableArchitectureMismatchError,
    NSExecutableErrorMaximum, NSExecutableErrorMinimum, NSExecutableLinkError,
    NSExecutableLoadError, NSExecutableNotLoadableError, NSExecutableRuntimeMismatchError,
    NSFeatureUnsupportedError, NSFileErrorMaximum, NSFileErrorMinimum, NSFileLockingError,
    NSFileManagerUnmountBusyError, NSFileManagerUnmountUnknownError, NSFileNoSuchFileError,
    NSFileReadCorruptFileError, NSFileReadInapplicableStringEncodingError,
    NSFileReadInvalidFileNameError, NSFileReadNoPermissionError, NSFileReadNoSuchFileError,
    NSFileReadTooLargeError, NSFileReadUnknownError, NSFileReadUnknownStringEncodingError,
    NSFileReadUnsupportedSchemeError, NSFileWriteFileExistsError,
    NSFileWriteInapplicableStringEncodingError, NSFileWriteInvalidFileNameError,
    NSFileWriteNoPermissionError, NSFileWriteOutOfSpaceError, NSFileWriteUnknownError,
    NSFileWriteUnsupportedSchemeError, NSFileWriteVolumeReadOnlyError, NSFormattingError,
    NSFormattingErrorMaximum, NSFormattingErrorMinimum, NSKeyValueValidationError,
    NSPropertyListErrorMaximum, NSPropertyListErrorMinimum, NSPropertyListReadCorruptError,
    NSPropertyListReadStreamError, NSPropertyListReadUnknownVersionError,
    NSPropertyListWriteInvalidError, NSPropertyListWriteStreamError, NSUbiquitousFileErrorMaximum,
    NSUbiquitousFileErrorMinimum, NSUbiquitousFileNotUploadedDueToQuotaError,
    NSUbiquitousFileUbiquityServerNotAvailable, NSUbiquitousFileUnavailableError,
    NSUserActivityConnectionUnavailableError, NSUserActivityErrorMaximum,
    NSUserActivityErrorMinimum, NSUserActivityHandoffFailedError,
    NSUserActivityHandoffUserInfoTooLargeError, NSUserActivityRemoteApplicationTimedOutError,
    NSUserCancelledError, NSValidationErrorMaximum, NSValidationErrorMinimum,
    NSXPCConnectionErrorMaximum, NSXPCConnectionErrorMinimum, NSXPCConnectionInterrupted,
    NSXPCConnectionInvalid, NSXPCConnectionReplyInvalid,
};
pub use self::generated::NSAffineTransform::{NSAffineTransform, NSAffineTransformStruct};
pub use self::generated::NSAppleEventDescriptor::{
    NSAppleEventDescriptor, NSAppleEventSendAlwaysInteract, NSAppleEventSendCanInteract,
    NSAppleEventSendCanSwitchLayer, NSAppleEventSendDefaultOptions, NSAppleEventSendDontAnnotate,
    NSAppleEventSendDontExecute, NSAppleEventSendDontRecord, NSAppleEventSendNeverInteract,
    NSAppleEventSendNoReply, NSAppleEventSendOptions, NSAppleEventSendQueueReply,
    NSAppleEventSendWaitForReply,
};
pub use self::generated::NSAppleEventManager::{
    NSAppleEventManager, NSAppleEventManagerSuspensionID,
    NSAppleEventManagerWillProcessFirstEventNotification, NSAppleEventTimeOutDefault,
    NSAppleEventTimeOutNone,
};
pub use self::generated::NSAppleScript::{
    NSAppleScript, NSAppleScriptErrorAppName, NSAppleScriptErrorBriefMessage,
    NSAppleScriptErrorMessage, NSAppleScriptErrorNumber, NSAppleScriptErrorRange,
};
pub use self::generated::NSArchiver::{NSArchiver, NSUnarchiver};
pub use self::generated::NSArray::{
    NSArray, NSBinarySearchingFirstEqual, NSBinarySearchingInsertionIndex,
    NSBinarySearchingLastEqual, NSBinarySearchingOptions, NSMutableArray,
};
pub use self::generated::NSAttributedString::{
    NSAlternateDescriptionAttributeName, NSAttributedString,
    NSAttributedStringEnumerationLongestEffectiveRangeNotRequired,
    NSAttributedStringEnumerationOptions, NSAttributedStringEnumerationReverse,
    NSAttributedStringFormattingApplyReplacementIndexAttribute,
    NSAttributedStringFormattingInsertArgumentAttributesWithoutMerging,
    NSAttributedStringFormattingOptions, NSAttributedStringKey,
    NSAttributedStringMarkdownInterpretedSyntax, NSAttributedStringMarkdownInterpretedSyntaxFull,
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
    NSPresentationIntentTableColumnAlignmentLeft, NSPresentationIntentTableColumnAlignmentRight,
    NSReplacementIndexAttributeName,
};
pub use self::generated::NSAutoreleasePool::NSAutoreleasePool;
pub use self::generated::NSBackgroundActivityScheduler::{
    NSBackgroundActivityCompletionHandler, NSBackgroundActivityResult,
    NSBackgroundActivityResultDeferred, NSBackgroundActivityResultFinished,
    NSBackgroundActivityScheduler,
};
pub use self::generated::NSBundle::{
    NSBundle, NSBundleDidLoadNotification, NSBundleExecutableArchitectureARM64,
    NSBundleExecutableArchitectureI386, NSBundleExecutableArchitecturePPC,
    NSBundleExecutableArchitecturePPC64, NSBundleExecutableArchitectureX86_64,
    NSBundleResourceRequest, NSBundleResourceRequestLoadingPriorityUrgent,
    NSBundleResourceRequestLowDiskSpaceNotification, NSLoadedClasses,
};
pub use self::generated::NSByteCountFormatter::{
    NSByteCountFormatter, NSByteCountFormatterCountStyle, NSByteCountFormatterCountStyleBinary,
    NSByteCountFormatterCountStyleDecimal, NSByteCountFormatterCountStyleFile,
    NSByteCountFormatterCountStyleMemory, NSByteCountFormatterUnits, NSByteCountFormatterUseAll,
    NSByteCountFormatterUseBytes, NSByteCountFormatterUseDefault, NSByteCountFormatterUseEB,
    NSByteCountFormatterUseGB, NSByteCountFormatterUseKB, NSByteCountFormatterUseMB,
    NSByteCountFormatterUsePB, NSByteCountFormatterUseTB, NSByteCountFormatterUseYBOrHigher,
    NSByteCountFormatterUseZB,
};
pub use self::generated::NSByteOrder::{NSSwappedDouble, NSSwappedFloat};
pub use self::generated::NSCache::{NSCache, NSCacheDelegate};
pub use self::generated::NSCalendar::{
    NSCalendar, NSCalendarCalendarUnit, NSCalendarDayChangedNotification, NSCalendarIdentifier,
    NSCalendarIdentifierBuddhist, NSCalendarIdentifierChinese, NSCalendarIdentifierCoptic,
    NSCalendarIdentifierEthiopicAmeteAlem, NSCalendarIdentifierEthiopicAmeteMihret,
    NSCalendarIdentifierGregorian, NSCalendarIdentifierHebrew, NSCalendarIdentifierISO8601,
    NSCalendarIdentifierIndian, NSCalendarIdentifierIslamic, NSCalendarIdentifierIslamicCivil,
    NSCalendarIdentifierIslamicTabular, NSCalendarIdentifierIslamicUmmAlQura,
    NSCalendarIdentifierJapanese, NSCalendarIdentifierPersian, NSCalendarIdentifierRepublicOfChina,
    NSCalendarMatchFirst, NSCalendarMatchLast, NSCalendarMatchNextTime,
    NSCalendarMatchNextTimePreservingSmallerUnits,
    NSCalendarMatchPreviousTimePreservingSmallerUnits, NSCalendarMatchStrictly, NSCalendarOptions,
    NSCalendarSearchBackwards, NSCalendarUnit, NSCalendarUnitCalendar, NSCalendarUnitDay,
    NSCalendarUnitEra, NSCalendarUnitHour, NSCalendarUnitMinute, NSCalendarUnitMonth,
    NSCalendarUnitNanosecond, NSCalendarUnitQuarter, NSCalendarUnitSecond, NSCalendarUnitTimeZone,
    NSCalendarUnitWeekOfMonth, NSCalendarUnitWeekOfYear, NSCalendarUnitWeekday,
    NSCalendarUnitWeekdayOrdinal, NSCalendarUnitYear, NSCalendarUnitYearForWeekOfYear,
    NSCalendarWrapComponents, NSDateComponentUndefined, NSDateComponents, NSDayCalendarUnit,
    NSEraCalendarUnit, NSHourCalendarUnit, NSMinuteCalendarUnit, NSMonthCalendarUnit,
    NSQuarterCalendarUnit, NSSecondCalendarUnit, NSTimeZoneCalendarUnit, NSUndefinedDateComponent,
    NSWeekCalendarUnit, NSWeekOfMonthCalendarUnit, NSWeekOfYearCalendarUnit, NSWeekdayCalendarUnit,
    NSWeekdayOrdinalCalendarUnit, NSYearCalendarUnit, NSYearForWeekOfYearCalendarUnit,
};
pub use self::generated::NSCalendarDate::NSCalendarDate;
pub use self::generated::NSCharacterSet::{
    NSCharacterSet, NSMutableCharacterSet, NSOpenStepUnicodeReservedBase,
};
pub use self::generated::NSClassDescription::{
    NSClassDescription, NSClassDescriptionNeededForClassNotification,
};
pub use self::generated::NSCoder::{
    NSCoder, NSDecodingFailurePolicy, NSDecodingFailurePolicyRaiseException,
    NSDecodingFailurePolicySetErrorAndReturn,
};
pub use self::generated::NSComparisonPredicate::{
    NSAllPredicateModifier, NSAnyPredicateModifier, NSBeginsWithPredicateOperatorType,
    NSBetweenPredicateOperatorType, NSCaseInsensitivePredicateOption, NSComparisonPredicate,
    NSComparisonPredicateModifier, NSComparisonPredicateOptions, NSContainsPredicateOperatorType,
    NSCustomSelectorPredicateOperatorType, NSDiacriticInsensitivePredicateOption,
    NSDirectPredicateModifier, NSEndsWithPredicateOperatorType, NSEqualToPredicateOperatorType,
    NSGreaterThanOrEqualToPredicateOperatorType, NSGreaterThanPredicateOperatorType,
    NSInPredicateOperatorType, NSLessThanOrEqualToPredicateOperatorType,
    NSLessThanPredicateOperatorType, NSLikePredicateOperatorType, NSMatchesPredicateOperatorType,
    NSNormalizedPredicateOption, NSNotEqualToPredicateOperatorType, NSPredicateOperatorType,
};
pub use self::generated::NSCompoundPredicate::{
    NSAndPredicateType, NSCompoundPredicate, NSCompoundPredicateType, NSNotPredicateType,
    NSOrPredicateType,
};
pub use self::generated::NSConnection::{
    NSConnection, NSConnectionDelegate, NSConnectionDidDieNotification,
    NSConnectionDidInitializeNotification, NSConnectionReplyMode, NSDistantObjectRequest,
    NSFailedAuthenticationException,
};
pub use self::generated::NSData::{
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
    NSDataWritingWithoutOverwriting, NSMappedRead, NSMutableData, NSPurgeableData, NSUncachedRead,
};
pub use self::generated::NSDate::{NSDate, NSSystemClockDidChangeNotification, NSTimeInterval};
pub use self::generated::NSDateComponentsFormatter::{
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
pub use self::generated::NSDateFormatter::{
    NSDateFormatter, NSDateFormatterBehavior, NSDateFormatterBehavior10_0,
    NSDateFormatterBehavior10_4, NSDateFormatterBehaviorDefault, NSDateFormatterFullStyle,
    NSDateFormatterLongStyle, NSDateFormatterMediumStyle, NSDateFormatterNoStyle,
    NSDateFormatterShortStyle, NSDateFormatterStyle,
};
pub use self::generated::NSDateInterval::NSDateInterval;
pub use self::generated::NSDateIntervalFormatter::{
    NSDateIntervalFormatter, NSDateIntervalFormatterFullStyle, NSDateIntervalFormatterLongStyle,
    NSDateIntervalFormatterMediumStyle, NSDateIntervalFormatterNoStyle,
    NSDateIntervalFormatterShortStyle, NSDateIntervalFormatterStyle,
};
pub use self::generated::NSDecimal::{
    NSCalculationDivideByZero, NSCalculationError, NSCalculationLossOfPrecision,
    NSCalculationNoError, NSCalculationOverflow, NSCalculationUnderflow, NSRoundBankers,
    NSRoundDown, NSRoundPlain, NSRoundUp, NSRoundingMode,
};
pub use self::generated::NSDecimalNumber::{
    NSDecimalNumber, NSDecimalNumberBehaviors, NSDecimalNumberDivideByZeroException,
    NSDecimalNumberExactnessException, NSDecimalNumberHandler, NSDecimalNumberOverflowException,
    NSDecimalNumberUnderflowException,
};
pub use self::generated::NSDictionary::{NSDictionary, NSMutableDictionary};
pub use self::generated::NSDistantObject::NSDistantObject;
pub use self::generated::NSDistributedLock::NSDistributedLock;
pub use self::generated::NSDistributedNotificationCenter::{
    NSDistributedNotificationCenter, NSDistributedNotificationCenterType,
    NSDistributedNotificationDeliverImmediately, NSDistributedNotificationOptions,
    NSDistributedNotificationPostToAllSessions, NSLocalNotificationCenterType,
    NSNotificationDeliverImmediately, NSNotificationPostToAllSessions,
    NSNotificationSuspensionBehavior, NSNotificationSuspensionBehaviorCoalesce,
    NSNotificationSuspensionBehaviorDeliverImmediately, NSNotificationSuspensionBehaviorDrop,
    NSNotificationSuspensionBehaviorHold,
};
pub use self::generated::NSEnergyFormatter::{
    NSEnergyFormatter, NSEnergyFormatterUnit, NSEnergyFormatterUnitCalorie,
    NSEnergyFormatterUnitJoule, NSEnergyFormatterUnitKilocalorie, NSEnergyFormatterUnitKilojoule,
};
pub use self::generated::NSEnumerator::{NSEnumerator, NSFastEnumeration, NSFastEnumerationState};
pub use self::generated::NSError::{
    NSCocoaErrorDomain, NSDebugDescriptionErrorKey, NSError, NSErrorDomain, NSErrorUserInfoKey,
    NSFilePathErrorKey, NSHelpAnchorErrorKey, NSLocalizedDescriptionKey,
    NSLocalizedFailureErrorKey, NSLocalizedFailureReasonErrorKey,
    NSLocalizedRecoveryOptionsErrorKey, NSLocalizedRecoverySuggestionErrorKey, NSMachErrorDomain,
    NSMultipleUnderlyingErrorsKey, NSOSStatusErrorDomain, NSPOSIXErrorDomain,
    NSRecoveryAttempterErrorKey, NSStringEncodingErrorKey, NSURLErrorKey, NSUnderlyingErrorKey,
};
pub use self::generated::NSException::{
    NSAssertionHandler, NSAssertionHandlerKey, NSDestinationInvalidException, NSException,
    NSGenericException, NSInconsistentArchiveException, NSInternalInconsistencyException,
    NSInvalidArgumentException, NSInvalidReceivePortException, NSInvalidSendPortException,
    NSMallocException, NSObjectInaccessibleException, NSObjectNotAvailableException,
    NSOldStyleException, NSPortReceiveException, NSPortSendException, NSPortTimeoutException,
    NSRangeException, NSUncaughtExceptionHandler,
};
pub use self::generated::NSExpression::{
    NSAggregateExpressionType, NSAnyKeyExpressionType, NSBlockExpressionType,
    NSConditionalExpressionType, NSConstantValueExpressionType, NSEvaluatedObjectExpressionType,
    NSExpression, NSExpressionType, NSFunctionExpressionType, NSIntersectSetExpressionType,
    NSKeyPathExpressionType, NSMinusSetExpressionType, NSSubqueryExpressionType,
    NSUnionSetExpressionType, NSVariableExpressionType,
};
pub use self::generated::NSExtensionContext::{
    NSExtensionContext, NSExtensionHostDidBecomeActiveNotification,
    NSExtensionHostDidEnterBackgroundNotification, NSExtensionHostWillEnterForegroundNotification,
    NSExtensionHostWillResignActiveNotification, NSExtensionItemsAndErrorsKey,
};
pub use self::generated::NSExtensionItem::{
    NSExtensionItem, NSExtensionItemAttachmentsKey, NSExtensionItemAttributedContentTextKey,
    NSExtensionItemAttributedTitleKey,
};
pub use self::generated::NSExtensionRequestHandling::NSExtensionRequestHandling;
pub use self::generated::NSFileCoordinator::{
    NSFileAccessIntent, NSFileCoordinator, NSFileCoordinatorReadingForUploading,
    NSFileCoordinatorReadingImmediatelyAvailableMetadataOnly, NSFileCoordinatorReadingOptions,
    NSFileCoordinatorReadingResolvesSymbolicLink, NSFileCoordinatorReadingWithoutChanges,
    NSFileCoordinatorWritingContentIndependentMetadataOnly, NSFileCoordinatorWritingForDeleting,
    NSFileCoordinatorWritingForMerging, NSFileCoordinatorWritingForMoving,
    NSFileCoordinatorWritingForReplacing, NSFileCoordinatorWritingOptions,
};
pub use self::generated::NSFileHandle::{
    NSFileHandle, NSFileHandleConnectionAcceptedNotification,
    NSFileHandleDataAvailableNotification, NSFileHandleNotificationDataItem,
    NSFileHandleNotificationFileHandleItem, NSFileHandleNotificationMonitorModes,
    NSFileHandleOperationException, NSFileHandleReadCompletionNotification,
    NSFileHandleReadToEndOfFileCompletionNotification, NSPipe,
};
pub use self::generated::NSFileManager::{
    NSDirectoryEnumerationIncludesDirectoriesPostOrder, NSDirectoryEnumerationOptions,
    NSDirectoryEnumerationProducesRelativePathURLs, NSDirectoryEnumerationSkipsHiddenFiles,
    NSDirectoryEnumerationSkipsPackageDescendants,
    NSDirectoryEnumerationSkipsSubdirectoryDescendants, NSDirectoryEnumerator, NSFileAppendOnly,
    NSFileAttributeKey, NSFileAttributeType, NSFileBusy, NSFileCreationDate,
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
pub use self::generated::NSFilePresenter::NSFilePresenter;
pub use self::generated::NSFileVersion::{
    NSFileVersion, NSFileVersionAddingByMoving, NSFileVersionAddingOptions,
    NSFileVersionReplacingByMoving, NSFileVersionReplacingOptions,
};
pub use self::generated::NSFileWrapper::{
    NSFileWrapper, NSFileWrapperReadingImmediate, NSFileWrapperReadingOptions,
    NSFileWrapperReadingWithoutMapping, NSFileWrapperWritingAtomic, NSFileWrapperWritingOptions,
    NSFileWrapperWritingWithNameUpdating,
};
pub use self::generated::NSFormatter::{
    NSFormatter, NSFormattingContext, NSFormattingContextBeginningOfSentence,
    NSFormattingContextDynamic, NSFormattingContextListItem, NSFormattingContextMiddleOfSentence,
    NSFormattingContextStandalone, NSFormattingContextUnknown, NSFormattingUnitStyle,
    NSFormattingUnitStyleLong, NSFormattingUnitStyleMedium, NSFormattingUnitStyleShort,
};
pub use self::generated::NSGarbageCollector::NSGarbageCollector;
pub use self::generated::NSGeometry::{
    NSAlignAllEdgesInward, NSAlignAllEdgesNearest, NSAlignAllEdgesOutward, NSAlignHeightInward,
    NSAlignHeightNearest, NSAlignHeightOutward, NSAlignMaxXInward, NSAlignMaxXNearest,
    NSAlignMaxXOutward, NSAlignMaxYInward, NSAlignMaxYNearest, NSAlignMaxYOutward,
    NSAlignMinXInward, NSAlignMinXNearest, NSAlignMinXOutward, NSAlignMinYInward,
    NSAlignMinYNearest, NSAlignMinYOutward, NSAlignRectFlipped, NSAlignWidthInward,
    NSAlignWidthNearest, NSAlignWidthOutward, NSAlignmentOptions, NSEdgeInsets, NSEdgeInsetsZero,
    NSMaxXEdge, NSMaxYEdge, NSMinXEdge, NSMinYEdge, NSPoint, NSPointArray, NSPointPointer, NSRect,
    NSRectArray, NSRectEdge, NSRectEdgeMaxX, NSRectEdgeMaxY, NSRectEdgeMinX, NSRectEdgeMinY,
    NSRectPointer, NSSize, NSSizeArray, NSSizePointer, NSZeroPoint, NSZeroRect, NSZeroSize,
};
pub use self::generated::NSHTTPCookie::{
    NSHTTPCookie, NSHTTPCookieComment, NSHTTPCookieCommentURL, NSHTTPCookieDiscard,
    NSHTTPCookieDomain, NSHTTPCookieExpires, NSHTTPCookieMaximumAge, NSHTTPCookieName,
    NSHTTPCookieOriginURL, NSHTTPCookiePath, NSHTTPCookiePort, NSHTTPCookiePropertyKey,
    NSHTTPCookieSameSiteLax, NSHTTPCookieSameSitePolicy, NSHTTPCookieSameSiteStrict,
    NSHTTPCookieSecure, NSHTTPCookieStringPolicy, NSHTTPCookieValue, NSHTTPCookieVersion,
};
pub use self::generated::NSHTTPCookieStorage::{
    NSHTTPCookieAcceptPolicy, NSHTTPCookieAcceptPolicyAlways, NSHTTPCookieAcceptPolicyNever,
    NSHTTPCookieAcceptPolicyOnlyFromMainDocumentDomain,
    NSHTTPCookieManagerAcceptPolicyChangedNotification,
    NSHTTPCookieManagerCookiesChangedNotification, NSHTTPCookieStorage,
};
pub use self::generated::NSHashTable::{
    NSHashEnumerator, NSHashTable, NSHashTableCallBacks, NSHashTableCopyIn,
    NSHashTableObjectPointerPersonality, NSHashTableOptions, NSHashTableStrongMemory,
    NSHashTableWeakMemory, NSHashTableZeroingWeakMemory, NSIntHashCallBacks,
    NSIntegerHashCallBacks, NSNonOwnedPointerHashCallBacks, NSNonRetainedObjectHashCallBacks,
    NSObjectHashCallBacks, NSOwnedObjectIdentityHashCallBacks, NSOwnedPointerHashCallBacks,
    NSPointerToStructHashCallBacks,
};
pub use self::generated::NSHost::NSHost;
pub use self::generated::NSISO8601DateFormatter::{
    NSISO8601DateFormatOptions, NSISO8601DateFormatWithColonSeparatorInTime,
    NSISO8601DateFormatWithColonSeparatorInTimeZone, NSISO8601DateFormatWithDashSeparatorInDate,
    NSISO8601DateFormatWithDay, NSISO8601DateFormatWithFractionalSeconds,
    NSISO8601DateFormatWithFullDate, NSISO8601DateFormatWithFullTime,
    NSISO8601DateFormatWithInternetDateTime, NSISO8601DateFormatWithMonth,
    NSISO8601DateFormatWithSpaceBetweenDateAndTime, NSISO8601DateFormatWithTime,
    NSISO8601DateFormatWithTimeZone, NSISO8601DateFormatWithWeekOfYear,
    NSISO8601DateFormatWithYear, NSISO8601DateFormatter,
};
pub use self::generated::NSIndexPath::NSIndexPath;
pub use self::generated::NSIndexSet::{NSIndexSet, NSMutableIndexSet};
pub use self::generated::NSInflectionRule::{NSInflectionRule, NSInflectionRuleExplicit};
pub use self::generated::NSInvocation::NSInvocation;
pub use self::generated::NSItemProvider::{
    NSExtensionJavaScriptFinalizeArgumentKey, NSExtensionJavaScriptPreprocessingResultsKey,
    NSItemProvider, NSItemProviderCompletionHandler, NSItemProviderErrorCode,
    NSItemProviderErrorDomain, NSItemProviderFileOptionOpenInPlace, NSItemProviderFileOptions,
    NSItemProviderItemUnavailableError, NSItemProviderLoadHandler,
    NSItemProviderPreferredImageSizeKey, NSItemProviderReading,
    NSItemProviderRepresentationVisibility, NSItemProviderRepresentationVisibilityAll,
    NSItemProviderRepresentationVisibilityGroup, NSItemProviderRepresentationVisibilityOwnProcess,
    NSItemProviderRepresentationVisibilityTeam, NSItemProviderUnavailableCoercionError,
    NSItemProviderUnexpectedValueClassError, NSItemProviderUnknownError, NSItemProviderWriting,
};
pub use self::generated::NSJSONSerialization::{
    NSJSONReadingAllowFragments, NSJSONReadingFragmentsAllowed, NSJSONReadingJSON5Allowed,
    NSJSONReadingMutableContainers, NSJSONReadingMutableLeaves, NSJSONReadingOptions,
    NSJSONReadingTopLevelDictionaryAssumed, NSJSONSerialization, NSJSONWritingFragmentsAllowed,
    NSJSONWritingOptions, NSJSONWritingPrettyPrinted, NSJSONWritingSortedKeys,
    NSJSONWritingWithoutEscapingSlashes,
};
pub use self::generated::NSKeyValueCoding::{
    NSAverageKeyValueOperator, NSCountKeyValueOperator, NSDistinctUnionOfArraysKeyValueOperator,
    NSDistinctUnionOfObjectsKeyValueOperator, NSDistinctUnionOfSetsKeyValueOperator,
    NSKeyValueOperator, NSMaximumKeyValueOperator, NSMinimumKeyValueOperator,
    NSSumKeyValueOperator, NSUndefinedKeyException, NSUnionOfArraysKeyValueOperator,
    NSUnionOfObjectsKeyValueOperator, NSUnionOfSetsKeyValueOperator,
};
pub use self::generated::NSKeyValueObserving::{
    NSKeyValueChange, NSKeyValueChangeIndexesKey, NSKeyValueChangeInsertion, NSKeyValueChangeKey,
    NSKeyValueChangeKindKey, NSKeyValueChangeNewKey, NSKeyValueChangeNotificationIsPriorKey,
    NSKeyValueChangeOldKey, NSKeyValueChangeRemoval, NSKeyValueChangeReplacement,
    NSKeyValueChangeSetting, NSKeyValueIntersectSetMutation, NSKeyValueMinusSetMutation,
    NSKeyValueObservingOptionInitial, NSKeyValueObservingOptionNew, NSKeyValueObservingOptionOld,
    NSKeyValueObservingOptionPrior, NSKeyValueObservingOptions, NSKeyValueSetMutationKind,
    NSKeyValueSetSetMutation, NSKeyValueUnionSetMutation,
};
pub use self::generated::NSKeyedArchiver::{
    NSInvalidArchiveOperationException, NSInvalidUnarchiveOperationException,
    NSKeyedArchiveRootObjectKey, NSKeyedArchiver, NSKeyedArchiverDelegate, NSKeyedUnarchiver,
    NSKeyedUnarchiverDelegate,
};
pub use self::generated::NSLengthFormatter::{
    NSLengthFormatter, NSLengthFormatterUnit, NSLengthFormatterUnitCentimeter,
    NSLengthFormatterUnitFoot, NSLengthFormatterUnitInch, NSLengthFormatterUnitKilometer,
    NSLengthFormatterUnitMeter, NSLengthFormatterUnitMile, NSLengthFormatterUnitMillimeter,
    NSLengthFormatterUnitYard,
};
pub use self::generated::NSLinguisticTagger::{
    NSLinguisticTag, NSLinguisticTagAdjective, NSLinguisticTagAdverb, NSLinguisticTagClassifier,
    NSLinguisticTagCloseParenthesis, NSLinguisticTagCloseQuote, NSLinguisticTagConjunction,
    NSLinguisticTagDash, NSLinguisticTagDeterminer, NSLinguisticTagIdiom,
    NSLinguisticTagInterjection, NSLinguisticTagNoun, NSLinguisticTagNumber,
    NSLinguisticTagOpenParenthesis, NSLinguisticTagOpenQuote, NSLinguisticTagOrganizationName,
    NSLinguisticTagOther, NSLinguisticTagOtherPunctuation, NSLinguisticTagOtherWhitespace,
    NSLinguisticTagOtherWord, NSLinguisticTagParagraphBreak, NSLinguisticTagParticle,
    NSLinguisticTagPersonalName, NSLinguisticTagPlaceName, NSLinguisticTagPreposition,
    NSLinguisticTagPronoun, NSLinguisticTagPunctuation, NSLinguisticTagScheme,
    NSLinguisticTagSchemeLanguage, NSLinguisticTagSchemeLemma, NSLinguisticTagSchemeLexicalClass,
    NSLinguisticTagSchemeNameType, NSLinguisticTagSchemeNameTypeOrLexicalClass,
    NSLinguisticTagSchemeScript, NSLinguisticTagSchemeTokenType, NSLinguisticTagSentenceTerminator,
    NSLinguisticTagVerb, NSLinguisticTagWhitespace, NSLinguisticTagWord, NSLinguisticTagWordJoiner,
    NSLinguisticTagger, NSLinguisticTaggerJoinNames, NSLinguisticTaggerOmitOther,
    NSLinguisticTaggerOmitPunctuation, NSLinguisticTaggerOmitWhitespace,
    NSLinguisticTaggerOmitWords, NSLinguisticTaggerOptions, NSLinguisticTaggerUnit,
    NSLinguisticTaggerUnitDocument, NSLinguisticTaggerUnitParagraph,
    NSLinguisticTaggerUnitSentence, NSLinguisticTaggerUnitWord,
};
pub use self::generated::NSListFormatter::NSListFormatter;
pub use self::generated::NSLocale::{
    NSBuddhistCalendar, NSChineseCalendar, NSCurrentLocaleDidChangeNotification,
    NSGregorianCalendar, NSHebrewCalendar, NSISO8601Calendar, NSIndianCalendar, NSIslamicCalendar,
    NSIslamicCivilCalendar, NSJapaneseCalendar, NSLocale,
    NSLocaleAlternateQuotationBeginDelimiterKey, NSLocaleAlternateQuotationEndDelimiterKey,
    NSLocaleCalendar, NSLocaleCollationIdentifier, NSLocaleCollatorIdentifier, NSLocaleCountryCode,
    NSLocaleCurrencyCode, NSLocaleCurrencySymbol, NSLocaleDecimalSeparator,
    NSLocaleExemplarCharacterSet, NSLocaleGroupingSeparator, NSLocaleIdentifier, NSLocaleKey,
    NSLocaleLanguageCode, NSLocaleLanguageDirection, NSLocaleLanguageDirectionBottomToTop,
    NSLocaleLanguageDirectionLeftToRight, NSLocaleLanguageDirectionRightToLeft,
    NSLocaleLanguageDirectionTopToBottom, NSLocaleLanguageDirectionUnknown,
    NSLocaleMeasurementSystem, NSLocaleQuotationBeginDelimiterKey,
    NSLocaleQuotationEndDelimiterKey, NSLocaleScriptCode, NSLocaleUsesMetricSystem,
    NSLocaleVariantCode, NSPersianCalendar, NSRepublicOfChinaCalendar,
};
pub use self::generated::NSLock::{
    NSCondition, NSConditionLock, NSLock, NSLocking, NSRecursiveLock,
};
pub use self::generated::NSMapTable::{
    NSIntMapKeyCallBacks, NSIntMapValueCallBacks, NSIntegerMapKeyCallBacks,
    NSIntegerMapValueCallBacks, NSMapEnumerator, NSMapTable, NSMapTableCopyIn,
    NSMapTableKeyCallBacks, NSMapTableObjectPointerPersonality, NSMapTableOptions,
    NSMapTableStrongMemory, NSMapTableValueCallBacks, NSMapTableWeakMemory,
    NSMapTableZeroingWeakMemory, NSNonOwnedPointerMapKeyCallBacks,
    NSNonOwnedPointerMapValueCallBacks, NSNonOwnedPointerOrNullMapKeyCallBacks,
    NSNonRetainedObjectMapKeyCallBacks, NSNonRetainedObjectMapValueCallBacks,
    NSObjectMapKeyCallBacks, NSObjectMapValueCallBacks, NSOwnedPointerMapKeyCallBacks,
    NSOwnedPointerMapValueCallBacks,
};
pub use self::generated::NSMassFormatter::{
    NSMassFormatter, NSMassFormatterUnit, NSMassFormatterUnitGram, NSMassFormatterUnitKilogram,
    NSMassFormatterUnitOunce, NSMassFormatterUnitPound, NSMassFormatterUnitStone,
};
pub use self::generated::NSMeasurement::NSMeasurement;
pub use self::generated::NSMeasurementFormatter::{
    NSMeasurementFormatter, NSMeasurementFormatterUnitOptions,
    NSMeasurementFormatterUnitOptionsNaturalScale, NSMeasurementFormatterUnitOptionsProvidedUnit,
    NSMeasurementFormatterUnitOptionsTemperatureWithoutUnit,
};
pub use self::generated::NSMetadata::{
    NSMetadataItem, NSMetadataQuery, NSMetadataQueryAccessibleUbiquitousExternalDocumentsScope,
    NSMetadataQueryAttributeValueTuple, NSMetadataQueryDelegate,
    NSMetadataQueryDidFinishGatheringNotification, NSMetadataQueryDidStartGatheringNotification,
    NSMetadataQueryDidUpdateNotification, NSMetadataQueryGatheringProgressNotification,
    NSMetadataQueryIndexedLocalComputerScope, NSMetadataQueryIndexedNetworkScope,
    NSMetadataQueryLocalComputerScope, NSMetadataQueryNetworkScope,
    NSMetadataQueryResultContentRelevanceAttribute, NSMetadataQueryResultGroup,
    NSMetadataQueryUbiquitousDataScope, NSMetadataQueryUbiquitousDocumentsScope,
    NSMetadataQueryUpdateAddedItemsKey, NSMetadataQueryUpdateChangedItemsKey,
    NSMetadataQueryUpdateRemovedItemsKey, NSMetadataQueryUserHomeScope,
};
pub use self::generated::NSMetadataAttributes::{
    NSMetadataItemAcquisitionMakeKey, NSMetadataItemAcquisitionModelKey, NSMetadataItemAlbumKey,
    NSMetadataItemAltitudeKey, NSMetadataItemApertureKey, NSMetadataItemAppleLoopDescriptorsKey,
    NSMetadataItemAppleLoopsKeyFilterTypeKey, NSMetadataItemAppleLoopsLoopModeKey,
    NSMetadataItemAppleLoopsRootKeyKey, NSMetadataItemApplicationCategoriesKey,
    NSMetadataItemAttributeChangeDateKey, NSMetadataItemAudiencesKey,
    NSMetadataItemAudioBitRateKey, NSMetadataItemAudioChannelCountKey,
    NSMetadataItemAudioEncodingApplicationKey, NSMetadataItemAudioSampleRateKey,
    NSMetadataItemAudioTrackNumberKey, NSMetadataItemAuthorAddressesKey,
    NSMetadataItemAuthorEmailAddressesKey, NSMetadataItemAuthorsKey,
    NSMetadataItemBitsPerSampleKey, NSMetadataItemCFBundleIdentifierKey,
    NSMetadataItemCameraOwnerKey, NSMetadataItemCityKey, NSMetadataItemCodecsKey,
    NSMetadataItemColorSpaceKey, NSMetadataItemCommentKey, NSMetadataItemComposerKey,
    NSMetadataItemContactKeywordsKey, NSMetadataItemContentCreationDateKey,
    NSMetadataItemContentModificationDateKey, NSMetadataItemContentTypeKey,
    NSMetadataItemContentTypeTreeKey, NSMetadataItemContributorsKey, NSMetadataItemCopyrightKey,
    NSMetadataItemCountryKey, NSMetadataItemCoverageKey, NSMetadataItemCreatorKey,
    NSMetadataItemDateAddedKey, NSMetadataItemDeliveryTypeKey, NSMetadataItemDescriptionKey,
    NSMetadataItemDirectorKey, NSMetadataItemDisplayNameKey, NSMetadataItemDownloadedDateKey,
    NSMetadataItemDueDateKey, NSMetadataItemDurationSecondsKey, NSMetadataItemEXIFGPSVersionKey,
    NSMetadataItemEXIFVersionKey, NSMetadataItemEditorsKey, NSMetadataItemEmailAddressesKey,
    NSMetadataItemEncodingApplicationsKey, NSMetadataItemExecutableArchitecturesKey,
    NSMetadataItemExecutablePlatformKey, NSMetadataItemExposureModeKey,
    NSMetadataItemExposureProgramKey, NSMetadataItemExposureTimeSecondsKey,
    NSMetadataItemExposureTimeStringKey, NSMetadataItemFNumberKey,
    NSMetadataItemFSContentChangeDateKey, NSMetadataItemFSCreationDateKey, NSMetadataItemFSNameKey,
    NSMetadataItemFSSizeKey, NSMetadataItemFinderCommentKey, NSMetadataItemFlashOnOffKey,
    NSMetadataItemFocalLength35mmKey, NSMetadataItemFocalLengthKey, NSMetadataItemFontsKey,
    NSMetadataItemGPSAreaInformationKey, NSMetadataItemGPSDOPKey, NSMetadataItemGPSDateStampKey,
    NSMetadataItemGPSDestBearingKey, NSMetadataItemGPSDestDistanceKey,
    NSMetadataItemGPSDestLatitudeKey, NSMetadataItemGPSDestLongitudeKey,
    NSMetadataItemGPSDifferentalKey, NSMetadataItemGPSMapDatumKey, NSMetadataItemGPSMeasureModeKey,
    NSMetadataItemGPSProcessingMethodKey, NSMetadataItemGPSStatusKey, NSMetadataItemGPSTrackKey,
    NSMetadataItemGenreKey, NSMetadataItemHasAlphaChannelKey, NSMetadataItemHeadlineKey,
    NSMetadataItemISOSpeedKey, NSMetadataItemIdentifierKey, NSMetadataItemImageDirectionKey,
    NSMetadataItemInformationKey, NSMetadataItemInstantMessageAddressesKey,
    NSMetadataItemInstructionsKey, NSMetadataItemIsApplicationManagedKey,
    NSMetadataItemIsGeneralMIDISequenceKey, NSMetadataItemIsLikelyJunkKey,
    NSMetadataItemIsUbiquitousKey, NSMetadataItemKeySignatureKey, NSMetadataItemKeywordsKey,
    NSMetadataItemKindKey, NSMetadataItemLanguagesKey, NSMetadataItemLastUsedDateKey,
    NSMetadataItemLatitudeKey, NSMetadataItemLayerNamesKey, NSMetadataItemLensModelKey,
    NSMetadataItemLongitudeKey, NSMetadataItemLyricistKey, NSMetadataItemMaxApertureKey,
    NSMetadataItemMediaTypesKey, NSMetadataItemMeteringModeKey, NSMetadataItemMusicalGenreKey,
    NSMetadataItemMusicalInstrumentCategoryKey, NSMetadataItemMusicalInstrumentNameKey,
    NSMetadataItemNamedLocationKey, NSMetadataItemNumberOfPagesKey, NSMetadataItemOrganizationsKey,
    NSMetadataItemOrientationKey, NSMetadataItemOriginalFormatKey, NSMetadataItemOriginalSourceKey,
    NSMetadataItemPageHeightKey, NSMetadataItemPageWidthKey, NSMetadataItemParticipantsKey,
    NSMetadataItemPathKey, NSMetadataItemPerformersKey, NSMetadataItemPhoneNumbersKey,
    NSMetadataItemPixelCountKey, NSMetadataItemPixelHeightKey, NSMetadataItemPixelWidthKey,
    NSMetadataItemProducerKey, NSMetadataItemProfileNameKey, NSMetadataItemProjectsKey,
    NSMetadataItemPublishersKey, NSMetadataItemRecipientAddressesKey,
    NSMetadataItemRecipientEmailAddressesKey, NSMetadataItemRecipientsKey,
    NSMetadataItemRecordingDateKey, NSMetadataItemRecordingYearKey, NSMetadataItemRedEyeOnOffKey,
    NSMetadataItemResolutionHeightDPIKey, NSMetadataItemResolutionWidthDPIKey,
    NSMetadataItemRightsKey, NSMetadataItemSecurityMethodKey, NSMetadataItemSpeedKey,
    NSMetadataItemStarRatingKey, NSMetadataItemStateOrProvinceKey, NSMetadataItemStreamableKey,
    NSMetadataItemSubjectKey, NSMetadataItemTempoKey, NSMetadataItemTextContentKey,
    NSMetadataItemThemeKey, NSMetadataItemTimeSignatureKey, NSMetadataItemTimestampKey,
    NSMetadataItemTitleKey, NSMetadataItemTotalBitRateKey, NSMetadataItemURLKey,
    NSMetadataItemVersionKey, NSMetadataItemVideoBitRateKey, NSMetadataItemWhereFromsKey,
    NSMetadataItemWhiteBalanceKey, NSMetadataUbiquitousItemContainerDisplayNameKey,
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
    NSMetadataUbiquitousSharedItemPermissionsReadWrite, NSMetadataUbiquitousSharedItemRoleOwner,
    NSMetadataUbiquitousSharedItemRoleParticipant,
};
pub use self::generated::NSMethodSignature::NSMethodSignature;
pub use self::generated::NSMorphology::{
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
pub use self::generated::NSNetServices::{
    NSNetService, NSNetServiceBrowser, NSNetServiceBrowserDelegate, NSNetServiceDelegate,
    NSNetServiceListenForConnections, NSNetServiceNoAutoRename, NSNetServiceOptions,
    NSNetServicesActivityInProgress, NSNetServicesBadArgumentError, NSNetServicesCancelledError,
    NSNetServicesCollisionError, NSNetServicesError, NSNetServicesErrorCode,
    NSNetServicesErrorDomain, NSNetServicesInvalidError,
    NSNetServicesMissingRequiredConfigurationError, NSNetServicesNotFoundError,
    NSNetServicesTimeoutError, NSNetServicesUnknownError,
};
pub use self::generated::NSNotification::{
    NSNotification, NSNotificationCenter, NSNotificationName,
};
pub use self::generated::NSNotificationQueue::{
    NSNotificationCoalescing, NSNotificationCoalescingOnName, NSNotificationCoalescingOnSender,
    NSNotificationNoCoalescing, NSNotificationQueue, NSPostASAP, NSPostNow, NSPostWhenIdle,
    NSPostingStyle,
};
pub use self::generated::NSNull::NSNull;
pub use self::generated::NSNumberFormatter::{
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
pub use self::generated::NSObjCRuntime::{
    NSComparator, NSComparisonResult, NSEnumerationConcurrent, NSEnumerationOptions,
    NSEnumerationReverse, NSExceptionName, NSFoundationVersionNumber, NSOrderedAscending,
    NSOrderedDescending, NSOrderedSame, NSQualityOfService, NSQualityOfServiceBackground,
    NSQualityOfServiceDefault, NSQualityOfServiceUserInitiated, NSQualityOfServiceUserInteractive,
    NSQualityOfServiceUtility, NSRunLoopMode, NSSortConcurrent, NSSortOptions, NSSortStable,
};
pub use self::generated::NSObject::{
    NSCoding, NSCopying, NSDiscardableContent, NSMutableCopying, NSSecureCoding,
};
pub use self::generated::NSOperation::{
    NSBlockOperation, NSInvocationOperation, NSInvocationOperationCancelledException,
    NSInvocationOperationVoidResultException, NSOperation, NSOperationQueue,
    NSOperationQueueDefaultMaxConcurrentOperationCount, NSOperationQueuePriority,
    NSOperationQueuePriorityHigh, NSOperationQueuePriorityLow, NSOperationQueuePriorityNormal,
    NSOperationQueuePriorityVeryHigh, NSOperationQueuePriorityVeryLow,
};
pub use self::generated::NSOrderedCollectionChange::{
    NSCollectionChangeInsert, NSCollectionChangeRemove, NSCollectionChangeType,
    NSOrderedCollectionChange,
};
pub use self::generated::NSOrderedCollectionDifference::{
    NSOrderedCollectionDifference, NSOrderedCollectionDifferenceCalculationInferMoves,
    NSOrderedCollectionDifferenceCalculationOmitInsertedObjects,
    NSOrderedCollectionDifferenceCalculationOmitRemovedObjects,
    NSOrderedCollectionDifferenceCalculationOptions,
};
pub use self::generated::NSOrderedSet::{NSMutableOrderedSet, NSOrderedSet};
pub use self::generated::NSOrthography::NSOrthography;
pub use self::generated::NSPathUtilities::{
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
pub use self::generated::NSPersonNameComponents::NSPersonNameComponents;
pub use self::generated::NSPersonNameComponentsFormatter::{
    NSPersonNameComponentDelimiter, NSPersonNameComponentFamilyName,
    NSPersonNameComponentGivenName, NSPersonNameComponentKey, NSPersonNameComponentMiddleName,
    NSPersonNameComponentNickname, NSPersonNameComponentPrefix, NSPersonNameComponentSuffix,
    NSPersonNameComponentsFormatter, NSPersonNameComponentsFormatterOptions,
    NSPersonNameComponentsFormatterPhonetic, NSPersonNameComponentsFormatterStyle,
    NSPersonNameComponentsFormatterStyleAbbreviated, NSPersonNameComponentsFormatterStyleDefault,
    NSPersonNameComponentsFormatterStyleLong, NSPersonNameComponentsFormatterStyleMedium,
    NSPersonNameComponentsFormatterStyleShort,
};
pub use self::generated::NSPointerArray::NSPointerArray;
pub use self::generated::NSPointerFunctions::{
    NSPointerFunctions, NSPointerFunctionsCStringPersonality, NSPointerFunctionsCopyIn,
    NSPointerFunctionsIntegerPersonality, NSPointerFunctionsMachVirtualMemory,
    NSPointerFunctionsMallocMemory, NSPointerFunctionsObjectPersonality,
    NSPointerFunctionsObjectPointerPersonality, NSPointerFunctionsOpaqueMemory,
    NSPointerFunctionsOpaquePersonality, NSPointerFunctionsOptions, NSPointerFunctionsStrongMemory,
    NSPointerFunctionsStructPersonality, NSPointerFunctionsWeakMemory,
    NSPointerFunctionsZeroingWeakMemory,
};
pub use self::generated::NSPort::{
    NSMachPort, NSMachPortDeallocateNone, NSMachPortDeallocateReceiveRight,
    NSMachPortDeallocateSendRight, NSMachPortDelegate, NSMachPortOptions, NSMessagePort, NSPort,
    NSPortDelegate, NSPortDidBecomeInvalidNotification, NSSocketNativeHandle, NSSocketPort,
};
pub use self::generated::NSPortCoder::NSPortCoder;
pub use self::generated::NSPortMessage::NSPortMessage;
pub use self::generated::NSPortNameServer::{
    NSMachBootstrapServer, NSMessagePortNameServer, NSPortNameServer, NSSocketPortNameServer,
};
pub use self::generated::NSPredicate::NSPredicate;
pub use self::generated::NSProcessInfo::{
    NSActivityAutomaticTerminationDisabled, NSActivityBackground,
    NSActivityIdleDisplaySleepDisabled, NSActivityIdleSystemSleepDisabled,
    NSActivityLatencyCritical, NSActivityOptions, NSActivitySuddenTerminationDisabled,
    NSActivityUserInitiated, NSActivityUserInitiatedAllowingIdleSystemSleep, NSHPUXOperatingSystem,
    NSMACHOperatingSystem, NSOSF1OperatingSystem, NSOperatingSystemVersion, NSProcessInfo,
    NSProcessInfoPowerStateDidChangeNotification, NSProcessInfoThermalState,
    NSProcessInfoThermalStateCritical, NSProcessInfoThermalStateDidChangeNotification,
    NSProcessInfoThermalStateFair, NSProcessInfoThermalStateNominal,
    NSProcessInfoThermalStateSerious, NSSolarisOperatingSystem, NSSunOSOperatingSystem,
    NSWindows95OperatingSystem, NSWindowsNTOperatingSystem,
};
pub use self::generated::NSProgress::{
    NSProgress, NSProgressEstimatedTimeRemainingKey, NSProgressFileAnimationImageKey,
    NSProgressFileAnimationImageOriginalRectKey, NSProgressFileCompletedCountKey,
    NSProgressFileIconKey, NSProgressFileOperationKind, NSProgressFileOperationKindCopying,
    NSProgressFileOperationKindDecompressingAfterDownloading,
    NSProgressFileOperationKindDownloading, NSProgressFileOperationKindDuplicating,
    NSProgressFileOperationKindKey, NSProgressFileOperationKindReceiving,
    NSProgressFileOperationKindUploading, NSProgressFileTotalCountKey, NSProgressFileURLKey,
    NSProgressKind, NSProgressKindFile, NSProgressPublishingHandler, NSProgressReporting,
    NSProgressThroughputKey, NSProgressUnpublishingHandler, NSProgressUserInfoKey,
};
pub use self::generated::NSPropertyList::{
    NSPropertyListBinaryFormat_v1_0, NSPropertyListFormat, NSPropertyListImmutable,
    NSPropertyListMutabilityOptions, NSPropertyListMutableContainers,
    NSPropertyListMutableContainersAndLeaves, NSPropertyListOpenStepFormat,
    NSPropertyListReadOptions, NSPropertyListSerialization, NSPropertyListWriteOptions,
    NSPropertyListXMLFormat_v1_0,
};
pub use self::generated::NSProtocolChecker::NSProtocolChecker;
pub use self::generated::NSRange::{NSRange, NSRangePointer};
pub use self::generated::NSRegularExpression::{
    NSDataDetector, NSMatchingAnchored, NSMatchingCompleted, NSMatchingFlags, NSMatchingHitEnd,
    NSMatchingInternalError, NSMatchingOptions, NSMatchingProgress, NSMatchingReportCompletion,
    NSMatchingReportProgress, NSMatchingRequiredEnd, NSMatchingWithTransparentBounds,
    NSMatchingWithoutAnchoringBounds, NSRegularExpression,
    NSRegularExpressionAllowCommentsAndWhitespace, NSRegularExpressionAnchorsMatchLines,
    NSRegularExpressionCaseInsensitive, NSRegularExpressionDotMatchesLineSeparators,
    NSRegularExpressionIgnoreMetacharacters, NSRegularExpressionOptions,
    NSRegularExpressionUseUnicodeWordBoundaries, NSRegularExpressionUseUnixLineSeparators,
};
pub use self::generated::NSRelativeDateTimeFormatter::{
    NSRelativeDateTimeFormatter, NSRelativeDateTimeFormatterStyle,
    NSRelativeDateTimeFormatterStyleNamed, NSRelativeDateTimeFormatterStyleNumeric,
    NSRelativeDateTimeFormatterUnitsStyle, NSRelativeDateTimeFormatterUnitsStyleAbbreviated,
    NSRelativeDateTimeFormatterUnitsStyleFull, NSRelativeDateTimeFormatterUnitsStyleShort,
    NSRelativeDateTimeFormatterUnitsStyleSpellOut,
};
pub use self::generated::NSRunLoop::{NSDefaultRunLoopMode, NSRunLoop, NSRunLoopCommonModes};
pub use self::generated::NSScanner::NSScanner;
pub use self::generated::NSScriptClassDescription::NSScriptClassDescription;
pub use self::generated::NSScriptCoercionHandler::NSScriptCoercionHandler;
pub use self::generated::NSScriptCommand::{
    NSArgumentEvaluationScriptError, NSArgumentsWrongScriptError, NSCannotCreateScriptCommandError,
    NSInternalScriptError, NSKeySpecifierEvaluationScriptError, NSNoScriptError,
    NSOperationNotSupportedForKeyScriptError, NSReceiverEvaluationScriptError,
    NSReceiversCantHandleCommandScriptError, NSRequiredArgumentsMissingScriptError,
    NSScriptCommand, NSUnknownKeyScriptError,
};
pub use self::generated::NSScriptCommandDescription::NSScriptCommandDescription;
pub use self::generated::NSScriptExecutionContext::NSScriptExecutionContext;
pub use self::generated::NSScriptKeyValueCoding::NSOperationNotSupportedForKeyException;
pub use self::generated::NSScriptObjectSpecifiers::{
    NSContainerSpecifierError, NSEverySubelement, NSIndexSpecifier, NSIndexSubelement,
    NSInsertionPosition, NSInternalSpecifierError, NSInvalidIndexSpecifierError, NSMiddleSpecifier,
    NSMiddleSubelement, NSNameSpecifier, NSNoSpecifierError, NSNoSubelement,
    NSNoTopLevelContainersSpecifierError, NSOperationNotSupportedForKeySpecifierError,
    NSPositionAfter, NSPositionBefore, NSPositionBeginning, NSPositionEnd, NSPositionReplace,
    NSPositionalSpecifier, NSPropertySpecifier, NSRandomSpecifier, NSRandomSubelement,
    NSRangeSpecifier, NSRelativeAfter, NSRelativeBefore, NSRelativePosition, NSRelativeSpecifier,
    NSScriptObjectSpecifier, NSUniqueIDSpecifier, NSUnknownKeySpecifierError, NSWhoseSpecifier,
    NSWhoseSubelementIdentifier,
};
pub use self::generated::NSScriptStandardSuiteCommands::{
    NSCloneCommand, NSCloseCommand, NSCountCommand, NSCreateCommand, NSDeleteCommand,
    NSExistsCommand, NSGetCommand, NSMoveCommand, NSQuitCommand, NSSaveOptions, NSSaveOptionsAsk,
    NSSaveOptionsNo, NSSaveOptionsYes, NSSetCommand,
};
pub use self::generated::NSScriptSuiteRegistry::NSScriptSuiteRegistry;
pub use self::generated::NSScriptWhoseTests::{
    NSBeginsWithComparison, NSContainsComparison, NSEndsWithComparison, NSEqualToComparison,
    NSGreaterThanComparison, NSGreaterThanOrEqualToComparison, NSLessThanComparison,
    NSLessThanOrEqualToComparison, NSLogicalTest, NSScriptWhoseTest, NSSpecifierTest,
    NSTestComparisonOperation,
};
pub use self::generated::NSSet::{NSCountedSet, NSMutableSet, NSSet};
pub use self::generated::NSSortDescriptor::NSSortDescriptor;
pub use self::generated::NSSpellServer::{
    NSGrammarCorrections, NSGrammarRange, NSGrammarUserDescription, NSSpellServer,
    NSSpellServerDelegate,
};
pub use self::generated::NSStream::{
    NSInputStream, NSOutputStream, NSStream, NSStreamDataWrittenToMemoryStreamKey,
    NSStreamDelegate, NSStreamEvent, NSStreamEventEndEncountered, NSStreamEventErrorOccurred,
    NSStreamEventHasBytesAvailable, NSStreamEventHasSpaceAvailable, NSStreamEventNone,
    NSStreamEventOpenCompleted, NSStreamFileCurrentOffsetKey, NSStreamNetworkServiceType,
    NSStreamNetworkServiceTypeBackground, NSStreamNetworkServiceTypeCallSignaling,
    NSStreamNetworkServiceTypeValue, NSStreamNetworkServiceTypeVideo,
    NSStreamNetworkServiceTypeVoIP, NSStreamNetworkServiceTypeVoice, NSStreamPropertyKey,
    NSStreamSOCKSErrorDomain, NSStreamSOCKSProxyConfiguration, NSStreamSOCKSProxyConfigurationKey,
    NSStreamSOCKSProxyHostKey, NSStreamSOCKSProxyPasswordKey, NSStreamSOCKSProxyPortKey,
    NSStreamSOCKSProxyUserKey, NSStreamSOCKSProxyVersion, NSStreamSOCKSProxyVersion4,
    NSStreamSOCKSProxyVersion5, NSStreamSOCKSProxyVersionKey, NSStreamSocketSSLErrorDomain,
    NSStreamSocketSecurityLevel, NSStreamSocketSecurityLevelKey,
    NSStreamSocketSecurityLevelNegotiatedSSL, NSStreamSocketSecurityLevelNone,
    NSStreamSocketSecurityLevelSSLv2, NSStreamSocketSecurityLevelSSLv3,
    NSStreamSocketSecurityLevelTLSv1, NSStreamStatus, NSStreamStatusAtEnd, NSStreamStatusClosed,
    NSStreamStatusError, NSStreamStatusNotOpen, NSStreamStatusOpen, NSStreamStatusOpening,
    NSStreamStatusReading, NSStreamStatusWriting,
};
pub use self::generated::NSString::{
    unichar, NSASCIIStringEncoding, NSAnchoredSearch, NSBackwardsSearch, NSCaseInsensitiveSearch,
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
    NSStringEnumerationByLines, NSStringEnumerationByParagraphs, NSStringEnumerationBySentences,
    NSStringEnumerationByWords, NSStringEnumerationLocalized, NSStringEnumerationOptions,
    NSStringEnumerationReverse, NSStringEnumerationSubstringNotRequired, NSStringTransform,
    NSStringTransformFullwidthToHalfwidth, NSStringTransformHiraganaToKatakana,
    NSStringTransformLatinToArabic, NSStringTransformLatinToCyrillic,
    NSStringTransformLatinToGreek, NSStringTransformLatinToHangul, NSStringTransformLatinToHebrew,
    NSStringTransformLatinToHiragana, NSStringTransformLatinToKatakana,
    NSStringTransformLatinToThai, NSStringTransformMandarinToLatin,
    NSStringTransformStripCombiningMarks, NSStringTransformStripDiacritics,
    NSStringTransformToLatin, NSStringTransformToUnicodeName, NSStringTransformToXMLHex,
    NSSymbolStringEncoding, NSUTF16BigEndianStringEncoding, NSUTF16LittleEndianStringEncoding,
    NSUTF16StringEncoding, NSUTF32BigEndianStringEncoding, NSUTF32LittleEndianStringEncoding,
    NSUTF32StringEncoding, NSUTF8StringEncoding, NSUnicodeStringEncoding, NSWidthInsensitiveSearch,
    NSWindowsCP1250StringEncoding, NSWindowsCP1251StringEncoding, NSWindowsCP1252StringEncoding,
    NSWindowsCP1253StringEncoding, NSWindowsCP1254StringEncoding,
};
pub use self::generated::NSTask::{
    NSTask, NSTaskDidTerminateNotification, NSTaskTerminationReason, NSTaskTerminationReasonExit,
    NSTaskTerminationReasonUncaughtSignal,
};
pub use self::generated::NSTextCheckingResult::{
    NSTextCheckingAirlineKey, NSTextCheckingAllCustomTypes, NSTextCheckingAllSystemTypes,
    NSTextCheckingAllTypes, NSTextCheckingCityKey, NSTextCheckingCountryKey,
    NSTextCheckingFlightKey, NSTextCheckingJobTitleKey, NSTextCheckingKey, NSTextCheckingNameKey,
    NSTextCheckingOrganizationKey, NSTextCheckingPhoneKey, NSTextCheckingResult,
    NSTextCheckingStateKey, NSTextCheckingStreetKey, NSTextCheckingType, NSTextCheckingTypeAddress,
    NSTextCheckingTypeCorrection, NSTextCheckingTypeDash, NSTextCheckingTypeDate,
    NSTextCheckingTypeGrammar, NSTextCheckingTypeLink, NSTextCheckingTypeOrthography,
    NSTextCheckingTypePhoneNumber, NSTextCheckingTypeQuote, NSTextCheckingTypeRegularExpression,
    NSTextCheckingTypeReplacement, NSTextCheckingTypeSpelling,
    NSTextCheckingTypeTransitInformation, NSTextCheckingTypes, NSTextCheckingZIPKey,
};
pub use self::generated::NSThread::{
    NSDidBecomeSingleThreadedNotification, NSThread, NSThreadWillExitNotification,
    NSWillBecomeMultiThreadedNotification,
};
pub use self::generated::NSTimeZone::{
    NSSystemTimeZoneDidChangeNotification, NSTimeZone, NSTimeZoneNameStyle,
    NSTimeZoneNameStyleDaylightSaving, NSTimeZoneNameStyleGeneric,
    NSTimeZoneNameStyleShortDaylightSaving, NSTimeZoneNameStyleShortGeneric,
    NSTimeZoneNameStyleShortStandard, NSTimeZoneNameStyleStandard,
};
pub use self::generated::NSTimer::NSTimer;
pub use self::generated::NSURLAuthenticationChallenge::{
    NSURLAuthenticationChallenge, NSURLAuthenticationChallengeSender,
};
pub use self::generated::NSURLCache::{
    NSCachedURLResponse, NSURLCache, NSURLCacheStorageAllowed,
    NSURLCacheStorageAllowedInMemoryOnly, NSURLCacheStorageNotAllowed, NSURLCacheStoragePolicy,
};
pub use self::generated::NSURLConnection::{
    NSURLConnection, NSURLConnectionDataDelegate, NSURLConnectionDelegate,
    NSURLConnectionDownloadDelegate,
};
pub use self::generated::NSURLCredential::{
    NSURLCredential, NSURLCredentialPersistence, NSURLCredentialPersistenceForSession,
    NSURLCredentialPersistenceNone, NSURLCredentialPersistencePermanent,
    NSURLCredentialPersistenceSynchronizable,
};
pub use self::generated::NSURLCredentialStorage::{
    NSURLCredentialStorage, NSURLCredentialStorageChangedNotification,
    NSURLCredentialStorageRemoveSynchronizableCredentials,
};
pub use self::generated::NSURLDownload::{NSURLDownload, NSURLDownloadDelegate};
pub use self::generated::NSURLError::{
    NSErrorFailingURLStringKey, NSURLErrorAppTransportSecurityRequiresSecureConnection,
    NSURLErrorBackgroundSessionInUseByAnotherProcess,
    NSURLErrorBackgroundSessionRequiresSharedContainer, NSURLErrorBackgroundSessionWasDisconnected,
    NSURLErrorBackgroundTaskCancelledReasonKey, NSURLErrorBadServerResponse, NSURLErrorBadURL,
    NSURLErrorCallIsActive, NSURLErrorCancelled,
    NSURLErrorCancelledReasonBackgroundUpdatesDisabled,
    NSURLErrorCancelledReasonInsufficientSystemResources,
    NSURLErrorCancelledReasonUserForceQuitApplication, NSURLErrorCannotCloseFile,
    NSURLErrorCannotConnectToHost, NSURLErrorCannotCreateFile, NSURLErrorCannotDecodeContentData,
    NSURLErrorCannotDecodeRawData, NSURLErrorCannotFindHost, NSURLErrorCannotLoadFromNetwork,
    NSURLErrorCannotMoveFile, NSURLErrorCannotOpenFile, NSURLErrorCannotParseResponse,
    NSURLErrorCannotRemoveFile, NSURLErrorCannotWriteToFile, NSURLErrorClientCertificateRejected,
    NSURLErrorClientCertificateRequired, NSURLErrorDNSLookupFailed,
    NSURLErrorDataLengthExceedsMaximum, NSURLErrorDataNotAllowed, NSURLErrorDomain,
    NSURLErrorDownloadDecodingFailedMidStream, NSURLErrorDownloadDecodingFailedToComplete,
    NSURLErrorFailingURLErrorKey, NSURLErrorFailingURLPeerTrustErrorKey,
    NSURLErrorFailingURLStringErrorKey, NSURLErrorFileDoesNotExist, NSURLErrorFileIsDirectory,
    NSURLErrorFileOutsideSafeArea, NSURLErrorHTTPTooManyRedirects,
    NSURLErrorInternationalRoamingOff, NSURLErrorNetworkConnectionLost,
    NSURLErrorNetworkUnavailableReason, NSURLErrorNetworkUnavailableReasonCellular,
    NSURLErrorNetworkUnavailableReasonConstrained, NSURLErrorNetworkUnavailableReasonExpensive,
    NSURLErrorNetworkUnavailableReasonKey, NSURLErrorNoPermissionsToReadFile,
    NSURLErrorNotConnectedToInternet, NSURLErrorRedirectToNonExistentLocation,
    NSURLErrorRequestBodyStreamExhausted, NSURLErrorResourceUnavailable,
    NSURLErrorSecureConnectionFailed, NSURLErrorServerCertificateHasBadDate,
    NSURLErrorServerCertificateHasUnknownRoot, NSURLErrorServerCertificateNotYetValid,
    NSURLErrorServerCertificateUntrusted, NSURLErrorTimedOut, NSURLErrorUnknown,
    NSURLErrorUnsupportedURL, NSURLErrorUserAuthenticationRequired,
    NSURLErrorUserCancelledAuthentication, NSURLErrorZeroByteResource,
};
pub use self::generated::NSURLHandle::{
    NSFTPPropertyActiveTransferModeKey, NSFTPPropertyFTPProxy, NSFTPPropertyFileOffsetKey,
    NSFTPPropertyUserLoginKey, NSFTPPropertyUserPasswordKey, NSHTTPPropertyErrorPageDataKey,
    NSHTTPPropertyHTTPProxy, NSHTTPPropertyRedirectionHeadersKey,
    NSHTTPPropertyServerHTTPVersionKey, NSHTTPPropertyStatusCodeKey, NSHTTPPropertyStatusReasonKey,
    NSURLHandle, NSURLHandleClient, NSURLHandleLoadFailed, NSURLHandleLoadInProgress,
    NSURLHandleLoadSucceeded, NSURLHandleNotLoaded, NSURLHandleStatus,
};
pub use self::generated::NSURLProtectionSpace::{
    NSURLAuthenticationMethodClientCertificate, NSURLAuthenticationMethodDefault,
    NSURLAuthenticationMethodHTMLForm, NSURLAuthenticationMethodHTTPBasic,
    NSURLAuthenticationMethodHTTPDigest, NSURLAuthenticationMethodNTLM,
    NSURLAuthenticationMethodNegotiate, NSURLAuthenticationMethodServerTrust, NSURLProtectionSpace,
    NSURLProtectionSpaceFTP, NSURLProtectionSpaceFTPProxy, NSURLProtectionSpaceHTTP,
    NSURLProtectionSpaceHTTPProxy, NSURLProtectionSpaceHTTPS, NSURLProtectionSpaceHTTPSProxy,
    NSURLProtectionSpaceSOCKSProxy,
};
pub use self::generated::NSURLProtocol::{NSURLProtocol, NSURLProtocolClient};
pub use self::generated::NSURLRequest::{
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
pub use self::generated::NSURLResponse::{NSHTTPURLResponse, NSURLResponse};
pub use self::generated::NSURLSession::{
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
    NSURLSessionResponseBecomeStream, NSURLSessionResponseCancel, NSURLSessionResponseDisposition,
    NSURLSessionStreamDelegate, NSURLSessionStreamTask, NSURLSessionTask, NSURLSessionTaskDelegate,
    NSURLSessionTaskMetrics, NSURLSessionTaskMetricsDomainResolutionProtocol,
    NSURLSessionTaskMetricsDomainResolutionProtocolHTTPS,
    NSURLSessionTaskMetricsDomainResolutionProtocolTCP,
    NSURLSessionTaskMetricsDomainResolutionProtocolTLS,
    NSURLSessionTaskMetricsDomainResolutionProtocolUDP,
    NSURLSessionTaskMetricsDomainResolutionProtocolUnknown,
    NSURLSessionTaskMetricsResourceFetchType, NSURLSessionTaskMetricsResourceFetchTypeLocalCache,
    NSURLSessionTaskMetricsResourceFetchTypeNetworkLoad,
    NSURLSessionTaskMetricsResourceFetchTypeServerPush,
    NSURLSessionTaskMetricsResourceFetchTypeUnknown, NSURLSessionTaskPriorityDefault,
    NSURLSessionTaskPriorityHigh, NSURLSessionTaskPriorityLow, NSURLSessionTaskState,
    NSURLSessionTaskStateCanceling, NSURLSessionTaskStateCompleted, NSURLSessionTaskStateRunning,
    NSURLSessionTaskStateSuspended, NSURLSessionTaskTransactionMetrics,
    NSURLSessionTransferSizeUnknown, NSURLSessionUploadTask, NSURLSessionWebSocketCloseCode,
    NSURLSessionWebSocketCloseCodeAbnormalClosure, NSURLSessionWebSocketCloseCodeGoingAway,
    NSURLSessionWebSocketCloseCodeInternalServerError, NSURLSessionWebSocketCloseCodeInvalid,
    NSURLSessionWebSocketCloseCodeInvalidFramePayloadData,
    NSURLSessionWebSocketCloseCodeMandatoryExtensionMissing,
    NSURLSessionWebSocketCloseCodeMessageTooBig, NSURLSessionWebSocketCloseCodeNoStatusReceived,
    NSURLSessionWebSocketCloseCodeNormalClosure, NSURLSessionWebSocketCloseCodePolicyViolation,
    NSURLSessionWebSocketCloseCodeProtocolError, NSURLSessionWebSocketCloseCodeTLSHandshakeFailure,
    NSURLSessionWebSocketCloseCodeUnsupportedData, NSURLSessionWebSocketDelegate,
    NSURLSessionWebSocketMessage, NSURLSessionWebSocketMessageType,
    NSURLSessionWebSocketMessageTypeData, NSURLSessionWebSocketMessageTypeString,
    NSURLSessionWebSocketTask,
};
pub use self::generated::NSUbiquitousKeyValueStore::{
    NSUbiquitousKeyValueStore, NSUbiquitousKeyValueStoreAccountChange,
    NSUbiquitousKeyValueStoreChangeReasonKey, NSUbiquitousKeyValueStoreChangedKeysKey,
    NSUbiquitousKeyValueStoreDidChangeExternallyNotification,
    NSUbiquitousKeyValueStoreInitialSyncChange, NSUbiquitousKeyValueStoreQuotaViolationChange,
    NSUbiquitousKeyValueStoreServerChange,
};
pub use self::generated::NSUndoManager::{
    NSUndoCloseGroupingRunLoopOrdering, NSUndoManager, NSUndoManagerCheckpointNotification,
    NSUndoManagerDidCloseUndoGroupNotification, NSUndoManagerDidOpenUndoGroupNotification,
    NSUndoManagerDidRedoChangeNotification, NSUndoManagerDidUndoChangeNotification,
    NSUndoManagerGroupIsDiscardableKey, NSUndoManagerWillCloseUndoGroupNotification,
    NSUndoManagerWillRedoChangeNotification, NSUndoManagerWillUndoChangeNotification,
};
pub use self::generated::NSUnit::{
    NSDimension, NSUnit, NSUnitAcceleration, NSUnitAngle, NSUnitArea, NSUnitConcentrationMass,
    NSUnitConverter, NSUnitConverterLinear, NSUnitDispersion, NSUnitDuration, NSUnitElectricCharge,
    NSUnitElectricCurrent, NSUnitElectricPotentialDifference, NSUnitElectricResistance,
    NSUnitEnergy, NSUnitFrequency, NSUnitFuelEfficiency, NSUnitIlluminance,
    NSUnitInformationStorage, NSUnitLength, NSUnitMass, NSUnitPower, NSUnitPressure, NSUnitSpeed,
    NSUnitTemperature, NSUnitVolume,
};
pub use self::generated::NSUserActivity::{
    NSUserActivity, NSUserActivityDelegate, NSUserActivityPersistentIdentifier,
    NSUserActivityTypeBrowsingWeb,
};
pub use self::generated::NSUserDefaults::{
    NSAMPMDesignation, NSArgumentDomain, NSCurrencySymbol, NSDateFormatString, NSDateTimeOrdering,
    NSDecimalDigits, NSDecimalSeparator, NSEarlierTimeDesignations, NSGlobalDomain,
    NSHourNameDesignations, NSInternationalCurrencyString, NSLaterTimeDesignations,
    NSMonthNameArray, NSNegativeCurrencyFormatString, NSNextDayDesignations,
    NSNextNextDayDesignations, NSPositiveCurrencyFormatString, NSPriorDayDesignations,
    NSRegistrationDomain, NSShortDateFormatString, NSShortMonthNameArray,
    NSShortTimeDateFormatString, NSShortWeekDayNameArray, NSThisDayDesignations,
    NSThousandsSeparator, NSTimeDateFormatString, NSTimeFormatString,
    NSUbiquitousUserDefaultsCompletedInitialSyncNotification,
    NSUbiquitousUserDefaultsDidChangeAccountsNotification,
    NSUbiquitousUserDefaultsNoCloudAccountNotification, NSUserDefaults,
    NSUserDefaultsDidChangeNotification, NSUserDefaultsSizeLimitExceededNotification,
    NSWeekDayNameArray, NSYearMonthWeekDesignations,
};
pub use self::generated::NSUserNotification::{
    NSUserNotification, NSUserNotificationAction, NSUserNotificationActivationType,
    NSUserNotificationActivationTypeActionButtonClicked,
    NSUserNotificationActivationTypeAdditionalActionClicked,
    NSUserNotificationActivationTypeContentsClicked, NSUserNotificationActivationTypeNone,
    NSUserNotificationActivationTypeReplied, NSUserNotificationCenter,
    NSUserNotificationCenterDelegate, NSUserNotificationDefaultSoundName,
};
pub use self::generated::NSUserScriptTask::{
    NSUserAppleScriptTask, NSUserAppleScriptTaskCompletionHandler, NSUserAutomatorTask,
    NSUserAutomatorTaskCompletionHandler, NSUserScriptTask, NSUserScriptTaskCompletionHandler,
    NSUserUnixTask, NSUserUnixTaskCompletionHandler,
};
pub use self::generated::NSValue::{NSNumber, NSValue};
pub use self::generated::NSValueTransformer::{
    NSIsNilTransformerName, NSIsNotNilTransformerName, NSKeyedUnarchiveFromDataTransformerName,
    NSNegateBooleanTransformerName, NSSecureUnarchiveFromDataTransformer,
    NSSecureUnarchiveFromDataTransformerName, NSUnarchiveFromDataTransformerName,
    NSValueTransformer, NSValueTransformerName,
};
pub use self::generated::NSXMLDTDNode::{
    NSXMLAttributeCDATAKind, NSXMLAttributeEntitiesKind, NSXMLAttributeEntityKind,
    NSXMLAttributeEnumerationKind, NSXMLAttributeIDKind, NSXMLAttributeIDRefKind,
    NSXMLAttributeIDRefsKind, NSXMLAttributeNMTokenKind, NSXMLAttributeNMTokensKind,
    NSXMLAttributeNotationKind, NSXMLDTDNode, NSXMLDTDNodeKind, NSXMLElementDeclarationAnyKind,
    NSXMLElementDeclarationElementKind, NSXMLElementDeclarationEmptyKind,
    NSXMLElementDeclarationMixedKind, NSXMLElementDeclarationUndefinedKind, NSXMLEntityGeneralKind,
    NSXMLEntityParameterKind, NSXMLEntityParsedKind, NSXMLEntityPredefined,
    NSXMLEntityUnparsedKind,
};
pub use self::generated::NSXMLDocument::{
    NSXMLDocument, NSXMLDocumentContentKind, NSXMLDocumentHTMLKind, NSXMLDocumentTextKind,
    NSXMLDocumentXHTMLKind, NSXMLDocumentXMLKind,
};
pub use self::generated::NSXMLElement::NSXMLElement;
pub use self::generated::NSXMLNode::{
    NSXMLAttributeDeclarationKind, NSXMLAttributeKind, NSXMLCommentKind, NSXMLDTDKind,
    NSXMLDocumentKind, NSXMLElementDeclarationKind, NSXMLElementKind, NSXMLEntityDeclarationKind,
    NSXMLInvalidKind, NSXMLNamespaceKind, NSXMLNode, NSXMLNodeKind, NSXMLNotationDeclarationKind,
    NSXMLProcessingInstructionKind, NSXMLTextKind,
};
pub use self::generated::NSXMLNodeOptions::{
    NSXMLDocumentIncludeContentTypeDeclaration, NSXMLDocumentTidyHTML, NSXMLDocumentTidyXML,
    NSXMLDocumentValidate, NSXMLDocumentXInclude, NSXMLNodeCompactEmptyElement,
    NSXMLNodeExpandEmptyElement, NSXMLNodeIsCDATA, NSXMLNodeLoadExternalEntitiesAlways,
    NSXMLNodeLoadExternalEntitiesNever, NSXMLNodeLoadExternalEntitiesSameOriginOnly,
    NSXMLNodeNeverEscapeContents, NSXMLNodeOptions, NSXMLNodeOptionsNone, NSXMLNodePreserveAll,
    NSXMLNodePreserveAttributeOrder, NSXMLNodePreserveCDATA, NSXMLNodePreserveCharacterReferences,
    NSXMLNodePreserveDTD, NSXMLNodePreserveEmptyElements, NSXMLNodePreserveEntities,
    NSXMLNodePreserveNamespaceOrder, NSXMLNodePreservePrefixes, NSXMLNodePreserveQuotes,
    NSXMLNodePreserveWhitespace, NSXMLNodePrettyPrint, NSXMLNodePromoteSignificantWhitespace,
    NSXMLNodeUseDoubleQuotes, NSXMLNodeUseSingleQuotes,
};
pub use self::generated::NSXMLParser::{
    NSXMLParser, NSXMLParserAttributeHasNoValueError, NSXMLParserAttributeListNotFinishedError,
    NSXMLParserAttributeListNotStartedError, NSXMLParserAttributeNotFinishedError,
    NSXMLParserAttributeNotStartedError, NSXMLParserAttributeRedefinedError,
    NSXMLParserCDATANotFinishedError, NSXMLParserCharacterRefAtEOFError,
    NSXMLParserCharacterRefInDTDError, NSXMLParserCharacterRefInEpilogError,
    NSXMLParserCharacterRefInPrologError, NSXMLParserCommentContainsDoubleHyphenError,
    NSXMLParserCommentNotFinishedError, NSXMLParserConditionalSectionNotFinishedError,
    NSXMLParserConditionalSectionNotStartedError, NSXMLParserDOCTYPEDeclNotFinishedError,
    NSXMLParserDelegate, NSXMLParserDelegateAbortedParseError, NSXMLParserDocumentStartError,
    NSXMLParserElementContentDeclNotFinishedError, NSXMLParserElementContentDeclNotStartedError,
    NSXMLParserEmptyDocumentError, NSXMLParserEncodingNotSupportedError,
    NSXMLParserEntityBoundaryError, NSXMLParserEntityIsExternalError,
    NSXMLParserEntityIsParameterError, NSXMLParserEntityNotFinishedError,
    NSXMLParserEntityNotStartedError, NSXMLParserEntityRefAtEOFError,
    NSXMLParserEntityRefInDTDError, NSXMLParserEntityRefInEpilogError,
    NSXMLParserEntityRefInPrologError, NSXMLParserEntityRefLoopError,
    NSXMLParserEntityReferenceMissingSemiError, NSXMLParserEntityReferenceWithoutNameError,
    NSXMLParserEntityValueRequiredError, NSXMLParserEqualExpectedError, NSXMLParserError,
    NSXMLParserErrorDomain, NSXMLParserExternalEntityResolvingPolicy,
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
    NSXMLParserNMTOKENRequiredError, NSXMLParserNamespaceDeclarationError, NSXMLParserNoDTDError,
    NSXMLParserNotWellBalancedError, NSXMLParserNotationNotFinishedError,
    NSXMLParserNotationNotStartedError, NSXMLParserOutOfMemoryError,
    NSXMLParserPCDATARequiredError, NSXMLParserParsedEntityRefAtEOFError,
    NSXMLParserParsedEntityRefInEpilogError, NSXMLParserParsedEntityRefInInternalError,
    NSXMLParserParsedEntityRefInInternalSubsetError, NSXMLParserParsedEntityRefInPrologError,
    NSXMLParserParsedEntityRefMissingSemiError, NSXMLParserParsedEntityRefNoNameError,
    NSXMLParserPrematureDocumentEndError, NSXMLParserProcessingInstructionNotFinishedError,
    NSXMLParserProcessingInstructionNotStartedError, NSXMLParserPublicIdentifierRequiredError,
    NSXMLParserResolveExternalEntitiesAlways, NSXMLParserResolveExternalEntitiesNever,
    NSXMLParserResolveExternalEntitiesNoNetwork, NSXMLParserResolveExternalEntitiesSameOriginOnly,
    NSXMLParserSeparatorRequiredError, NSXMLParserSpaceRequiredError,
    NSXMLParserStandaloneValueError, NSXMLParserStringNotClosedError,
    NSXMLParserStringNotStartedError, NSXMLParserTagNameMismatchError, NSXMLParserURIFragmentError,
    NSXMLParserURIRequiredError, NSXMLParserUndeclaredEntityError, NSXMLParserUnfinishedTagError,
    NSXMLParserUnknownEncodingError, NSXMLParserUnparsedEntityError,
    NSXMLParserXMLDeclNotFinishedError, NSXMLParserXMLDeclNotStartedError,
};
pub use self::generated::NSXPCConnection::{
    NSXPCCoder, NSXPCConnection, NSXPCConnectionOptions, NSXPCConnectionPrivileged, NSXPCInterface,
    NSXPCListener, NSXPCListenerDelegate, NSXPCListenerEndpoint, NSXPCProxyCreating,
};
pub use self::generated::NSZone::{NSCollectorDisabledOption, NSScannedOption};
pub use self::generated::NSURL::{
    NSFileSecurity, NSThumbnail1024x1024SizeKey, NSURLAddedToDirectoryDateKey,
    NSURLApplicationIsScriptableKey, NSURLAttributeModificationDateKey,
    NSURLBookmarkCreationMinimalBookmark, NSURLBookmarkCreationOptions,
    NSURLBookmarkCreationPreferFileIDResolution,
    NSURLBookmarkCreationSecurityScopeAllowOnlyReadAccess,
    NSURLBookmarkCreationSuitableForBookmarkFile, NSURLBookmarkCreationWithSecurityScope,
    NSURLBookmarkCreationWithoutImplicitSecurityScope, NSURLBookmarkFileCreationOptions,
    NSURLBookmarkResolutionOptions, NSURLBookmarkResolutionWithSecurityScope,
    NSURLBookmarkResolutionWithoutImplicitStartAccessing, NSURLBookmarkResolutionWithoutMounting,
    NSURLBookmarkResolutionWithoutUI, NSURLCanonicalPathKey, NSURLComponents,
    NSURLContentAccessDateKey, NSURLContentModificationDateKey, NSURLContentTypeKey,
    NSURLCreationDateKey, NSURLCustomIconKey, NSURLDocumentIdentifierKey, NSURLEffectiveIconKey,
    NSURLFileAllocatedSizeKey, NSURLFileContentIdentifierKey, NSURLFileProtectionComplete,
    NSURLFileProtectionCompleteUnlessOpen, NSURLFileProtectionCompleteUntilFirstUserAuthentication,
    NSURLFileProtectionKey, NSURLFileProtectionNone, NSURLFileProtectionType,
    NSURLFileResourceIdentifierKey, NSURLFileResourceType, NSURLFileResourceTypeBlockSpecial,
    NSURLFileResourceTypeCharacterSpecial, NSURLFileResourceTypeDirectory,
    NSURLFileResourceTypeKey, NSURLFileResourceTypeNamedPipe, NSURLFileResourceTypeRegular,
    NSURLFileResourceTypeSocket, NSURLFileResourceTypeSymbolicLink, NSURLFileResourceTypeUnknown,
    NSURLFileScheme, NSURLFileSecurityKey, NSURLFileSizeKey, NSURLGenerationIdentifierKey,
    NSURLHasHiddenExtensionKey, NSURLIsAliasFileKey, NSURLIsApplicationKey, NSURLIsDirectoryKey,
    NSURLIsExcludedFromBackupKey, NSURLIsExecutableKey, NSURLIsHiddenKey, NSURLIsMountTriggerKey,
    NSURLIsPackageKey, NSURLIsPurgeableKey, NSURLIsReadableKey, NSURLIsRegularFileKey,
    NSURLIsSparseKey, NSURLIsSymbolicLinkKey, NSURLIsSystemImmutableKey, NSURLIsUbiquitousItemKey,
    NSURLIsUserImmutableKey, NSURLIsVolumeKey, NSURLIsWritableKey, NSURLKeysOfUnsetValuesKey,
    NSURLLabelColorKey, NSURLLabelNumberKey, NSURLLinkCountKey, NSURLLocalizedLabelKey,
    NSURLLocalizedNameKey, NSURLLocalizedTypeDescriptionKey, NSURLMayHaveExtendedAttributesKey,
    NSURLMayShareFileContentKey, NSURLNameKey, NSURLParentDirectoryURLKey, NSURLPathKey,
    NSURLPreferredIOBlockSizeKey, NSURLQuarantinePropertiesKey, NSURLQueryItem, NSURLResourceKey,
    NSURLTagNamesKey, NSURLThumbnailDictionaryItem, NSURLThumbnailDictionaryKey, NSURLThumbnailKey,
    NSURLTotalFileAllocatedSizeKey, NSURLTotalFileSizeKey, NSURLTypeIdentifierKey,
    NSURLUbiquitousItemContainerDisplayNameKey, NSURLUbiquitousItemDownloadRequestedKey,
    NSURLUbiquitousItemDownloadingErrorKey, NSURLUbiquitousItemDownloadingStatus,
    NSURLUbiquitousItemDownloadingStatusCurrent, NSURLUbiquitousItemDownloadingStatusDownloaded,
    NSURLUbiquitousItemDownloadingStatusKey, NSURLUbiquitousItemDownloadingStatusNotDownloaded,
    NSURLUbiquitousItemHasUnresolvedConflictsKey, NSURLUbiquitousItemIsDownloadedKey,
    NSURLUbiquitousItemIsDownloadingKey, NSURLUbiquitousItemIsExcludedFromSyncKey,
    NSURLUbiquitousItemIsSharedKey, NSURLUbiquitousItemIsUploadedKey,
    NSURLUbiquitousItemIsUploadingKey, NSURLUbiquitousItemPercentDownloadedKey,
    NSURLUbiquitousItemPercentUploadedKey, NSURLUbiquitousItemUploadingErrorKey,
    NSURLUbiquitousSharedItemCurrentUserPermissionsKey,
    NSURLUbiquitousSharedItemCurrentUserRoleKey,
    NSURLUbiquitousSharedItemMostRecentEditorNameComponentsKey,
    NSURLUbiquitousSharedItemOwnerNameComponentsKey, NSURLUbiquitousSharedItemPermissions,
    NSURLUbiquitousSharedItemPermissionsReadOnly, NSURLUbiquitousSharedItemPermissionsReadWrite,
    NSURLUbiquitousSharedItemRole, NSURLUbiquitousSharedItemRoleOwner,
    NSURLUbiquitousSharedItemRoleParticipant, NSURLVolumeAvailableCapacityForImportantUsageKey,
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
    NSURLVolumeSupportsVolumeSizesKey, NSURLVolumeSupportsZeroRunsKey, NSURLVolumeTotalCapacityKey,
    NSURLVolumeURLForRemountingKey, NSURLVolumeURLKey, NSURLVolumeUUIDStringKey, NSURL,
};
pub use self::generated::NSUUID::NSUUID;
pub use self::generated::NSXMLDTD::NSXMLDTD;
