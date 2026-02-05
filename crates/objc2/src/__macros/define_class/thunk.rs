//! # Function wrappers for the `define_class!` macro.
//!
//!
//! ## Transformation axes
//!
//! From the declaration in `define_class!`, we statically know:
//! - Class vs. instance methods.
//! - Error vs. no error (returns `Result<R, Retained<E>>`).
//!
//! Which means we get 4 different kinds of methods, as represented by the
//! `ClassFnKind`, `MethodKind`, `ClassFnResultKind` and `MethodResultKind`.
//!
//! (When _sending_ messages, there's one more axis: super vs. non-super).
//!
//! Note that we could in theory detect `Result` from non-Result in just the
//! type-system, but since the selector requires one more argument than the
//! function, I think there is value in doing the detection in the macro as
//! is currently done (you have to add `_` at the end of the selector), so
//! that we can retain nice error messages when the user has too many or too
//! few arguments compared to the selector.
use core::marker::PhantomData;

use super::super::{ConvertArgument, ConvertError, KindDefined, RetainSemantics};
use crate::rc::Retained;
use crate::runtime::{MethodImplementation, Sel};
use crate::ClassType;

/// The trait implemented for each function by the `define_class!` macro.
///
/// Design choices:
/// - This is implemented on the class being defined, so that any usage of
///   `Self` in the function type refers to the correct type.
/// - The lifetime is needed to constrain the lifetime used by
///   `LifetimeAssign`, see that for details.
/// - We need to add a marker type `FnMarker` so that the trait can be
///   implemented multiple times, once for each function.
/// - We always specify the retain semantics, even if the method doesn't
///   return `Id`, since we cannot know in advance whether we'll need it.
/// - The kind is kept as a generic type instead of an associated type, since
///   it contains generic parameters that are otherwise hard to bound
///   properly (at least without GATs, which are not yet in our MSRV).
pub trait ConvertDefinedFn<'f, FnMarker, MethodFamily, Kind>: ClassType {
    /// The type of the function that we want to create a thunk for.
    type Func: FnToExternFn<FnMarker, MethodFamily, Kind, Self>;

    /// The function that we want to create a thunk for.
    const FN: Self::Func;

    /// The generated thunk.
    const THUNK: <Self::Func as FnToExternFn<FnMarker, MethodFamily, Kind, Self>>::ExternFn =
        { Self::Func::THUNK };
}

/// Function pointers that can be converted to `extern "C-unwind"` function
/// pointers with the correct ABI for Objective-C.
pub trait FnToExternFn<FnMarker, MethodFamily, Kind, Cls: ?Sized> {
    type ExternFn: MethodImplementation;

    const THUNK: Self::ExternFn;
}

/// Class method (associated function).
///
/// From: `fn(args*) -> R`
/// To:   `extern "C-unwind" fn(&AnyClass, Sel, args*) -> R`
#[derive(Debug)]
pub struct ClassFnKind<Receiver>(PhantomData<Receiver>);

/// Instance method.
///
/// From: `fn(Receiver, args*) -> R`
/// To:   `extern "C-unwind" fn(Receiver, Sel, args*) -> R`
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct MethodKind;

/// Associated function that returns a `Result`.
///
/// From: `fn(args*) -> Result<R, Retained<E>>`
/// To:   `extern "C-unwind" fn(&AnyClass, Sel, args*, Option<&mut *mut E>) -> R`
#[derive(Debug)]
pub struct ClassFnResultKind<Receiver>(PhantomData<Receiver>);

/// Method that returns a `Result`.
///
/// From: `fn(Receiver, args*) -> Result<R, Retained<E>>`
/// To:   `extern "C-unwind" fn(Receiver, &AnyClass, Sel, args*, Option<&mut *mut E>) -> R`
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct MethodResultKind;

