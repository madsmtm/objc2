#![allow(clippy::missing_safety_doc)]
use core::ffi::c_int;
use core::mem::{size_of, ManuallyDrop};
use std::ffi::CString;

use objc2::rc::{autoreleasepool, AutoreleasePool, Retained};
use objc2::runtime::{
    AnyClass, AnyObject, AnyProtocol, Bool, NSObject, NSObjectProtocol, ProtocolObject,
};
use objc2::{class, extern_protocol, msg_send, msg_send_id, AllocAnyThread, ClassType};
use objc2::{extern_class, sel};
#[cfg(feature = "all")]
use objc2_foundation::{NSArray, NSException, NSMutableString, NSNumber, NSString};

// TODO: Remove once c"" strings are in MSRV
fn c(s: &str) -> CString {
    CString::new(s).unwrap()
}

extern_protocol!(
    #[name = "MyTestProtocol"]
    unsafe trait MyTestProtocol: NSObjectProtocol {
        #[method(a)]
        fn a(&self) -> c_int;

        #[method(b)]
        fn b() -> c_int;

        #[cfg(feature = "all")]
        #[method_id(c)]
        fn c(&self) -> Retained<NSNumber>;

        #[cfg(feature = "all")]
        #[method_id(d)]
        fn d() -> Retained<NSNumber>;

        #[method(e)]
        #[optional]
        fn e(&self) -> c_int;

        #[method(f)]
        #[optional]
        fn f() -> c_int;

        #[cfg(feature = "all")]
        #[optional]
        #[method_id(g)]
        fn g(&self) -> Retained<NSNumber>;

        #[cfg(feature = "all")]
        #[optional]
        #[method_id(h)]
        fn h() -> Retained<NSNumber>;
    }
);

extern_protocol!(
    #[name = "MyTestProtocol2"]
    unsafe trait MyTestProtocol2 {}
);

extern_protocol!(
    #[name = "MyTestProtocol3"]
    unsafe trait MyTestProtocol3: MyTestProtocol + NSObjectProtocol {}
);

extern_protocol!(
    #[name = "MyTestProtocol4"]
    unsafe trait MyTestProtocol4:
        MyTestProtocol + MyTestProtocol3 + MyTestProtocol2 + NSObjectProtocol
    {
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = AllocAnyThread]
    struct MyTestObject;
);

unsafe impl NSObjectProtocol for MyTestObject {}
unsafe impl MyTestProtocol for MyTestObject {}

#[cfg(all(target_vendor = "apple", target_arch = "aarch64"))]
#[used]
static FIX_LINKING: &AnyClass = {
    extern "C" {
        #[link_name = "OBJC_CLASS_$_MyTestObject"]
        static CLASS: AnyClass;
    }

    unsafe { &CLASS }
};

impl MyTestObject {
    fn new() -> Retained<Self> {
        let cls = Self::class();
        unsafe { msg_send_id![cls, new] }
    }

