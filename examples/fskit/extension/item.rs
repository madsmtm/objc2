use std::mem;

use libc::{timespec, S_IFDIR};
use objc2::{define_class, msg_send, rc::Retained, AnyThread, Encode, Encoding, Ivars};
use objc2_foundation::{NSObject, NSObjectProtocol};
use objc2_fs_kit::{FSItem, FSItemAttributes, FSItemID, FSItemSetAttributesRequest, FSItemType};

define_class!(
    #[derive(Debug)]
    #[unsafe(super(FSItem, NSObject))]
    pub(crate) struct Item {
        pub(crate) attributes: Retained<FSItemAttributes>,
    }

    unsafe impl NSObjectProtocol for Item {}
);

impl Item {
    pub(crate) fn new(id: FSItemID) -> Retained<Self> {
        let attributes = unsafe { FSItemAttributes::new() };

        unsafe { attributes.setFileID(id) };

        if id == FSItemID::RootDirectory {
            unsafe { attributes.setParentID(FSItemID::ParentOfRoot) };
            unsafe { attributes.setFileID(FSItemID::RootDirectory) };
            unsafe { attributes.setUid(0) };
            unsafe { attributes.setGid(0) };
            unsafe { attributes.setLinkCount(1) };
            unsafe { attributes.setType(FSItemType::Directory) };
            unsafe { attributes.setMode((S_IFDIR | 0b111_000_000) as u32) };
            unsafe { attributes.setAllocSize(1) };
            unsafe { attributes.setSize(1) };
            unsafe { attributes.setFlags(0) };
        } else {
            unsafe { attributes.setType(FSItemType::File) };

            unsafe { attributes.setSize(0) };
            unsafe { attributes.setAllocSize(0) };
            unsafe { attributes.setFlags(0) };
        }

        let timespec: timespec = unsafe { mem::zeroed() };
        // timespec_get(&timespec, TIME_UTC);

        #[repr(transparent)]
        #[derive(Clone, Copy)]
        struct Timespec(timespec);

        unsafe impl Encode for Timespec {
            const ENCODING: Encoding =
                Encoding::Struct("timespec", &[Encoding::C_LONG, Encoding::C_LONG]);
        }

        let timespec = Timespec(timespec);
        let _: () = unsafe { msg_send![&*attributes, setAddedTime: timespec] };
        let _: () = unsafe { msg_send![&*attributes, setBirthTime: timespec] };
        let _: () = unsafe { msg_send![&*attributes, setChangeTime: timespec] };
        let _: () = unsafe { msg_send![&*attributes, setModifyTime: timespec] };
        let _: () = unsafe { msg_send![&*attributes, setAccessTime: timespec] };

        let this = Self::alloc().set_ivars(Ivars::<Self> {
            attributes,
            // Set other item state.
        });
        unsafe { msg_send![super(this), init] }
    }

    pub(crate) fn set_attributes(&self, _new_attributes: &FSItemSetAttributesRequest) {}
}
