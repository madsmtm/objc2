//! # Dynamically creating classes and protocols.
//!
//! Classes can be declared using the [`ClassBuilder`] struct. Instance
//! variables and methods can then be added before the class is ultimately
//! registered.
//!
//! **Note**: You likely don't need the dynamicism that this module provides!
//! Consider using the [`declare_class!`][crate::declare_class] macro instead.
//!
//!
//! ## Example
//!
//! The following example demonstrates declaring a class named `MyNumber` that
//! has one ivar, a `u32` named `_number` and a few methods for constructor
//! methods and methods for interfacing with the number (using interior
//! mutability, as is common for Objective-C objects).
//!
//! ```
//! use core::cell::Cell;
//!
//! use objc2::declare::ClassBuilder;
//! use objc2::rc::Id;
//! use objc2::runtime::{AnyClass, AnyObject, NSObject, Sel};
//! use objc2::{sel, msg_send, msg_send_id, ClassType};
//!
//! fn register_class() -> &'static AnyClass {
//!     // Inherit from NSObject
//!     let mut builder = ClassBuilder::new("MyNumber", NSObject::class())
//!         .expect("a class with the name MyNumber likely already exists");
//!
//!     // Add an instance variable of type `Cell<u32>`
//!     builder.add_ivar::<Cell<u32>>("_number");
//!
//!     // Add an Objective-C method for initializing an instance with a number
//!     //
//!     // We "cheat" a bit here, and use `AnyObject` instead of `NSObject`,
//!     // since only the former is allowed to be a mutable receiver (which is
//!     // always safe in `init` methods, but not in others).
//!     unsafe extern "C" fn init_with_number(
//!         this: &mut AnyObject,
//!         _cmd: Sel,
//!         number: u32,
//!     ) -> Option<&mut AnyObject> {
//!         let this: Option<&mut AnyObject> = msg_send![super(this, NSObject::class()), init];
//!         this.map(|this| {
//!             // SAFETY: The ivar is added with the same type above
//!             this.set_ivar::<Cell<u32>>("_number", Cell::new(number));
//!             this
//!         })
//!     }
//!     unsafe {
//!         builder.add_method(
//!             sel!(initWithNumber:),
//!             init_with_number as unsafe extern "C" fn(_, _, _) -> _,
//!         );
//!     }
//!
//!     // Add convenience method for getting a new instance with the number
//!     extern "C" fn with_number(
//!         cls: &AnyClass,
//!         _cmd: Sel,
//!         number: u32,
//!     ) -> *mut NSObject {
//!         let obj: Option<Id<NSObject>> = unsafe {
//!             msg_send_id![
//!                 msg_send_id![cls, alloc],
//!                 initWithNumber: number,
//!             ]
//!         };
//!         obj.map(Id::autorelease_return).unwrap_or(std::ptr::null_mut())
//!     }
//!     unsafe {
//!         builder.add_class_method(
//!             sel!(withNumber:),
//!             with_number as extern "C" fn(_, _, _) -> _,
//!         );
//!     }
//!
//!     // Add an Objective-C method for setting the number
//!     extern "C" fn my_number_set(this: &NSObject, _cmd: Sel, number: u32) {
//!         // SAFETY: The ivar is added with the same type above
//!         unsafe { this.ivar::<Cell<u32>>("_number") }.set(number);
//!     }
//!     unsafe {
//!         builder.add_method(sel!(setNumber:), my_number_set as extern "C" fn(_, _, _));
//!     }
//!
//!     // Add an Objective-C method for getting the number
//!     extern "C" fn my_number_get(this: &NSObject, _cmd: Sel) -> u32 {
//!         // SAFETY: The ivar is added with the same type above
//!         unsafe { this.ivar::<Cell<u32>>("_number") }.get()
//!     }
//!     unsafe {
//!         builder.add_method(sel!(number), my_number_get as extern "C" fn(_, _) -> _);
//!     }
//!
//!     builder.register()
//! }
//!
//! // Usage
//!
//! // Note: you should only do class registration once! This can be ensure
//! // with `std::sync::Once` or the `once_cell` crate.
//! let cls = register_class();
//!
//! let obj: Id<NSObject> = unsafe {
//!     msg_send_id![cls, withNumber: 42u32]
//! };
//!
//! let n: u32 = unsafe { msg_send![&obj, number] };
//! assert_eq!(n, 42);
//!
//! let _: () = unsafe { msg_send![&obj, setNumber: 12u32] };
//! let n: u32 = unsafe { msg_send![&obj, number] };
//! assert_eq!(n, 12);
//! ```

#[cfg(test)]
mod declare_class_tests;
mod ivar;
mod ivar_bool;
mod ivar_drop;
mod ivar_encode;
mod ivar_forwarding_impls;

