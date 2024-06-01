use objc2::extern_methods;

use crate::{UIGestureRecognizer, UIGestureRecognizerState};

extern_methods!(
    unsafe impl UIGestureRecognizer {
        #[method(state)]
        pub fn state(&self) -> UIGestureRecognizerState;
    }
);
