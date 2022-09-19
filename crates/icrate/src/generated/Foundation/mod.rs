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
    pub use super::NSAffineTransform::NSAffineTransform;
    pub use super::NSAppleEventDescriptor::NSAppleEventDescriptor;
    pub use super::NSAppleEventManager::NSAppleEventManager;
    pub use super::NSAppleScript::NSAppleScript;
    pub use super::NSArchiver::{NSArchiver, NSUnarchiver};
    pub use super::NSArray::{NSArray, NSMutableArray};
    pub use super::NSAttributedString::{
        NSAttributedString, NSAttributedStringKey, NSAttributedStringMarkdownParsingOptions,
        NSMutableAttributedString, NSPresentationIntent,
    };
    pub use super::NSAutoreleasePool::NSAutoreleasePool;
    pub use super::NSBackgroundActivityScheduler::NSBackgroundActivityScheduler;
    pub use super::NSBundle::{NSBundle, NSBundleResourceRequest};
    pub use super::NSByteCountFormatter::NSByteCountFormatter;
    pub use super::NSCache::{NSCache, NSCacheDelegate};
    pub use super::NSCalendar::{NSCalendar, NSCalendarIdentifier, NSDateComponents};
    pub use super::NSCalendarDate::NSCalendarDate;
    pub use super::NSCharacterSet::{NSCharacterSet, NSMutableCharacterSet};
    pub use super::NSClassDescription::NSClassDescription;
    pub use super::NSCoder::NSCoder;
    pub use super::NSComparisonPredicate::NSComparisonPredicate;
    pub use super::NSCompoundPredicate::NSCompoundPredicate;
    pub use super::NSConnection::{NSConnection, NSConnectionDelegate, NSDistantObjectRequest};
    pub use super::NSData::{NSData, NSMutableData, NSPurgeableData};
    pub use super::NSDate::NSDate;
    pub use super::NSDateComponentsFormatter::NSDateComponentsFormatter;
    pub use super::NSDateFormatter::NSDateFormatter;
    pub use super::NSDateInterval::NSDateInterval;
    pub use super::NSDateIntervalFormatter::NSDateIntervalFormatter;
    pub use super::NSDecimalNumber::{
        NSDecimalNumber, NSDecimalNumberBehaviors, NSDecimalNumberHandler,
    };
    pub use super::NSDictionary::{NSDictionary, NSMutableDictionary};
    pub use super::NSDistantObject::NSDistantObject;
    pub use super::NSDistributedLock::NSDistributedLock;
    pub use super::NSDistributedNotificationCenter::{
        NSDistributedNotificationCenter, NSDistributedNotificationCenterType,
    };
    pub use super::NSEnergyFormatter::NSEnergyFormatter;
    pub use super::NSEnumerator::{NSEnumerator, NSFastEnumeration};
    pub use super::NSError::{NSError, NSErrorDomain, NSErrorUserInfoKey};
    pub use super::NSException::{NSAssertionHandler, NSException};
    pub use super::NSExpression::NSExpression;
    pub use super::NSExtensionContext::NSExtensionContext;
    pub use super::NSExtensionItem::NSExtensionItem;
    pub use super::NSExtensionRequestHandling::NSExtensionRequestHandling;
    pub use super::NSFileCoordinator::{NSFileAccessIntent, NSFileCoordinator};
    pub use super::NSFileHandle::{NSFileHandle, NSPipe};
    pub use super::NSFileManager::{
        NSDirectoryEnumerator, NSFileAttributeKey, NSFileAttributeType, NSFileManager,
        NSFileManagerDelegate, NSFileProtectionType, NSFileProviderService,
        NSFileProviderServiceName,
    };
    pub use super::NSFilePresenter::NSFilePresenter;
    pub use super::NSFileVersion::NSFileVersion;
    pub use super::NSFileWrapper::NSFileWrapper;
    pub use super::NSFormatter::NSFormatter;
    pub use super::NSGarbageCollector::NSGarbageCollector;
    pub use super::NSGeometry::{NSPoint, NSRect, NSSize};
    pub use super::NSHTTPCookie::{
        NSHTTPCookie, NSHTTPCookiePropertyKey, NSHTTPCookieStringPolicy,
    };
    pub use super::NSHTTPCookieStorage::NSHTTPCookieStorage;
    pub use super::NSHashTable::{NSHashTable, NSHashTableOptions};
    pub use super::NSHost::NSHost;
    pub use super::NSISO8601DateFormatter::NSISO8601DateFormatter;
    pub use super::NSIndexPath::NSIndexPath;
    pub use super::NSIndexSet::{NSIndexSet, NSMutableIndexSet};
    pub use super::NSInflectionRule::{NSInflectionRule, NSInflectionRuleExplicit};
    pub use super::NSInvocation::NSInvocation;
    pub use super::NSItemProvider::{NSItemProvider, NSItemProviderReading, NSItemProviderWriting};
    pub use super::NSJSONSerialization::NSJSONSerialization;
    pub use super::NSKeyValueCoding::NSKeyValueOperator;
    pub use super::NSKeyValueObserving::NSKeyValueChangeKey;
    pub use super::NSKeyedArchiver::{
        NSKeyedArchiver, NSKeyedArchiverDelegate, NSKeyedUnarchiver, NSKeyedUnarchiverDelegate,
    };
    pub use super::NSLengthFormatter::NSLengthFormatter;
    pub use super::NSLinguisticTagger::{
        NSLinguisticTag, NSLinguisticTagScheme, NSLinguisticTagger,
    };
    pub use super::NSListFormatter::NSListFormatter;
    pub use super::NSLocale::{NSLocale, NSLocaleKey};
    pub use super::NSLock::{NSCondition, NSConditionLock, NSLock, NSLocking, NSRecursiveLock};
    pub use super::NSMapTable::{NSMapTable, NSMapTableOptions};
    pub use super::NSMassFormatter::NSMassFormatter;
    pub use super::NSMeasurement::NSMeasurement;
    pub use super::NSMeasurementFormatter::NSMeasurementFormatter;
    pub use super::NSMetadata::{
        NSMetadataItem, NSMetadataQuery, NSMetadataQueryAttributeValueTuple,
        NSMetadataQueryDelegate, NSMetadataQueryResultGroup,
    };
    pub use super::NSMethodSignature::NSMethodSignature;
    pub use super::NSMorphology::{NSMorphology, NSMorphologyCustomPronoun};
    pub use super::NSNetServices::{
        NSNetService, NSNetServiceBrowser, NSNetServiceBrowserDelegate, NSNetServiceDelegate,
    };
    pub use super::NSNotification::{NSNotification, NSNotificationCenter, NSNotificationName};
    pub use super::NSNotificationQueue::NSNotificationQueue;
    pub use super::NSNull::NSNull;
    pub use super::NSNumberFormatter::NSNumberFormatter;
    pub use super::NSObjCRuntime::{NSExceptionName, NSRunLoopMode};
    pub use super::NSObject::{
        NSCoding, NSCopying, NSDiscardableContent, NSMutableCopying, NSSecureCoding,
    };
    pub use super::NSOperation::{
        NSBlockOperation, NSInvocationOperation, NSOperation, NSOperationQueue,
    };
    pub use super::NSOrderedCollectionChange::NSOrderedCollectionChange;
    pub use super::NSOrderedCollectionDifference::NSOrderedCollectionDifference;
    pub use super::NSOrderedSet::{NSMutableOrderedSet, NSOrderedSet};
    pub use super::NSOrthography::NSOrthography;
    pub use super::NSPersonNameComponents::NSPersonNameComponents;
    pub use super::NSPersonNameComponentsFormatter::NSPersonNameComponentsFormatter;
    pub use super::NSPointerArray::NSPointerArray;
    pub use super::NSPointerFunctions::NSPointerFunctions;
    pub use super::NSPort::{
        NSMachPort, NSMachPortDelegate, NSMessagePort, NSPort, NSPortDelegate, NSSocketPort,
    };
    pub use super::NSPortCoder::NSPortCoder;
    pub use super::NSPortMessage::NSPortMessage;
    pub use super::NSPortNameServer::{
        NSMachBootstrapServer, NSMessagePortNameServer, NSPortNameServer, NSSocketPortNameServer,
    };
    pub use super::NSPredicate::NSPredicate;
    pub use super::NSProcessInfo::NSProcessInfo;
    pub use super::NSProgress::{
        NSProgress, NSProgressFileOperationKind, NSProgressKind, NSProgressReporting,
        NSProgressUserInfoKey,
    };
    pub use super::NSPropertyList::{
        NSPropertyListReadOptions, NSPropertyListSerialization, NSPropertyListWriteOptions,
    };
    pub use super::NSProtocolChecker::NSProtocolChecker;
    pub use super::NSProxy::NSProxy;
    pub use super::NSRegularExpression::{NSDataDetector, NSRegularExpression};
    pub use super::NSRelativeDateTimeFormatter::NSRelativeDateTimeFormatter;
    pub use super::NSRunLoop::NSRunLoop;
    pub use super::NSScanner::NSScanner;
    pub use super::NSScriptClassDescription::NSScriptClassDescription;
    pub use super::NSScriptCoercionHandler::NSScriptCoercionHandler;
    pub use super::NSScriptCommand::NSScriptCommand;
    pub use super::NSScriptCommandDescription::NSScriptCommandDescription;
    pub use super::NSScriptExecutionContext::NSScriptExecutionContext;
    pub use super::NSScriptObjectSpecifiers::{
        NSIndexSpecifier, NSMiddleSpecifier, NSNameSpecifier, NSPositionalSpecifier,
        NSPropertySpecifier, NSRandomSpecifier, NSRangeSpecifier, NSRelativeSpecifier,
        NSScriptObjectSpecifier, NSUniqueIDSpecifier, NSWhoseSpecifier,
    };
    pub use super::NSScriptStandardSuiteCommands::{
        NSCloneCommand, NSCloseCommand, NSCountCommand, NSCreateCommand, NSDeleteCommand,
        NSExistsCommand, NSGetCommand, NSMoveCommand, NSQuitCommand, NSSetCommand,
    };
    pub use super::NSScriptSuiteRegistry::NSScriptSuiteRegistry;
    pub use super::NSScriptWhoseTests::{NSLogicalTest, NSScriptWhoseTest, NSSpecifierTest};
    pub use super::NSSet::{NSCountedSet, NSMutableSet, NSSet};
    pub use super::NSSortDescriptor::NSSortDescriptor;
    pub use super::NSSpellServer::{NSSpellServer, NSSpellServerDelegate};
    pub use super::NSStream::{
        NSInputStream, NSOutputStream, NSStream, NSStreamDelegate, NSStreamNetworkServiceTypeValue,
        NSStreamPropertyKey, NSStreamSOCKSProxyConfiguration, NSStreamSOCKSProxyVersion,
        NSStreamSocketSecurityLevel,
    };
    pub use super::NSString::{
        NSConstantString, NSMutableString, NSSimpleCString, NSString, NSStringEncoding,
        NSStringEncodingDetectionOptionsKey, NSStringTransform,
    };
    pub use super::NSTask::NSTask;
    pub use super::NSTextCheckingResult::{
        NSTextCheckingKey, NSTextCheckingResult, NSTextCheckingTypes,
    };
    pub use super::NSThread::NSThread;
    pub use super::NSTimeZone::NSTimeZone;
    pub use super::NSTimer::NSTimer;
    pub use super::NSURLAuthenticationChallenge::{
        NSURLAuthenticationChallenge, NSURLAuthenticationChallengeSender,
    };
    pub use super::NSURLCache::{NSCachedURLResponse, NSURLCache};
    pub use super::NSURLConnection::{
        NSURLConnection, NSURLConnectionDataDelegate, NSURLConnectionDelegate,
        NSURLConnectionDownloadDelegate,
    };
    pub use super::NSURLCredential::NSURLCredential;
    pub use super::NSURLCredentialStorage::NSURLCredentialStorage;
    pub use super::NSURLDownload::{NSURLDownload, NSURLDownloadDelegate};
    pub use super::NSURLHandle::{NSURLHandle, NSURLHandleClient};
    pub use super::NSURLProtectionSpace::NSURLProtectionSpace;
    pub use super::NSURLProtocol::{NSURLProtocol, NSURLProtocolClient};
    pub use super::NSURLRequest::{NSMutableURLRequest, NSURLRequest};
    pub use super::NSURLResponse::{NSHTTPURLResponse, NSURLResponse};
    pub use super::NSURLSession::{
        NSURLSession, NSURLSessionConfiguration, NSURLSessionDataDelegate, NSURLSessionDataTask,
        NSURLSessionDelegate, NSURLSessionDownloadDelegate, NSURLSessionDownloadTask,
        NSURLSessionStreamDelegate, NSURLSessionStreamTask, NSURLSessionTask,
        NSURLSessionTaskDelegate, NSURLSessionTaskMetrics, NSURLSessionTaskTransactionMetrics,
        NSURLSessionUploadTask, NSURLSessionWebSocketDelegate, NSURLSessionWebSocketMessage,
        NSURLSessionWebSocketTask,
    };
    pub use super::NSUbiquitousKeyValueStore::NSUbiquitousKeyValueStore;
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
        NSUserNotification, NSUserNotificationAction, NSUserNotificationCenter,
        NSUserNotificationCenterDelegate,
    };
    pub use super::NSUserScriptTask::{
        NSUserAppleScriptTask, NSUserAutomatorTask, NSUserScriptTask, NSUserUnixTask,
    };
    pub use super::NSValue::{NSNumber, NSValue};
    pub use super::NSValueTransformer::{
        NSSecureUnarchiveFromDataTransformer, NSValueTransformer, NSValueTransformerName,
    };
    pub use super::NSXMLDTDNode::NSXMLDTDNode;
    pub use super::NSXMLDocument::NSXMLDocument;
    pub use super::NSXMLElement::NSXMLElement;
    pub use super::NSXMLNode::NSXMLNode;
    pub use super::NSXMLParser::{NSXMLParser, NSXMLParserDelegate};
    pub use super::NSXPCConnection::{
        NSXPCCoder, NSXPCConnection, NSXPCInterface, NSXPCListener, NSXPCListenerDelegate,
        NSXPCListenerEndpoint, NSXPCProxyCreating,
    };
    pub use super::NSURL::{
        NSFileSecurity, NSURLBookmarkFileCreationOptions, NSURLComponents, NSURLFileProtectionType,
        NSURLFileResourceType, NSURLQueryItem, NSURLResourceKey, NSURLThumbnailDictionaryItem,
        NSURLUbiquitousItemDownloadingStatus, NSURLUbiquitousSharedItemPermissions,
        NSURLUbiquitousSharedItemRole, NSURL,
    };
    pub use super::NSUUID::NSUUID;
    pub use super::NSXMLDTD::NSXMLDTD;
}
