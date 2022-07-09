#[doc(hidden)]
#[macro_export]
macro_rules! __inner_declare_class {
    {@rewrite_methods @$output_type:ident @$builder:ident} => {};
    {
        @rewrite_methods
        @$output_type:ident
        @$builder:ident

        $(#[$m:meta])*
        @sel($($sel:tt)+)
        fn $name:ident($($args:tt)*) $(-> $ret:ty)? $body:block

        $($rest:tt)*
    } => {
        $crate::__inner_declare_class! {
            @rewrite_methods_inner
            @$output_type
            @$builder
            // Split args out so that we can match on `self`, while still use
            // it as a function argument
            ($($args)*)

            $(#[$m])*
            @sel($($sel)+)
            fn $name($($args)*) $(-> $ret)? $body
        }

        $crate::__inner_declare_class! {
            @rewrite_methods
            @$output_type
            @$builder

            $($rest)*
        }
    };

    // Instance method
    {
        @rewrite_methods_inner
        @$output_type:ident
        @$builder:ident
        (&mut self $($__rest_args:tt)*)

        $(#[$m:meta])*
        @sel($($sel:tt)+)
        fn $name:ident(
            &mut $self:ident
            $(, $($rest_args:tt)*)?
        ) $(-> $ret:ty)? $body:block
    } => {
        $crate::__inner_declare_class! {
            @$output_type
            @instance_method
            @sel($($sel)*)
            @$name
            @$builder
            @($($($rest_args)*)?)

            $(#[$m])*
            extern "C" fn $name(
                &mut $self,
                _: $crate::objc2::runtime::Sel,
                $($($rest_args)*)?
            ) $(-> $ret)? $body
        }
    };
    {
        @rewrite_methods_inner
        @$output_type:ident
        @$builder:ident
        (&self $($__rest_args:tt)*)

        $(#[$m:meta])*
        @sel($($sel:tt)+)
        fn $name:ident(
            &$self:ident
            $(, $($rest_args:tt)*)?
        ) $(-> $ret:ty)? $body:block
    } => {
        $crate::__inner_declare_class! {
            @$output_type
            @instance_method
            @sel($($sel)*)
            @$name
            @$builder
            @($($($rest_args)*)?)

            $(#[$m])*
            extern "C" fn $name(
                &$self,
                _: $crate::objc2::runtime::Sel,
                $($($rest_args)*)?
            ) $(-> $ret)? $body
        }
    };
    {
        @rewrite_methods_inner
        @$output_type:ident
        @$builder:ident
        (
            mut self: $__self_ty:ty
            $(, $($__rest_args:tt)*)?
        )

        $(#[$m:meta])*
        @sel($($sel:tt)+)
        fn $name:ident(
            mut $self:ident: $self_ty:ty
            $(, $($rest_args:tt)*)?
        ) $(-> $ret:ty)? $body:block
    } => {
        $crate::__inner_declare_class! {
            @$output_type
            @instance_method
            @sel($($sel)*)
            @$name
            @$builder
            @($($($rest_args)*)?)

            $(#[$m])*
            extern "C" fn $name(
                mut $self: $self_ty,
                _: $crate::objc2::runtime::Sel,
                $($($rest_args)*)?
            ) $(-> $ret)? $body
        }
    };
    {
        @rewrite_methods_inner
        @$output_type:ident
        @$builder:ident
        (
            self: $__self_ty:ty
            $(, $($__rest_args:tt)*)?
        )

        $(#[$m:meta])*
        @sel($($sel:tt)+)
        fn $name:ident(
            $self:ident: $self_ty:ty
            $(, $($rest_args:tt)*)?
        ) $(-> $ret:ty)? $body:block
    } => {
        $crate::__inner_declare_class! {
            @$output_type
            @instance_method
            @sel($($sel)*)
            @$name
            @$builder
            @($($($rest_args)*)?)

            $(#[$m])*
            extern "C" fn $name(
                $self: $self_ty,
                _: $crate::objc2::runtime::Sel,
                $($($rest_args)*)?
            ) $(-> $ret)? $body
        }
    };

    // Class method
    {
        @rewrite_methods_inner
        @$output_type:ident
        @$builder:ident
        ($($__args:tt)*)

        $(#[$m:meta])*
        @sel($($sel:tt)+)
        fn $name:ident(
            $($args:tt)*
        ) $(-> $ret:ty)? $body:block
    } => {
        $crate::__inner_declare_class! {
            @$output_type
            @class_method
            @sel($($sel)*)
            @$name
            @$builder
            @($($args)*)

            $(#[$m])*
            extern "C" fn $name(
                _: &$crate::objc2::runtime::Class,
                _: $crate::objc2::runtime::Sel,
                $($args)*
            ) $(-> $ret)? $body
        }
    };

    {
        @method_out
        @$method_type:ident
        @sel($($sel:tt)*)
        @$name:ident
        @$builder:ident
        @($($builder_args:tt)*)

        $method:item
    } => {
        $method
    };
    {
        @register_out
        @class_method
        @sel($($sel:tt)*)
        @$name:ident
        @$builder:ident
        @($($builder_args:tt)*)

        $method:item
    } => {
        $builder.add_class_method(
            $crate::objc2::sel!($($sel)*),
            $crate::__inner_declare_class! {
                @cast_extern_fn
                @$name
                $($builder_args)*
            },
        );
    };
    {
        @register_out
        @instance_method
        @sel($($sel:tt)*)
        @$name:ident
        @$builder:ident
        @($($builder_args:tt)*)

        $method:item
    } => {
        $builder.add_method(
            $crate::objc2::sel!($($sel)*),
            $crate::__inner_declare_class! {
                @cast_extern_fn
                @$name
                $($builder_args)*
            },
        );
    };

    // Create the `as extern "C" fn(...) -> _` cast
    //
    // TODO: Investigate if there's a better way of doing this
    {
        @cast_extern_fn
        @$name:ident

        $(,)?
    } => {
        Self::$name as extern "C" fn(_, _) -> _
    };
    {
        @cast_extern_fn
        @$name:ident

        $param1:ident: $param1_ty:ty $(,)?
    } => {
        Self::$name as extern "C" fn(_, _, _) -> _
    };
    {
        @cast_extern_fn
        @$name:ident

        $param1:ident: $param1_ty:ty,
        $param2:ident: $param2_ty:ty $(,)?
    } => {
        Self::$name as extern "C" fn(_, _, _, _) -> _
    };
    {
        @cast_extern_fn
        @$name:ident

        $param1:ident: $param1_ty:ty,
        $param2:ident: $param2_ty:ty,
        $param3:ident: $param3_ty:ty $(,)?
    } => {
        Self::$name as extern "C" fn(_, _, _, _, _) -> _
    };
    {
        @cast_extern_fn
        @$name:ident

        $param1:ident: $param1_ty:ty,
        $param2:ident: $param2_ty:ty,
        $param3:ident: $param3_ty:ty,
        $param4:ident: $param4_ty:ty $(,)?
    } => {
        Self::$name as extern "C" fn(_, _, _, _, _, _) -> _
    };
    {
        @cast_extern_fn
        @$name:ident

        $param1:ident: $param1_ty:ty,
        $param2:ident: $param2_ty:ty,
        $param3:ident: $param3_ty:ty,
        $param4:ident: $param4_ty:ty,
        $param5:ident: $param5_ty:ty $(,)?
    } => {
        Self::$name as extern "C" fn(_, _, _, _, _, _, _) -> _
    };
    {
        @cast_extern_fn
        @$name:ident

        $param1:ident: $param1_ty:ty,
        $param2:ident: $param2_ty:ty,
        $param3:ident: $param3_ty:ty,
        $param4:ident: $param4_ty:ty,
        $param5:ident: $param5_ty:ty,
        $param6:ident: $param6_ty:ty $(,)?
    } => {
        Self::$name as extern "C" fn(_, _, _, _, _, _, _, _) -> _
    };
    {
        @cast_extern_fn
        @$name:ident

        $param1:ident: $param1_ty:ty,
        $param2:ident: $param2_ty:ty,
        $param3:ident: $param3_ty:ty,
        $param4:ident: $param4_ty:ty,
        $param5:ident: $param5_ty:ty,
        $param6:ident: $param6_ty:ty,
        $param7:ident: $param7_ty:ty $(,)?
    } => {
        Self::$name as extern "C" fn(_, _, _, _, _, _, _, _, _) -> _
    };
    {
        @cast_extern_fn
        @$name:ident

        $param1:ident: $param1_ty:ty,
        $param2:ident: $param2_ty:ty,
        $param3:ident: $param3_ty:ty,
        $param4:ident: $param4_ty:ty,
        $param5:ident: $param5_ty:ty,
        $param6:ident: $param6_ty:ty,
        $param7:ident: $param7_ty:ty,
        $param8:ident: $param8_ty:ty $(,)?
    } => {
        Self::$name as extern "C" fn(_, _, _, _, _, _, _, _, _, _) -> _
    };
    {
        @cast_extern_fn
        @$name:ident

        $param1:ident: $param1_ty:ty,
        $param2:ident: $param2_ty:ty,
        $param3:ident: $param3_ty:ty,
        $param4:ident: $param4_ty:ty,
        $param5:ident: $param5_ty:ty,
        $param6:ident: $param6_ty:ty,
        $param7:ident: $param7_ty:ty,
        $param8:ident: $param8_ty:ty,
        $param9:ident: $param9_ty:ty $(,)?
    } => {
        Self::$name as extern "C" fn(_, _, _, _, _, _, _, _, _, _, _) -> _
    };
    {
        @cast_extern_fn
        @$name:ident

        $param1:ident: $param1_ty:ty,
        $param2:ident: $param2_ty:ty,
        $param3:ident: $param3_ty:ty,
        $param4:ident: $param4_ty:ty,
        $param5:ident: $param5_ty:ty,
        $param6:ident: $param6_ty:ty,
        $param7:ident: $param7_ty:ty,
        $param8:ident: $param8_ty:ty,
        $param9:ident: $param9_ty:ty,
        $param10:ident: $param10_ty:ty $(,)?
    } => {
        Self::$name as extern "C" fn(_, _, _, _, _, _, _, _, _, _, _, _) -> _
    };
    {
        @cast_extern_fn
        @$name:ident

        $param1:ident: $param1_ty:ty,
        $param2:ident: $param2_ty:ty,
        $param3:ident: $param3_ty:ty,
        $param4:ident: $param4_ty:ty,
        $param5:ident: $param5_ty:ty,
        $param6:ident: $param6_ty:ty,
        $param7:ident: $param7_ty:ty,
        $param8:ident: $param8_ty:ty,
        $param9:ident: $param9_ty:ty,
        $param10:ident: $param10_ty:ty,
        $param11:ident: $param11_ty:ty $(,)?
    } => {
        Self::$name as extern "C" fn(_, _, _, _, _, _, _, _, _, _, _, _, _) -> _
    };
    {
        @cast_extern_fn
        @$name:ident

        $param1:ident: $param1_ty:ty,
        $param2:ident: $param2_ty:ty,
        $param3:ident: $param3_ty:ty,
        $param4:ident: $param4_ty:ty,
        $param5:ident: $param5_ty:ty,
        $param6:ident: $param6_ty:ty,
        $param7:ident: $param7_ty:ty,
        $param8:ident: $param8_ty:ty,
        $param9:ident: $param9_ty:ty,
        $param10:ident: $param10_ty:ty,
        $param11:ident: $param11_ty:ty,
        $param12:ident: $param12_ty:ty $(,)?
    } => {
        Self::$name as extern "C" fn(_, _, _, _, _, _, _, _, _, _, _, _, _, _) -> _
    };
}

/// Declare a new Objective-C class.
///
/// This is mostly just a convenience macro on top of [`extern_class!`] and
/// the functionality in the [`objc2::declare`] module, but it can really help
/// with cutting down on boilerplate, in particular when defining delegate
/// classes!
///
///
/// # Specification
///
/// This macro consists of three parts; the class definition, the method
/// definition, and the protocol definition.
///
///
/// ## Class and ivar definition
///
/// The class definition works a lot like [`extern_class!`], with the added
/// functionality that you can define custom instance variables on your class,
/// which are then wrapped in a [`objc2::runtime::Ivar`] and made accessible
/// through the class. (E.g. you can use `self.my_ivar` as if it was a normal
/// Rust struct).
///
/// Note that the class name should be unique across the entire application!
/// As a tip, you can declare the class with the desired unique name like
/// `MyCrateCustomObject` using this macro, and then expose a renamed type
/// alias like `pub type CustomObject = MyCrateCustomObject;` instead.
///
/// The class is guaranteed to have been created and registered with the
/// Objective-C runtime after the associated function `class` has been called.
///
///
/// ## Method definition
///
/// Within the `impl` block you can define two types of functions;
/// ["associated functions"] and ["methods"]. These are then mapped to the
/// Objective-C equivalents "class methods" and "instance methods". In
/// particular, if you use `self` your method will be registered as an
/// instance method, and if you don't it will be registered as a class method.
///
/// The desired selector can be specified using a special `@sel(my:selector:)`
/// directive directly before the function definition.
///
/// A transformation step is performed on the functions (to make them have the
/// correct ABI) and hence they shouldn't really be called manually. (You
/// can't mark them as `pub` for the same reason). Instead, define a new
/// function that calls it via. [`objc2::msg_send!`].
///
/// ["associated functions"]: https://doc.rust-lang.org/reference/items/associated-items.html#methods
/// ["methods"]: https://doc.rust-lang.org/reference/items/associated-items.html#methods
///
///
/// ## Protocol definition
///
/// You can specify the protocols that the class should implement, along with
/// any required methods for said protocols.
///
/// The methods work exactly as above, they're only put here to make things
/// easier to read.
///
///
/// # Safety
///
/// Using this macro requires writing a few `unsafe` markers:
///
/// The one for the class definition has the following safety requirements:
/// - Same as [`extern_class!`] (the inheritance chain has to be correct).
/// - Any instance variables you specify must either be able to be created
///   using [`MaybeUninit::zeroed`], or be properly initialized in an `init`
///   method.
///
/// The one for the method implementation asserts that the types match those
/// that are expected when the method is invoked from Objective-C. Note that
/// there are no safe-guards here; you can easily write `i8`, but if
/// Objective-C thinks it's an `u32`, it will cause UB when called!
///
/// The one for the protocol definition requires that all required methods of
/// the specified protocol is implemented, and that any extra requirements
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
/// use objc2::{msg_send, msg_send_bool, msg_send_id};
/// use objc2::rc::{Id, Owned};
/// use objc2::runtime::Bool;
/// use objc2_foundation::{declare_class, NSCopying, NSObject, NSZone};
/// #
/// # #[cfg(feature = "gnustep-1-7")]
/// # unsafe { objc2::__gnustep_hack::get_class_to_force_linkage() };
///
/// declare_class! {
///     unsafe struct MyCustomObject: NSObject {
///         foo: u8,
///         pub bar: c_int,
///     }
///
///     unsafe impl {
///         @sel(initWithFoo:)
///         fn init_with(&mut self, foo: u8) -> Option<&mut Self> {
///             let this: Option<&mut Self> = unsafe {
///                 msg_send![super(self, NSObject::class()), init]
///             };
///             this.map(|this| {
///                 // TODO: Initialization through MaybeUninit
///                 // (The below is only safe because these variables are
///                 // safe to initialize with `MaybeUninit::zeroed`).
///                 *this.foo = foo;
///                 *this.bar = 42;
///                 this
///             })
///         }
///
///         @sel(foo)
///         fn __get_foo(&self) -> u8 {
///             *self.foo
///         }
///
///         @sel(myClassMethod)
///         fn __my_class_method() -> Bool {
///             Bool::YES
///         }
///     }
///
///     unsafe impl protocol NSCopying {
///         @sel(copyWithZone:)
///         fn copy_with_zone(&self, _zone: *const NSZone) -> *mut Self {
///             let mut obj = Self::new(*self.foo);
///             *obj.bar = *self.bar;
///             obj.autorelease_return()
///         }
///     }
/// }
///
/// impl MyCustomObject {
///     pub fn new(foo: u8) -> Id<Self, Owned> {
///         let cls = Self::class();
///         unsafe { msg_send_id![msg_send_id![cls, alloc], initWithFoo: foo,].unwrap() }
///     }
///
///     pub fn get_foo(&self) -> u8 {
///         unsafe { msg_send![self, foo] }
///     }
///
///     pub fn my_class_method() -> bool {
///         unsafe { msg_send_bool![Self::class(), myClassMethod] }
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
///
///     let obj = obj.copy();
///     assert_eq!(obj.get_foo(), 3);
///
///     assert!(MyCustomObject::my_class_method());
/// }
/// ```
///
/// Approximately equivalent to the following Objective-C code.
///
/// ```text
/// #import <Foundation/Foundation.h>
///
/// @interface MyCustomObject: NSObject <NSCopying> {
///     int bar;
/// }
///
/// - (instancetype)initWithFoo:(uint8_t)foo;
/// - (uint8_t)foo;
/// + (BOOL)myClassMethod;
///
/// @end
///
///
/// @implementation MyCustomObject {
///     // Private
///     uint8_t foo;
/// }
///
/// - (instancetype)initWithFoo:(uint8_t)foo_arg {
///     self = [super init];
///     if (self) {
///         self->foo = foo_arg;
///         self->bar = 42;
///     }
///     return self;
/// }
///
/// - (uint8_t)foo {
///     return self->foo; // Or just `foo`
/// }
///
/// + (BOOL)myClassMethod {
///     return YES;
/// }
///
/// - (id)copyWithZone:(NSZone *)_zone {
///     MyCustomObject* obj = [[MyCustomObject alloc] initWithFoo: self->foo];
///     obj->bar = self->bar;
///     return obj;
/// }
///
/// @end
/// ```
#[macro_export]
macro_rules! declare_class {
    {
        $(#[$m:meta])*
        unsafe $v:vis struct $name:ident: $inherits:ty $(, $inheritance_rest:ty)* {
            $($ivar_v:vis $ivar:ident: $ivar_ty:ty,)*
        }

        $(#[$impl_m:meta])*
        unsafe impl {
            $($methods:tt)*
        }

        $(
            $(#[$impl_protocol_m:meta])*
            unsafe impl protocol $protocols:ident {
                $($protocol_methods:tt)*
            }
        ),*
    } => {
        $(
            #[allow(non_camel_case_types)]
            $ivar_v struct $ivar {
                __priv: (),
            }

            unsafe impl $crate::objc2::declare::IvarType for $ivar {
                type Type = $ivar_ty;
                const NAME: &'static str = stringify!($ivar);
            }
        )*

        $crate::__inner_extern_class! {
            @__inner
            $(#[$m])*
            // SAFETY: Upheld by caller
            unsafe $v struct $name<>: $inherits, $($inheritance_rest,)* $crate::objc2::runtime::Object {
                // SAFETY:
                // - The ivars are in a type used as an Objective-C object
                // - The instance variable is defined in the exact same manner
                //   in `create_class`.
                // - Rust prevents having two fields with the same name.
                $($ivar_v $ivar: $crate::objc2::declare::Ivar<$ivar>,)*
            }
        }

        // Creation
        impl $name {
            fn class() -> &'static $crate::objc2::runtime::Class {
                // TODO: Use `core::cell::LazyCell`
                use $crate::__std::sync::Once;

                use $crate::objc2::declare::ClassBuilder;
                use $crate::objc2::runtime::Protocol;
                static REGISTER_CLASS: Once = Once::new();

                REGISTER_CLASS.call_once(|| {
                    let superclass = <$inherits>::class();
                    let mut builder = ClassBuilder::new(stringify!($name), superclass).unwrap();

                    $(
                        builder.add_ivar::<<$ivar as $crate::objc2::declare::IvarType>::Type>(
                            <$ivar as $crate::objc2::declare::IvarType>::NAME
                        );
                    )*

                    // SAFETY: Upheld by caller
                    unsafe {
                        $crate::__inner_declare_class! {
                            @rewrite_methods
                            @register_out
                            @builder

                            $($methods)*
                        }
                    }

                    // Implement protocols
                    $(
                        builder.add_protocol(Protocol::get(stringify!($protocols)).unwrap());

                        // SAFETY: Upheld by caller
                        unsafe {
                            $crate::__inner_declare_class! {
                                @rewrite_methods
                                @register_out
                                @builder

                                $($protocol_methods)*
                            }
                        }
                    )*

                    let _cls = builder.register();
                });

                $crate::objc2::class!($name)
            }
        }

        // Methods
        $(#[$impl_m])*
        impl $name {
            $crate::__inner_declare_class! {
                @rewrite_methods
                @method_out
                @__builder

                $($methods)*
            }
        }

        // Protocol methods
        $(
            $(#[$impl_protocol_m])*
            impl $name {
                $crate::__inner_declare_class! {
                    @rewrite_methods
                    @method_out
                    @__builder

                    $($protocol_methods)*
                }
            }
        )*
    };
}
