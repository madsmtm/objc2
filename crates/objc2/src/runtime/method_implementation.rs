use core::mem;

use crate::__macro_helpers::IdReturnValue;
use crate::encode::{EncodeArgument, EncodeArguments, EncodeReturn, RefEncode};
use crate::mutability::IsMutable;
use crate::rc::Allocated;
use crate::runtime::{AnyClass, AnyObject, Imp, Sel};
use crate::Message;

mod private {
    pub trait Sealed {}
}

/// Types that can be used as the implementation of an Objective-C method.
///
/// This is a sealed trait that is implemented for a lot of `extern "C"`
/// function pointer types.
//
// Note: `Sized` is intentionally added to make the trait not object safe.
pub trait MethodImplementation: private::Sealed + Sized {
    /// The callee type of the method.
    type Callee: RefEncode + ?Sized;
    /// The return type of the method.
    type Ret: EncodeReturn;
    /// The argument types of the method.
    type Args: EncodeArguments;

    #[doc(hidden)]
    fn __imp(self) -> Imp;
}

macro_rules! method_impl_generic {
    (<$($l:lifetime),*> T: $t_bound:ident $(+ $t_bound2:ident)?, $r:ident, $f:ty, $($t:ident),*) => {
        impl<$($l,)* T, $r, $($t),*> private::Sealed for $f
        where
            T: ?Sized + $t_bound $(+ $t_bound2)?,
            $r: EncodeReturn,
            $($t: EncodeArgument,)*
        {}

        impl<$($l,)* T, $r, $($t),*> MethodImplementation for $f
        where
            T: ?Sized + $t_bound $(+ $t_bound2)?,
            $r: EncodeReturn,
            $($t: EncodeArgument,)*
        {
            type Callee = T;
            type Ret = $r;
            type Args = ($($t,)*);

            fn __imp(self) -> Imp {
                unsafe { mem::transmute(self) }
            }
        }
    };
}

macro_rules! method_impl_concrete {
    (<$($l:lifetime),*> $callee:ident, $r:ident, $f:ty, $($t:ident),*) => {
        impl<$($l,)* $r, $($t),*> private::Sealed for $f
        where
            $r: EncodeReturn,
            $($t: EncodeArgument,)*
        {}

        impl<$($l,)* $r, $($t),*> MethodImplementation for $f
        where
            $r: EncodeReturn,
            $($t: EncodeArgument,)*
        {
            type Callee = $callee;
            type Ret = $r;
            type Args = ($($t,)*);

            fn __imp(self) -> Imp {
                unsafe { mem::transmute(self) }
            }
        }
    };
}

macro_rules! method_impl_allocated {
    (<> Allocated<T>, $f:ty, $($t:ident),*) => {
        #[doc(hidden)]
        impl<T, $($t),*> private::Sealed for $f
        where
            T: ?Sized + Message,
            $($t: EncodeArgument,)*
        {}

        #[doc(hidden)]
        impl<T, $($t),*> MethodImplementation for $f
        where
            T: ?Sized + Message,
            $($t: EncodeArgument,)*
        {
            type Callee = T;
            type Ret = IdReturnValue;
            type Args = ($($t,)*);

            fn __imp(self) -> Imp {
                // SAFETY: `Allocated<T>` is the same as `NonNull<T>`, except
                // with the assumption of a +1 calling convention.
                //
                // The calling convention is ensured to be upheld by having
                // `IdReturnValue` in the type, since that type is private
                // and hence only internal macros like `#[method_id]` will be
                // able to produce it (and that, in turn, only allows it if
                // the selector is `init` as checked by `MessageRecieveId`).
                unsafe { mem::transmute(self) }
            }
        }
    };
}

macro_rules! method_impl_abi {
    ($abi:literal; $($t:ident),*) => {
        method_impl_generic!(<'a> T: Message, R, extern $abi fn(&'a T, Sel $(, $t)*) -> R, $($t),*);
        method_impl_generic!(<'a> T: Message + IsMutable, R, extern $abi fn(&'a mut T, Sel $(, $t)*) -> R, $($t),*);
        method_impl_generic!(<> T: Message, R, unsafe extern $abi fn(*const T, Sel $(, $t)*) -> R, $($t),*);
        method_impl_generic!(<> T: Message, R, unsafe extern $abi fn(*mut T, Sel $(, $t)*) -> R, $($t),*);
        method_impl_generic!(<'a> T: Message, R, unsafe extern $abi fn(&'a T, Sel $(, $t)*) -> R, $($t),*);
        method_impl_generic!(<'a> T: Message + IsMutable, R, unsafe extern $abi fn(&'a mut T, Sel $(, $t)*) -> R, $($t),*);

        method_impl_concrete!(<'a> AnyObject, R, extern $abi fn(&'a mut AnyObject, Sel $(, $t)*) -> R, $($t),*);
        method_impl_concrete!(<'a> AnyObject, R, unsafe extern $abi fn(&'a mut AnyObject, Sel $(, $t)*) -> R, $($t),*);

        method_impl_concrete!(<'a> AnyClass, R, extern $abi fn(&'a AnyClass, Sel $(, $t)*) -> R, $($t),*);
        method_impl_concrete!(<> AnyClass, R, unsafe extern $abi fn(*const AnyClass, Sel $(, $t)*) -> R, $($t),*);
        method_impl_concrete!(<'a> AnyClass, R, unsafe extern $abi fn(&'a AnyClass, Sel $(, $t)*) -> R, $($t),*);

        method_impl_allocated!(<> Allocated<T>, extern $abi fn(Allocated<T>, Sel $(, $t)*) -> IdReturnValue, $($t),*);
        method_impl_allocated!(<> Allocated<T>, unsafe extern $abi fn(Allocated<T>, Sel $(, $t)*) -> IdReturnValue, $($t),*);
    };
}

macro_rules! method_impl {
    ($($t:ident),*) => {
        method_impl_abi!("C"; $($t),*);
        #[cfg(feature = "unstable-c-unwind")]
        method_impl_abi!("C-unwind"; $($t),*);
    };
}

method_impl!();
method_impl!(A);
method_impl!(A, B);
method_impl!(A, B, C);
method_impl!(A, B, C, D);
method_impl!(A, B, C, D, E);
method_impl!(A, B, C, D, E, F);
method_impl!(A, B, C, D, E, F, G);
method_impl!(A, B, C, D, E, F, G, H);
method_impl!(A, B, C, D, E, F, G, H, I);
method_impl!(A, B, C, D, E, F, G, H, I, J);
method_impl!(A, B, C, D, E, F, G, H, I, J, K);
method_impl!(A, B, C, D, E, F, G, H, I, J, K, L);
method_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M);
method_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
method_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
method_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
