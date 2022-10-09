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
use objc2::{extern_class, extern_methods, ClassType};
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
extern_methods!(
    unsafe impl NSProgress {
        #[method_id(currentProgress)]
        pub unsafe fn currentProgress() -> Option<Id<NSProgress, Shared>>;
        #[method_id(progressWithTotalUnitCount:)]
        pub unsafe fn progressWithTotalUnitCount(unitCount: int64_t) -> Id<NSProgress, Shared>;
        #[method_id(discreteProgressWithTotalUnitCount:)]
        pub unsafe fn discreteProgressWithTotalUnitCount(
            unitCount: int64_t,
        ) -> Id<NSProgress, Shared>;
        #[method_id(progressWithTotalUnitCount:parent:pendingUnitCount:)]
        pub unsafe fn progressWithTotalUnitCount_parent_pendingUnitCount(
            unitCount: int64_t,
            parent: &NSProgress,
            portionOfParentTotalUnitCount: int64_t,
        ) -> Id<NSProgress, Shared>;
        #[method_id(initWithParent:userInfo:)]
        pub unsafe fn initWithParent_userInfo(
            &self,
            parentProgressOrNil: Option<&NSProgress>,
            userInfoOrNil: Option<&NSDictionary<NSProgressUserInfoKey, Object>>,
        ) -> Id<Self, Shared>;
        #[method(becomeCurrentWithPendingUnitCount:)]
        pub unsafe fn becomeCurrentWithPendingUnitCount(&self, unitCount: int64_t);
        #[method(performAsCurrentWithPendingUnitCount:usingBlock:)]
        pub unsafe fn performAsCurrentWithPendingUnitCount_usingBlock(
            &self,
            unitCount: int64_t,
            work: TodoBlock,
        );
        #[method(resignCurrent)]
        pub unsafe fn resignCurrent(&self);
        #[method(addChild:withPendingUnitCount:)]
        pub unsafe fn addChild_withPendingUnitCount(
            &self,
            child: &NSProgress,
            inUnitCount: int64_t,
        );
        #[method(totalUnitCount)]
        pub unsafe fn totalUnitCount(&self) -> int64_t;
        #[method(setTotalUnitCount:)]
        pub unsafe fn setTotalUnitCount(&self, totalUnitCount: int64_t);
        #[method(completedUnitCount)]
        pub unsafe fn completedUnitCount(&self) -> int64_t;
        #[method(setCompletedUnitCount:)]
        pub unsafe fn setCompletedUnitCount(&self, completedUnitCount: int64_t);
        #[method_id(localizedDescription)]
        pub unsafe fn localizedDescription(&self) -> Id<NSString, Shared>;
        #[method(setLocalizedDescription:)]
        pub unsafe fn setLocalizedDescription(&self, localizedDescription: Option<&NSString>);
        #[method_id(localizedAdditionalDescription)]
        pub unsafe fn localizedAdditionalDescription(&self) -> Id<NSString, Shared>;
        #[method(setLocalizedAdditionalDescription:)]
        pub unsafe fn setLocalizedAdditionalDescription(
            &self,
            localizedAdditionalDescription: Option<&NSString>,
        );
        #[method(isCancellable)]
        pub unsafe fn isCancellable(&self) -> bool;
        #[method(setCancellable:)]
        pub unsafe fn setCancellable(&self, cancellable: bool);
        #[method(isPausable)]
        pub unsafe fn isPausable(&self) -> bool;
        #[method(setPausable:)]
        pub unsafe fn setPausable(&self, pausable: bool);
        #[method(isCancelled)]
        pub unsafe fn isCancelled(&self) -> bool;
        #[method(isPaused)]
        pub unsafe fn isPaused(&self) -> bool;
        #[method(cancellationHandler)]
        pub unsafe fn cancellationHandler(&self) -> TodoBlock;
        #[method(setCancellationHandler:)]
        pub unsafe fn setCancellationHandler(&self, cancellationHandler: TodoBlock);
        #[method(pausingHandler)]
        pub unsafe fn pausingHandler(&self) -> TodoBlock;
        #[method(setPausingHandler:)]
        pub unsafe fn setPausingHandler(&self, pausingHandler: TodoBlock);
        #[method(resumingHandler)]
        pub unsafe fn resumingHandler(&self) -> TodoBlock;
        #[method(setResumingHandler:)]
        pub unsafe fn setResumingHandler(&self, resumingHandler: TodoBlock);
        #[method(setUserInfoObject:forKey:)]
        pub unsafe fn setUserInfoObject_forKey(
            &self,
            objectOrNil: Option<&Object>,
            key: &NSProgressUserInfoKey,
        );
        #[method(isIndeterminate)]
        pub unsafe fn isIndeterminate(&self) -> bool;
        #[method(fractionCompleted)]
        pub unsafe fn fractionCompleted(&self) -> c_double;
        #[method(isFinished)]
        pub unsafe fn isFinished(&self) -> bool;
        #[method(cancel)]
        pub unsafe fn cancel(&self);
        #[method(pause)]
        pub unsafe fn pause(&self);
        #[method(resume)]
        pub unsafe fn resume(&self);
        #[method_id(userInfo)]
        pub unsafe fn userInfo(&self) -> Id<NSDictionary<NSProgressUserInfoKey, Object>, Shared>;
        #[method_id(kind)]
        pub unsafe fn kind(&self) -> Option<Id<NSProgressKind, Shared>>;
        #[method(setKind:)]
        pub unsafe fn setKind(&self, kind: Option<&NSProgressKind>);
        #[method_id(estimatedTimeRemaining)]
        pub unsafe fn estimatedTimeRemaining(&self) -> Option<Id<NSNumber, Shared>>;
        #[method(setEstimatedTimeRemaining:)]
        pub unsafe fn setEstimatedTimeRemaining(&self, estimatedTimeRemaining: Option<&NSNumber>);
        #[method_id(throughput)]
        pub unsafe fn throughput(&self) -> Option<Id<NSNumber, Shared>>;
        #[method(setThroughput:)]
        pub unsafe fn setThroughput(&self, throughput: Option<&NSNumber>);
        #[method_id(fileOperationKind)]
        pub unsafe fn fileOperationKind(&self) -> Option<Id<NSProgressFileOperationKind, Shared>>;
        #[method(setFileOperationKind:)]
        pub unsafe fn setFileOperationKind(
            &self,
            fileOperationKind: Option<&NSProgressFileOperationKind>,
        );
        #[method_id(fileURL)]
        pub unsafe fn fileURL(&self) -> Option<Id<NSURL, Shared>>;
        #[method(setFileURL:)]
        pub unsafe fn setFileURL(&self, fileURL: Option<&NSURL>);
        #[method_id(fileTotalCount)]
        pub unsafe fn fileTotalCount(&self) -> Option<Id<NSNumber, Shared>>;
        #[method(setFileTotalCount:)]
        pub unsafe fn setFileTotalCount(&self, fileTotalCount: Option<&NSNumber>);
        #[method_id(fileCompletedCount)]
        pub unsafe fn fileCompletedCount(&self) -> Option<Id<NSNumber, Shared>>;
        #[method(setFileCompletedCount:)]
        pub unsafe fn setFileCompletedCount(&self, fileCompletedCount: Option<&NSNumber>);
        #[method(publish)]
        pub unsafe fn publish(&self);
        #[method(unpublish)]
        pub unsafe fn unpublish(&self);
        #[method_id(addSubscriberForFileURL:withPublishingHandler:)]
        pub unsafe fn addSubscriberForFileURL_withPublishingHandler(
            url: &NSURL,
            publishingHandler: NSProgressPublishingHandler,
        ) -> Id<Object, Shared>;
        #[method(removeSubscriber:)]
        pub unsafe fn removeSubscriber(subscriber: &Object);
        #[method(isOld)]
        pub unsafe fn isOld(&self) -> bool;
    }
);
pub type NSProgressReporting = NSObject;
