// IOKit/IOReturn.h
#![allow(non_upper_case_globals)]
use core::ffi::c_int;
use core::fmt;
use core::num::NonZeroI32;
#[cfg(feature = "objc2")]
use objc2::encode::{Encode, Encoding, OptionEncode, RefEncode};

use crate::{err_sub, err_system, iokit_common_err};

/// IOKit-specific result type.
///
/// This has the same memory layout and ABI as [`c_int`].
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/ioreturn?language=objc)
pub type IOReturn = Result<(), IOError>;

/// [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnsuccess?language=objc)
pub const kIOReturnSuccess: IOReturn = Ok(());

/// IOKit-specific errors.
///
/// This is used in [`IOReturn`] to make it work well with Rust's [`Result`].
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
// kern_return_t => int => i32
// TODO(MSRV): Use NonZero<c_int>.
pub struct IOError(pub NonZeroI32);

#[cfg(feature = "objc2")]
unsafe impl Encode for IOError {
    const ENCODING: Encoding = NonZeroI32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for IOError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(feature = "objc2")]
unsafe impl OptionEncode for IOError {}

impl IOError {
    #[inline]
    pub const fn from_raw(value: c_int) -> Option<Self> {
        if let Some(value) = NonZeroI32::new(value) {
            Some(Self(value))
        } else {
            None
        }
    }

    #[inline]
    pub const fn get(self) -> c_int {
        self.0.get()
    }