use alloc::format;
use alloc::string::ToString;
use core::mem;
use core::mem::ManuallyDrop;
use core::ptr;
use core::ptr::NonNull;
use std::ffi::CString;

use crate::encode::__unstable::{EncodeArguments, EncodeReturn};
use crate::encode::{Encode, Encoding, RefEncode};
use crate::ffi;
use crate::mutability::IsMutable;
use crate::rc::Allocated;
use crate::runtime::{AnyClass, AnyObject, AnyProtocol, Bool, Imp, Sel};
use crate::sel;
use crate::Message;

pub use ivar::{InnerIvarType, Ivar, IvarType};
pub use ivar_bool::IvarBool;
pub use ivar_drop::IvarDrop;
pub use ivar_encode::IvarEncode;

pub(crate) mod private {
    pub trait Sealed {}
}

/// Types that can be used as the implementation of an Objective-C method.
///
/// This is a sealed trait that is implemented for a lot of `extern "C"`
/// function pointer types.
pub trait MethodImplementation: private::Sealed {
    /// The callee type of the method.
    type Callee: RefEncode + ?Sized;
    /// The return type of the method.
    type Ret: EncodeReturn;
    /// The argument types of the method.
    type Args: EncodeArguments;

    #[doc(hidden)]
    fn __imp(self) -> Imp;
}

macro_rules! method_decl_impl {
    (@<$($l:lifetime),*> T: $t_bound:ident, $r:ident, $f:ty, $($t:ident),*) => {
        impl<$($l,)* T, $r, $($t),*> private::Sealed for $f
        where
            T: ?Sized + $t_bound,
            $r: EncodeReturn,
            $($t: Encode,)*
        {}

        impl<$($l,)* T, $r, $($t),*> MethodImplementation for $f
        where
            T: ?Sized + $t_bound,
            $r: EncodeReturn,
            $($t: Encode,)*
        {
            type Callee = T;
            type Ret = $r;
            type Args = ($($t,)*);

            fn __imp(self) -> Imp {
                unsafe { mem::transmute(self) }
            }
        }
    };
    (@<$($l:lifetime),*> $callee:ident, $r:ident, $f:ty, $($t:ident),*) => {
        impl<$($l,)* $r, $($t),*> private::Sealed for $f
        where
            $r: EncodeReturn,
            $($t: Encode,)*
        {}

        impl<$($l,)* $r, $($t),*> MethodImplementation for $f
        where
            $r: EncodeReturn,
            $($t: Encode,)*
        {
            type Callee = $callee;
            type Ret = $r;
            type Args = ($($t,)*);

            fn __imp(self) -> Imp {
                unsafe { mem::transmute(self) }
            }
        }
    };
    (@<> Allocated<T>, $f:ty, $($t:ident),*) => {
        #[doc(hidden)]
        impl<T, $($t),*> private::Sealed for $f
        where
            T: ?Sized + Message,
            $($t: Encode,)*
        {}

        #[doc(hidden)]
        impl<T, $($t),*> MethodImplementation for $f
        where
            T: ?Sized + Message,
            $($t: Encode,)*
        {
            type Callee = T;
            type Ret = __IdReturnValue;
            type Args = ($($t,)*);

            fn __imp(self) -> Imp {
                // SAFETY: `Allocated<T>` is the same as `NonNull<T>`, except
                // with the assumption of a +1 calling convention.
                //
                // The calling convention is ensured to be upheld by having
                // `__IdReturnValue` in the type, since that type is private
                // and hence only internal macros like `#[method_id]` will be
                // able to produce it (and that, in turn, only allows it if
                // the selector is `init` as checked by `MessageRecieveId`).
                unsafe { mem::transmute(self) }
            }
        }
    };
    (# $abi:literal; $($t:ident),*) => {
        method_decl_impl!(@<'a> T: Message, R, extern $abi fn(&'a T, Sel $(, $t)*) -> R, $($t),*);
        method_decl_impl!(@<'a> T: IsMutable, R, extern $abi fn(&'a mut T, Sel $(, $t)*) -> R, $($t),*);
        method_decl_impl!(@<> T: Message, R, unsafe extern $abi fn(*const T, Sel $(, $t)*) -> R, $($t),*);
        method_decl_impl!(@<> T: Message, R, unsafe extern $abi fn(*mut T, Sel $(, $t)*) -> R, $($t),*);
        method_decl_impl!(@<'a> T: Message, R, unsafe extern $abi fn(&'a T, Sel $(, $t)*) -> R, $($t),*);
        method_decl_impl!(@<'a> T: IsMutable, R, unsafe extern $abi fn(&'a mut T, Sel $(, $t)*) -> R, $($t),*);

        method_decl_impl!(@<'a> AnyObject, R, extern $abi fn(&'a mut AnyObject, Sel $(, $t)*) -> R, $($t),*);
        method_decl_impl!(@<'a> AnyObject, R, unsafe extern $abi fn(&'a mut AnyObject, Sel $(, $t)*) -> R, $($t),*);

        method_decl_impl!(@<'a> AnyClass, R, extern $abi fn(&'a AnyClass, Sel $(, $t)*) -> R, $($t),*);
        method_decl_impl!(@<> AnyClass, R, unsafe extern $abi fn(*const AnyClass, Sel $(, $t)*) -> R, $($t),*);
        method_decl_impl!(@<'a> AnyClass, R, unsafe extern $abi fn(&'a AnyClass, Sel $(, $t)*) -> R, $($t),*);

        method_decl_impl!(@<> Allocated<T>, extern $abi fn(Allocated<T>, Sel $(, $t)*) -> __IdReturnValue, $($t),*);
        method_decl_impl!(@<> Allocated<T>, unsafe extern $abi fn(Allocated<T>, Sel $(, $t)*) -> __IdReturnValue, $($t),*);
    };
    ($($t:ident),*) => {
        method_decl_impl!(# "C"; $($t),*);
        #[cfg(feature = "unstable-c-unwind")]
        method_decl_impl!(# "C-unwind"; $($t),*);
    };
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
method_decl_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M);
method_decl_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
method_decl_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
method_decl_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);

