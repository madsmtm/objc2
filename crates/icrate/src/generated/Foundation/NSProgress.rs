//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub type NSProgressKind = NSString;

pub type NSProgressUserInfoKey = NSString;

pub type NSProgressFileOperationKind = NSString;

pub type NSProgressUnpublishingHandler = *mut Block<(), ()>;

pub type NSProgressPublishingHandler =
    *mut Block<(NonNull<NSProgress>,), NSProgressUnpublishingHandler>;

extern_class!(
    #[derive(Debug)]
    pub struct NSProgress;

    unsafe impl ClassType for NSProgress {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSProgress {
        #[method_id(@__retain_semantics Other currentProgress)]
        pub unsafe fn currentProgress() -> Option<Id<NSProgress, Shared>>;

        #[method_id(@__retain_semantics Other progressWithTotalUnitCount:)]
        pub unsafe fn progressWithTotalUnitCount(unitCount: i64) -> Id<NSProgress, Shared>;

        #[method_id(@__retain_semantics Other discreteProgressWithTotalUnitCount:)]
        pub unsafe fn discreteProgressWithTotalUnitCount(unitCount: i64) -> Id<NSProgress, Shared>;

        #[method_id(@__retain_semantics Other progressWithTotalUnitCount:parent:pendingUnitCount:)]
        pub unsafe fn progressWithTotalUnitCount_parent_pendingUnitCount(
            unitCount: i64,
            parent: &NSProgress,
            portionOfParentTotalUnitCount: i64,
        ) -> Id<NSProgress, Shared>;

        #[method_id(@__retain_semantics Init initWithParent:userInfo:)]
        pub unsafe fn initWithParent_userInfo(
            this: Option<Allocated<Self>>,
            parentProgressOrNil: Option<&NSProgress>,
            userInfoOrNil: Option<&NSDictionary<NSProgressUserInfoKey, Object>>,
        ) -> Id<Self, Shared>;

        #[method(becomeCurrentWithPendingUnitCount:)]
        pub unsafe fn becomeCurrentWithPendingUnitCount(&self, unitCount: i64);

        #[method(performAsCurrentWithPendingUnitCount:usingBlock:)]
        pub unsafe fn performAsCurrentWithPendingUnitCount_usingBlock(
            &self,
            unitCount: i64,
            work: &Block<(), ()>,
        );

        #[method(resignCurrent)]
        pub unsafe fn resignCurrent(&self);

        #[method(addChild:withPendingUnitCount:)]
        pub unsafe fn addChild_withPendingUnitCount(&self, child: &NSProgress, inUnitCount: i64);

        #[method(totalUnitCount)]
        pub unsafe fn totalUnitCount(&self) -> i64;

        #[method(setTotalUnitCount:)]
        pub unsafe fn setTotalUnitCount(&self, totalUnitCount: i64);

        #[method(completedUnitCount)]
        pub unsafe fn completedUnitCount(&self) -> i64;

        #[method(setCompletedUnitCount:)]
        pub unsafe fn setCompletedUnitCount(&self, completedUnitCount: i64);

        #[method_id(@__retain_semantics Other localizedDescription)]
        pub unsafe fn localizedDescription(&self) -> Id<NSString, Shared>;

        #[method(setLocalizedDescription:)]
        pub unsafe fn setLocalizedDescription(&self, localizedDescription: Option<&NSString>);

        #[method_id(@__retain_semantics Other localizedAdditionalDescription)]
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
        pub unsafe fn cancellationHandler(&self) -> *mut Block<(), ()>;

        #[method(setCancellationHandler:)]
        pub unsafe fn setCancellationHandler(&self, cancellationHandler: Option<&Block<(), ()>>);

        #[method(pausingHandler)]
        pub unsafe fn pausingHandler(&self) -> *mut Block<(), ()>;

        #[method(setPausingHandler:)]
        pub unsafe fn setPausingHandler(&self, pausingHandler: Option<&Block<(), ()>>);

        #[method(resumingHandler)]
        pub unsafe fn resumingHandler(&self) -> *mut Block<(), ()>;

        #[method(setResumingHandler:)]
        pub unsafe fn setResumingHandler(&self, resumingHandler: Option<&Block<(), ()>>);

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

        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Id<NSDictionary<NSProgressUserInfoKey, Object>, Shared>;

        #[method_id(@__retain_semantics Other kind)]
        pub unsafe fn kind(&self) -> Option<Id<NSProgressKind, Shared>>;

        #[method(setKind:)]
        pub unsafe fn setKind(&self, kind: Option<&NSProgressKind>);

        #[method_id(@__retain_semantics Other estimatedTimeRemaining)]
        pub unsafe fn estimatedTimeRemaining(&self) -> Option<Id<NSNumber, Shared>>;

        #[method(setEstimatedTimeRemaining:)]
        pub unsafe fn setEstimatedTimeRemaining(&self, estimatedTimeRemaining: Option<&NSNumber>);

        #[method_id(@__retain_semantics Other throughput)]
        pub unsafe fn throughput(&self) -> Option<Id<NSNumber, Shared>>;

        #[method(setThroughput:)]
        pub unsafe fn setThroughput(&self, throughput: Option<&NSNumber>);

        #[method_id(@__retain_semantics Other fileOperationKind)]
        pub unsafe fn fileOperationKind(&self) -> Option<Id<NSProgressFileOperationKind, Shared>>;

        #[method(setFileOperationKind:)]
        pub unsafe fn setFileOperationKind(
            &self,
            fileOperationKind: Option<&NSProgressFileOperationKind>,
        );

        #[method_id(@__retain_semantics Other fileURL)]
        pub unsafe fn fileURL(&self) -> Option<Id<NSURL, Shared>>;

        #[method(setFileURL:)]
        pub unsafe fn setFileURL(&self, fileURL: Option<&NSURL>);

        #[method_id(@__retain_semantics Other fileTotalCount)]
        pub unsafe fn fileTotalCount(&self) -> Option<Id<NSNumber, Shared>>;

        #[method(setFileTotalCount:)]
        pub unsafe fn setFileTotalCount(&self, fileTotalCount: Option<&NSNumber>);

        #[method_id(@__retain_semantics Other fileCompletedCount)]
        pub unsafe fn fileCompletedCount(&self) -> Option<Id<NSNumber, Shared>>;

        #[method(setFileCompletedCount:)]
        pub unsafe fn setFileCompletedCount(&self, fileCompletedCount: Option<&NSNumber>);

        #[method(publish)]
        pub unsafe fn publish(&self);

        #[method(unpublish)]
        pub unsafe fn unpublish(&self);

        #[method_id(@__retain_semantics Other addSubscriberForFileURL:withPublishingHandler:)]
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

extern_protocol!(
    pub struct NSProgressReporting;

    unsafe impl NSProgressReporting {
        #[method_id(@__retain_semantics Other progress)]
        pub unsafe fn progress(&self) -> Id<NSProgress, Shared>;
    }
);

extern_static!(NSProgressEstimatedTimeRemainingKey: &'static NSProgressUserInfoKey);

extern_static!(NSProgressThroughputKey: &'static NSProgressUserInfoKey);

extern_static!(NSProgressKindFile: &'static NSProgressKind);

extern_static!(NSProgressFileOperationKindKey: &'static NSProgressUserInfoKey);

extern_static!(NSProgressFileOperationKindDownloading: &'static NSProgressFileOperationKind);

extern_static!(
    NSProgressFileOperationKindDecompressingAfterDownloading: &'static NSProgressFileOperationKind
);

extern_static!(NSProgressFileOperationKindReceiving: &'static NSProgressFileOperationKind);

extern_static!(NSProgressFileOperationKindCopying: &'static NSProgressFileOperationKind);

extern_static!(NSProgressFileOperationKindUploading: &'static NSProgressFileOperationKind);

extern_static!(NSProgressFileOperationKindDuplicating: &'static NSProgressFileOperationKind);

extern_static!(NSProgressFileURLKey: &'static NSProgressUserInfoKey);

extern_static!(NSProgressFileTotalCountKey: &'static NSProgressUserInfoKey);

extern_static!(NSProgressFileCompletedCountKey: &'static NSProgressUserInfoKey);

extern_static!(NSProgressFileAnimationImageKey: &'static NSProgressUserInfoKey);

extern_static!(NSProgressFileAnimationImageOriginalRectKey: &'static NSProgressUserInfoKey);

extern_static!(NSProgressFileIconKey: &'static NSProgressUserInfoKey);
