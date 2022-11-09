/// Declare a new Objective-C class.
///
/// This is mostly just a convenience macro on top of [`extern_class!`] and
/// the functionality in the [`declare`] module, but it can really help
/// with cutting down on boilerplate, in particular when defining delegate
/// classes!
///
///
/// # Specification
///
/// This macro consists of three parts:
/// - The class definition + ivar definition + inheritance specification.
/// - A set of method definitions.
/// - A set of protocol definitions.
///
///
/// ## Class and ivar definition
///
/// The class definition works a lot like [`extern_class!`], with the added
/// functionality that you can define custom instance variables on your class,
/// which are then wrapped in a [`declare::Ivar`] and made accessible
/// through the class. (E.g. you can use `self.my_ivar` as if it was a normal
/// Rust struct).
///
/// Note that the class name should be unique across the entire application!
/// You can declare the class with the desired unique name like
/// `"MyCrateCustomObject"` by specifying it in `ClassType::NAME`, and then
/// give the exposed type a different name like `CustomObject`.
///
/// The class is guaranteed to have been created and registered with the
/// Objective-C runtime after the [`ClassType::class`] function has been
/// called.
///
/// If any of the instance variables require being `Drop`'ed (e.g. are wrapped
/// in [`declare::IvarDrop`]), this macro will generate a `dealloc` method
/// automatically.
///
/// [`ClassType::class`]: crate::ClassType::class
/// [`declare::IvarDrop`]: crate::declare::IvarDrop
///
///
/// ## Method definitions
///
/// Within the `impl` block you can define two types of functions;
/// ["associated functions"] and ["methods"]. These are then mapped to the
/// Objective-C equivalents "class methods" and "instance methods". In
/// particular, if you use `self` or the special name `this` (or `_this`),
/// your method will be registered as an instance method, and if you don't it
/// will be registered as a class method.
///
/// The desired selector can be specified using the `#[method(my:selector:)]`
/// attribute, similar to the [`extern_methods!`] macro.
///
/// A transformation step is performed on the functions (to make them have the
/// correct ABI) and hence they shouldn't really be called manually. (You
/// can't mark them as `pub` for the same reason). Instead, use the
/// [`extern_methods!`] macro to create a Rust interface to the methods.
///
/// If the argument or return type is [`bool`], a conversion is performed to
/// make it behave similarly to the Objective-C `BOOL`. Use [`runtime::Bool`]
/// if you want to control this manually.
///
/// ["associated functions"]: https://doc.rust-lang.org/reference/items/associated-items.html#methods
/// ["methods"]: https://doc.rust-lang.org/reference/items/associated-items.html#methods
/// [`extern_methods!`]: crate::extern_methods
/// [`msg_send!`]: crate::msg_send
/// [`runtime::Bool`]: crate::runtime::Bool
///
///
/// ## Protocol definitions
///
/// You can specify protocols that the class should implement, along with any
/// required/optional methods for said protocols.
///
/// The methods work exactly as normal, they're only put "under" the protocol
/// definition to make things easier to read.
///
///
/// # Panics
///
/// The implemented `ClassType::class` method may panic in a few cases, such
/// as if:
/// - A class with the name specified with `const NAME` already exists.
/// - One of the
/// - The `verify_message` feature is enabled, and an overriden method's
///   signature is not equal to the superclass'.
/// - The `verify_message` feature is enabled, and the required protocol
///   methods are not implemented.
///
///
/// # Safety
///
/// Using this macro requires writing a few `unsafe` markers:
///
/// `unsafe impl ClassType for T` has the following safety requirements:
/// - Same as [`extern_class!`] (the inheritance chain has to be correct).
/// - Any instance variables you specify under the struct definition must
///   either be able to be created using [`MaybeUninit::zeroed`], or be
///   properly initialized in an `init` method.
///
/// `unsafe impl T { ... }` asserts that the types match those that are
/// expected when the method is invoked from Objective-C. Note that there are
/// no safe-guards here; you can easily write `i8`, but if Objective-C thinks
/// it's an `u32`, it will cause UB when called!
///
/// `unsafe impl Protocol<P> for T { ... }` requires that all required methods
/// of the specified protocol is implemented, and that any extra requirements
/// (implicit or explicit) that the protocol has are upheld. The methods in
/// this definition has the same safety requirements as above.
///
/// [`MaybeUninit::zeroed`]: core::mem::MaybeUninit::zeroed
///
///
/// # Examples
///
/// Declare a class `MyCustomObject` that inherits `NSObject`, has a few
/// instance variables and methods, and implements the `NSCopying` protocol.
///
/// ```
/// use std::os::raw::c_int;
/// use objc2::declare::{Ivar, IvarDrop};
/// use objc2::rc::{Id, Owned, Shared};
/// use objc2::foundation::{NSCopying, NSObject, NSString, NSZone};
/// use objc2::{declare_class, msg_send, msg_send_id, ns_string, ClassType};
/// #
/// # #[cfg(feature = "gnustep-1-7")]
/// # unsafe { objc2::__gnustep_hack::get_class_to_force_linkage() };
///
/// declare_class!(
///     struct MyCustomObject {
///         foo: u8,
///         pub bar: c_int,
///         string: IvarDrop<Id<NSString, Shared>>,
///     }
///
///     unsafe impl ClassType for MyCustomObject {
///         type Super = NSObject;
///         // Optionally specify a different name
///         // const NAME: &'static str = "MyCustomObject";
///     }
///
///     unsafe impl MyCustomObject {
///         #[method(initWithFoo:)]
///         fn init_with(this: &mut Self, foo: u8) -> Option<&mut Self> {
///             let this: Option<&mut Self> = unsafe {
///                 msg_send![super(this), init]
///             };
///
///             // TODO: `ns_string` can't be used inside closures.
///             let s = ns_string!("abc");
///
///             this.map(|this| {
///                 // Initialize instance variables
///
///                 // Some types like `u8`, `bool`, `Option<Box<T>>` and
///                 // `Option<Id<T, O>>` are safe to zero-initialize, and
///                 // we can simply write to the variable as normal:
///                 *this.foo = foo;
///                 *this.bar = 42;
///
///                 // For others like `&u8`, `Box<T>` or `Id<T, O>`, we have
///                 // to initialize them with `Ivar::write`:
///                 Ivar::write(&mut this.string, s.copy());
///
///                 // All the instance variables have been initialized; our
///                 // initializer is sound
///                 this
///             })
///         }
///
///         #[method(foo)]
///         fn __get_foo(&self) -> u8 {
///             *self.foo
///         }
///
///         #[method(string)]
///         fn __get_string(&self) -> *mut NSString {
///             Id::autorelease_return((*self.string).copy())
///         }
///
///         #[method(myClassMethod)]
///         fn __my_class_method() -> bool {
///             true
///         }
///     }
///
///     unsafe impl Protocol<NSCopying> for MyCustomObject {
///         #[method(copyWithZone:)]
///         fn copy_with_zone(&self, _zone: *const NSZone) -> *mut Self {
///             let mut obj = Self::new(*self.foo);
///             *obj.bar = *self.bar;
///             obj.autorelease_return()
///         }
///     }
/// );
///
/// impl MyCustomObject {
///     pub fn new(foo: u8) -> Id<Self, Owned> {
///         unsafe { msg_send_id![Self::alloc(), initWithFoo: foo] }
///     }
///
///     pub fn get_foo(&self) -> u8 {
///         unsafe { msg_send![self, foo] }
///     }
///
///     pub fn get_string(&self) -> Id<NSString, Shared> {
///         unsafe { msg_send_id![self, string] }
///     }
///
///     pub fn my_class_method() -> bool {
///         unsafe { msg_send![Self::class(), myClassMethod] }
///     }
/// }
///
/// unsafe impl NSCopying for MyCustomObject {
///     type Ownership = Owned;
///     type Output = Self;
/// }
///
/// fn main() {
///     let obj = MyCustomObject::new(3);
///     assert_eq!(*obj.foo, 3);
///     assert_eq!(*obj.bar, 42);
///     assert_eq!(*obj.string, NSString::from_str("abc"));
///
///     let obj = obj.copy();
///     assert_eq!(obj.get_foo(), 3);
///     assert_eq!(obj.get_string(), NSString::from_str("abc"));
///
///     assert!(MyCustomObject::my_class_method());
/// }
/// ```
///
/// Approximately equivalent to the following ARC-enabled Objective-C code.
///
/// ```text
/// #import <Foundation/Foundation.h>
///
/// @interface MyCustomObject: NSObject <NSCopying> {
///     // Public ivar
///     int bar;
/// }
///
/// - (instancetype)initWithFoo:(uint8_t)foo;
/// - (uint8_t)foo;
/// - (NSString*)string;
/// + (BOOL)myClassMethod;
///
/// @end
///
///
/// @implementation MyCustomObject {
///     // Private ivar
///     uint8_t foo;
///     NSString* _Nonnull string;
/// }
///
/// - (instancetype)initWithFoo:(uint8_t)foo_arg {
///     self = [super init];
///     if (self) {
///         self->foo = foo_arg;
///         self->bar = 42;
///         self->string = @"abc";
///     }
///     return self;
/// }
///
/// - (uint8_t)foo {
///     return self->foo; // Or just `foo`
/// }
///
/// - (NSString*)string {
///     return self->string;
/// }
///
/// + (BOOL)myClassMethod {
///     return YES;
/// }
///
/// // NSCopying
///
/// - (id)copyWithZone:(NSZone *)_zone {
///     MyCustomObject* obj = [[MyCustomObject alloc] initWithFoo: self->foo];
///     obj->bar = self->bar;
///     obj->string = self->string;
///     return obj;
/// }
///
/// @end
/// ```
///
/// [`extern_class!`]: crate::extern_class
/// [`declare`]: crate::declare
/// [`declare::Ivar`]: crate::declare::Ivar
#[doc(alias = "@interface")]
#[doc(alias = "@implementation")]
#[macro_export]
macro_rules! declare_class {
    {
        $(#[$m:meta])*
        $v:vis struct $name:ident {
            $($ivar_v:vis $ivar:ident: $ivar_ty:ty,)*
        }

        unsafe impl ClassType for $for:ty {
            $(#[inherits($($inheritance_rest:ty),+)])?
            type Super = $superclass:ty;

            $(const NAME: &'static str = $name_const:literal;)?
        }

        $($methods:tt)*
    } => {
        $(
            #[allow(non_camel_case_types)]
            $ivar_v struct $ivar {
                __priv: (),
            }

            unsafe impl $crate::declare::IvarType for $ivar {
                type Type = $ivar_ty;
                const NAME: &'static str = stringify!($ivar);
            }
        )*

        $crate::__inner_extern_class! {
            @__inner
            $(#[$m])*
            // SAFETY: Upheld by caller
            $v struct ($name) {
                // SAFETY:
                // - The ivars are in a type used as an Objective-C object.
                // - The ivar is added to the class below.
                // - Rust prevents having two fields with the same name.
                // - Caller upholds that the ivars are properly initialized
                //
                // Note that while I couldn't find a reference on whether
                // ivars are zero-initialized or not, it has been true since
                // the Objective-C version shipped with Mac OS X 10.0 [link]
                // and is generally what one would expect coming from C. So I
                // think it's a valid assumption to make!
                //
                // [link]: https://github.com/apple-oss-distributions/objc4/blob/objc4-208/runtime/objc-class.m#L367
                $($ivar_v $ivar: $crate::declare::Ivar<$ivar>,)*
            }

            unsafe impl () for $for {
                INHERITS = [$superclass, $($($inheritance_rest,)+)? $crate::runtime::Object];
            }
        }

        // Creation
        unsafe impl ClassType for $for {
            type Super = $superclass;
            const NAME: &'static str = $crate::__select_name!($name; $($name_const)?);

            fn class() -> &'static $crate::runtime::Class {
                // TODO: Use `core::cell::LazyCell`
                use $crate::__macro_helpers::Once;

                static REGISTER_CLASS: Once = Once::new();

                REGISTER_CLASS.call_once(|| {
                    let superclass = <$superclass as $crate::ClassType>::class();
                    let mut builder = $crate::declare::ClassBuilder::new(Self::NAME, superclass).unwrap_or_else(|| {
                        $crate::__macro_helpers::panic!(
                            "could not create new class {}. Perhaps a class with that name already exists?",
                            Self::NAME,
                        )
                    });

                    // Ivars
                    $(
                        builder.add_static_ivar::<$ivar>();
                    )*

                    // Check whether we need to add a `dealloc` method
                    if false $(
                        || <<$ivar as $crate::declare::IvarType>::Type as $crate::declare::InnerIvarType>::__MAY_DROP
                    )* {
                        unsafe {
                            builder.add_method(
                                $crate::sel!(dealloc),
                                Self::__objc2_dealloc as unsafe extern "C" fn(_, _),
                            );
                        }
                    }

                    // Implement protocols and methods
                    $crate::__declare_class_methods!(
                        @register_out(builder)
                        $($methods)*
                    );

                    let _cls = builder.register();
                });

                // We just registered the class, so it should be available
                $crate::runtime::Class::get(Self::NAME).unwrap()
            }

            #[inline]
            fn as_super(&self) -> &Self::Super {
                &self.__inner
            }

            #[inline]
            fn as_super_mut(&mut self) -> &mut Self::Super {
                &mut self.__inner
            }
        }

        impl $for {
            // See the following links for more details:
            // - <https://clang.llvm.org/docs/AutomaticReferenceCounting.html#dealloc>
            // - <https://developer.apple.com/documentation/objectivec/nsobject/1571947-dealloc>
            // - <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/MemoryMgmt/Articles/mmRules.html#//apple_ref/doc/uid/20000994-SW2>
            unsafe extern "C" fn __objc2_dealloc(&mut self, _cmd: $crate::runtime::Sel) {
                $(
                    let ptr: *mut $crate::declare::Ivar<$ivar> = &mut self.$ivar;
                    // SAFETY: The ivar is valid, and since this is the
                    // `dealloc` method, we know the ivars are never going to
                    // be touched again.
                    unsafe { $crate::__macro_helpers::drop_in_place(ptr) };
                )*

                // Invoke the super class' `dealloc` method.
                //
                // Note: ARC does this automatically, so most Objective-C code
                // in the wild don't contain this; but we don't have ARC, so
                // we must do this.
                unsafe { $crate::msg_send![super(self), dealloc] }
            }
        }

        // Methods
        $crate::__declare_class_methods!(
            @method_out
            $($methods)*
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __select_name {
    ($_name:ident; $name_const:literal) => {
        $name_const
    };
    ($name:ident;) => {
        $crate::__macro_helpers::stringify!($name)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_class_methods {
    (@method_out) => {};
    // With protocol
    (
        @method_out

        $(#[$m:meta])*
        unsafe impl Protocol<$protocol:ident> for $for:ty {
            $($methods:tt)*
        }

        $($rest:tt)*
    ) => {
        $(#[$m])*
        impl $for {
            $crate::__declare_class_rewrite_methods! {
                @($crate::__declare_class_method_out)
                @()
                $($methods)*
            }
        }

        $crate::__declare_class_methods!(
            @method_out
            $($rest)*
        );
    };
    // Without protocol
    (
        @method_out

        $(#[$m:meta])*
        unsafe impl $for:ty {
            $($methods:tt)*
        }

        $($rest:tt)*
    ) => {
        $(#[$m])*
        impl $for {
            $crate::__declare_class_rewrite_methods! {
                @($crate::__declare_class_method_out)
                @()
                $($methods)*
            }
        }

        $crate::__declare_class_methods!(
            @method_out
            $($rest)*
        );
    };

    (@register_out($builder:ident)) => {};
    // With protocol
    (
        @register_out($builder:ident)

        $(#[$($m:tt)*])*
        unsafe impl Protocol<$protocol:ident> for $for:ty {
            $($methods:tt)*
        }

        $($rest:tt)*
    ) => {
        $crate::__extract_and_apply_cfg_attributes! {
            @($(#[$($m)*])*)
            @(
                // Implement protocol
                #[allow(unused_mut)]
                let mut protocol_builder = $builder.__add_protocol_methods(
                    $crate::runtime::Protocol::get(stringify!($protocol)).unwrap_or_else(|| {
                        $crate::__macro_helpers::panic!(
                            "could not find protocol {}",
                            $crate::__macro_helpers::stringify!($protocol),
                        )
                    })
                );

                // In case the user's function is marked `deprecated`
                #[allow(deprecated)]
                // In case the user did not specify any methods
                #[allow(unused_unsafe)]
                // SAFETY: Upheld by caller
                unsafe {
                    $crate::__declare_class_rewrite_methods! {
                        @($crate::__declare_class_register_out)
                        @(protocol_builder)

                        $($methods)*
                    }
                }

                // Finished declaring protocol; get error message if any
                #[allow(clippy::drop_ref)]
                drop(protocol_builder);
            )
        }

        $crate::__declare_class_methods!(
            @register_out($builder)
            $($rest)*
        );
    };
    // Without protocol
    (
        @register_out($builder:ident)

        $(#[$($m:tt)*])*
        unsafe impl $for:ty {
            $($methods:tt)*
        }

        $($rest:tt)*
    ) => {
        $crate::__extract_and_apply_cfg_attributes! {
            @($(#[$($m)*])*)
            @(
                // In case the user's function is marked `deprecated`
                #[allow(deprecated)]
                // In case the user did not specify any methods
                #[allow(unused_unsafe)]
                // SAFETY: Upheld by caller
                unsafe {
                    $crate::__declare_class_rewrite_methods! {
                        @($crate::__declare_class_register_out)
                        @($builder)

                        $($methods)*
                    }
                }
            )
        }

        $crate::__declare_class_methods!(
            @register_out($builder)
            $($rest)*
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_class_rewrite_methods {
    {
        @($($macro:tt)*)
        @($($macro_arguments:tt)*)
    } => {};

    // Unsafe variant
    {
        @($($macro:tt)*)
        @($($macro_arguments:tt)*)

        $(#[$($m:tt)*])*
        unsafe fn $name:ident($($args:tt)*) $(-> $ret:ty)? $body:block

        $($rest:tt)*
    } => {
        $crate::__rewrite_self_arg! {
            ($($macro)*)
            ($($args)*)

            // Split the function into parts, and send the arguments down to
            // be used later on
            @($($macro_arguments)*)
            @($(#[$($m)*])*)
            @(unsafe extern "C")
            @($name)
            @($($ret)?)
            @($body)
            // Will add @(kind)
            // Will add @(args_start)
            // Will add @(args_rest)
        }

        $crate::__declare_class_rewrite_methods! {
            @($($macro)*)
            @($($macro_arguments)*)

            $($rest)*
        }
    };

    // Safe variant
    {
        @($($macro:tt)*)
        @($($macro_arguments:tt)*)

        $(#[$($m:tt)*])*
        fn $name:ident($($args:tt)*) $(-> $ret:ty)? $body:block

        $($rest:tt)*
    } => {
        $crate::__rewrite_self_arg! {
            ($($macro)*)
            ($($args)*)

            @($($macro_arguments)*)
            @($(#[$($m)*])*)
            @(extern "C")
            @($name)
            @($($ret)?)
            @($body)

            // Same as above
        }

        $crate::__declare_class_rewrite_methods! {
            @($($macro)*)
            @($($macro_arguments)*)

            $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_class_method_out {
    {
        @() // No arguments needed
        @($(#[$($m:tt)*])*)
        @($($qualifiers:tt)*)
        @($name:ident)
        @($($ret:ty)?)
        @($($body:tt)*)
        @($($_kind:tt)*)
        @($($args_start:tt)*)
        @($($args_rest:tt)*)
    } => {
        $crate::__fn_args! {
            ($crate::__declare_class_method_out)
            ($($args_rest)*,)
            ()
            ()
            @inner
            @($(#[$($m)*])*)
            @($($qualifiers)*)
            @($name)
            @($($ret)?)
            @($($body)*)
            @($($_kind)*)
            @($($args_start)*)
            // Will add @(args_converted)
            // Will add @(body_prefix)
        }
    };

    // No return type
    {
        @inner
        @($(#[$($m:tt)*])*)
        @($($qualifiers:tt)*)
        @($name:ident)
        @()
        @($($body:tt)*)
        @($($_kind:tt)*)
        @($($args_start:tt)*)
        @($($args_converted:tt)*)
        @($($body_prefix:tt)*)
    } => {
        $crate::__strip_custom_attributes! {
            @($(#[$($m)*])*)
            @($($qualifiers)* fn $name($($args_start)* $($args_converted)*) {
                $($body_prefix)*
                $($body)*
            })
            @()
        }
    };

    // With return type
    {
        @inner
        @($(#[$($m:tt)*])*)
        @($($qualifiers:tt)*)
        @($name:ident)
        @($ret:ty)
        @($($body:tt)*)
        @($($_kind:tt)*)
        @($($args_start:tt)*)
        @($($args_converted:tt)*)
        @($($body_prefix:tt)*)
    } => {
        $crate::__strip_custom_attributes! {
            @($(#[$($m)*])*)
            @($($qualifiers)* fn $name($($args_start)* $($args_converted)*) -> <$ret as $crate::encode::EncodeConvert>::__Inner {
                $($body_prefix)*
                <$ret as $crate::encode::EncodeConvert>::__into_inner($($body)*)
            })
            @()
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __fn_args {
    // Ignore `_`
    {
        ($out_macro:path)
        (_: $param_ty:ty, $($rest:tt)*)
        ($($args_converted:tt)*)
        ($($body_prefix:tt)*)
        $($macro_args:tt)*
    } => {
        $crate::__fn_args! {
            ($out_macro)
            ($($rest)*)
            ($($args_converted)* _: $param_ty,)
            ($($body_prefix)*)
            $($macro_args)*
        }
    };
    // Convert mut
    {
        ($out_macro:path)
        (mut $param:ident: $param_ty:ty, $($rest:tt)*)
        ($($args_converted:tt)*)
        ($($body_prefix:tt)*)
        $($macro_args:tt)*
    } => {
        $crate::__fn_args! {
            ($out_macro)
            ($($rest)*)
            ($($args_converted)* $param: <$param_ty as $crate::encode::EncodeConvert>::__Inner,)
            (
                $($body_prefix)*
                let mut $param = <$param_ty as $crate::encode::EncodeConvert>::__from_inner($param);
            )
            $($macro_args)*
        }
    };
    // Convert
    {
        ($out_macro:path)
        ($param:ident: $param_ty:ty, $($rest:tt)*)
        ($($args_converted:tt)*)
        ($($body_prefix:tt)*)
        $($macro_args:tt)*
    } => {
        $crate::__fn_args! {
            ($out_macro)
            ($($rest)*)
            ($($args_converted)* $param: <$param_ty as $crate::encode::EncodeConvert>::__Inner,)
            (
                $($body_prefix)*
                let $param = <$param_ty as $crate::encode::EncodeConvert>::__from_inner($param);
            )
            $($macro_args)*
        }
    };
    // Output result
    {
        ($out_macro:path)
        ($(,)*)
        ($($args_converted:tt)*)
        ($($body_prefix:tt)*)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*
            @($($args_converted)*)
            @($($body_prefix)*)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_class_register_out {
    // Class method
    {
        @($builder:ident)
        @($(#[$($m:tt)*])*)
        @($($qualifiers:tt)*)
        @($name:ident)
        @($($_ret:tt)*)
        @($($_body:tt)*)
        @(class_method)
        @($($args_start:tt)*)
        @($($args_rest:tt)*)
    } => {
        $crate::__extract_and_apply_cfg_attributes! {
            @($(#[$($m)*])*)
            @(
                $builder.add_class_method(
                    $crate::__extract_custom_attributes! {
                        @($(#[$($m)*])*)
                        @($crate::__declare_class_register_out)
                        @(
                            @call_sel
                            // Macro will add:
                            // @(method attribute)
                        )
                        @()
                    },
                    Self::$name as $crate::__fn_ptr! {
                        @($($qualifiers)*) $($args_start)* $($args_rest)*
                    },
                );
            )
        }
    };

    // Instance method
    {
        @($builder:ident)
        @($(#[$($m:tt)*])*)
        @($($qualifiers:tt)*)
        @($name:ident)
        @($($_ret:tt)*)
        @($($_body:tt)*)
        @(instance_method)
        @($($args_start:tt)*)
        @($($args_rest:tt)*)
    } => {
        $crate::__extract_and_apply_cfg_attributes! {
            @($(#[$($m)*])*)
            @(
                $builder.add_method(
                    $crate::__extract_custom_attributes! {
                        @($(#[$($m)*])*)
                        @($crate::__declare_class_register_out)
                        @(
                            @call_sel
                            // Macro will add:
                            // @(method attribute)
                        )
                        @()
                    },
                    Self::$name as $crate::__fn_ptr! {
                        @($($qualifiers)*) $($args_start)* $($args_rest)*
                    },
                );
            )
        }
    };

    {
        @call_sel
        @(#[method($($sel:tt)*)])
    } => {
        $crate::sel!($($sel)*)
    };

    {
        @call_sel
        @(#[method_id($($sel:tt)*)])
    } => {
        compile_error!("`#[method_id(...)]` is not supported in `declare_class!` yet");
    };
}

/// Create function pointer type with inferred arguments.
#[doc(hidden)]
#[macro_export]
macro_rules! __fn_ptr {
    (
        @($($qualifiers:tt)*)
        $($(mut)? $($param:ident)? $(_)?: $param_ty:ty),* $(,)?
    ) => {
        $($qualifiers)* fn($($crate::__fn_ptr!(@__to_anonymous $param_ty)),*) -> _
    };
    (@__to_anonymous $param_ty:ty) => { _ }
}