/// Helper type for implementing `MethodImplementation` with a receiver of
/// `Allocated<T>`, without exposing that implementation to users.
#[doc(hidden)]
#[repr(transparent)]
pub struct __IdReturnValue(pub(crate) *mut AnyObject);

// SAFETY: `__IdReturnValue` is `#[repr(transparent)]`
unsafe impl Encode for __IdReturnValue {
    const ENCODING: Encoding = <*mut AnyObject>::ENCODING;
}

fn method_type_encoding(ret: &Encoding, args: &[Encoding]) -> CString {
    // First two arguments are always self and the selector
    let mut types = format!("{ret}{}{}", <*mut AnyObject>::ENCODING, Sel::ENCODING);
    for enc in args {
        use core::fmt::Write;
        write!(&mut types, "{enc}").unwrap();
    }
    CString::new(types).unwrap()
}

trait Log2Alignment {
    const LOG2_ALIGNMENT: u8;
}

impl<T> Log2Alignment for T {
    const LOG2_ALIGNMENT: u8 = {
        let align = mem::align_of::<T>();
        assert!(
            align.count_ones() == 1,
            "alignment required to be a power of 2"
        );
        // log2 of a power of 2 is the number of trailing zeros
        align.trailing_zeros() as u8
    };
}

/// A type for declaring a new class and adding new methods and ivars to it
/// before registering it.
#[derive(Debug)]
pub struct ClassBuilder {
    // Note: Don't ever construct a &mut objc_class, since it is possible to
    // get this pointer using `AnyClass::classes`!
    cls: NonNull<ffi::objc_class>,
}

#[doc(hidden)]
#[deprecated = "Use `ClassBuilder` instead."]
pub type ClassDecl = ClassBuilder;

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
// same thread that allocated it (so Send is safe).
unsafe impl Send for ClassBuilder {}
unsafe impl Sync for ClassBuilder {}

impl ClassBuilder {
    fn as_mut_ptr(&mut self) -> *mut ffi::objc_class {
        self.cls.as_ptr()
    }

    #[allow(unused)]
    fn superclass(&self) -> Option<&AnyClass> {
        // SAFETY: Though the class is not finalized, `class_getSuperclass` is
        // still safe to call.
        unsafe { AnyClass::superclass_raw(self.cls.as_ptr()) }
    }

    #[allow(unused)]
    fn name(&self) -> &str {
        // SAFETY: Same as `superclass`
        unsafe { AnyClass::name_raw(self.cls.as_ptr()) }
    }

    fn with_superclass(name: &str, superclass: Option<&AnyClass>) -> Option<Self> {
        let name = CString::new(name).unwrap();
        let super_ptr = superclass.map_or(ptr::null(), |c| c).cast();
        let cls = unsafe { ffi::objc_allocateClassPair(super_ptr, name.as_ptr(), 0) };
        NonNull::new(cls).map(|cls| Self { cls })
    }

    /// Constructs a [`ClassBuilder`] with the given name and superclass.
    ///
    /// Returns [`None`] if the class couldn't be allocated, or a class with
    /// that name already exist.
    pub fn new(name: &str, superclass: &AnyClass) -> Option<Self> {
        Self::with_superclass(name, Some(superclass))
    }

