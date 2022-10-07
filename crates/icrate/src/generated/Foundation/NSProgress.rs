use super::__exported::NSDictionary;
use super::__exported::NSLock;
use super::__exported::NSMutableDictionary;
use super::__exported::NSMutableSet;
use super::__exported::NSXPCConnection;
use super::__exported::NSURL;
use super::__exported::NSUUID;
use crate::Foundation::generated::NSDictionary::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
pub type NSProgressKind = NSString;
pub type NSProgressUserInfoKey = NSString;
pub type NSProgressFileOperationKind = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSProgress;
    unsafe impl ClassType for NSProgress {
        type Super = NSObject;
    }
);
impl NSProgress {
    pub unsafe fn currentProgress() -> Option<Id<NSProgress, Shared>> {
        msg_send_id![Self::class(), currentProgress]
    }
    pub unsafe fn progressWithTotalUnitCount(unitCount: int64_t) -> Id<NSProgress, Shared> {
        msg_send_id![Self::class(), progressWithTotalUnitCount: unitCount]
    }
    pub unsafe fn discreteProgressWithTotalUnitCount(unitCount: int64_t) -> Id<NSProgress, Shared> {
        msg_send_id![Self::class(), discreteProgressWithTotalUnitCount: unitCount]
    }
    pub unsafe fn progressWithTotalUnitCount_parent_pendingUnitCount(
        unitCount: int64_t,
        parent: &NSProgress,
        portionOfParentTotalUnitCount: int64_t,
    ) -> Id<NSProgress, Shared> {
        msg_send_id![
            Self::class(),
            progressWithTotalUnitCount: unitCount,
            parent: parent,
            pendingUnitCount: portionOfParentTotalUnitCount
        ]
    }
    pub unsafe fn initWithParent_userInfo(
        &self,
        parentProgressOrNil: Option<&NSProgress>,
        userInfoOrNil: Option<&NSDictionary<NSProgressUserInfoKey, Object>>,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithParent: parentProgressOrNil,
            userInfo: userInfoOrNil
        ]
    }
    pub unsafe fn becomeCurrentWithPendingUnitCount(&self, unitCount: int64_t) {
        msg_send![self, becomeCurrentWithPendingUnitCount: unitCount]
    }
    pub unsafe fn performAsCurrentWithPendingUnitCount_usingBlock(
        &self,
        unitCount: int64_t,
        work: TodoBlock,
    ) {
        msg_send![
            self,
            performAsCurrentWithPendingUnitCount: unitCount,
            usingBlock: work
        ]
    }
    pub unsafe fn resignCurrent(&self) {
        msg_send![self, resignCurrent]
    }
    pub unsafe fn addChild_withPendingUnitCount(&self, child: &NSProgress, inUnitCount: int64_t) {
        msg_send![self, addChild: child, withPendingUnitCount: inUnitCount]
    }
    pub unsafe fn totalUnitCount(&self) -> int64_t {
        msg_send![self, totalUnitCount]
    }
    pub unsafe fn setTotalUnitCount(&self, totalUnitCount: int64_t) {
        msg_send![self, setTotalUnitCount: totalUnitCount]
    }
    pub unsafe fn completedUnitCount(&self) -> int64_t {
        msg_send![self, completedUnitCount]
    }
    pub unsafe fn setCompletedUnitCount(&self, completedUnitCount: int64_t) {
        msg_send![self, setCompletedUnitCount: completedUnitCount]
    }
    pub unsafe fn localizedDescription(&self) -> Id<NSString, Shared> {
        msg_send_id![self, localizedDescription]
    }
    pub unsafe fn setLocalizedDescription(&self, localizedDescription: Option<&NSString>) {
        msg_send![self, setLocalizedDescription: localizedDescription]
    }
    pub unsafe fn localizedAdditionalDescription(&self) -> Id<NSString, Shared> {
        msg_send_id![self, localizedAdditionalDescription]
    }
    pub unsafe fn setLocalizedAdditionalDescription(
        &self,
        localizedAdditionalDescription: Option<&NSString>,
    ) {
        msg_send![
            self,
            setLocalizedAdditionalDescription: localizedAdditionalDescription
        ]
    }
    pub unsafe fn isCancellable(&self) -> bool {
        msg_send![self, isCancellable]
    }
    pub unsafe fn setCancellable(&self, cancellable: bool) {
        msg_send![self, setCancellable: cancellable]
    }
    pub unsafe fn isPausable(&self) -> bool {
        msg_send![self, isPausable]
    }
    pub unsafe fn setPausable(&self, pausable: bool) {
        msg_send![self, setPausable: pausable]
    }
    pub unsafe fn isCancelled(&self) -> bool {
        msg_send![self, isCancelled]
    }
    pub unsafe fn isPaused(&self) -> bool {
        msg_send![self, isPaused]
    }
    pub unsafe fn cancellationHandler(&self) -> TodoBlock {
        msg_send![self, cancellationHandler]
    }
    pub unsafe fn setCancellationHandler(&self, cancellationHandler: TodoBlock) {
        msg_send![self, setCancellationHandler: cancellationHandler]
    }
    pub unsafe fn pausingHandler(&self) -> TodoBlock {
        msg_send![self, pausingHandler]
    }
    pub unsafe fn setPausingHandler(&self, pausingHandler: TodoBlock) {
        msg_send![self, setPausingHandler: pausingHandler]
    }
    pub unsafe fn resumingHandler(&self) -> TodoBlock {
        msg_send![self, resumingHandler]
    }
    pub unsafe fn setResumingHandler(&self, resumingHandler: TodoBlock) {
        msg_send![self, setResumingHandler: resumingHandler]
    }
    pub unsafe fn setUserInfoObject_forKey(
        &self,
        objectOrNil: Option<&Object>,
        key: &NSProgressUserInfoKey,
    ) {
        msg_send![self, setUserInfoObject: objectOrNil, forKey: key]
    }
    pub unsafe fn isIndeterminate(&self) -> bool {
        msg_send![self, isIndeterminate]
    }
    pub unsafe fn fractionCompleted(&self) -> c_double {
        msg_send![self, fractionCompleted]
    }
    pub unsafe fn isFinished(&self) -> bool {
        msg_send![self, isFinished]
    }
    pub unsafe fn cancel(&self) {
        msg_send![self, cancel]
    }
    pub unsafe fn pause(&self) {
        msg_send![self, pause]
    }
    pub unsafe fn resume(&self) {
        msg_send![self, resume]
    }
    pub unsafe fn userInfo(&self) -> Id<NSDictionary<NSProgressUserInfoKey, Object>, Shared> {
        msg_send_id![self, userInfo]
    }
    pub unsafe fn kind(&self) -> Option<Id<NSProgressKind, Shared>> {
        msg_send_id![self, kind]
    }
    pub unsafe fn setKind(&self, kind: Option<&NSProgressKind>) {
        msg_send![self, setKind: kind]
    }
    pub unsafe fn estimatedTimeRemaining(&self) -> Option<Id<NSNumber, Shared>> {
        msg_send_id![self, estimatedTimeRemaining]
    }
    pub unsafe fn setEstimatedTimeRemaining(&self, estimatedTimeRemaining: Option<&NSNumber>) {
        msg_send![self, setEstimatedTimeRemaining: estimatedTimeRemaining]
    }
    pub unsafe fn throughput(&self) -> Option<Id<NSNumber, Shared>> {
        msg_send_id![self, throughput]
    }
    pub unsafe fn setThroughput(&self, throughput: Option<&NSNumber>) {
        msg_send![self, setThroughput: throughput]
    }
    pub unsafe fn fileOperationKind(&self) -> Option<Id<NSProgressFileOperationKind, Shared>> {
        msg_send_id![self, fileOperationKind]
    }
    pub unsafe fn setFileOperationKind(
        &self,
        fileOperationKind: Option<&NSProgressFileOperationKind>,
    ) {
        msg_send![self, setFileOperationKind: fileOperationKind]
    }
    pub unsafe fn fileURL(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, fileURL]
    }
    pub unsafe fn setFileURL(&self, fileURL: Option<&NSURL>) {
        msg_send![self, setFileURL: fileURL]
    }
    pub unsafe fn fileTotalCount(&self) -> Option<Id<NSNumber, Shared>> {
        msg_send_id![self, fileTotalCount]
    }
    pub unsafe fn setFileTotalCount(&self, fileTotalCount: Option<&NSNumber>) {
        msg_send![self, setFileTotalCount: fileTotalCount]
    }
    pub unsafe fn fileCompletedCount(&self) -> Option<Id<NSNumber, Shared>> {
        msg_send_id![self, fileCompletedCount]
    }
    pub unsafe fn setFileCompletedCount(&self, fileCompletedCount: Option<&NSNumber>) {
        msg_send![self, setFileCompletedCount: fileCompletedCount]
    }
    pub unsafe fn publish(&self) {
        msg_send![self, publish]
    }
    pub unsafe fn unpublish(&self) {
        msg_send![self, unpublish]
    }
    pub unsafe fn addSubscriberForFileURL_withPublishingHandler(
        url: &NSURL,
        publishingHandler: NSProgressPublishingHandler,
    ) -> Id<Object, Shared> {
        msg_send_id![
            Self::class(),
            addSubscriberForFileURL: url,
            withPublishingHandler: publishingHandler
        ]
    }
    pub unsafe fn removeSubscriber(subscriber: &Object) {
        msg_send![Self::class(), removeSubscriber: subscriber]
    }
    pub unsafe fn isOld(&self) -> bool {
        msg_send![self, isOld]
    }
}
pub type NSProgressReporting = NSObject;
