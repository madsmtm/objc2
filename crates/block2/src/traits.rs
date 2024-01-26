use core::mem;
use core::ptr;

use objc2::encode::{EncodeArgument, EncodeReturn};

use crate::{Block, StackBlock};

mod private {
    pub trait Sealed<A> {}
}

/// Types that may be used as the arguments of an Objective-C block.
///
/// This is implemented for tuples of up to 12 arguments, where each argument
/// implements [`EncodeArgument`].
///
///
/// # Safety
///
/// This is a sealed trait, and should not need to be implemented. Open an
/// issue if you know a use-case where this restrition should be lifted!
pub unsafe trait BlockArguments: Sized {
    /// Calls the given method the block and arguments.
    #[doc(hidden)]
    unsafe fn __call_block<R: EncodeReturn>(
        invoke: unsafe extern "C" fn(),
        block: *mut Block<Self, R>,
        args: Self,
    ) -> R;
}

/// Types that may be converted into a block.
///
/// This is implemented for [`Fn`] closures of up to 12 arguments, where each
/// argument implements [`EncodeArgument`] and the return type implements
/// [`EncodeReturn`].
///
///
/// # Safety
///
/// This is a sealed trait, and should not need to be implemented. Open an
/// issue if you know a use-case where this restrition should be lifted!
pub unsafe trait IntoBlock<A: BlockArguments>: private::Sealed<A> + Sized {
    /// The return type of the resulting block.
    type Output: EncodeReturn;

    #[doc(hidden)]
    fn __get_invoke_stack_block() -> unsafe extern "C" fn();
}

macro_rules! impl_traits {
    ($($a:ident : $t:ident),*) => (
        impl<$($t: EncodeArgument,)* R: EncodeReturn, Closure> private::Sealed<($($t,)*)> for Closure
        where
            Closure: Fn($($t,)*) -> R,
        {}

        unsafe impl<$($t: EncodeArgument),*> BlockArguments for ($($t,)*) {
            #[inline]
            unsafe fn __call_block<R: EncodeReturn>(
                invoke: unsafe extern "C" fn(),
                block: *mut Block<Self, R>,
                ($($a,)*): Self,
            ) -> R {
                // Very similar to `MessageArguments::__invoke`
                let invoke: unsafe extern "C" fn(*mut Block<Self, R> $(, $t)*) -> R = unsafe {
                    mem::transmute(invoke)
                };

                unsafe { invoke(block $(, $a)*) }
            }
        }

        unsafe impl<$($t: EncodeArgument,)* R: EncodeReturn, Closure> IntoBlock<($($t,)*)> for Closure
        where
            Closure: Fn($($t,)*) -> R,
        {
            type Output = R;

            #[inline]
            fn __get_invoke_stack_block() -> unsafe extern "C" fn() {
                unsafe extern "C" fn invoke<$($t,)* R, Closure>(
                    block: *mut StackBlock<($($t,)*), R, Closure>,
                    $($a: $t,)*
                ) -> R
                where
                    Closure: Fn($($t,)*) -> R,
                {
                    let closure = unsafe { &*ptr::addr_of!((*block).closure) };
                    (closure)($($a),*)
                }

                unsafe {
                    mem::transmute::<
                        unsafe extern "C" fn(*mut StackBlock<($($t,)*), R, Closure>, $($a: $t,)*) -> R,
                        unsafe extern "C" fn(),
                    >(invoke)
                }
            }
        }
    );
}

impl_traits!();
impl_traits!(t0: T0);
impl_traits!(t0: T0, t1: T1);
impl_traits!(t0: T0, t1: T1, t2: T2);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9, t10: T10);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9, t10: T10, t11: T11);
