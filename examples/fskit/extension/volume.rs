use std::{ptr::null_mut, sync::LazyLock};

use libc::{EIO, ENOENT};
use objc2::{define_class, msg_send, rc::Retained, AnyThread, Ivars};
use objc2_foundation::{
    ns_string, NSError, NSInteger, NSObject, NSObjectProtocol, NSString, NSUUID,
};
use objc2_fs_kit::{
    FSBlockDeviceResource, FSDeactivateOptions, FSDirectoryCookie, FSDirectoryEntryPacker,
    FSDirectoryVerifier, FSFileName, FSItem, FSItemAttributes, FSItemGetAttributesRequest,
    FSItemID, FSItemSetAttributesRequest, FSItemType, FSStatFSResult, FSSyncFlags, FSTaskOptions,
    FSVolume, FSVolumeIdentifier, FSVolumeOperations, FSVolumePathConfOperations,
    FSVolumeSupportedCapabilities,
};
use tracing::trace;

use crate::{item::Item, util::posix_err};

define_class!(
    #[derive(Debug)]
    #[unsafe(super(FSVolume, NSObject))]
    pub(crate) struct Volume {
        // Volume state
    }

    unsafe impl NSObjectProtocol for Volume {}

    #[allow(non_snake_case)]
    unsafe impl FSVolumePathConfOperations for Volume {
        // _PC_LINK_MAX
        #[unsafe(method(maximumLinkCount))]
        fn maximumLinkCount(&self) -> NSInteger {
            trace!("maximumLinkCount");
            -1
        }

        // _PC_NAME_MAX
        #[unsafe(method(maximumNameLength))]
        fn maximumNameLength(&self) -> NSInteger {
            trace!("maximumNameLength");
            -1
        }

        // _PC_CHOWN_RESTRICTED
        #[unsafe(method(restrictsOwnershipChanges))]
        fn restrictsOwnershipChanges(&self) -> bool {
            trace!("restrictsOwnershipChanges");
            false
        }

        // _PC_NO_TRUNC
        #[unsafe(method(truncatesLongNames))]
        fn truncatesLongNames(&self) -> bool {
            trace!("truncatesLongNames");
            false
        }

        // _PC_FILESIZEBITS
        #[unsafe(method(maximumXattrSizeInBits))]
        fn maximumXattrSizeInBits(&self) -> NSInteger {
            trace!("maximumXattrSizeInBits");
            NSInteger::MAX
        }

        // _PC_XATTR_SIZE_BITS
        #[unsafe(method(maximumFileSizeInBits))]
        fn maximumFileSizeInBits(&self) -> u64 {
            trace!("maximumFileSizeInBits");
            u64::MAX
        }
    }

    #[allow(non_snake_case)]
    unsafe impl FSVolumeOperations for Volume {
        #[unsafe(method(supportedVolumeCapabilities))]
        fn supportedVolumeCapabilities(&self) -> Retained<FSVolumeSupportedCapabilities> {
            trace!("supportedVolumeCapabilities");
            // TODO: Set capabilities.
            unsafe { FSVolumeSupportedCapabilities::new() }
        }

        #[unsafe(method(volumeStatistics))]
        fn volumeStatistics(&self) -> Retained<FSStatFSResult> {
            trace!("volumeStatistics");
            let stats = unsafe {
                FSStatFSResult::initWithFileSystemTypeName(
                    FSStatFSResult::alloc(),
                    ns_string!("fskitexample"), // FSShortName
                )
            };

            unsafe {
                stats.setBlockSize(1024000);
                stats.setIoSize(1024000);
                stats.setTotalBlocks(1024000);
                stats.setAvailableBlocks(1024000);
                stats.setFreeBlocks(1024000);
                stats.setTotalFiles(1024000);
                stats.setFreeFiles(1024000);
            }

            stats
        }

        #[unsafe(method(activateWithOptions:replyHandler:))]
        fn activateWithOptions_replyHandler(
            &self,
            options: &FSTaskOptions,
            reply: &block2::DynBlock<dyn Fn(*mut FSItem, *mut NSError)>,
        ) {
            trace!(taskOptions = ?unsafe { options.taskOptions() }, "activate");
            let item = Item::new(FSItemID::RootDirectory).into_super();
            reply.call(Retained::as_ptr(&item).cast_mut(), null_mut());
        }

        #[unsafe(method(deactivateWithOptions:replyHandler:))]
        fn deactivateWithOptions_replyHandler(
            &self,
            options: FSDeactivateOptions,
            reply: &block2::DynBlock<dyn Fn(*mut NSError)>,
        ) {
            trace!(?options, "deactivate");
            reply.call(null_mut());
        }

        #[unsafe(method(mountWithOptions:replyHandler:))]
        fn mountWithOptions_replyHandler(
            &self,
            options: &FSTaskOptions,
            reply: &block2::DynBlock<dyn Fn(*mut NSError)>,
        ) {
            trace!(taskOptions = ?unsafe { options.taskOptions() }, "mount");
            reply.call(null_mut());
        }

        #[unsafe(method(unmountWithReplyHandler:))]
        fn unmountWithReplyHandler(&self, reply: &block2::DynBlock<dyn Fn()>) {
            trace!("unmount");
            reply.call();
        }

        #[unsafe(method(synchronizeWithFlags:replyHandler:))]
        fn synchronizeWithFlags_replyHandler(
            &self,
            flags: FSSyncFlags,
            reply: &block2::DynBlock<dyn Fn(*mut NSError)>,
        ) {
            trace!(?flags, "synchronize");
            reply.call(null_mut());
        }

        #[unsafe(method(getAttributes:ofItem:replyHandler:))]
        fn getAttributes_ofItem_replyHandler(
            &self,
            desired_attributes: &FSItemGetAttributesRequest,
            item: &FSItem,
            reply: &block2::DynBlock<dyn Fn(*mut FSItemAttributes, *mut NSError)>,
        ) {
            let item = item.downcast_ref::<Item>().unwrap();
            trace!(wantedAttributes = ?unsafe { desired_attributes.wantedAttributes() }, ?item, "attributes");
            let attributes: *const FSItemAttributes = &**item.attributes();
            reply.call(attributes.cast_mut(), null_mut());
        }

        #[unsafe(method(setAttributes:onItem:replyHandler:))]
        fn setAttributes_onItem_replyHandler(
            &self,
            new_attributes: &FSItemSetAttributesRequest,
            item: &FSItem,
            reply: &block2::DynBlock<dyn Fn(*mut FSItemAttributes, *mut NSError)>,
        ) {
            let item = item.downcast_ref::<Item>().unwrap();
            trace!(?new_attributes, ?item, "setAttributes");
            item.set_attributes(new_attributes);
            let attributes: *const FSItemAttributes = &**item.attributes();
            reply.call(attributes.cast_mut(), null_mut());
        }

        #[unsafe(method(lookupItemNamed:inDirectory:replyHandler:))]
        fn lookupItemNamed_inDirectory_replyHandler(
            &self,
            name: &FSFileName,
            directory: &FSItem,
            reply: &block2::DynBlock<dyn Fn(*mut FSItem, *mut FSFileName, *mut NSError)>,
        ) {
            let directory = directory.downcast_ref::<Item>().unwrap();
            trace!(?name, ?directory, "lookupItem");
            reply.call(null_mut(), null_mut(), posix_err(ENOENT));
        }

        #[unsafe(method(reclaimItem:replyHandler:))]
        fn reclaimItem_replyHandler(
            &self,
            item: &FSItem,
            reply: &block2::DynBlock<dyn Fn(*mut NSError)>,
        ) {
            let item = item.downcast_ref::<Item>().unwrap();
            trace!(?item, "reclaimItem");
            reply.call(null_mut());
        }

        #[unsafe(method(readSymbolicLink:replyHandler:))]
        fn readSymbolicLink_replyHandler(
            &self,
            item: &FSItem,
            reply: &block2::DynBlock<dyn Fn(*mut FSFileName, *mut NSError)>,
        ) {
            let item = item.downcast_ref::<Item>().unwrap();
            trace!(?item, "readSymbolicLink");
            reply.call(null_mut(), posix_err(EIO));
        }

        #[unsafe(method(createItemNamed:type:inDirectory:attributes:replyHandler:))]
        fn createItemNamed_type_inDirectory_attributes_replyHandler(
            &self,
            name: &FSFileName,
            r#type: FSItemType,
            directory: &FSItem,
            new_attributes: &FSItemSetAttributesRequest,
            reply: &block2::DynBlock<dyn Fn(*mut FSItem, *mut FSFileName, *mut NSError)>,
        ) {
            let directory = directory.downcast_ref::<Item>().unwrap();
            trace!(?name, ?r#type, ?directory, ?new_attributes, "createItem");
            reply.call(null_mut(), null_mut(), posix_err(EIO));
        }

        #[unsafe(method(createSymbolicLinkNamed:inDirectory:attributes:linkContents:replyHandler:))]
        fn createSymbolicLinkNamed_inDirectory_attributes_linkContents_replyHandler(
            &self,
            name: &FSFileName,
            directory: &FSItem,
            new_attributes: &FSItemSetAttributesRequest,
            contents: &FSFileName,
            reply: &block2::DynBlock<dyn Fn(*mut FSItem, *mut FSFileName, *mut NSError)>,
        ) {
            let directory = directory.downcast_ref::<Item>().unwrap();
            trace!(
                ?name,
                ?directory,
                ?new_attributes,
                ?contents,
                "createSymbolicLink"
            );
            reply.call(null_mut(), null_mut(), posix_err(EIO));
        }

        #[unsafe(method(createLinkToItem:named:inDirectory:replyHandler:))]
        fn createLinkToItem_named_inDirectory_replyHandler(
            &self,
            item: &FSItem,
            name: &FSFileName,
            directory: &FSItem,
            reply: &block2::DynBlock<dyn Fn(*mut FSFileName, *mut NSError)>,
        ) {
            let item = item.downcast_ref::<Item>().unwrap();
            let directory = directory.downcast_ref::<Item>().unwrap();
            trace!(?item, ?name, ?directory, "createLink");
            reply.call(null_mut(), posix_err(EIO));
        }

        #[unsafe(method(removeItem:named:fromDirectory:replyHandler:))]
        fn removeItem_named_fromDirectory_replyHandler(
            &self,
            item: &FSItem,
            name: &FSFileName,
            directory: &FSItem,
            reply: &block2::DynBlock<dyn Fn(*mut NSError)>,
        ) {
            let item = item.downcast_ref::<Item>().unwrap();
            let directory = directory.downcast_ref::<Item>().unwrap();
            trace!(?item, ?name, ?directory, "removeItem");
            reply.call(posix_err(EIO));
        }

        #[allow(clippy::too_many_arguments)]
        #[unsafe(method(renameItem:inDirectory:named:toNewName:inDirectory:overItem:replyHandler:))]
        fn renameItem_inDirectory_named_toNewName_inDirectory_overItem_replyHandler(
            &self,
            item: &FSItem,
            source_directory: &FSItem,
            source_name: &FSFileName,
            destination_name: &FSFileName,
            destination_directory: &FSItem,
            over_item: Option<&FSItem>,
            reply: &block2::DynBlock<dyn Fn(*mut FSFileName, *mut NSError)>,
        ) {
            let item = item.downcast_ref::<Item>().unwrap();
            let source_directory = source_directory.downcast_ref::<Item>().unwrap();
            let destination_directory = destination_directory.downcast_ref::<Item>().unwrap();
            trace!(
                ?item,
                ?source_directory,
                ?source_name,
                ?destination_name,
                ?destination_directory,
                ?over_item,
                "renameItem",
            );
            reply.call(null_mut(), posix_err(EIO));
        }

        #[unsafe(method(enumerateDirectory:startingAtCookie:verifier:providingAttributes:usingPacker:replyHandler:))]
        fn enumerateDirectory_startingAtCookie_verifier_providingAttributes_usingPacker_replyHandler(
            &self,
            directory: &FSItem,
            cookie: FSDirectoryCookie,
            verifier: FSDirectoryVerifier,
            attributes: Option<&FSItemGetAttributesRequest>,
            packer: &FSDirectoryEntryPacker,
            reply: &block2::DynBlock<dyn Fn(FSDirectoryVerifier, *mut NSError)>,
        ) {
            let directory = directory.downcast_ref::<Item>().unwrap();
            trace!(
                ?directory,
                ?cookie,
                ?verifier,
                ?attributes,
                ?packer,
                "enumerateDirectory",
            );
            reply.call(0, null_mut());
        }
    }
);

static DEFAULT_VOLUME_UUID: LazyLock<Retained<NSUUID>> = LazyLock::new(NSUUID::UUID);

pub(crate) fn volume_name(resource: &FSBlockDeviceResource) -> Retained<NSString> {
    unsafe { resource.BSDName() }.stringByAppendingString(ns_string!("_test"))
}

impl Volume {
    pub(crate) fn new(resource: &FSBlockDeviceResource) -> Retained<Self> {
        let this = Self::alloc().set_ivars(Ivars::<Self> {
            // Set volume state in here.
        });
        let volume_id = unsafe {
            FSVolumeIdentifier::initWithUUID(FSVolumeIdentifier::alloc(), &DEFAULT_VOLUME_UUID)
        };
        let name = unsafe { FSFileName::nameWithString(&volume_name(resource)) };
        unsafe { msg_send![super(this), initWithVolumeID: &*volume_id, volumeName: &*name] }
    }
}