macro_rules! impl_fn_to_extern {
    ($($param:ident: $param_ty:ident),* $(,)?) => {
        // Save variant
        impl_fn_to_extern_delegate! {
            (normal)
            ()
            ($($param: $param_ty,)*)
        }

        // Unsafe variant
        impl_fn_to_extern_delegate! {
            (normal)
            (unsafe)
            ($($param: $param_ty,)*)
        }

        // Safe error variant
        impl_fn_to_extern_delegate! {
            (error)
            ()
            ($($param: $param_ty,)*)
        }

        // Unsafe error variant
        impl_fn_to_extern_delegate! {
            (error)
            (unsafe)
            ($($param: $param_ty,)*)
        }
    };
}

macro_rules! impl_fn_to_extern_delegate {
    (
        (normal)
        ($($unsafe:tt)?)
        ($($param:ident: $param_ty:ident),* $(,)?)
    ) => {
        // Class methods.
        impl_fn_to_extern_inner! {
            ($($unsafe)?)
            ($($param: $param_ty,)*)
            ()
            (Return)
            (ClassFnKind<Receiver>)
            ($($unsafe)? fn($($param_ty),*) -> Return)
            // TODO: Somehow allow access to the receiver?
            // SAFETY: Upheld by caller.
            (|_receiver| $($unsafe)? { Cls::FN($($param),*) })
        }

        // Instance methods.
        impl_fn_to_extern_inner! {
            ($($unsafe)?)
            ($($param: $param_ty,)*)
            ()
            (Return)
            (MethodKind)
            ($($unsafe)? fn(Receiver, $($param_ty),*) -> Return)
            // SAFETY: Upheld by caller.
            (|receiver| $($unsafe)? { Cls::FN(receiver, $($param),*) })
        }
    };
    (
        (error)
        ($($unsafe:tt)?)
        ($($param:ident: $param_ty:ident),* $(,)?)
    ) => {
        // Class methods.
        impl_fn_to_extern_inner! {
            ($($unsafe)?)
            ($($param: $param_ty,)*)
            (error_out: Error)
            (Return::Inner)
            (ClassFnResultKind<Receiver>)
            ($($unsafe)? fn($($param_ty),*) -> Result<Return, Retained<Error>>)
            // TODO: Somehow allow access to the receiver?
            (|_receiver| write_result(
                // SAFETY: Upheld by caller.
                $($unsafe)? { Cls::FN($($param),*) },
                // SAFETY: The error parameter is guaranteed by the caller to
                // be valid and writable.
                unsafe { error_out.as_mut() },
            ))
        }

        // Instance methods.
        impl_fn_to_extern_inner! {
            ($($unsafe)?)
            ($($param: $param_ty,)*)
            (error_out: Error)
            (Return::Inner)
            (MethodResultKind)
            ($($unsafe)? fn(Receiver, $($param_ty),*) -> Result<Return, Retained<Error>>)
            (|receiver| write_result(
                // SAFETY: Upheld by caller.
                $($unsafe)? { Cls::FN(receiver, $($param),*) },
                // SAFETY: The error parameter is guaranteed by the caller to
                // be valid and writable.
                unsafe { error_out.as_mut() },
            ))
        }
    }
}

