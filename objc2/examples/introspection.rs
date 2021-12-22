use core::ptr::NonNull;

use objc2::rc::{Id, Owned};
use objc2::runtime::{Class, Object};
use objc2::{class, msg_send};
#[cfg(feature = "malloc")]
use objc2::{sel, Encode};

fn main() {
    // Get a class
    let cls = class!(NSObject);
    println!("NSObject size: {}", cls.instance_size());

    #[cfg(feature = "malloc")]
    {
        // Inspect its ivars
        println!("NSObject ivars:");
        for ivar in cls.instance_variables().iter() {
            println!("{}", ivar.name());
        }
    }

    // Allocate an instance
    let obj: Id<Object, Owned> = unsafe {
        let obj: *mut Object = msg_send![cls, alloc];
        let obj: NonNull<Object> = msg_send![obj, init];
        Id::new(obj)
    };
    println!("NSObject address: {:p}", obj);

    // Access an ivar of the object
    // TODO: Fix this!
    let isa: *const Class = unsafe { *obj.ivar("isa") };
    println!("NSObject isa: {:?}", isa);

    #[cfg(feature = "malloc")]
    {
        // Inspect a method of the class
        let hash_sel = sel!(hash);
        let hash_method = cls.instance_method(hash_sel).unwrap();
        let hash_return = hash_method.return_type();
        println!("-[NSObject hash] return type: {:?}", hash_return);
        assert!(usize::ENCODING.equivalent_to_str(&hash_return));
    }

    // Invoke a method on the object
    let hash: usize = unsafe { msg_send![obj, hash] };
    println!("NSObject hash: {}", hash);
}
