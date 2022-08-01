#[doc(hidden)]
#[macro_export]
macro_rules! __inner_declare_class {
    {@rewrite_methods @($($output:tt)*)} => {};
    {
        // Unsafe variant
        @rewrite_methods
        @($($output:tt)*)

        $(#[$($m:tt)*])*
        unsafe fn $name:ident($($args:tt)*) $(-> $ret:ty)? $body:block

        $($rest:tt)*
    } => {
        $crate::__rewrite_self_arg! {
            ($crate::__inner_declare_class)
            ($($args)*)

            // Split the function into parts, and send the arguments down to
            // be used later on
            @$($output)*
            @($(#[$($m)*])*)
            @(unsafe extern "C")
            @($name)
            @($($ret)?)
            @($body)
            // Will add @(kind)
            // Will add @(args)
        }

        $crate::__inner_declare_class! {
            @rewrite_methods
            @($($output)*)

            $($rest)*
        }
    };
    {
        // Safe variant
        @rewrite_methods
        @($($output:tt)*)

        $(#[$($m:tt)*])*
        fn $name:ident($($args:tt)*) $(-> $ret:ty)? $body:block

        $($rest:tt)*
    } => {
        $crate::__rewrite_self_arg! {
            ($crate::__inner_declare_class)
            ($($args)*)
            @$($output)*
            @($(#[$($m)*])*)
            @(extern "C")
            @($name)
            @($($ret)?)
            @($body)
        }

        $crate::__inner_declare_class! {
            @rewrite_methods
            @($($output)*)

            $($rest)*
        }
    };

    {
        @method_out
        @($(#[$($m:tt)*])*)
        @($($qualifiers:tt)*)
        @($name:ident)
        @($($ret:ty)?)
        @($($body:tt)*)
        @($($_kind:tt)*)
        @($($args:tt)*)
    } => {
        $crate::__attribute_helper! {
            @strip_sel
            $(@[$($m)*])*
            ($($qualifiers)* fn $name($($args)*) $(-> $ret)? $($body)*)
        }
    };

    {
        @register_out($builder:ident)
        @($(#[$($m:tt)*])*)
        @($($qualifiers:tt)*)
        @($name:ident)
        @($($_ret:tt)*)
        @($($_body:tt)*)
        @(class_method)
        @($($args:tt)*)
    } => {
        $builder.add_class_method(
            $crate::__attribute_helper! {
                @extract_sel
                ($crate::__inner_declare_class)
                ($(#[$($m)*])*)
                @call_sel
            },
            Self::$name as $crate::__fn_ptr! {
                @($($qualifiers)*) $($args)*
            },
        );
    };
    {
        @register_out($builder:ident)
        @($(#[$($m:tt)*])*)
        @($($qualifiers:tt)*)
        @($name:ident)
        @($($_ret:tt)*)
        @($($_body:tt)*)
        @(instance_method)
        @($($args:tt)*)
    } => {
        $builder.add_method(
            $crate::__attribute_helper! {
                @extract_sel
                ($crate::__inner_declare_class)
                ($(#[$($m)*])*)
                @call_sel
            },
            Self::$name as $crate::__fn_ptr! {
                @($($qualifiers)*) $($args)*
            },
        );
    };

    {
        @call_sel
        @($($sel:tt)*)
    } => {
        $crate::sel!($($sel)*)
    };
}

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
/// This macro consists of three parts; the class definition, the method
/// definition, and the protocol definition.
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
/// The desired selector can be specified using the `#[sel(my:selector:)]`
/// attribute.
///
/// A transformation step is performed on the functions (to make them have the
/// correct ABI) and hence they shouldn't really be called manually. (You
/// can't mark them as `pub` for the same reason). Instead, define a new
/// function that calls it via. [`msg_send!`].
///
/// ["associated functions"]: https://doc.rust-lang.org/reference/items/associated-items.html#methods
/// ["methods"]: https://doc.rust-lang.org/reference/items/associated-items.html#methods
/// [`msg_send!`]: crate::msg_send
///
///
/// ## Protocol definition
///
/// You can specify the protocols that the class should implement, along with
/// any required methods for said protocols.
///
/// The methods work exactly as normal, they're only put "under" the protocol
/// definition to make things easier to read.
///
///
/// # Safety
///
/// Using this macro requires writing a few `unsafe` markers:
///
/// `unsafe struct ...` has the following safety requirements:
/// - Same as [`extern_class!`] (the inheritance chain has to be correct).
/// - Any instance variables you specify must either be able to be created
///   using [`MaybeUninit::zeroed`], or be properly initialized in an `init`
///   method.
///
/// `unsafe impl { ... }` asserts that the types match those that are expected
/// when the method is invoked from Objective-C. Note that there are no
/// safe-guards here; you can easily write `i8`, but if Objective-C thinks
/// it's an `u32`, it will cause UB when called!
///
/// `unsafe impl protocol ... { ... }` requires that all required methods of
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
/// use objc2::rc::{Id, Owned};
/// use objc2::foundation::{NSCopying, NSObject, NSZone};
/// use objc2::runtime::Bool;
/// use objc2::{declare_class, msg_send, msg_send_bool, msg_send_id, ClassType};
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
///         #[sel(initWithFoo:)]
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
///         #[sel(foo)]
///         fn __get_foo(&self) -> u8 {
///             *self.foo
///         }
///
///         #[sel(myClassMethod)]
///         fn __my_class_method() -> Bool {
///             Bool::YES
///         }
///     }
///
///     unsafe impl protocol NSCopying {
///         #[sel(copyWithZone:)]
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
///         unsafe { msg_send_id![msg_send_id![cls, alloc], initWithFoo: foo].unwrap() }
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
///     // Public ivar
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
///     // Private ivar
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
/// // NSCopying
///
/// - (id)copyWithZone:(NSZone *)_zone {
///     MyCustomObject* obj = [[MyCustomObject alloc] initWithFoo: self->foo];
///     obj->bar = self->bar;
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
        unsafe $v:vis struct $name:ident: $inherits:ty $(, $inheritance_rest:ty)* {
            $($ivar_v:vis $ivar:ident: $ivar_ty:ty,)*
        }

        $(
            $(#[$impl_m:meta])*
            unsafe impl $(protocol $protocol:ident)? {
                $($methods:tt)*
            }
        )*
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
            unsafe $v struct $name<>: $inherits, $($inheritance_rest,)* $crate::runtime::Object {
                // SAFETY:
                // - The ivars are in a type used as an Objective-C object.
                // - The ivar is added to the class below.
                // - Rust prevents having two fields with the same name.
                // - Caller upholds that the ivars are properly initialized.
                $($ivar_v $ivar: $crate::declare::Ivar<$ivar>,)*
            }
        }

        // Creation
        unsafe impl $crate::ClassType for $name {
            type Superclass = $inherits;

            fn class() -> &'static $crate::runtime::Class {
                // TODO: Use `core::cell::LazyCell`
                use $crate::__macro_helpers::Once;

                static REGISTER_CLASS: Once = Once::new();

                REGISTER_CLASS.call_once(|| {
                    let superclass = <$inherits as $crate::ClassType>::class();
                    let err_str = concat!(
                        "could not create new class ",
                        stringify!($name),
                        ". Perhaps a class with that name already exists?",
                    );
                    let mut builder = $crate::declare::ClassBuilder::new(stringify!($name), superclass).expect(err_str);

                    // Ivars
                    $(
                        builder.add_static_ivar::<$ivar>();
                    )*

                    $(
                        // Implement protocol if any specified
                        $(
                            let err_str = concat!("could not find protocol ", stringify!($protocol));
                            builder.add_protocol($crate::runtime::Protocol::get(stringify!($protocol)).expect(err_str));
                        )?

                        // Implement methods
                        // SAFETY: Upheld by caller
                        unsafe {
                            $crate::__inner_declare_class! {
                                @rewrite_methods
                                @(register_out(builder))

                                $($methods)*
                            }
                        }
                    )*

                    let _cls = builder.register();
                });

                // We just registered the class, so it should be available
                $crate::runtime::Class::get(stringify!($name)).unwrap()
            }
        }

        // Methods
        $(
            $(#[$impl_m])*
            impl $name {
                $crate::__inner_declare_class! {
                    @rewrite_methods
                    @(method_out)

                    $($methods)*
                }
            }
        )*
    };
}