macro_rules! impl_fn_to_extern_inner {
    (
        ($($unsafe:tt)?)
        ($($param:ident: $param_ty:ident,)*)
        ($($error_out_param:ident: $Error:ident)?)
        ($actual_return:ty)
        // The kind of method, see further above in this file.
        ($Kind:ty)
        // The signature of the method we want to convert.
        ($actual_method:ty)
        // An expression to call the method.
        (|$receiver_name:ident| $actual_method_call:expr)
    ) => {
        const _: () = {
            // The actual function that will be called by the runtime.
            $($unsafe)? extern "C-unwind" fn thunk<'f, FnMarker, Receiver, Return, $($Error,)? MethodFamily, Cls, $($param_ty: ConvertArgument,)*>(
                receiver: MethodFamily::ReceiverInner,
                // TODO: Somehow allow access to the selector?
                _sel: Sel,
                $($param: $param_ty::__Inner,)*
                // Error parameters come last.
                $($error_out_param: *mut *mut $Error,)?
            ) -> MethodFamily::ReturnInner
            where
                $($Error: ClassType, Return: ConvertError,)?
                MethodFamily: RetainSemantics<Receiver, $actual_return, KindDefined>,
                Cls: ?Sized + ConvertDefinedFn<'f, FnMarker, MethodFamily, $Kind, Func = $actual_method>,
            {
                // Convert receiver.
                //
                // SAFETY: We're a thunk that is called by Objective-C, so
                // the receiver is guaranteed to be valid by the runtime.
                let $receiver_name = unsafe { MethodFamily::prepare_defined_method(receiver) };

                // Convert each parameter.
                $(let $param = $param_ty::__from_defined_param($param);)*

                // Call the method.
                let result = $actual_method_call;

                // Convert the result.
                MethodFamily::convert_defined_return(result)
            }

            // A very (!) generic implementation that allows converting from
            // `fn(...) -> R` to `extern "C-unwind" fn(recv?, Sel, ..., err?) -> R`.
            impl<'f, FnMarker, Receiver, Return, $($Error,)? MethodFamily, Cls, $($param_ty: ConvertArgument,)*>
                FnToExternFn<FnMarker, MethodFamily, $Kind, Cls> for $actual_method
            where
                $($Error: ClassType, Return: ConvertError,)?
                MethodFamily: RetainSemantics<Receiver, $actual_return, KindDefined>,
                Cls: ?Sized + ConvertDefinedFn<'f, FnMarker, MethodFamily, $Kind, Func = Self>,
            {
                // The type we're converting to.
                type ExternFn = $($unsafe)? extern "C-unwind" fn(
                    MethodFamily::ReceiverInner,
                    Sel,
                    $($param_ty::__Inner,)*
                    $(*mut *mut $Error,)?
                ) -> MethodFamily::ReturnInner;

                const THUNK: Self::ExternFn = {
                    thunk::<FnMarker, Receiver, Return, $($Error,)? MethodFamily, Cls $(, $param_ty)*>
                };
            }
        };
    };
}

/// Write the error to the given pointer if an error occurred.
///
/// This is the mirror of `MsgSendError::send_message_error`.
#[inline]
fn write_result<T: ConvertError, E: ClassType>(
    value: Result<T, Retained<E>>,
    out_ptr: Option<&mut *mut E>,
) -> T::Inner {
    T::from_option(match value {
        Ok(value) => Some(value),
        Err(err) => {
            // If an error pointer was provided, autorelease the error and
            // write it
            if let Some(out_ptr) = out_ptr {
                // `autorelease_return` doesn't make sense to use, there's too
                // far between the autorelease and the location that the
                // caller picks it up.
                *out_ptr = Retained::autorelease_ptr(err);
            }
            None
        }
    })
}

// Support functions with up to 16 arguments.
impl_fn_to_extern!();
impl_fn_to_extern!(a: A);
impl_fn_to_extern!(a: A, b: B);
impl_fn_to_extern!(a: A, b: B, c: C);
impl_fn_to_extern!(a: A, b: B, c: C, d: D);
impl_fn_to_extern!(a: A, b: B, c: C, d: D, e: E);
impl_fn_to_extern!(a: A, b: B, c: C, d: D, e: E, f: F);
impl_fn_to_extern!(a: A, b: B, c: C, d: D, e: E, f: F, g: G);
impl_fn_to_extern!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H);
impl_fn_to_extern!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I);
impl_fn_to_extern!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J);
impl_fn_to_extern!(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
    k: K,
);
impl_fn_to_extern!(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
    k: K,
    l: L,
);
impl_fn_to_extern!(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
    k: K,
    l: L,
    m: M,
);
impl_fn_to_extern!(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
    k: K,
    l: L,
    m: M,
    n: N,
);
impl_fn_to_extern!(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
    k: K,
    l: L,
    m: M,
    n: N,
    o: O,
);

