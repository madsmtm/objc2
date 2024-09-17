use alloc::ffi::CString;
#[cfg(debug_assertions)]
use alloc::vec::Vec;
use core::marker::PhantomData;
#[cfg(debug_assertions)]
use std::collections::HashSet;

use crate::encode::{Encode, Encoding};
use crate::rc::{Allocated, Retained};
use crate::runtime::{
    AnyClass, AnyObject, ClassBuilder, MessageReceiver, MethodImplementation, Sel,
};
#[cfg(debug_assertions)]
use crate::runtime::{AnyProtocol, MethodDescription};
use crate::{ClassType, DeclaredClass, Message, ProtocolType};

use super::declared_ivars::{register_with_ivars, setup_dealloc};
use super::{CopyOrMutCopy, Init, MaybeUnwrap, New, Other};

/// Helper type for implementing `MethodImplementation` with a receiver of
/// `Allocated<T>`, without exposing that implementation to users.
//
// Must be private, soundness of MethodImplementation relies on this.
#[doc(hidden)]
#[repr(transparent)]
#[derive(Debug)]
#[allow(dead_code)]
pub struct IdReturnValue(pub(crate) *mut AnyObject);

// SAFETY: `IdReturnValue` is `#[repr(transparent)]`
unsafe impl Encode for IdReturnValue {
    const ENCODING: Encoding = <*mut AnyObject>::ENCODING;
}

// One could imagine a different design where we had a method like
// `fn convert_receiver()`, but that won't work in `declare_class!` since we
// can't actually modify the `self` argument (e.g. `let self = foo(self)` is
// not allowed).
//
// See `MsgSendId` and `RetainSemantics` for details on the retain semantics
// we're following here.
pub trait MessageRecieveId<Receiver, Ret> {
    fn into_return(obj: Ret) -> IdReturnValue;
}

// Receiver and return type have no correlation
impl<Receiver, Ret> MessageRecieveId<Receiver, Ret> for New
where
    Receiver: MessageReceiver,
    Ret: MaybeOptionId,
{
    #[inline]
    fn into_return(obj: Ret) -> IdReturnValue {
        obj.consumed_return()
    }
}

// Explicitly left unimplemented for now!
// impl MessageRecieveId<impl MessageReceiver, Allocated<T>> for Alloc {}

// Note: `MethodImplementation` allows for `Allocated` as the receiver, so we
// restrict it here to only be when the selector is `init`.
//
// Additionally, the receiver and return type must have the same generic
// parameter `T`.
impl<Ret, T> MessageRecieveId<Allocated<T>, Ret> for Init
where
    T: Message,
    Ret: MaybeOptionId<Input = Option<Retained<T>>>,
{
    #[inline]
    fn into_return(obj: Ret) -> IdReturnValue {
        obj.consumed_return()
    }
}

// Receiver and return type have no correlation
impl<Receiver, Ret> MessageRecieveId<Receiver, Ret> for CopyOrMutCopy
where
    Receiver: MessageReceiver,
    Ret: MaybeOptionId,
{
    #[inline]
    fn into_return(obj: Ret) -> IdReturnValue {
        obj.consumed_return()
    }
}

// Receiver and return type have no correlation
impl<Receiver, Ret> MessageRecieveId<Receiver, Ret> for Other
where
    Receiver: MessageReceiver,
    Ret: MaybeOptionId,
{
    #[inline]
    fn into_return(obj: Ret) -> IdReturnValue {
        obj.autorelease_return()
    }
}

/// Helper trait for specifying an `Retained<T>` or an `Option<Retained<T>>`.
///
/// (Both of those are valid return types from declare_class! `#[method_id]`).
pub trait MaybeOptionId: MaybeUnwrap {
    fn consumed_return(self) -> IdReturnValue;
    fn autorelease_return(self) -> IdReturnValue;
}

