#![cfg(target_os = "macos")]
// Test the behaviour of objects with invalid classes.

use std::mem::ManuallyDrop;

use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{class, msg_send};

#[link(name = "Cinematic", kind = "framework")]
extern "C" {}

#[test]
#[should_panic = "invalid object"]
fn when_copying_rendering_session_frame_attributes() {
    unsafe {
        // init is not a valid selector for CNRenderingSessionFrameAttributes.
        let obj: Retained<AnyObject> = msg_send![
            msg_send![class!(CNRenderingSessionFrameAttributes), alloc],
            init
        ];
        let obj_copy: Retained<AnyObject> = msg_send![&obj, copy];
        let obj_copy = ManuallyDrop::new(obj_copy); // Prevent crash
        let _ = obj_copy.class();
    }
}