// Only implement normal methods for 16-wide functions, as the error methods
// need to be able to store an extra parameter.
impl_fn_to_extern_delegate! {
    (normal)
    ()
    (
        a: A,
        b: B,
        c: C,
        d: D,
        e: E,
        f: F,
        g: G,
        h: H,
        i: I,
        j: J,
        k: K,
        l: L,
        m: M,
        n: N,
        o: O,
        p: P,
    )
}
impl_fn_to_extern_delegate! {
    (normal)
    (unsafe)
    (
        a: A,
        b: B,
        c: C,
        d: D,
        e: E,
        f: F,
        g: G,
        h: H,
        i: I,
        j: J,
        k: K,
        l: L,
        m: M,
        n: N,
        o: O,
        p: P,
    )
}

/// A tool for overwriting the lifetime of types in a function pointer where
/// they would normally have been inferred to be too general.
///
/// Due to limitations in the type-system, our [`MethodImplementation`] impls
/// are not general enough to support higher-ranked lifetimes like `'a` in
/// `for<'a> fn(&'a T)`.
///
/// In reality though, the only lifetime we care about is that of the return
/// type, in particular that it has no lifetime (it generally isn't safe to
/// return inner pointers in Objective-C).
///
/// The input lifetimes though don't matter because `define_class!`
/// (intentionally) provides no syntax to bound lifetimes to each other; so
/// they're always of the form `for<'a, 'b, ...> fn(T1<'a>, T2<'b>, ...)`.
///
/// So to support methods with those (i.e. most methods), we shorten the input
/// lifetimes to no longer be higher-ranked.
///
/// # How it works
///
/// Consider the following method:
/// ```ignore
/// fn foo(&self, foo: &i32, bar: Type<'_>) -> i32 { ... }
/// ```
///
/// For this, we have the tokens `&Self`, `&i32`, `Type<'_>` and `i32`, and we
/// want to specify a function pointer with non-higher-ranked lifetimes.
///
/// The naive approach is wrong:
///
/// ```ignore
/// type FnType = fn(&Self, &i32, Type<'_>) -> i32;
/// ```
///
/// Since the lifetimes will be inferred as:
///
/// ```ignore
/// type FnType = for<'x, 'y, 'z> fn(&'x Self, &'y i32, Type<'y>) -> i32;
/// ```
///
/// This contains higher-ranked lifetimes, which is impossible to write a
/// generic trait implementation for.
///
/// What we actually want is:
///
/// ```ignore
/// type FnType<'a> = fn(&'a Self, &'a i32, Type<'a>) -> i32;
/// ```
///
/// But we can't write that, since we can't edit the tokenstream reliably.
/// (the lifetime parameter can be omitted, e.g. types like
/// `struct Type<'a>(...);` can be named as just `Type`).
///
/// Instead, we use lifetime elision!
///
/// The reference says this on [lifetime elision]:
/// > If there is exactly one input lifetime position (elided or not), that
/// > lifetime is assigned to all elided output lifetimes.
///
/// Which we use by putting the types with elided lifetime (`&Self`, `&i32`
/// and `Type<'_>`) **into the output position of a new function**:
///
/// ```ignore
/// type NewLifetimes<'a> = fn(&'a ()) -> (&Self, &i32, Type<'_>);
/// // => fn(&'a ()) -> (&'a Self, &'a i32, Type<'a>)
/// ```
///
/// This gives us:
///
/// ```ignore
/// type NewLifetimes<'a> = fn(&'a ()) -> (&'a Self, &'a i32, Type<'a>);
/// ```
///
/// We can then use that with this trait to construct a new function type with
/// the desired signature.
///
/// ```ignore
/// type FnType<'a> = <(NewLifetimes<'a>, i32) as LifetimeAssign>::OutputFn;
/// ```
///
/// And we get the desired:
///
/// ```ignore
/// type FnType<'a> = fn(&'a Self, &'a i32, Type<'a>) -> i32;
/// ```
///
/// (In that sense, this trait is perhaps badly named, it's really the `fn`
/// itself that reassigns the lifetime, this trait just extracts things into
/// a `fn` again).
///
/// [lifetime elision]: https://doc.rust-lang.org/nomicon/lifetime-elision.html
pub trait LifetimeAssign {
    type OutputFn;
}

