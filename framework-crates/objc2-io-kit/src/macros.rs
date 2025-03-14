#![allow(unused_macros, unused_imports)]

macro_rules! IOUSBBit {
    ($bit:expr) => {
        1 << $bit
    };
}

macro_rules! IOUSBBitRange {
    ($start:expr, $end:expr) => {
        !((1 << $start) - 1) & ((1 << $end) | ((1 << $end) - 1))
    };
}

macro_rules! IOUSBBitRangePhase {
    ($start:expr, $end:expr) => {
        $start
    };
}

macro_rules! EncodeRequest {
    ($request:expr, $direction:expr, $type:expr, $recipient:expr) => {
        (($request << 8)
            + ($recipient + ($type << kUSBRqTypeShift) + ($direction << kUSBRqDirnShift)))
    };
}

pub(crate) use EncodeRequest;
pub(crate) use IOUSBBit;
pub(crate) use IOUSBBit as IOUSBHostFamilyBit;
pub(crate) use IOUSBBitRange;
pub(crate) use IOUSBBitRange as USBBitRange;
pub(crate) use IOUSBBitRange as IOUSBHostFamilyBitRange;
pub(crate) use IOUSBBitRangePhase;
pub(crate) use IOUSBBitRangePhase as USBBitRangePhase;
pub(crate) use IOUSBBitRangePhase as IOUSBHostFamilyBitRangePhase;
