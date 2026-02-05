use core::mem;

use crate::encode::{EncodeArgument, EncodeArguments, EncodeReturn, RefEncode};
use crate::runtime::{Imp, MessageReceiver, Sel};

mod private {
    pub trait Sealed {}
}

/// Types that can be used as the implementation of an Objective-C method.
///
/// This is a sealed trait that is implemented for a lot of `extern "C"`
/// function pointer types.
//
// Note: `Sized` is intentionally added to make the trait dyn-incompatible.
pub trait MethodImplementation: private::Sealed + Sized {
    /// The callee type of the method.
    type Callee: ?Sized + RefEncode;

    /// The argument types of the method.
    type Arguments: EncodeArguments;

    /// The return type of the method.
    type Return: EncodeReturn;

    #[doc(hidden)]
    fn __imp(self) -> Imp;
}

macro_rules! method_impl_inner {
    ($(($unsafe:ident))? $abi:literal; $($t:ident),*) => {
        impl<T, R, $($t),*> private::Sealed for $($unsafe)? extern $abi fn(T, Sel $(, $t)*) -> R
        where
            T: ?Sized + MessageReceiver,
            R: EncodeReturn,
            $($t: EncodeArgument,)*
        {}

        impl<T, R, $($t),*> MethodImplementation for $($unsafe)? extern $abi fn(T, Sel $(, $t)*) -> R
        where
            T: ?Sized + MessageReceiver,
            R: EncodeReturn,
            $($t: EncodeArgument,)*
        {
            type Callee = T::__Inner;
            type Arguments = ($($t,)*);
            type Return = R;

            fn __imp(self) -> Imp {
                // SAFETY: Transmuting to an `unsafe` function pointer
                unsafe { mem::transmute(self) }
            }
        }
    };
}

macro_rules! method_impl {
    ($($t:ident),*) => {
        method_impl_inner!((unsafe) "C"; $($t),*);
        method_impl_inner!("C"; $($t),*);
        method_impl_inner!((unsafe) "C-unwind"; $($t),*);
        method_impl_inner!("C-unwind"; $($t),*);
    };
}

// Up to 16 arguments.
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
