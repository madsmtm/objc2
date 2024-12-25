use alloc::boxed::Box;
use core::ffi::c_void;
use core::time::Duration;

use super::ffi::{dispatch_time, dispatch_time_t, DISPATCH_TIME_NOW};

impl TryFrom<Duration> for dispatch_time_t {
    type Error = ();

    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        let secs = value.as_secs() as i64;

        secs.checked_mul(1_000_000_000)
            .and_then(|x| x.checked_add(i64::from(value.subsec_nanos())))
            .map(|delta| {
                // Safety: delta cannot overflow
                unsafe { dispatch_time(DISPATCH_TIME_NOW, delta) }
            })
            .ok_or(())
    }
}

pub(crate) extern "C" fn function_wrapper<F>(work_boxed: *mut c_void)
where
    F: FnOnce(),
{
    // Safety: we reconstruct from a Box.
    let work = unsafe { Box::from_raw(work_boxed.cast::<F>()) };

    (*work)();
}