    /// Constructs a [`ClassBuilder`] declaring a new root class with the
    /// given name.
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
    pub fn root<F>(name: &str, intitialize_fn: F) -> Option<Self>
    where
        F: MethodImplementation<Callee = AnyClass, Args = (), Ret = ()>,
    {
        Self::with_superclass(name, None).map(|mut this| {
            unsafe { this.add_class_method(sel!(initialize), intitialize_fn) };
            this
        })
    }

    /// Adds a method with the given name and implementation.
    ///
    ///
    /// # Panics
    ///
    /// Panics if the method wasn't sucessfully added (e.g. a method with that
    /// name already exists).
    ///
    /// May also panic if the method was detected to be invalid in some way;
    /// for example if `debug_assertions` are enabled and the method is
    /// overriding another method, we verify that their encodings are equal.
    ///
    ///
    /// # Safety
    ///
    /// The caller must ensure that the types match those that are expected
    /// when the method is invoked from Objective-C.
    pub unsafe fn add_method<T, F>(&mut self, sel: Sel, func: F)
    where
        T: Message + ?Sized,
        F: MethodImplementation<Callee = T>,
    {
        unsafe {
            self.add_method_inner(
                sel,
                F::Args::ENCODINGS,
                F::Ret::ENCODING_RETURN,
                func.__imp(),
            )
        }
    }

    unsafe fn add_method_inner(
        &mut self,
        sel: Sel,
        enc_args: &[Encoding],
        enc_ret: Encoding,
        func: Imp,
    ) {
        let sel_args = sel.number_of_arguments();
        assert_eq!(
            sel_args,
            enc_args.len(),
            "selector {sel} accepts {sel_args} arguments, but function accepts {}",
            enc_args.len(),
        );

        // Verify that, if the method is present on the superclass, that the
        // encoding is correct.
        #[cfg(debug_assertions)]
        if let Some(superclass) = self.superclass() {
            if let Some(method) = superclass.instance_method(sel) {
                if let Err(err) = crate::verify::verify_method_signature(method, enc_args, &enc_ret)
                {
                    panic!("declared invalid method -[{} {sel}]: {err}", self.name())
                }
            }
        }

        let types = method_type_encoding(&enc_ret, enc_args);
        let success = Bool::from_raw(unsafe {
            ffi::class_addMethod(self.as_mut_ptr(), sel.as_ptr(), Some(func), types.as_ptr())
        });
        assert!(success.as_bool(), "failed to add method {sel}");
    }

    fn metaclass_mut(&mut self) -> *mut ffi::objc_class {
        unsafe { ffi::object_getClass(self.as_mut_ptr().cast()) as *mut ffi::objc_class }
    }

    /// Adds a class method with the given name and implementation.
    ///
    ///
    /// # Panics
    ///
    /// Panics in the same cases as [`add_method`][Self::add_method].
    ///
    ///
    /// # Safety
    ///
    /// The caller must ensure that the types match those that are expected
    /// when the method is invoked from Objective-C.
    pub unsafe fn add_class_method<F>(&mut self, sel: Sel, func: F)
    where
        F: MethodImplementation<Callee = AnyClass>,
    {
        unsafe {
            self.add_class_method_inner(
                sel,
                F::Args::ENCODINGS,
                F::Ret::ENCODING_RETURN,
                func.__imp(),
            )
        }
    }

    unsafe fn add_class_method_inner(
        &mut self,
        sel: Sel,
        enc_args: &[Encoding],
        enc_ret: Encoding,
        func: Imp,
    ) {
        let sel_args = sel.number_of_arguments();
        assert_eq!(
            sel_args,
            enc_args.len(),
            "selector {sel} accepts {sel_args} arguments, but function accepts {}",
            enc_args.len(),
        );

        // Verify that, if the method is present on the superclass, that the
        // encoding is correct.
        #[cfg(debug_assertions)]
        if let Some(superclass) = self.superclass() {
            if let Some(method) = superclass.class_method(sel) {
                if let Err(err) = crate::verify::verify_method_signature(method, enc_args, &enc_ret)
                {
                    panic!("declared invalid method +[{} {sel}]: {err}", self.name())
                }
            }
        }

        let types = method_type_encoding(&enc_ret, enc_args);
        let success = Bool::from_raw(unsafe {
            ffi::class_addMethod(
                self.metaclass_mut(),
                sel.as_ptr(),
                Some(func),
                types.as_ptr(),
            )
        });
        assert!(success.as_bool(), "failed to add class method {sel}");
    }