    #[allow(clippy::needless_lifetimes)]
    unsafe fn new_autoreleased<'p>(pool: AutoreleasePool<'p>) -> &'p Self {
        let cls = Self::class();
        let ptr: *const Self = unsafe { msg_send![cls, getAutoreleasedInstance] };
        unsafe { pool.ptr_as_ref(ptr) }
    }

    fn new_autoreleased_retained() -> Retained<Self> {
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
        let ivar = Self::class().instance_variable(&c("var1")).unwrap();
        unsafe { ivar.load(self) }
    }

    fn var1_ivar_ptr(&self) -> *mut c_int {
        let ivar = Self::class().instance_variable(&c("var1")).unwrap();
        unsafe { ivar.load_ptr(self) }
    }

    fn add_to_ivar1(&self, number: c_int) {
        unsafe { msg_send![self, addToVar1: number] }
    }

    fn var2(&self) -> bool {
        unsafe { msg_send![self, var2] }
    }

    fn var2_ivar(&self) -> &Bool {
        let ivar = Self::class().instance_variable(&c("var2")).unwrap();
        unsafe { ivar.load(self) }
    }

    fn var2_ivar_ptr(&self) -> *mut Bool {
        let ivar = Self::class().instance_variable(&c("var2")).unwrap();
        unsafe { ivar.load_ptr(self) }
    }

    fn var3(&self) -> *mut AnyObject {
        unsafe { msg_send![self, var3] }
    }

    fn set_var3(&self, obj: *mut AnyObject) {
        unsafe { msg_send![self, setVar3: obj] }
    }

    fn var3_ivar(&self) -> &*mut AnyObject {
        let ivar = Self::class().instance_variable(&c("var3")).unwrap();
        unsafe { ivar.load(self) }
    }

    fn var3_ivar_ptr(&self) -> *mut *mut AnyObject {
        let ivar = Self::class().instance_variable(&c("var3")).unwrap();
        unsafe { ivar.load_ptr(self) }
    }
}

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
macro_rules! assert_not_in {
    ($item:expr, $lst:expr) => {{
        let mut found = false;
        for &x in $lst.iter() {
            if x == $item {
                found = true;
            }
        }
        assert!(
            !found,
            "Found {} in {}",
            stringify!($item),
            stringify!($lst),
        );
    }};
}

#[test]
#[cfg_attr(
    all(target_vendor = "apple", not(target_arch = "aarch64")),
    ignore = "has trouble linking"
)]
fn test_class() {
    let cls = MyTestObject::class();
    assert_eq!(MyTestObject::add_numbers(-3, 15), 12);

    let classes = AnyClass::classes();
    assert_eq!(classes.len(), AnyClass::classes_count());
    assert_in!(cls, classes);

    // Test objc2::runtime functionality
    assert_eq!(AnyClass::get(&c("MyTestObject")), Some(cls));
    assert_ne!(cls, class!(NSObject));
    assert_eq!(cls.name(), &*c("MyTestObject"));
    assert_eq!(cls.superclass(), Some(class!(NSObject)));
    assert_eq!(cls.metaclass().name(), &*c("MyTestObject"));
    assert_ne!(cls.metaclass(), cls);
    assert_eq!(cls.instance_size(), {
        #[repr(C)]
        struct MyTestObjectLayout {
            isa: *const AnyClass,
            var1: c_int,
            var2: Bool,
            var3: *mut NSObject,
        }
        size_of::<MyTestObjectLayout>()
    });

    let protocol = AnyProtocol::get(&c("NSObject")).unwrap();
    assert!(cls.conforms_to(protocol));
    assert!(!cls.conforms_to(AnyProtocol::get(&c("NSCopying")).unwrap()));
    assert_not_in!(protocol, cls.adopted_protocols());
    assert_in!(
        AnyProtocol::get(&c("MyTestProtocol")).unwrap(),
        cls.adopted_protocols()
    );

    let method = cls.instance_method(sel!(addToVar1:)).unwrap();
    assert_in!(method, cls.instance_methods());

    let ivar = cls.instance_variable(&c("var1")).unwrap();
    assert_in!(ivar, cls.instance_variables());
}