macro_rules! impl_lifetime_assign {
    ($($param_ty:ident),* $(,)?) => {
        // The `R: 'static` bound here isn't really necessary, the user is
        // gonna get a compile error anyhow if they try to use a non-static
        // return type, but we include it just to make sure (and to make
        // diagnostics a bit better).
        impl<'f, R: 'static $(, $param_ty)*> LifetimeAssign for (fn(&'f ()) -> ($($param_ty,)*), R) {
            type OutputFn = fn($($param_ty),*) -> R;
        }

        impl<'f, R: 'static $(, $param_ty)*> LifetimeAssign for (unsafe fn(&'f ()) -> ($($param_ty,)*), R) {
            type OutputFn = unsafe fn($($param_ty),*) -> R;
        }
    };
}

// Support functions with up to 16 arguments.
impl_lifetime_assign!();
impl_lifetime_assign!(A);
impl_lifetime_assign!(A, B);
impl_lifetime_assign!(A, B, C);
impl_lifetime_assign!(A, B, C, D);
impl_lifetime_assign!(A, B, C, D, E);
impl_lifetime_assign!(A, B, C, D, E, F);
impl_lifetime_assign!(A, B, C, D, E, F, G);
impl_lifetime_assign!(A, B, C, D, E, F, G, H);
impl_lifetime_assign!(A, B, C, D, E, F, G, H, I);
impl_lifetime_assign!(A, B, C, D, E, F, G, H, I, J);
impl_lifetime_assign!(A, B, C, D, E, F, G, H, I, J, K);
impl_lifetime_assign!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_lifetime_assign!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_lifetime_assign!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_lifetime_assign!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_lifetime_assign!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);

#[cfg(test)]
#[allow(non_local_definitions)]
#[allow(unexpected_cfgs)]
mod tests {
    use super::*;
    use crate::__macros::{CopyFamily, InitFamily, NewFamily, NoneFamily};
    use crate::define_class;
    use crate::rc::Allocated;
    use crate::runtime::{AnyClass, Bool, NSObject};

    // Test different manual implementations of ConvertDefinedFn
    define_class!(
        #[unsafe(super(NSObject))]
        struct MyClass;
    );

    #[test]
    fn test_new() {
        impl MyClass {
            fn new() -> Retained<Self> {
                unimplemented!()
            }
        }

        struct FnMarker;

        impl<'f> ConvertDefinedFn<'f, FnMarker, NewFamily, ClassFnKind<&AnyClass>> for MyClass {
            type Func = <(fn(&'f ()) -> (), Retained<Self>) as LifetimeAssign>::OutputFn;
            const FN: Self::Func = MyClass::new;
        }

        type Expected<'a> = extern "C-unwind" fn(&'a AnyClass, Sel) -> *mut MyClass;

        let _: Expected<'_> =
            <MyClass as ConvertDefinedFn<'_, FnMarker, NewFamily, ClassFnKind<&AnyClass>>>::THUNK;
    }

    #[test]
    fn test_init() {
        impl MyClass {
            fn init(_this: Allocated<Self>) -> Retained<Self> {
                unimplemented!()
            }
        }

        struct FnMarker;

        impl<'f> ConvertDefinedFn<'f, FnMarker, InitFamily, MethodKind> for MyClass {
            type Func =
                <(fn(&'f ()) -> (Allocated<Self>,), Retained<Self>) as LifetimeAssign>::OutputFn;
            const FN: Self::Func = MyClass::init;
        }

        type Expected<'a> = extern "C-unwind" fn(*mut MyClass, Sel) -> *mut MyClass;

        let _: Expected<'_> =
            <MyClass as ConvertDefinedFn<'_, FnMarker, InitFamily, MethodKind>>::THUNK;
    }

    #[test]
    fn test_bool() {
        impl MyClass {
            fn bool() -> bool {
                unimplemented!()
            }
        }

        struct FnMarker;

        impl<'f> ConvertDefinedFn<'f, FnMarker, NoneFamily, ClassFnKind<&AnyClass>> for MyClass {
            type Func = <(fn(&'f ()) -> (), bool) as LifetimeAssign>::OutputFn;
            // type Func = fn() -> bool;
            const FN: Self::Func = MyClass::bool;
        }

        type Expected<'a> = extern "C-unwind" fn(&'a AnyClass, Sel) -> Bool;

        let _: Expected<'_> =
            <MyClass as ConvertDefinedFn<'_, FnMarker, NoneFamily, ClassFnKind<&AnyClass>>>::THUNK;
    }

    #[test]
    fn test_method_retained() {
        impl MyClass {
            fn method_retained(&self, _arg1: bool, _arg2: u32) -> Retained<Self> {
                unimplemented!()
            }
        }

        struct FnMarker;

        impl<'f> ConvertDefinedFn<'f, FnMarker, NoneFamily, MethodKind> for MyClass {
            type Func =
                <(fn(&'f ()) -> (&Self, bool, u32), Retained<Self>) as LifetimeAssign>::OutputFn;
            const FN: Self::Func = MyClass::method_retained;
        }

        type Expected<'a> = extern "C-unwind" fn(&'a MyClass, Sel, Bool, u32) -> *mut MyClass;

        let _: Expected<'_> =
            <MyClass as ConvertDefinedFn<'_, FnMarker, NoneFamily, MethodKind>>::THUNK;
    }

    #[test]
    fn test_method_option_id() {
        impl MyClass {
            fn method_option_id(&self, _arg: u8) -> Option<Retained<Self>> {
                unimplemented!()
            }
        }

        struct FnMarker;

        impl<'f> ConvertDefinedFn<'f, FnMarker, CopyFamily, MethodKind> for MyClass {
            type Func =
                <(fn(&'f ()) -> (&Self, u8), Option<Retained<Self>>) as LifetimeAssign>::OutputFn;
            const FN: Self::Func = MyClass::method_option_id;
        }

        type Expected<'a> = extern "C-unwind" fn(&'a MyClass, Sel, u8) -> *mut MyClass;

        let _: Expected<'_> =
            <MyClass as ConvertDefinedFn<'_, FnMarker, CopyFamily, MethodKind>>::THUNK;
    }

    // TODO: Add way to allow output lifetime to refer to `&self` / other things.
    #[test]
    #[cfg(unsupported)]
    fn test_return_lifetime() {
        impl MyClass {
            fn return_lifetime(&self, _arg: &u8) -> &Self {
                self
            }
        }

        struct FnMarker;

        impl<'f> ConvertDefinedFn<'f, FnMarker, NoneFamily, MethodKind> for MyClass {
            type Func = <(fn(&'f ()) -> (&Self, &u8), &Self) as LifetimeAssign>::OutputFn;
            // type Func = fn(&'f Self, &'f u8) -> &'f Self;
            const FN: Self::Func = MyClass::return_lifetime;
        }

        type Expected<'a> = extern "C-unwind" fn(&'a MyClass, Sel, &'a u8) -> &'a MyClass;

        let _: Expected<'_> =
            <MyClass as ConvertDefinedFn<'_, FnMarker, NoneFamily, MethodKind>>::THUNK;
    }
}
