use objc2::runtime::NSObject;

fn main() {
    #[allow(unused_mut)]
    let mut obj = NSObject::new();
    let _: &mut NSObject = &mut *obj;
}
