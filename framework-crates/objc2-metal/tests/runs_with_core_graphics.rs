#![cfg(feature = "MTLDevice")]
use block2::RcBlock;
use objc2::rc::autoreleasepool;
use objc2_foundation::NSObjectProtocol;
use objc2_metal::{MTLCreateSystemDefaultDevice, MTLRemoveDeviceObserver};

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}

#[test]
#[ignore = "doesn't work in CI"]
fn test_create_default() {
    let _ = MTLCreateSystemDefaultDevice();
}

#[test]
fn get_all() {
    let _ = objc2_metal::MTLCopyAllDevices();
}

#[test]
fn get_all_with_observer() {
    let mut observer = None;
    let _ = autoreleasepool(|_| unsafe {
        objc2_metal::MTLCopyAllDevicesWithObserver(
            &mut observer,
            &RcBlock::new(|_device, _notification| {}),
        )
    });
    let observer = observer.unwrap();
    assert_eq!(observer.retainCount(), 2);
    autoreleasepool(|_| unsafe { MTLRemoveDeviceObserver(&observer) });
    assert_eq!(observer.retainCount(), 1);
}
