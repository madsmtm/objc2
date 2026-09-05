#![cfg(feature = "MTLDevice")]
use objc2_metal::MTLCreateSystemDefaultDevice;

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
#[cfg(target_os = "macos")]
#[cfg(target_arch = "aarch64")] // Workaround to not try to compile this with 10.12 SDK
#[allow(deprecated)]
fn get_all_with_observer() {
    use objc2::rc::autoreleasepool;
    use objc2_foundation::NSObjectProtocol;

    let mut observer = None;
    let _ = autoreleasepool(|_| {
        objc2_metal::MTLCopyAllDevicesWithObserver(
            &mut observer,
            &block2::RcBlock::new(|_device, _notification| {}),
        )
    });
    let observer = observer.unwrap();
    assert_eq!(observer.retainCount(), 2);
    autoreleasepool(|_| unsafe { objc2_metal::MTLRemoveDeviceObserver(&observer) });
    assert_eq!(observer.retainCount(), 1);
}