    /// Adds an ivar with type `T` and the provided name.
    ///
    ///
    /// # Panics
    ///
    /// If the ivar wasn't successfully added for some reason - this usually
    /// happens if there already was an ivar with that name.
    pub fn add_ivar<T: Encode>(&mut self, name: &str) {
        // SAFETY: The encoding is correct
        unsafe { self.add_ivar_inner::<T>(name, &T::ENCODING) }
    }

    // Monomorphized version
    unsafe fn add_ivar_inner_mono(
        &mut self,
        name: &str,
        size: usize,
        align: u8,
        encoding: &Encoding,
    ) {
        // `class_addIvar` sadly doesn't check this for us.
        //
        // We must _always_ do the check, since there is no way for the user
        // to statically know if the superclass has a declared instance
        // variable on it, since that may change if a new version of the
        // library/framework the class came from is released.
        if let Some(_ivar) = self
            .superclass()
            .and_then(|superclass| superclass.instance_variable(name))
        {
            panic!("instance variable {name:?} already exists on a superclass");
        }

        let c_name = CString::new(name).unwrap();
        let encoding = CString::new(encoding.to_string()).unwrap();
        let success = Bool::from_raw(unsafe {
            ffi::class_addIvar(
                self.as_mut_ptr(),
                c_name.as_ptr(),
                size,
                align,
                encoding.as_ptr(),
            )
        });
        assert!(success.as_bool(), "failed to add ivar {name}");
    }

    unsafe fn add_ivar_inner<T>(&mut self, name: &str, encoding: &Encoding) {
        unsafe { self.add_ivar_inner_mono(name, mem::size_of::<T>(), T::LOG2_ALIGNMENT, encoding) }
    }

    /// Adds an instance variable from an [`IvarType`].
    ///
    ///
    /// # Panics
    ///
    /// Same as [`ClassBuilder::add_ivar`].
    pub fn add_static_ivar<T: IvarType>(&mut self) {
        // SAFETY: The encoding is correct
        unsafe { self.add_ivar_inner::<T::Type>(T::NAME, &T::Type::ENCODING) }
    }

    /// Adds the given protocol to self.
    ///
    /// # Panics
    ///
    /// If the protocol wasn't successfully added.
    pub fn add_protocol(&mut self, proto: &AnyProtocol) {
        let success = unsafe { ffi::class_addProtocol(self.as_mut_ptr(), proto.as_ptr()) };
        let success = Bool::from_raw(success).as_bool();
        assert!(success, "failed to add protocol {proto}");
    }

    // fn add_property(&self, name: &str, attributes: &[ffi::objc_property_attribute_t]);

    /// Registers the [`ClassBuilder`], consuming it, and returns a reference
    /// to the newly registered [`AnyClass`].
    pub fn register(self) -> &'static AnyClass {
        // Forget self, otherwise the class will be disposed in drop
        let mut this = ManuallyDrop::new(self);
        unsafe { ffi::objc_registerClassPair(this.as_mut_ptr()) };
        unsafe { this.cls.cast::<AnyClass>().as_ref() }
    }
}

impl Drop for ClassBuilder {
    fn drop(&mut self) {
        // Disposing un-registered classes doesn't work properly on GNUStep,
        // so we register the class before disposing it.
        //
        // Doing it this way is _technically_ a race-condition, since other
        // code could read e.g. `AnyClass::classes()` and then pick the class
        // before it got disposed - but let's not worry about that for now.
        #[cfg(feature = "gnustep-1-7")]
        unsafe {
            ffi::objc_registerClassPair(self.as_mut_ptr());
        }

        unsafe { ffi::objc_disposeClassPair(self.as_mut_ptr()) }
    }
}

/// A type for declaring a new protocol and adding new methods to it
/// before registering it.
#[derive(Debug)]
pub struct ProtocolBuilder {
    proto: NonNull<AnyProtocol>,
}

#[doc(hidden)]
#[deprecated = "Use `ProtocolBuilder` instead."]
pub type ProtocolDecl = ProtocolBuilder;

// SAFETY: Similar to ClassBuilder
unsafe impl Send for ProtocolBuilder {}
unsafe impl Sync for ProtocolBuilder {}

impl ProtocolBuilder {
    fn as_mut_ptr(&mut self) -> *mut ffi::objc_protocol {
        self.proto.as_ptr().cast()
    }

    /// Constructs a [`ProtocolBuilder`] with the given name.
    ///
    /// Returns [`None`] if the protocol couldn't be allocated.
    pub fn new(name: &str) -> Option<Self> {
        let c_name = CString::new(name).unwrap();
        let proto = unsafe { ffi::objc_allocateProtocol(c_name.as_ptr()) };
        NonNull::new(proto.cast()).map(|proto| Self { proto })
    }

