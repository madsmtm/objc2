#![allow(missing_docs)] // TODO
use core::ffi::{c_int, c_long, c_ulong};

enum_with_val! {
    #[doc(alias = "dispatch_io_type_t")]
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct DispatchIOStreamType(pub c_ulong) {
        #[doc(alias = "DISPATCH_IO_STREAM")]
        Stream = 0,
        #[doc(alias = "DISPATCH_IO_RANDOM")]
        Random = 1,
    }
}

enum_with_val! {
    #[doc(alias = "dispatch_io_close_flags_t")]
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct DispatchIOCloseFlags(pub c_ulong) {
        #[doc(alias = "DISPATCH_IO_STOP")]
        Stop = 0x1,
    }
}

enum_with_val! {
    #[doc(alias = "dispatch_io_interval_flags_t")]
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct DispatchIOIntervalFlags(pub c_long) {
        #[doc(alias = "DISPATCH_IO_STRICT_INTERVAL")]
        StrictInterval = 0x1,
    }
}

pub(crate) type DispatchFd = c_int;