impl<T: Message> MaybeOptionId for Retained<T> {
    #[inline]
    fn consumed_return(self) -> IdReturnValue {
        let ptr: *mut T = Retained::into_raw(self);
        IdReturnValue(ptr.cast())
    }

    #[inline]
    fn autorelease_return(self) -> IdReturnValue {
        let ptr: *mut T = Retained::autorelease_return(self);
        IdReturnValue(ptr.cast())
    }
}

impl<T: Message> MaybeOptionId for Option<Retained<T>> {
    #[inline]
    fn consumed_return(self) -> IdReturnValue {
        let ptr: *mut T = Retained::consume_as_ptr_option(self);
        IdReturnValue(ptr.cast())
    }

    #[inline]
    fn autorelease_return(self) -> IdReturnValue {
        let ptr: *mut T = Retained::autorelease_return_option(self);
        IdReturnValue(ptr.cast())
    }
}

#[derive(Debug)]
pub struct ClassBuilderHelper<T: ?Sized> {
    builder: ClassBuilder,
    p: PhantomData<T>,
}

// Outlined for code size
#[track_caller]
fn create_builder(name: &str, superclass: &AnyClass) -> ClassBuilder {
    let c_name = CString::new(name).expect("class name must be UTF-8");
    match ClassBuilder::new(&c_name, superclass) {
        Some(builder) => builder,
        None => panic!(
            "could not create new class {name}. Perhaps a class with that name already exists?"
        ),
    }
}

impl<T: DeclaredClass> ClassBuilderHelper<T> {
    #[inline]
    #[track_caller]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self
    where
        T::Super: ClassType,
    {
        let mut builder = create_builder(T::NAME, <T::Super as ClassType>::class());

        setup_dealloc::<T>(&mut builder);

        Self {
            builder,
            p: PhantomData,
        }
    }

    #[inline]
    pub fn add_protocol_methods<P>(&mut self) -> ClassProtocolMethodsBuilder<'_, T>
    where
        P: ?Sized + ProtocolType,
    {
        let protocol = P::protocol();

        if let Some(protocol) = protocol {
            // Ignore the return value; whether the protocol is added is
            // inherently dependent on the order of the protocols.
            self.builder.add_protocol(protocol);
        }

        #[cfg(debug_assertions)]
        {
            ClassProtocolMethodsBuilder {
                builder: self,
                protocol,
                required_instance_methods: protocol
                    .map(|p| p.method_descriptions(true))
                    .unwrap_or_default(),
                optional_instance_methods: protocol
                    .map(|p| p.method_descriptions(false))
                    .unwrap_or_default(),
                registered_instance_methods: HashSet::new(),
                required_class_methods: protocol
                    .map(|p| p.class_method_descriptions(true))
                    .unwrap_or_default(),
                optional_class_methods: protocol
                    .map(|p| p.class_method_descriptions(false))
                    .unwrap_or_default(),
                registered_class_methods: HashSet::new(),
            }
        }

        #[cfg(not(debug_assertions))]
        {
            ClassProtocolMethodsBuilder { builder: self }
        }
    }

    // Addition: This restricts to callee `T`
    #[inline]
    pub unsafe fn add_method<F>(&mut self, sel: Sel, func: F)
    where
        F: MethodImplementation<Callee = T>,
    {
        // SAFETY: Checked by caller
        unsafe { self.builder.add_method(sel, func) }
    }

    #[inline]
    pub unsafe fn add_class_method<F>(&mut self, sel: Sel, func: F)
    where
        F: MethodImplementation<Callee = AnyClass>,
    {
        // SAFETY: Checked by caller
        unsafe { self.builder.add_class_method(sel, func) }
    }

    #[inline]
    pub fn register(self) -> (&'static AnyClass, isize, isize) {
        register_with_ivars::<T>(self.builder)
    }
}