    fn add_method_description_inner(
        &mut self,
        sel: Sel,
        enc_args: &[Encoding],
        enc_ret: Encoding,
        required: bool,
        instance_method: bool,
    ) {
        let sel_args = sel.number_of_arguments();
        assert_eq!(
            sel_args,
            enc_args.len(),
            "selector {sel} accepts {sel_args} arguments, but function accepts {}",
            enc_args.len(),
        );
        let types = method_type_encoding(&enc_ret, enc_args);
        unsafe {
            ffi::protocol_addMethodDescription(
                self.as_mut_ptr(),
                sel.as_ptr(),
                types.as_ptr(),
                Bool::new(required).as_raw(),
                Bool::new(instance_method).as_raw(),
            );
        }
    }

    /// Adds an instance method declaration with a given description.
    pub fn add_method_description<Args, Ret>(&mut self, sel: Sel, required: bool)
    where
        Args: EncodeArguments,
        Ret: EncodeReturn,
    {
        self.add_method_description_inner(
            sel,
            Args::ENCODINGS,
            Ret::ENCODING_RETURN,
            required,
            true,
        )
    }

    /// Adds a class method declaration with a given description.
    pub fn add_class_method_description<Args, Ret>(&mut self, sel: Sel, required: bool)
    where
        Args: EncodeArguments,
        Ret: EncodeReturn,
    {
        self.add_method_description_inner(
            sel,
            Args::ENCODINGS,
            Ret::ENCODING_RETURN,
            required,
            false,
        )
    }

    /// Adds a requirement on another protocol.
    pub fn add_protocol(&mut self, proto: &AnyProtocol) {
        unsafe {
            ffi::protocol_addProtocol(self.as_mut_ptr(), proto.as_ptr());
        }
    }

