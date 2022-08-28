/// Define methods on an external class.
///
/// This is a convenience macro to easily generate associated functions and
/// methods that call [`msg_send!`][crate::msg_send] appropriately.
///
///
/// # Specification
///
/// Within the `impl` block you can define two types of functions without
/// bodies; ["associated functions"] and ["methods"]. These are then mapped to
/// the Objective-C equivalents "class methods" and "instance methods", and an
/// appropriate body is created for you. In particular, if you use `self` your
/// method will assumbed to be an instance method, and if you don't it will be
/// assumed to be a class method.
///
/// The desired selector can be specified using the `#[sel(my:selector:)]`
/// attribute. The name of the function doesn't matter.
///
/// If you specify a function/method with a body, the macro will simply ignore
/// it.
///
/// ["associated functions"]: https://doc.rust-lang.org/reference/items/associated-items.html#methods
/// ["methods"]: https://doc.rust-lang.org/reference/items/associated-items.html#methods
///
///
/// # Safety
///
/// You must ensure that any methods you declare with the `#[sel(...)]`
/// attribute upholds the safety guarantees decribed in the
/// [`msg_send!`][crate::msg_send] macro, _or_ are marked `unsafe`.
///
///
/// # Examples
///
/// Let's create a quick interface to the [`NSCalendar`] class:
///
/// [`NSCalendar`]: https://developer.apple.com/documentation/foundation/nscalendar?language=objc
///
/// ```
/// use objc2::foundation::{NSObject, NSRange, NSString, NSUInteger};
/// use objc2::rc::{Id, Shared};
/// use objc2::runtime::Bool;
/// use objc2::{extern_class, extern_methods, msg_send_id, Encode, Encoding, ClassType};
/// #
/// # #[cfg(feature = "gnustep-1-7")]
/// # unsafe { objc2::__gnustep_hack::get_class_to_force_linkage() };
///
/// extern_class!(
///     #[derive(PartialEq, Eq, Hash)]
///     pub struct NSCalendar;
///
///     unsafe impl ClassType for NSCalendar {
///         type Super = NSObject;
///     }
/// );
///
/// pub type NSCalendarIdentifier = NSString;
///
/// #[repr(usize)] // NSUInteger
/// pub enum NSCalendarUnit {
///     Hour = 32,
///     Minute = 64,
///     Second = 128,
///     // TODO: More units
/// }
///
/// unsafe impl Encode for NSCalendarUnit {
///     const ENCODING: Encoding = usize::ENCODING;
/// }
///
/// extern_methods!(
///     /// Creation methods.
///     // TODO: Support methods returning `Id`
///     unsafe impl NSCalendar {
///         pub fn current() -> Id<Self, Shared> {
///             unsafe { msg_send_id![Self::class(), currentCalendar] }
///         }
///
///         pub fn new(identifier: &NSCalendarIdentifier) -> Id<Self, Shared> {
///             unsafe {
///                 msg_send_id![
///                     msg_send_id![Self::class(), alloc],
///                     initWithCalendarIdentifier: identifier,
///                 ]
///             }
///         }
///     }
///
///     /// Accessor methods.
///     // SAFETY: `first_weekday` is correctly defined
///     // TODO: Support methods returning `bool`
///     unsafe impl NSCalendar {
///         #[sel(firstWeekday)]
///         pub fn first_weekday(&self) -> NSUInteger;
///
///         pub fn am_symbol(&self) -> Id<NSString, Shared> {
///             unsafe { msg_send_id![self, amSymbol] }
///         }
///
///         #[sel(date:matchesComponents:)]
///         // `unsafe` because we don't have definitions for `NSDate` and
///         // `NSDateComponents` yet, so the user must ensure that is what's
///         // passed.
///         pub unsafe fn date_matches_raw(&self, date: &NSObject, components: &NSObject) -> Bool;
///
///         pub unsafe fn date_matches(&self, date: &NSObject, components: &NSObject) -> bool {
///             self.date_matches_raw(date, components).as_bool()
///         }
///
///         #[sel(maximumRangeOfUnit:)]
///         pub fn max_range(&self, unit: NSCalendarUnit) -> NSRange;
///     }
/// );
/// ```
///
/// The `extern_methods!` declaration then becomes:
///
/// ```
/// # use objc2::foundation::{NSObject, NSRange, NSString, NSUInteger};
/// # use objc2::rc::{Id, Shared};
/// # use objc2::runtime::Bool;
/// # use objc2::{extern_class, extern_methods, msg_send_id, Encode, Encoding, ClassType};
/// #
/// # #[cfg(feature = "gnustep-1-7")]
/// # unsafe { objc2::__gnustep_hack::get_class_to_force_linkage() };
/// #
/// # extern_class!(
/// #     #[derive(PartialEq, Eq, Hash)]
/// #     pub struct NSCalendar;
/// #
/// #     unsafe impl ClassType for NSCalendar {
/// #         type Super = NSObject;
/// #     }
/// # );
/// #
/// # pub type NSCalendarIdentifier = NSString;
/// #
/// # #[repr(usize)] // NSUInteger
/// # pub enum NSCalendarUnit {
/// #     Hour = 32,
/// #     Minute = 64,
/// #     Second = 128,
/// #     // TODO: More units
/// # }
/// #
/// # unsafe impl Encode for NSCalendarUnit {
/// #     const ENCODING: Encoding = usize::ENCODING;
/// # }
/// #
/// # use objc2::msg_send;
/// /// Creation methods.
/// impl NSCalendar {
///     pub fn current() -> Id<Self, Shared> {
///         unsafe { msg_send_id![Self::class(), currentCalendar] }
///     }
///
///     pub fn new(identifier: &NSCalendarIdentifier) -> Id<Self, Shared> {
///         unsafe {
///             msg_send_id![
///                 msg_send_id![Self::class(), alloc],
///                 initWithCalendarIdentifier: identifier,
///             ]
///         }
///     }
/// }
///
/// /// Accessor methods.
/// impl NSCalendar {
///     pub fn first_weekday(&self) -> NSUInteger {
///         unsafe { msg_send![self, firstWeekday] }
///     }
///
///     pub fn am_symbol(&self) -> Id<NSString, Shared> {
///         unsafe { msg_send_id![self, amSymbol] }
///     }
///
///     pub unsafe fn date_matches_raw(&self, date: &NSObject, components: &NSObject) -> Bool {
///         unsafe { msg_send![self, date: date, matchesComponents: components] }
///     }
///
///     pub unsafe fn date_matches(&self, date: &NSObject, components: &NSObject) -> bool {
///         self.date_matches_raw(date, components).as_bool()
///     }
///
///     pub fn max_range(&self, unit: NSCalendarUnit) -> NSRange {
///         unsafe { msg_send![self, maximumRangeOfUnit: unit] }
///     }
/// }
/// ```
#[macro_export]
macro_rules! extern_methods {
    (
        $(
            $(#[$impl_m:meta])*
            unsafe impl<$($t:ident $(: $b:ident $(+ $rest:ident)*)?),*> $type:ty {
                $($methods:tt)*
            }
        )+
    ) => {
        $(
            $(#[$impl_m])*
            impl<$($t $(: $b $(+ $rest)*)?),*> $type {
                $crate::__inner_extern_methods! {
                    @rewrite_methods
                    $($methods)*
                }
            }
        )+
    };
    (
        $(
            $(#[$impl_m:meta])*
            unsafe impl $type:ty {
                $($methods:tt)*
            }
        )+
    ) => {
        $(
            $(#[$impl_m])*
            impl $type {
                $crate::__inner_extern_methods! {
                    @rewrite_methods
                    $($methods)*
                }
            }
        )+
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __inner_extern_methods {
    {@rewrite_methods} => {};
    {
        @rewrite_methods
        // Unsafe variant
        $(#[$($m:tt)*])*
        $v:vis unsafe fn $name:ident($($args:tt)*) $(-> $ret:ty)?;

        $($rest:tt)*
    } => {
        // Detect instance vs. class method.
        $crate::__rewrite_self_arg! {
            ($crate::__inner_extern_methods)
            ($($args)*)
            @method_out
            @($(#[$($m)*])*)
            @($v unsafe fn $name($($args)*) $(-> $ret)?)
        }

        $crate::__inner_extern_methods! {
            @rewrite_methods
            $($rest)*
        }
    };
    {
        @rewrite_methods
        // Safe variant
        $(#[$($m:tt)*])*
        $v:vis fn $name:ident($($args:tt)*) $(-> $ret:ty)?;

        $($rest:tt)*
    } => {
        $crate::__rewrite_self_arg! {
            ($crate::__inner_extern_methods)
            ($($args)*)
            @method_out
            @($(#[$($m)*])*)
            @($v fn $name($($args)*) $(-> $ret)?)
        }

        $crate::__inner_extern_methods! {
            @rewrite_methods
            $($rest)*
        }
    };
    {
        @rewrite_methods
        // Other items that people might want to put here (e.g. functions with
        // a body).
        $associated_item:item

        $($rest:tt)*
    } => {
        $associated_item

        $crate::__inner_extern_methods! {
            @rewrite_methods
            $($rest)*
        }
    };
    {
        @method_out
        @($(#[$($m:tt)*])*)
        @($($function_start:tt)*)
        @($($kind:tt)*)
        @($($args:tt)*)
    } => {
        $crate::__attribute_helper! {
            @strip_sel
            $(@[$($m)*])*
            ($($function_start)* {
                #[allow(unused_unsafe)]
                unsafe {
                    $crate::__attribute_helper! {
                        @extract_sel
                        ($crate::__inner_extern_methods)
                        ($(#[$($m)*])*)
                        @unsafe_method_body
                        @($($kind)*)
                        @($($args)*)
                    }
                }
            })
        }
    };

    {
        @unsafe_method_body
        @(instance_method)
        @(
            $self:ident: $self_ty:ty,
            _: $sel_ty:ty,
            $($arg:ident: $arg_ty:ty),* $(,)?
        )
        @($($sel:tt)*)
    } => {
        $crate::__collect_msg_send!(
            $crate::msg_send;
            $self;
            ($($sel)*);
            ($($arg),*);
        )
    };
    {
        @unsafe_method_body
        @(class_method)
        @(
            _: $cls_ty:ty,
            _: $sel_ty:ty,
            $($arg:ident: $arg_ty:ty),* $(,)?
        )
        @($($sel:tt)*)
    } => {
        $crate::__collect_msg_send!(
            $crate::msg_send;
            Self::class();
            ($($sel)*);
            ($($arg),*);
        )
    };
}

/// Zip selector and arguments, and forward to macro.
///
/// TODO: Investigate if there's a better way of doing this.
#[doc(hidden)]
#[macro_export]
macro_rules! __collect_msg_send {
    (
        $macro:path;
        $obj:expr;
        ($s:ident);
        ();
    ) => {{
        $macro![$obj, $s]
    }};
    (
        $macro:path;
        $obj:expr;
        ($s1:ident:);
        ($a1:expr);
    ) => {{
        $macro![$obj, $s1: $a1]
    }};
    (
        $macro:path;
        $obj:expr;
        ($s1:ident: $s2:ident:);
        ($a1:expr, $a2:expr);
    ) => {{
        $macro![$obj, $s1: $a1, $s2: $a2]
    }};
    (
        $macro:path;
        $obj:expr;
        ($s1:ident: $s2:ident: $s3:ident:);
        ($a1:expr, $a2:expr, $a3:expr);
    ) => {{
        $macro![$obj, $s1: $a1, $s2: $a2, $s3: $a3]
    }};
    (
        $macro:path;
        $obj:expr;
        ($s1:ident: $s2:ident: $s3:ident: $s4:ident:);
        ($a1:expr, $a2:expr, $a3:expr, $a4:expr);
    ) => {{
        $macro![$obj, $s1: $a1, $s2: $a2, $s3: $a3, $s4: $a4]
    }};
    (
        $macro:path;
        $obj:expr;
        ($s1:ident: $s2:ident: $s3:ident: $s4:ident: $s5:ident:);
        ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr);
    ) => {{
        $macro![$obj, $s1: $a1, $s2: $a2, $s3: $a3, $s4: $a4, $s5: $a5]
    }};
    (
        $macro:path;
        $obj:expr;
        ($s1:ident: $s2:ident: $s3:ident: $s4:ident: $s5:ident: $s6:ident:);
        ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr);
    ) => {{
        $macro![
            $obj,
            $s1: $a1,
            $s2: $a2,
            $s3: $a3,
            $s4: $a4,
            $s5: $a5,
            $s6: $a6
        ]
    }};
    (
        $macro:path;
        $obj:expr;
        ($s1:ident: $s2:ident: $s3:ident: $s4:ident: $s5:ident: $s6:ident: $s7:ident:);
        ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr);
    ) => {{
        $macro![
            $obj,
            $s1: $a1,
            $s2: $a2,
            $s3: $a3,
            $s4: $a4,
            $s5: $a5,
            $s6: $a6,
            $s7: $a7
        ]
    }};
    (
        $macro:path;
        $obj:expr;
        ($s1:ident: $s2:ident: $s3:ident: $s4:ident: $s5:ident: $s6:ident: $s7:ident: $s8:ident:);
        ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr);
    ) => {{
        $macro![
            $obj,
            $s1: $a1,
            $s2: $a2,
            $s3: $a3,
            $s4: $a4,
            $s5: $a5,
            $s6: $a6,
            $s7: $a7,
            $s8: $a8
        ]
    }};
    (
        $macro:path;
        $obj:expr;
        ($s1:ident: $s2:ident: $s3:ident: $s4:ident: $s5:ident: $s6:ident: $s7:ident: $s8:ident: $s9:ident:);
        ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr);
    ) => {{
        $macro![
            $obj,
            $s1: $a1,
            $s2: $a2,
            $s3: $a3,
            $s4: $a4,
            $s5: $a5,
            $s6: $a6,
            $s7: $a7,
            $s8: $a8,
            $s9: $a9
        ]
    }};
    (
        $macro:path;
        $obj:expr;
        ($s1:ident: $s2:ident: $s3:ident: $s4:ident: $s5:ident: $s6:ident: $s7:ident: $s8:ident: $s9:ident: $s10:ident:);
        ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr);
    ) => {{
        $macro![
            $obj,
            $s1: $a1,
            $s2: $a2,
            $s3: $a3,
            $s4: $a4,
            $s5: $a5,
            $s6: $a6,
            $s7: $a7,
            $s8: $a8,
            $s9: $a9,
            $s10: $a10
        ]
    }};
    (
        $macro:path;
        $obj:expr;
        ($s1:ident: $s2:ident: $s3:ident: $s4:ident: $s5:ident: $s6:ident: $s7:ident: $s8:ident: $s9:ident: $s10:ident: $s11:ident:);
        ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr, $a11:expr);
    ) => {{
        $macro![
            $obj,
            $s1: $a1,
            $s2: $a2,
            $s3: $a3,
            $s4: $a4,
            $s5: $a5,
            $s6: $a6,
            $s7: $a7,
            $s8: $a8,
            $s9: $a9,
            $s10: $a10,
            $s11: $a11
        ]
    }};
    (
        $macro:path;
        $obj:expr;
        ($s1:ident: $s2:ident: $s3:ident: $s4:ident: $s5:ident: $s6:ident: $s7:ident: $s8:ident: $s9:ident: $s10:ident: $s11:ident: $s12:ident:);
        ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr, $a11:expr, $a12:expr);
    ) => {{
        $macro![
            $obj,
            $s1: $a1,
            $s2: $a2,
            $s3: $a3,
            $s4: $a4,
            $s5: $a5,
            $s6: $a6,
            $s7: $a7,
            $s8: $a8,
            $s9: $a9,
            $s10: $a10,
            $s11: $a11,
            $s12: $a12
        ]
    }};
    ($($_any:tt)*) => {{
        compile_error!("Number of arguments in function and selector did not match!")
    }};
}
