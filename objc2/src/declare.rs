/*!
Functionality for declaring Objective-C classes.

Classes can be declared using the [`ClassDecl`] struct. Instance variables and
methods can then be added before the class is ultimately registered.

# Example

The following example demonstrates declaring a class named `MyNumber` that has
one ivar, a `u32` named `_number` and a `number` method that returns it:

``` no_run
# use objc2::{class, sel};
# use objc2::declare::ClassDecl;
# use objc2::runtime::{Class, Object, Sel};
let superclass = class!(NSObject);
let mut decl = ClassDecl::new("MyNumber", superclass).unwrap();

// Add an instance variable
decl.add_ivar::<u32>("_number");

// Add an ObjC method for getting the number
extern fn my_number_get(this: &Object, _cmd: Sel) -> u32 {
    unsafe { *this.get_ivar("_number") }
}
unsafe {
    decl.add_method(sel!(number),
        my_number_get as extern fn(&Object, Sel) -> u32);
}

decl.register();
```
*/

use alloc::format;
use alloc::string::ToString;
use core::mem;
use core::mem::ManuallyDrop;
use std::ffi::CString;

use crate::runtime::{Bool, Class, Imp, Object, Protocol, Sel};
use crate::{ffi, Encode, EncodeArguments, Encoding, Message};

/// Types that can be used as the implementation of an Objective-C method.
pub trait MethodImplementation {
    /// The callee type of the method.
    type Callee: Message + ?Sized;
    /// The return type of the method.
    type Ret: Encode;
    /// The argument types of the method.
    type Args: EncodeArguments;

    /// Returns self as an [`Imp`] of a method.
    fn imp(self) -> Imp;
}

macro_rules! method_decl_impl {
    (-$s:ident, $r:ident, $f:ty, $($t:ident),*) => (
        impl<$s, $r, $($t),*> MethodImplementation for $f
        where
            $s: Message + ?Sized,
            $r: Encode,
            $($t: Encode,)*
        {
            type Callee = $s;
            type Ret = $r;
            type Args = ($($t,)*);

            fn imp(self) -> Imp {
                unsafe { mem::transmute(self) }
            }
        }
    );
    ($($t:ident),*) => (
        method_decl_impl!(-T, R, extern "C" fn(&T, Sel $(, $t)*) -> R, $($t),*);
        method_decl_impl!(-T, R, extern "C" fn(&mut T, Sel $(, $t)*) -> R, $($t),*);
    );
}

method_decl_impl!();
method_decl_impl!(A);
method_decl_impl!(A, B);
method_decl_impl!(A, B, C);
method_decl_impl!(A, B, C, D);
method_decl_impl!(A, B, C, D, E);
method_decl_impl!(A, B, C, D, E, F);
method_decl_impl!(A, B, C, D, E, F, G);
method_decl_impl!(A, B, C, D, E, F, G, H);
method_decl_impl!(A, B, C, D, E, F, G, H, I);
method_decl_impl!(A, B, C, D, E, F, G, H, I, J);
method_decl_impl!(A, B, C, D, E, F, G, H, I, J, K);
method_decl_impl!(A, B, C, D, E, F, G, H, I, J, K, L);

fn count_args(sel: Sel) -> usize {
    sel.name().chars().filter(|&c| c == ':').count()
}

fn method_type_encoding(ret: &Encoding<'_>, args: &[Encoding<'_>]) -> CString {
    // First two arguments are always self and the selector
    let mut types = format!("{}{}{}", ret, <*mut Object>::ENCODING, Sel::ENCODING);
    for enc in args {
        use core::fmt::Write;
        write!(&mut types, "{}", enc).unwrap();
    }
    CString::new(types).unwrap()
}

fn log2_align_of<T>() -> u8 {
    let align = mem::align_of::<T>();
    // Alignments are required to be powers of 2
    debug_assert!(align.count_ones() == 1);
    // log2 of a power of 2 is the number of trailing zeros
    align.trailing_zeros() as u8
}

/// A type for declaring a new class and adding new methods and ivars to it
/// before registering it.
pub struct ClassDecl {
    cls: *mut Class,
}

// SAFETY: The stuff that touch global state does so using locks internally.
//
// Modifying the class itself can only be done through `&mut`, so Sync is
// safe (e.g. we can't accidentally call `add_ivar` at the same time from two
// different threads).
//
// (Though actually, that would be safe since the entire runtime is locked
// when doing so...).
//
// Finally, there are no requirements that the class must be registered on the
// same thread that allocated it.
unsafe impl Send for ClassDecl {}
unsafe impl Sync for ClassDecl {}