#[test]
#[cfg_attr(
    all(target_vendor = "apple", not(target_arch = "aarch64")),
    ignore = "has trouble linking"
)]
fn test_object() {
    autoreleasepool(|pool| {
        let _obj = unsafe { MyTestObject::new_autoreleased(pool) };
    });
    let _obj = MyTestObject::new_autoreleased_retained();

    let obj = MyTestObject::new();
    assert_eq!((**obj).class(), MyTestObject::class());

    assert_eq!(obj.var1(), 42);
    assert_eq!(*obj.var1_ivar(), 42);

    obj.add_to_ivar1(3);
    assert_eq!(obj.var1(), 45);
    assert_eq!(*obj.var1_ivar(), 45);

    unsafe { *obj.var1_ivar_ptr() = 100 };
    assert_eq!(obj.var1(), 100);
    assert_eq!(*obj.var1_ivar(), 100);

    assert!(obj.var2());
    assert!(obj.var2_ivar().is_true());

    unsafe { *obj.var2_ivar_ptr() = Bool::NO };
    assert!(!obj.var2());
    assert!(obj.var2_ivar().is_false());

    assert!(obj.var3().is_null());
    assert!(obj.var3_ivar().is_null());

    let obj2 = Retained::as_ptr(&*ManuallyDrop::new(NSObject::new())) as _;
    obj.set_var3(obj2);
    assert_eq!(obj.var3(), obj2);
    assert_eq!(*obj.var3_ivar(), obj2);

    let obj3 = Retained::as_ptr(&*ManuallyDrop::new(NSObject::new())) as _;
    unsafe { *obj.var3_ivar_ptr() = obj3 };
    assert_ne!(obj.var3(), obj2);
    assert_ne!(*obj.var3_ivar(), obj2);
    assert_eq!(obj.var3(), obj3);
    assert_eq!(*obj.var3_ivar(), obj3);
}

#[test]
#[cfg_attr(
    all(target_vendor = "apple", not(target_arch = "aarch64")),
    ignore = "has trouble linking"
)]
fn test_protocol() {
    let obj = MyTestObject::new();
    let proto: Retained<ProtocolObject<dyn MyTestProtocol>> = ProtocolObject::from_id(obj);
    assert_eq!(proto.a(), 1);
    assert_eq!(MyTestObject::b(), 2);
    #[cfg(feature = "all")]
    assert_eq!(proto.c().as_i32(), 3);
    #[cfg(feature = "all")]
    assert_eq!(MyTestObject::d().as_i32(), 4);
    assert_eq!(proto.e(), 5);
    assert_eq!(MyTestObject::f(), 6);
    #[cfg(feature = "all")]
    assert_eq!(proto.g().as_i32(), 7);
    #[cfg(feature = "all")]
    assert_eq!(MyTestObject::h().as_i32(), 8);

    // Check that transforming to `NSObjectProtocol` works
    let _obj: &ProtocolObject<dyn NSObjectProtocol> = ProtocolObject::from_ref(&*proto);
}

#[test]
#[cfg(feature = "all")]
fn downcast_basics() {
    let obj = NSString::new();
    assert!(obj.downcast_ref::<NSString>().is_some());

    let obj = obj.into_super();
    assert!(obj.downcast_ref::<NSNumber>().is_none());
    assert!(obj.downcast_ref::<NSString>().is_some());

    let obj = NSMutableString::new();
    assert!(obj.downcast_ref::<NSMutableString>().is_some());
    assert!(obj.downcast_ref::<NSString>().is_some());
    assert!(obj.downcast_ref::<NSObject>().is_some());
    assert!(obj.downcast_ref::<NSException>().is_none());

    let obj = obj.into_super().into_super();
    assert!(obj.downcast_ref::<NSMutableString>().is_some());
    assert!(obj.downcast_ref::<NSString>().is_some());
    assert!(obj.downcast_ref::<NSObject>().is_some());
    assert!(obj.downcast_ref::<NSException>().is_none());

    let obj: Retained<NSArray<NSString>> = NSArray::new();
    assert!(obj.downcast_ref::<NSString>().is_none());
    assert!(obj.downcast_ref::<NSArray<AnyObject>>().is_some());
}

#[test]
#[cfg(feature = "all")]
fn test_downcast_class() {
    // Ensure that downcasting `AnyClass` doesn't cause unsoundness.
    let cls = NSString::class();
    let obj = unsafe { &*(cls as *const AnyClass).cast::<AnyObject>() };

    // AnyClass is an NSObject internally.
    assert!(obj.downcast_ref::<NSObject>().is_some());

    // But it is _not_ NSString, even though that's the class itself.
    assert!(obj.downcast_ref::<NSString>().is_none());
}