    /// General error.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnerror?language=objc)
    #[doc(alias = "kIOReturnError")]
    pub const Error: Self = Self(new(iokit_common_err!(0x2bc)));
    /// Can't allocate memory.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnnomemory?language=objc)
    #[doc(alias = "kIOReturnNoMemory")]
    pub const NoMemory: Self = Self(new(iokit_common_err!(0x2bd)));
    /// Resource shortage.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnnoresources?language=objc)
    #[doc(alias = "kIOReturnNoResources")]
    pub const NoResources: Self = Self(new(iokit_common_err!(0x2be)));
    /// Error during IPC.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnipcerror?language=objc)
    #[doc(alias = "kIOReturnIPCError")]
    pub const IPCError: Self = Self(new(iokit_common_err!(0x2bf)));
    /// No such device.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnnodevice?language=objc)
    #[doc(alias = "kIOReturnNoDevice")]
    pub const NoDevice: Self = Self(new(iokit_common_err!(0x2c0)));
    /// Privilege violation.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnnotprivileged?language=objc)
    #[doc(alias = "kIOReturnNotPrivileged")]
    pub const NotPrivileged: Self = Self(new(iokit_common_err!(0x2c1)));
    /// Invalid argument.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnbadargument?language=objc)
    #[doc(alias = "kIOReturnBadArgument")]
    pub const BadArgument: Self = Self(new(iokit_common_err!(0x2c2)));
    /// Device read locked.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnlockedread?language=objc)
    #[doc(alias = "kIOReturnLockedRead")]
    pub const LockedRead: Self = Self(new(iokit_common_err!(0x2c3)));
    /// Device write locked.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnlockedwrite?language=objc)
    #[doc(alias = "kIOReturnLockedWrite")]
    pub const LockedWrite: Self = Self(new(iokit_common_err!(0x2c4)));
    /// Exclusive access and device already open.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnexclusiveaccess?language=objc)
    #[doc(alias = "kIOReturnExclusiveAccess")]
    pub const ExclusiveAccess: Self = Self(new(iokit_common_err!(0x2c5)));
    /// Sent/received messages had different msg_id.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnbadmessageid?language=objc)
    #[doc(alias = "kIOReturnBadMessageID")]
    pub const BadMessageID: Self = Self(new(iokit_common_err!(0x2c6)));
    /// Unsupported function.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnunsupported?language=objc)
    #[doc(alias = "kIOReturnUnsupported")]
    pub const Unsupported: Self = Self(new(iokit_common_err!(0x2c7)));
    /// Misc. VM failure.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnvmerror?language=objc)
    #[doc(alias = "kIOReturnVMError")]
    pub const VMError: Self = Self(new(iokit_common_err!(0x2c8)));
    /// Internal error.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturninternalerror?language=objc)
    #[doc(alias = "kIOReturnInternalError")]
    pub const InternalError: Self = Self(new(iokit_common_err!(0x2c9)));
    /// General I/O error.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnioerror?language=objc)
    #[doc(alias = "kIOReturnIOError")]
    pub const IOError: Self = Self(new(iokit_common_err!(0x2ca)));
    // iokit_common_err(0x2cb)
    /// Can't acquire lock.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturncannotlock?language=objc)
    #[doc(alias = "kIOReturnCannotLock")]
    pub const CannotLock: Self = Self(new(iokit_common_err!(0x2cc)));
    /// Device not open.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnnotopen?language=objc)
    #[doc(alias = "kIOReturnNotOpen")]
    pub const NotOpen: Self = Self(new(iokit_common_err!(0x2cd)));
    /// Read not supported.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnnotreadable?language=objc)
    #[doc(alias = "kIOReturnNotReadable")]
    pub const NotReadable: Self = Self(new(iokit_common_err!(0x2ce)));
    /// Write not supported.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnnotwritable?language=objc)
    #[doc(alias = "kIOReturnNotWritable")]
    pub const NotWritable: Self = Self(new(iokit_common_err!(0x2cf)));
    /// Alignment error.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnnotaligned?language=objc)
    #[doc(alias = "kIOReturnNotAligned")]
    pub const NotAligned: Self = Self(new(iokit_common_err!(0x2d0)));
    /// Media Error.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnbadmedia?language=objc)
    #[doc(alias = "kIOReturnBadMedia")]
    pub const BadMedia: Self = Self(new(iokit_common_err!(0x2d1)));
    /// Device(s) still open.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnstillopen?language=objc)
    #[doc(alias = "kIOReturnStillOpen")]
    pub const StillOpen: Self = Self(new(iokit_common_err!(0x2d2)));
    /// Rld failure.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnrlderror?language=objc)
    #[doc(alias = "kIOReturnRLDError")]
    pub const RLDError: Self = Self(new(iokit_common_err!(0x2d3)));
    /// DMA failure.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturndmaerror?language=objc)
    #[doc(alias = "kIOReturnDMAError")]
    pub const DMAError: Self = Self(new(iokit_common_err!(0x2d4)));
    /// Device Busy.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnbusy?language=objc)
    #[doc(alias = "kIOReturnBusy")]
    pub const Busy: Self = Self(new(iokit_common_err!(0x2d5)));
    /// I/O Timeout.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturntimeout?language=objc)
    #[doc(alias = "kIOReturnTimeout")]
    pub const Timeout: Self = Self(new(iokit_common_err!(0x2d6)));
    /// Device offline.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnoffline?language=objc)
    #[doc(alias = "kIOReturnOffline")]
    pub const Offline: Self = Self(new(iokit_common_err!(0x2d7)));
    /// Not ready.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnnotready?language=objc)
    #[doc(alias = "kIOReturnNotReady")]
    pub const NotReady: Self = Self(new(iokit_common_err!(0x2d8)));
    /// Device not attached.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnnotattached?language=objc)
    #[doc(alias = "kIOReturnNotAttached")]
    pub const NotAttached: Self = Self(new(iokit_common_err!(0x2d9)));
    /// No DMA channels left.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnnochannels?language=objc)
    #[doc(alias = "kIOReturnNoChannels")]
    pub const NoChannels: Self = Self(new(iokit_common_err!(0x2da)));
    /// No space for data.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnnospace?language=objc)
    #[doc(alias = "kIOReturnNoSpace")]
    pub const NoSpace: Self = Self(new(iokit_common_err!(0x2db)));
    // iokit_common_err(0x2dc)
    /// Port already exists.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnportexists?language=objc)
    #[doc(alias = "kIOReturnPortExists")]
    pub const PortExists: Self = Self(new(iokit_common_err!(0x2dd)));
    /// Can't wire down physical memory.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturncannotwire?language=objc)
    #[doc(alias = "kIOReturnCannotWire")]
    pub const CannotWire: Self = Self(new(iokit_common_err!(0x2de)));
    /// No interrupt attached.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnnointerrupt?language=objc)
    #[doc(alias = "kIOReturnNoInterrupt")]
    pub const NoInterrupt: Self = Self(new(iokit_common_err!(0x2df)));
    /// No DMA frames enqueued.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnnoframes?language=objc)
    #[doc(alias = "kIOReturnNoFrames")]
    pub const NoFrames: Self = Self(new(iokit_common_err!(0x2e0)));
    /// Oversized msg received on interrupt port.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnmessagetoolarge?language=objc)
    #[doc(alias = "kIOReturnMessageTooLarge")]
    pub const MessageTooLarge: Self = Self(new(iokit_common_err!(0x2e1)));
    /// Not permitted.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnnotpermitted?language=objc)
    #[doc(alias = "kIOReturnNotPermitted")]
    pub const NotPermitted: Self = Self(new(iokit_common_err!(0x2e2)));
    /// No power to device.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnnopower?language=objc)
    #[doc(alias = "kIOReturnNoPower")]
    pub const NoPower: Self = Self(new(iokit_common_err!(0x2e3)));
    /// Media not present.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnnomedia?language=objc)
    #[doc(alias = "kIOReturnNoMedia")]
    pub const NoMedia: Self = Self(new(iokit_common_err!(0x2e4)));
    /// Media not formatted.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnunformattedmedia?language=objc)
    #[doc(alias = "kIOReturnUnformattedMedia")]
    pub const UnformattedMedia: Self = Self(new(iokit_common_err!(0x2e5)));
    /// No such mode.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnunsupportedmode?language=objc)
    #[doc(alias = "kIOReturnUnsupportedMode")]
    pub const UnsupportedMode: Self = Self(new(iokit_common_err!(0x2e6)));
    /// Data underrun.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnunderrun?language=objc)
    #[doc(alias = "kIOReturnUnderrun")]
    pub const Underrun: Self = Self(new(iokit_common_err!(0x2e7)));
    /// Data overrun.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnoverrun?language=objc)
    #[doc(alias = "kIOReturnOverrun")]
    pub const Overrun: Self = Self(new(iokit_common_err!(0x2e8)));
    /// The device is not working properly!.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturndeviceerror?language=objc)
    #[doc(alias = "kIOReturnDeviceError")]
    pub const DeviceError: Self = Self(new(iokit_common_err!(0x2e9)));
    /// A completion routine is required.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnnocompletion?language=objc)
    #[doc(alias = "kIOReturnNoCompletion")]
    pub const NoCompletion: Self = Self(new(iokit_common_err!(0x2ea)));
    /// Operation aborted.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnaborted?language=objc)
    #[doc(alias = "kIOReturnAborted")]
    pub const Aborted: Self = Self(new(iokit_common_err!(0x2eb)));
    /// Bus bandwidth would be exceeded.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnnobandwidth?language=objc)
    #[doc(alias = "kIOReturnNoBandwidth")]
    pub const NoBandwidth: Self = Self(new(iokit_common_err!(0x2ec)));
    /// Device not responding.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnnotresponding?language=objc)
    #[doc(alias = "kIOReturnNotResponding")]
    pub const NotResponding: Self = Self(new(iokit_common_err!(0x2ed)));
    /// Isochronous I/O request for distant past!.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnisotooold?language=objc)
    #[doc(alias = "kIOReturnIsoTooOld")]
    pub const IsoTooOld: Self = Self(new(iokit_common_err!(0x2ee)));
    /// Isochronous I/O request for distant future.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnisotoonew?language=objc)
    #[doc(alias = "kIOReturnIsoTooNew")]
    pub const IsoTooNew: Self = Self(new(iokit_common_err!(0x2ef)));
    /// Data was not found.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturnnotfound?language=objc)
    #[doc(alias = "kIOReturnNotFound")]
    pub const NotFound: Self = Self(new(iokit_common_err!(0x2f0)));
    /// Should never be seen.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kioreturninvalid?language=objc)
    #[doc(alias = "kIOReturnInvalid")]
    pub const Invalid: Self = Self(new(iokit_common_err!(0x1)));