impl ClassDecl {
    fn with_superclass(name: &str, superclass: Option<&Class>) -> Option<ClassDecl> {
        let name = CString::new(name).unwrap();
        let super_ptr = superclass.map_or(0 as *const _, |c| c) as _;
        let cls = unsafe { ffi::objc_allocateClassPair(super_ptr, name.as_ptr(), 0) };
        if cls.is_null() {
            None
        } else {
            Some(ClassDecl { cls: cls as _ })
        }
    }

    /// Constructs a [`ClassDecl`] with the given name and superclass.
    ///
    /// Returns [`None`] if the class couldn't be allocated, or a class with
    /// that name already exist.
    pub fn new(name: &str, superclass: &Class) -> Option<ClassDecl> {
        ClassDecl::with_superclass(name, Some(superclass))
    }

    /// Constructs a [`ClassDecl`] declaring a new root class with the given
    /// name.
    ///
    /// Returns [`None`] if the class couldn't be allocated.
    ///
    /// An implementation for `+initialize` must also be given; the runtime
    /// calls this method for all classes, so it must be defined on root
    /// classes.
    ///
    /// Note that implementing a root class is not a simple endeavor!
    /// For example, your class probably cannot be passed to Cocoa code unless
    /// the entire `NSObject` protocol is implemented.
    /// Functionality it expects, like implementations of `-retain` and
    /// `-release` used by ARC, will not be present otherwise.
    pub fn root(name: &str, intitialize_fn: extern "C" fn(&Class, Sel)) -> Option<ClassDecl> {
        let mut decl = ClassDecl::with_superclass(name, None);
        if let Some(ref mut decl) = decl {
            unsafe {
                decl.add_class_method(sel!(initialize), intitialize_fn);
            }
        }
        decl
    }

    /// Adds a method with the given name and implementation.
    ///
    /// # Panics
    ///
    /// Panics if the method wasn't sucessfully added or if the selector and
    /// function take different numbers of arguments.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the types match those that are expected
    /// when the method is invoked from Objective-C.
    pub unsafe fn add_method<F>(&mut self, sel: Sel, func: F)
    where
        F: MethodImplementation<Callee = Object>,
    {
        let encs = F::Args::ENCODINGS;
        let sel_args = count_args(sel);
        assert_eq!(
            sel_args,
            encs.len(),
            "Selector accepts {} arguments, but function accepts {}",
            sel_args,
            encs.len(),
        );

        let types = method_type_encoding(&F::Ret::ENCODING, encs);
        let success = Bool::from_raw(unsafe {
            ffi::class_addMethod(
                self.cls as _,
                sel.as_ptr() as _,
                Some(func.imp()),
                types.as_ptr(),
            )
        });
        assert!(success.is_true(), "Failed to add method {:?}", sel);
    }

    /// Adds a class method with the given name and implementation.
    ///
    /// # Panics
    ///
    /// Panics if the method wasn't sucessfully added or if the selector and
    /// function take different numbers of arguments.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the types match those that are expected
    /// when the method is invoked from Objective-C.
    pub unsafe fn add_class_method<F>(&mut self, sel: Sel, func: F)
    where
        F: MethodImplementation<Callee = Class>,
    {
        let encs = F::Args::ENCODINGS;
        let sel_args = count_args(sel);
        assert_eq!(
            sel_args,
            encs.len(),
            "Selector accepts {} arguments, but function accepts {}",
            sel_args,
            encs.len(),
        );

        let types = method_type_encoding(&F::Ret::ENCODING, encs);
        let metaclass = unsafe { &*self.cls }.metaclass() as *const _ as *mut _;
        let success = Bool::from_raw(unsafe {
            ffi::class_addMethod(
                metaclass,
                sel.as_ptr() as _,
                Some(func.imp()),
                types.as_ptr(),
            )
        });
        assert!(success.is_true(), "Failed to add class method {:?}", sel);
    }

    /// Adds an ivar with type `T` and the provided name.
    ///
    /// # Panics
    ///
    /// If the ivar wasn't successfully added.
    pub fn add_ivar<T: Encode>(&mut self, name: &str) {
        let c_name = CString::new(name).unwrap();
        let encoding = CString::new(T::ENCODING.to_string()).unwrap();
        let size = mem::size_of::<T>();
        let align = log2_align_of::<T>();
        let success = Bool::from_raw(unsafe {
            ffi::class_addIvar(
                self.cls as _,
                c_name.as_ptr(),
                size,
                align,
                encoding.as_ptr(),
            )
        });
        assert!(success.is_true(), "Failed to add ivar {}", name);
    }

