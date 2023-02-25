#![cfg(feature = "Foundation_NSProxy")]
use icrate::Foundation::NSProxy;
use objc2::ClassType;

#[test]
fn dummy() {
    let _cls = NSProxy::class();
}