    const fn as_str(self) -> Option<&'static str> {
        Some(match self {
            Self::Error => "general error",
            Self::NoMemory => "can't allocate memory",
            Self::NoResources => "resource shortage",
            Self::IPCError => "error during IPC",
            Self::NoDevice => "no such device",
            Self::NotPrivileged => "privilege violation",
            Self::BadArgument => "invalid argument",
            Self::LockedRead => "device read locked",
            Self::LockedWrite => "device write locked",
            Self::ExclusiveAccess => "exclusive access and device already open",
            Self::BadMessageID => "sent/received messages had different msg_id",
            Self::Unsupported => "unsupported function",
            Self::VMError => "misc. VM failure",
            Self::InternalError => "internal error",
            Self::IOError => "General I/O error",
            Self::CannotLock => "can't acquire lock",
            Self::NotOpen => "device not open",
            Self::NotReadable => "read not supported",
            Self::NotWritable => "write not supported",
            Self::NotAligned => "alignment error",
            Self::BadMedia => "Media Error",
            Self::StillOpen => "device(s) still open",
            Self::RLDError => "rld failure",
            Self::DMAError => "DMA failure",
            Self::Busy => "Device Busy",
            Self::Timeout => "I/O Timeout",
            Self::Offline => "device offline",
            Self::NotReady => "not ready",
            Self::NotAttached => "device not attached",
            Self::NoChannels => "no DMA channels left",
            Self::NoSpace => "no space for data",
            Self::PortExists => "port already exists",
            Self::CannotWire => "can't wire down physical memory",
            Self::NoInterrupt => "no interrupt attached",
            Self::NoFrames => "no DMA frames enqueued",
            Self::MessageTooLarge => "oversized msg received on interrupt port",
            Self::NotPermitted => "not permitted",
            Self::NoPower => "no power to device",
            Self::NoMedia => "media not present",
            Self::UnformattedMedia => "media not formatted",
            Self::UnsupportedMode => "no such mode",
            Self::Underrun => "data underrun",
            Self::Overrun => "data overrun",
            Self::DeviceError => "the device is not working properly!",
            Self::NoCompletion => "a completion routine is required",
            Self::Aborted => "operation aborted",
            Self::NoBandwidth => "bus bandwidth would be exceeded",
            Self::NotResponding => "device not responding",
            Self::IsoTooOld => "isochronous I/O request for distant past!",
            Self::IsoTooNew => "isochronous I/O request for distant future",
            Self::NotFound => "data was not found",
            Self::Invalid => "should never be seen",
            _ => return None,
        })
    }
}

/// Helper to construct `NonZeroI32` in `const`.
///
/// TODO(MSRV): Replace with inline `NonZeroI32::new(x as _).unwrap()`.
const fn new(x: u32) -> NonZeroI32 {
    if let Some(x) = NonZeroI32::new(x as _) {
        x
    } else {
        panic!("was given a constant of 0")
    }
}

impl fmt::Debug for IOError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Usually, for normal enums, we'd prefer to derive `Debug`, but for
        // `IOError`, that's a bit too opaque, it just prints the error code.
        //
        // So instead, we enrich this with the error string. This makes
        // `.unwrap()` and similar work much better.
        if let Some(s) = self.as_str() {
            write!(f, "{s} ({})", self.0)
        } else {
            write!(f, "unknown IOReturn ({})", self.0)
        }
    }
}

impl fmt::Display for IOError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(s) = self.as_str() {
            write!(f, "{s}")
        } else {
            write!(f, "unknown IOReturn ({})", self.0)
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for IOError {}