/// Helper for ensuring that:
/// - Only methods on the protocol are overriden.
/// - TODO: The methods have the correct signature.
/// - All required methods are overridden.
#[derive(Debug)]
pub struct ClassProtocolMethodsBuilder<'a, T: ?Sized> {
    builder: &'a mut ClassBuilderHelper<T>,
    #[cfg(debug_assertions)]
    protocol: Option<&'static AnyProtocol>,
    #[cfg(debug_assertions)]
    required_instance_methods: Vec<MethodDescription>,
    #[cfg(debug_assertions)]
    optional_instance_methods: Vec<MethodDescription>,
    #[cfg(debug_assertions)]
    registered_instance_methods: HashSet<Sel>,
    #[cfg(debug_assertions)]
    required_class_methods: Vec<MethodDescription>,
    #[cfg(debug_assertions)]
    optional_class_methods: Vec<MethodDescription>,
    #[cfg(debug_assertions)]
    registered_class_methods: HashSet<Sel>,
}

impl<T: DeclaredClass> ClassProtocolMethodsBuilder<'_, T> {
    // Addition: This restricts to callee `T`
    #[inline]
    pub unsafe fn add_method<F>(&mut self, sel: Sel, func: F)
    where
        F: MethodImplementation<Callee = T>,
    {
        #[cfg(debug_assertions)]
        if let Some(protocol) = self.protocol {
            let _types = self
                .required_instance_methods
                .iter()
                .chain(&self.optional_instance_methods)
                .find(|desc| desc.sel == sel)
                .map(|desc| desc.types)
                .unwrap_or_else(|| {
                    panic!(
                        "failed overriding protocol method -[{protocol} {sel}]: method not found"
                    )
                });
        }

        // SAFETY: Checked by caller
        unsafe { self.builder.add_method(sel, func) };

        #[cfg(debug_assertions)]
        if !self.registered_instance_methods.insert(sel) {
            unreachable!("already added")
        }
    }

    #[inline]
    pub unsafe fn add_class_method<F>(&mut self, sel: Sel, func: F)
    where
        F: MethodImplementation<Callee = AnyClass>,
    {
        #[cfg(debug_assertions)]
        if let Some(protocol) = self.protocol {
            let _types = self
                .required_class_methods
                .iter()
                .chain(&self.optional_class_methods)
                .find(|desc| desc.sel == sel)
                .map(|desc| desc.types)
                .unwrap_or_else(|| {
                    panic!(
                        "failed overriding protocol method +[{protocol} {sel}]: method not found"
                    )
                });
        }

        // SAFETY: Checked by caller
        unsafe { self.builder.add_class_method(sel, func) };

        #[cfg(debug_assertions)]
        if !self.registered_class_methods.insert(sel) {
            unreachable!("already added")
        }
    }

    #[cfg(debug_assertions)]
    pub fn finish(self) {
        let superclass = self.builder.builder.superclass();

        if let Some(protocol) = self.protocol {
            for desc in &self.required_instance_methods {
                if self.registered_instance_methods.contains(&desc.sel) {
                    continue;
                }

                // TODO: Don't do this when `NS_PROTOCOL_REQUIRES_EXPLICIT_IMPLEMENTATION`
                if superclass
                    .and_then(|superclass| superclass.instance_method(desc.sel))
                    .is_some()
                {
                    continue;
                }

                panic!(
                    "must implement required protocol method -[{protocol} {}]",
                    desc.sel
                )
            }
        }

        if let Some(protocol) = self.protocol {
            for desc in &self.required_class_methods {
                if self.registered_class_methods.contains(&desc.sel) {
                    continue;
                }

                // TODO: Don't do this when `NS_PROTOCOL_REQUIRES_EXPLICIT_IMPLEMENTATION`
                if superclass
                    .and_then(|superclass| superclass.class_method(desc.sel))
                    .is_some()
                {
                    continue;
                }

                panic!(
                    "must implement required protocol method +[{protocol} {}]",
                    desc.sel
                );
            }
        }
    }

    #[inline]
    #[cfg(not(debug_assertions))]
    pub fn finish(self) {}
}
