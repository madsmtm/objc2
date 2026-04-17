use std::ptr::null_mut;

use libc::{EINVAL, ENODEV};
use objc2::{
    define_class, msg_send,
    rc::{Allocated, Retained},
    AnyThread, Ivars,
};
use objc2_foundation::{NSError, NSObjectProtocol, NSPOSIXErrorDomain, NSProgress, NSUUID};
use objc2_fs_kit::{
    self as _, FSBlockDeviceResource, FSContainerIdentifier, FSContainerStatus, FSFileSystemBase,
    FSManageableResourceMaintenanceOperations, FSProbeResult, FSResource, FSTask, FSTaskOptions,
    FSUnaryFileSystem, FSUnaryFileSystemOperations, FSVolume,
};
use tracing::{error, info, trace};

use crate::{util::posix_err, volume};

define_class!(
    #[derive(Debug)]
    #[unsafe(super(FSUnaryFileSystem))]
    #[name = "CustomFileSystem"] // Must match `EXExtensionPrincipalClass`
    pub(crate) struct FileSystem;

    /// Dummy doc comment to get rustfmt to work.
    impl FileSystem {
        #[unsafe(method(init))]
        fn init(this: Allocated<Self>) -> Retained<Self> {
            // A new process, and thus a new `FileSystem`,
            // will be created (probably) every mount.
            trace!("init");
            let this = this.set_ivars(Ivars::<Self> {});
            unsafe { msg_send![super(this), init] }
        }
    }

    unsafe impl NSObjectProtocol for FileSystem {}

    unsafe impl FSUnaryFileSystemOperations for FileSystem {
        #[unsafe(method(probeResource:replyHandler:))]
        fn probe_resource_reply_handler(
            &self,
            resource: &FSResource,
            reply: &block2::DynBlock<dyn Fn(*mut FSProbeResult, *mut NSError)>,
        ) {
            let Some(resource) = resource.downcast_ref::<FSBlockDeviceResource>() else {
                info!(?resource, "resource was not FSBlockDeviceResource");
                reply.call(null_mut(), posix_err(EINVAL));
                return;
            };
            trace!(
                BSDName = ?unsafe { resource.BSDName() },
                isWritable = ?unsafe { resource.isWritable() },
                blockSize = ?unsafe { resource.blockSize() },
                blockCount = ?unsafe { resource.blockCount() },
                physicalBlockSize = ?unsafe { resource.physicalBlockSize() },
                "probeResource",
            );

            let name = volume::volume_name(resource);
            let random_uuid = NSUUID::UUID();
            let container_identifier = unsafe {
                FSContainerIdentifier::initWithUUID(FSContainerIdentifier::alloc(), &random_uuid)
            };
            let probe_result = unsafe {
                FSProbeResult::usableProbeResultWithName_containerID(&name, &container_identifier)
            };
            trace!(?probe_result, "probeResource response");
            reply.call(Retained::as_ptr(&probe_result).cast_mut(), null_mut());
        }

        #[unsafe(method(loadResource:options:replyHandler:))]
        fn load_resource_options_reply_handler(
            &self,
            resource: &FSResource,
            options: &FSTaskOptions,
            reply: &block2::DynBlock<dyn Fn(*mut FSVolume, *mut NSError)>,
        ) {
            let Some(resource) = resource.downcast_ref::<FSBlockDeviceResource>() else {
                info!("resource was not FSBlockDeviceResource");
                reply.call(null_mut(), posix_err(EINVAL));
                return;
            };
            trace!(
                BSDName = ?unsafe { resource.BSDName() },
                isWritable = ?unsafe { resource.isWritable() },
                blockSize = ?unsafe { resource.blockSize() },
                blockCount = ?unsafe { resource.blockCount() },
                physicalBlockSize = ?unsafe { resource.physicalBlockSize() },
                taskOptions = ?unsafe { options.taskOptions() },
                "loadResource",
            );

            let volume = volume::Volume::new(resource).into_super();

            unsafe { self.setContainerStatus(&FSContainerStatus::ready()) };

            trace!(
                volumeID = ?unsafe { volume.volumeID() },
                name = ?unsafe { volume.name() },
                "loadResource response",
            );
            reply.call(Retained::as_ptr(&volume).cast_mut(), null_mut());
        }

        #[unsafe(method(unloadResource:options:replyHandler:))]
        fn unload_resource_options_reply_handler(
            &self,
            resource: &FSResource,
            options: &FSTaskOptions,
            reply: &block2::DynBlock<dyn Fn(*mut NSError)>,
        ) {
            let Some(resource) = resource.downcast_ref::<FSBlockDeviceResource>() else {
                error!("resource was not FSBlockDeviceResource");
                reply.call(posix_err(EINVAL));
                return;
            };
            trace!(
                BSDName = ?unsafe { resource.BSDName() },
                isWritable = ?unsafe { resource.isWritable() },
                blockSize = ?unsafe { resource.blockSize() },
                blockCount = ?unsafe { resource.blockCount() },
                physicalBlockSize = ?unsafe { resource.physicalBlockSize() },
                taskOptions = ?unsafe { options.taskOptions() },
                "unloadResource",
            );

            unsafe {
                self.setContainerStatus(&FSContainerStatus::notReadyWithStatus(&NSError::new(
                    ENODEV as _,
                    NSPOSIXErrorDomain,
                )))
            };

            reply.call(null_mut());
        }

        #[unsafe(method(didFinishLoading))]
        fn did_finish_loading(&self) {
            trace!("didFinishLoading");
        }
    }

    // Implement `FSManageableResourceMaintenanceOperations` because we use
    // `FSBlockDeviceResource`.
    unsafe impl FSManageableResourceMaintenanceOperations for FileSystem {
        #[unsafe(method(startCheckWithTask:options:error:_))]
        fn start_check_with_task_options_error(
            &self,
            task: &FSTask,
            options: &FSTaskOptions,
        ) -> Result<Retained<NSProgress>, Retained<NSError>> {
            trace!(?task, ?options, "startCheckWithTask");
            // We do no work here, simply return an NSProgress that indicates we're done.
            let progress = NSProgress::progressWithTotalUnitCount(100);
            progress.setCompletedUnitCount(100);
            Ok(progress)
        }

        #[unsafe(method(startFormatWithTask:options:error:_))]
        fn start_format_with_task_options_error(
            &self,
            task: &FSTask,
            options: &FSTaskOptions,
        ) -> Result<Retained<NSProgress>, Retained<NSError>> {
            trace!(?task, ?options, "startFormatWithTask");
            // Same as above.
            let progress = NSProgress::progressWithTotalUnitCount(100);
            progress.setCompletedUnitCount(100);
            Ok(progress)
        }
    }
);