    /// Adds the given protocol to self.
    ///
    /// # Panics
    ///
    /// If the protocol wasn't successfully added.
    pub fn add_protocol(&mut self, proto: &Protocol) {
        let success = unsafe { ffi::class_addProtocol(self.cls as _, proto.as_ptr()) };
        let success = Bool::from_raw(success).is_true();
        assert!(success, "Failed to add protocol {:?}", proto);
    }

    // fn add_property(&self, name: &str, attributes: &[ffi::objc_property_attribute_t]);

    /// Registers the [`ClassDecl`], consuming it, and returns a reference to
    /// the newly registered [`Class`].
    pub fn register(self) -> &'static Class {
        // Forget self, otherwise the class will be disposed in drop
        let cls = ManuallyDrop::new(self).cls;
        unsafe { ffi::objc_registerClassPair(cls as _) };
        unsafe { &*cls }
    }
}

impl Drop for ClassDecl {
    fn drop(&mut self) {
        unsafe { ffi::objc_disposeClassPair(self.cls as _) }
    }
}

/// A type for declaring a new protocol and adding new methods to it
/// before registering it.
pub struct ProtocolDecl {
    proto: *mut Protocol,
}

// SAFETY: Similar to ClassDecl
unsafe impl Send for ProtocolDecl {}
unsafe impl Sync for ProtocolDecl {}

impl ProtocolDecl {
    /// Constructs a [`ProtocolDecl`] with the given name.
    ///
    /// Returns [`None`] if the protocol couldn't be allocated.
    pub fn new(name: &str) -> Option<ProtocolDecl> {
        let c_name = CString::new(name).unwrap();
        let proto = unsafe { ffi::objc_allocateProtocol(c_name.as_ptr()) } as *mut Protocol;
        if proto.is_null() {
            None
        } else {
            Some(ProtocolDecl { proto })
        }
    }

    fn add_method_description_common<Args, Ret>(
        &mut self,
        sel: Sel,
        is_required: bool,
        is_instance_method: bool,
    ) where
        Args: EncodeArguments,
        Ret: Encode,
    {
        let encs = Args::ENCODINGS;
        let sel_args = count_args(sel);
        assert_eq!(
            sel_args,
            encs.len(),
            "Selector accepts {} arguments, but function accepts {}",
            sel_args,
            encs.len(),
        );
        let types = method_type_encoding(&Ret::ENCODING, encs);
        unsafe {
            ffi::protocol_addMethodDescription(
                self.proto as _,
                sel.as_ptr() as _,
                types.as_ptr(),
                Bool::new(is_required).as_raw(),
                Bool::new(is_instance_method).as_raw(),
            );
        }
    }

    /// Adds an instance method declaration with a given description.
    pub fn add_method_description<Args, Ret>(&mut self, sel: Sel, is_required: bool)
    where
        Args: EncodeArguments,
        Ret: Encode,
    {
        self.add_method_description_common::<Args, Ret>(sel, is_required, true)
    }

    /// Adds a class method declaration with a given description.
    pub fn add_class_method_description<Args, Ret>(&mut self, sel: Sel, is_required: bool)
    where
        Args: EncodeArguments,
        Ret: Encode,
    {
        self.add_method_description_common::<Args, Ret>(sel, is_required, false)
    }

    /// Adds a requirement on another protocol.
    pub fn add_protocol(&mut self, proto: &Protocol) {
        unsafe {
            ffi::protocol_addProtocol(self.proto as _, proto.as_ptr());
        }
    }

    /// Registers the [`ProtocolDecl`], consuming it and returning a reference
    /// to the newly registered [`Protocol`].
    pub fn register(self) -> &'static Protocol {
        unsafe {
            ffi::objc_registerProtocol(self.proto as _);
            &*self.proto
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils;

    #[test]
    fn test_custom_class() {
        // Registering the custom class is in test_utils
        let obj = test_utils::custom_object();
        let _: () = unsafe { msg_send![obj, setFoo: 13u32] };
        let result: u32 = unsafe { msg_send![obj, foo] };
        assert_eq!(result, 13);
    }

    #[test]
    fn test_class_method() {
        let cls = test_utils::custom_class();
        let result: u32 = unsafe { msg_send![cls, classFoo] };
        assert_eq!(result, 7);
    }
}
