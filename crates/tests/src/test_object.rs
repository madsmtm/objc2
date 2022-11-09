use core::mem::{size_of, ManuallyDrop};
use std::os::raw::c_int;

use objc2::foundation::NSObject;
use objc2::rc::{autoreleasepool, AutoreleasePool, Id, Owned};
use objc2::runtime::{Bool, Class, Object, Protocol};
#[cfg(feature = "malloc")]
use objc2::sel;
use objc2::{class, msg_send, msg_send_id};
use objc2::{ClassType, Encoding, Message, RefEncode};

#[repr(C)]
struct MyTestObject {
    inner: NSObject,
}

unsafe impl Message for MyTestObject {}

unsafe impl RefEncode for MyTestObject {
    const ENCODING_REF: Encoding = Encoding::Object;
}

unsafe impl ClassType for MyTestObject {
    type Super = NSObject;
    const NAME: &'static str = "MyTestObject";

    fn class() -> &'static Class {
        class!(MyTestObject)
    }

    fn as_super(&self) -> &Self::Super {
        &self.inner
    }

    fn as_super_mut(&mut self) -> &mut Self::Super {
        &mut self.inner
    }
}

impl MyTestObject {
    fn new() -> Id<Self, Owned> {
        let cls = Self::class();
        unsafe { msg_send_id![cls, new] }
    }

    #[allow(clippy::needless_lifetimes)]
    fn new_autoreleased<'p>(pool: &'p AutoreleasePool) -> &'p Self {
        let cls = Self::class();
        let ptr: *const Self = unsafe { msg_send![cls, getAutoreleasedInstance] };
        unsafe { pool.ptr_as_ref(ptr) }
    }

    fn new_autoreleased_retained() -> Id<Self, Owned> {
        let cls = Self::class();
        unsafe { msg_send_id![cls, getAutoreleasedInstance] }
    }

    fn add_numbers(a: c_int, b: c_int) -> c_int {
        let cls = Self::class();
        unsafe { msg_send![cls, add: a, and: b] }
    }

    fn var1(&self) -> c_int {
        unsafe { msg_send![self, var1] }
    }

    fn var1_ivar(&self) -> &c_int {
        unsafe { self.inner.ivar("var1") }
    }

    fn var1_ivar_mut(&mut self) -> &mut c_int {
        unsafe { self.inner.ivar_mut("var1") }
    }

    fn add_to_ivar1(&mut self, number: c_int) {
        unsafe { msg_send![self, addToVar1: number] }
    }

    fn var2(&self) -> bool {
        unsafe { msg_send![self, var2] }
    }

    fn var2_ivar(&self) -> &Bool {
        unsafe { self.inner.ivar("var2") }
    }

    fn var2_ivar_mut(&mut self) -> &mut Bool {
        unsafe { self.inner.ivar_mut("var2") }
    }

    fn var3(&self) -> *mut Object {
        unsafe { msg_send![self, var3] }
    }

    fn set_var3(&mut self, obj: *mut Object) {
        unsafe { msg_send![self, setVar3: obj] }
    }

    fn var3_ivar(&self) -> &*mut Object {
        unsafe { self.inner.ivar("var3") }
    }

    fn var3_ivar_mut(&mut self) -> &mut *mut Object {
        unsafe { self.inner.ivar_mut("var3") }
    }
}

#[cfg(feature = "malloc")]
macro_rules! assert_in {
    ($item:expr, $lst:expr) => {{
        let mut found = false;
        for &x in $lst.iter() {
            if x == $item {
                found = true;
            }
        }
        assert!(
            found,
            "Did not find {} in {}",
            stringify!($item),
            stringify!($lst),
        );
    }};
}

#[test]
fn test_class() {
    let cls = MyTestObject::class();
    assert_eq!(MyTestObject::add_numbers(-3, 15), 12);

    #[cfg(feature = "malloc")]
    {
        let classes = Class::classes();
        assert_eq!(classes.len(), Class::classes_count());
        assert_in!(cls, classes);
    }

    // Test objc2::runtime functionality
    assert_eq!(Class::get("MyTestObject"), Some(cls));
    assert_ne!(cls, class!(NSObject));
    assert_eq!(cls.name(), "MyTestObject");
    assert_eq!(cls.superclass(), Some(class!(NSObject)));
    assert_eq!(cls.metaclass().name(), "MyTestObject");
    assert_ne!(cls.metaclass(), cls);
    assert_eq!(cls.instance_size(), {
        #[repr(C)]
        struct MyTestObjectLayout {
            isa: *const Class,
            var1: c_int,
            var2: Bool,
            var3: *mut NSObject,
        }
        size_of::<MyTestObjectLayout>()
    });

    let protocol = Protocol::get("NSObject").unwrap();
    assert!(cls.conforms_to(protocol));
    assert!(!cls.conforms_to(Protocol::get("NSCopying").unwrap()));
    #[cfg(feature = "malloc")]
    assert_in!(protocol, cls.adopted_protocols());

    #[cfg(feature = "malloc")]
    {
        let method = cls.instance_method(sel!(addToVar1:)).unwrap();
        assert_in!(method, cls.instance_methods());

        let ivar = cls.instance_variable("var1").unwrap();
        assert_in!(ivar, cls.instance_variables());
    }
}

#[test]
fn test_object() {
    autoreleasepool(|pool| {
        let _obj = MyTestObject::new_autoreleased(pool);
    });
    let _obj = MyTestObject::new_autoreleased_retained();

    let mut obj = MyTestObject::new();
    assert_eq!((*obj.inner).class(), MyTestObject::class());

    assert_eq!(obj.var1(), 42);
    assert_eq!(*obj.var1_ivar(), 42);

    obj.add_to_ivar1(3);
    assert_eq!(obj.var1(), 45);
    assert_eq!(*obj.var1_ivar(), 45);

    *obj.var1_ivar_mut() = 100;
    assert_eq!(obj.var1(), 100);
    assert_eq!(*obj.var1_ivar(), 100);

    assert!(obj.var2());
    assert!(obj.var2_ivar().is_true());

    *obj.var2_ivar_mut() = Bool::NO;
    assert!(!obj.var2());
    assert!(obj.var2_ivar().is_false());

    assert!(obj.var3().is_null());
    assert!(obj.var3_ivar().is_null());

    let obj2 = Id::as_mut_ptr(&mut *ManuallyDrop::new(NSObject::new())).cast::<Object>();
    obj.set_var3(obj2);
    assert_eq!(obj.var3(), obj2);
    assert_eq!(*obj.var3_ivar(), obj2);

    let obj3 = Id::as_mut_ptr(&mut *ManuallyDrop::new(NSObject::new())).cast::<Object>();
    *obj.var3_ivar_mut() = obj3;
    assert_ne!(obj.var3(), obj2);
    assert_ne!(*obj.var3_ivar(), obj2);
    assert_eq!(obj.var3(), obj3);
    assert_eq!(*obj.var3_ivar(), obj3);
}