    /// Registers the [`ProtocolBuilder`], consuming it and returning a reference
    /// to the newly registered [`AnyProtocol`].
    pub fn register(mut self) -> &'static AnyProtocol {
        unsafe {
            ffi::objc_registerProtocol(self.as_mut_ptr());
            self.proto.as_ref()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutability::Immutable;
    use crate::rc::Id;
    use crate::runtime::{NSObject, NSZone, __NSCopying as NSCopying};
    use crate::test_utils;
    use crate::{declare_class, msg_send, ClassType, ProtocolType};

    #[test]
    fn test_alignment() {
        assert_eq!(<()>::LOG2_ALIGNMENT, 0);

        assert_eq!(u8::LOG2_ALIGNMENT, 0);
        assert_eq!(u16::LOG2_ALIGNMENT, 1);
        assert_eq!(u32::LOG2_ALIGNMENT, 2);

        assert_eq!(
            u64::LOG2_ALIGNMENT,
            if cfg!(target_pointer_width = "64") {
                3
            } else {
                2
            }
        );

        #[repr(align(16))]
        struct Align16;
        assert_eq!(Align16::LOG2_ALIGNMENT, 4);

        #[repr(align(32))]
        struct Align32;
        assert_eq!(Align32::LOG2_ALIGNMENT, 5);

        #[repr(align(64))]
        struct Align64;
        assert_eq!(Align64::LOG2_ALIGNMENT, 6);

        #[repr(align(128))]
        struct Align128;
        assert_eq!(Align128::LOG2_ALIGNMENT, 7);

        #[repr(align(256))]
        struct Align256;
        assert_eq!(Align256::LOG2_ALIGNMENT, 8);

        #[repr(align(536870912))]
        struct Align536870912;
        assert_eq!(Align536870912::LOG2_ALIGNMENT, 29);
    }

    #[test]
    fn test_classbuilder_duplicate() {
        let cls = test_utils::custom_class();
        let builder = ClassBuilder::new("TestClassBuilderDuplicate", cls).unwrap();
        let _ = builder.register();

        assert!(ClassBuilder::new("TestClassBuilderDuplicate", cls).is_none());
    }

    #[test]
    #[should_panic = "failed to add ivar xyz"]
    fn duplicate_ivar() {
        let cls = test_utils::custom_class();
        let mut builder = ClassBuilder::new("TestClassBuilderDuplicateIvar", cls).unwrap();

        builder.add_ivar::<i32>("xyz");
        // Should panic:
        builder.add_ivar::<i32>("xyz");
    }

    #[test]
    #[should_panic = "failed to add method xyz"]
    fn duplicate_method() {
        let cls = test_utils::custom_class();
        let mut builder = ClassBuilder::new("TestClassBuilderDuplicateMethod", cls).unwrap();

        extern "C" fn xyz(_this: &NSObject, _cmd: Sel) {}

        unsafe {
            builder.add_method(sel!(xyz), xyz as extern "C" fn(_, _));
            // Should panic:
            builder.add_method(sel!(xyz), xyz as extern "C" fn(_, _));
        }
    }

    #[test]
    #[should_panic = "selector xyz: accepts 1 arguments, but function accepts 0"]
    fn wrong_arguments() {
        let cls = test_utils::custom_class();
        let mut builder = ClassBuilder::new("TestClassBuilderWrongArguments", cls).unwrap();

        extern "C" fn xyz(_this: &NSObject, _cmd: Sel) {}

        unsafe {
            // Should panic:
            builder.add_method(sel!(xyz:), xyz as extern "C" fn(_, _));
        }
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic = "declared invalid method -[TestClassBuilderInvalidMethod foo]: expected return to have type code 'I', but found 'i'"
    )]
    fn invalid_method() {
        let cls = test_utils::custom_class();
        let mut builder = ClassBuilder::new("TestClassBuilderInvalidMethod", cls).unwrap();

        extern "C" fn foo(_this: &NSObject, _cmd: Sel) -> i32 {
            0
        }

        unsafe {
            builder.add_method(sel!(foo), foo as extern "C" fn(_, _) -> _);
        }
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic = "declared invalid method +[TestClassBuilderInvalidClassMethod classFoo]: expected return to have type code 'I', but found 'i'"
    )]
    fn invalid_class_method() {
        let cls = test_utils::custom_class();
        let mut builder = ClassBuilder::new("TestClassBuilderInvalidClassMethod", cls).unwrap();

        extern "C" fn class_foo(_cls: &AnyClass, _cmd: Sel) -> i32 {
            0
        }

        unsafe {
            builder.add_class_method(sel!(classFoo), class_foo as extern "C" fn(_, _) -> _);
        }
    }

    #[test]
    #[should_panic = "failed to add protocol NSObject"]
    fn duplicate_protocol() {
        let cls = test_utils::custom_class();
        let mut builder = ClassBuilder::new("TestClassBuilderDuplicateProtocol", cls).unwrap();

        let protocol = AnyProtocol::get("NSObject").unwrap();

        builder.add_protocol(protocol);
        // Should panic:
        builder.add_protocol(protocol);
    }

    #[test]
    fn test_classbuilder_drop() {
        let cls = test_utils::custom_class();
        let builder = ClassBuilder::new("TestClassBuilderDrop", cls).unwrap();
        drop(builder);
        // After we dropped the class, we can create a new one with the same name:
        let _builder = ClassBuilder::new("TestClassBuilderDrop", cls).unwrap();
    }

    #[test]
    fn test_custom_class() {
        // Registering the custom class is in test_utils
        let mut obj = test_utils::custom_object();
        let _: () = unsafe { msg_send![&mut obj, setFoo: 13u32] };
        let result: u32 = unsafe { msg_send![&obj, foo] };
        assert_eq!(result, 13);
    }

    #[test]
    #[cfg(feature = "malloc")]
    fn test_in_all_classes() {
        fn is_present(cls: *const AnyClass) -> bool {
            // Check whether the class is present in AnyClass::classes()
            AnyClass::classes().iter().any(|item| ptr::eq(cls, *item))
        }

        let superclass = test_utils::custom_class();
        let builder = ClassBuilder::new("TestFetchWhileCreatingClass", superclass).unwrap();

        if cfg!(all(
            feature = "apple",
            any(target_arch = "aarch64", target_arch = "x86_64")
        )) {
            // It is IMO a bug that it is present here!
            assert!(is_present(builder.cls.as_ptr().cast()));
        } else {
            assert!(!is_present(builder.cls.as_ptr().cast()));
        }

        let cls = builder.register();
        assert!(is_present(cls));
    }

    #[test]
    fn test_class_method() {
        let cls = test_utils::custom_class();
        let result: u32 = unsafe { msg_send![cls, classFoo] };
        assert_eq!(result, 7);
    }

    #[test]
    #[should_panic = "could not create new class TestDeclareClassDuplicate. Perhaps a class with that name already exists?"]
    fn test_declare_class_duplicate() {
        declare_class!(
            struct Custom1;

            unsafe impl ClassType for Custom1 {
                type Super = NSObject;
                type Mutability = Immutable;
                const NAME: &'static str = "TestDeclareClassDuplicate";
            }
        );

        declare_class!(
            struct Custom2;

            unsafe impl ClassType for Custom2 {
                type Super = NSObject;
                type Mutability = Immutable;
                const NAME: &'static str = "TestDeclareClassDuplicate";
            }
        );

        let _cls = Custom1::class();
        // Should panic
        let _cls = Custom2::class();
    }

    #[test]
    fn test_declare_class_protocol() {
        declare_class!(
            struct Custom;

            unsafe impl ClassType for Custom {
                type Super = NSObject;
                type Mutability = Immutable;
                const NAME: &'static str = "TestDeclareClassProtocolNotFound";
            }

            unsafe impl NSCopying for Custom {
                #[method_id(copyWithZone:)]
                fn copy_with_zone(&self, _zone: *const NSZone) -> Id<Self> {
                    unimplemented!()
                }
            }
        );

        let cls = Custom::class();
        assert!(cls.conforms_to(<dyn NSCopying>::protocol().unwrap()));
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic = "declared invalid method -[TestDeclareClassInvalidMethod description]: expected return to have type code '@', but found 'v'"
    )]
    fn test_declare_class_invalid_method() {
        declare_class!(
            struct Custom;

            unsafe impl ClassType for Custom {
                type Super = NSObject;
                type Mutability = Immutable;
                const NAME: &'static str = "TestDeclareClassInvalidMethod";
            }

            unsafe impl Custom {
                // Override `description` with a bad return type
                #[method(description)]
                fn description(&self) {}
            }
        );

        let _cls = Custom::class();
    }

    #[test]
    #[cfg_attr(
        all(debug_assertions, feature = "verify"),
        should_panic = "must implement required protocol method -[NSCopying copyWithZone:]"
    )]
    fn test_declare_class_missing_protocol_method() {
        declare_class!(
            struct Custom;

            unsafe impl ClassType for Custom {
                type Super = NSObject;
                type Mutability = Immutable;
                const NAME: &'static str = "TestDeclareClassMissingProtocolMethod";
            }

            unsafe impl NSCopying for Custom {
                // Missing required method
            }
        );

        let _cls = Custom::class();
    }

    #[test]
    // #[cfg_attr(all(debug_assertions, feature = "verify"), should_panic = "...")]
    fn test_declare_class_invalid_protocol_method() {
        declare_class!(
            struct Custom;

            unsafe impl ClassType for Custom {
                type Super = NSObject;
                type Mutability = Immutable;
                const NAME: &'static str = "TestDeclareClassInvalidProtocolMethod";
            }

            unsafe impl NSCopying for Custom {
                // Override with a bad return type
                #[method(copyWithZone:)]
                fn copy_with_zone(&self, _zone: *const NSZone) -> u8 {
                    42
                }
            }
        );

        let _cls = Custom::class();
    }

    #[test]
    #[cfg_attr(
        all(debug_assertions, feature = "verify"),
        should_panic = "failed overriding protocol method -[NSCopying someOtherMethod]: method not found"
    )]
    fn test_declare_class_extra_protocol_method() {
        declare_class!(
            struct Custom;

            unsafe impl ClassType for Custom {
                type Super = NSObject;
                type Mutability = Immutable;
                const NAME: &'static str = "TestDeclareClassExtraProtocolMethod";
            }

            unsafe impl NSCopying for Custom {
                #[method_id(copyWithZone:)]
                fn copy_with_zone(&self, _zone: *const NSZone) -> Id<Self> {
                    unimplemented!()
                }

                // This doesn't exist on the protocol
                #[method(someOtherMethod)]
                fn some_other_method(&self) {}
            }
        );

        let _cls = Custom::class();
    }

    // Proof-of-concept how we could make declare_class! accept generic types.
    #[test]
    fn test_generic() {
        struct GenericDeclareClass<T>(T);

        unsafe impl<T> RefEncode for GenericDeclareClass<T> {
            const ENCODING_REF: Encoding = Encoding::Object;
        }

        unsafe impl<T> Message for GenericDeclareClass<T> {}

        unsafe impl<T> ClassType for GenericDeclareClass<T> {
            type Super = NSObject;
            type Mutability = Immutable;
            const NAME: &'static str = "GenericDeclareClass";

            #[inline]
            fn as_super(&self) -> &Self::Super {
                unimplemented!()
            }

            #[inline]
            fn as_super_mut(&mut self) -> &mut Self::Super {
                unimplemented!()
            }

            fn class() -> &'static AnyClass {
                let superclass = NSObject::class();
                let mut builder = ClassBuilder::new(Self::NAME, superclass).unwrap();

                unsafe {
                    builder.add_method(
                        sel!(generic),
                        <GenericDeclareClass<T>>::generic as unsafe extern "C" fn(_, _),
                    );
                }

                builder.register()
            }
        }

        impl<T> GenericDeclareClass<T> {
            extern "C" fn generic(&self, _cmd: Sel) {}
        }

        let _ = GenericDeclareClass::<()>::class();
    }
}
