#![cfg(feature = "Foundation_NSException")]
use crate::common::*;
use crate::Foundation::NSException;

extern_methods!(
    unsafe impl NSException {
        #[method(raise)]
        pub(crate) unsafe fn raise_raw(&self);
    }
);
