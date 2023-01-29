/// Declare a new Objective-C class.
///
/// This is mostly just a convenience macro on top of [`extern_class!`] and
/// the functionality in the [`declare`] module, but it can really help
/// with cutting down on boilerplate, in particular when defining delegate
/// classes!
///
/// [`extern_class!`]: crate::extern_class
/// [`declare`]: crate::declare
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
/// which are then wrapped in a [`declare::Ivar`] with the given name, and
/// made accessible through the class. (E.g. you can use `self.my_ivar` as if
/// it was a normal Rust struct).
///
/// The instance variable names are specified as such:
/// - [`IvarEncode<T, "my_crate_ivar">`](crate::declare::IvarEncode)
/// - [`IvarBool<"my_crate_ivar">`](crate::declare::IvarBool)
/// - [`IvarDrop<T, "my_crate_ivar">`](crate::declare::IvarDrop)
///
/// This is special syntax that will be used to generate helper types that
/// implement [`declare::IvarType`], which is then used inside the new struct.
///
/// Instance variable names must be unique, and must not conflict with any
/// superclass' instance variables - this means is is good practice to name
/// them with a prefix of your crate name, or similar.
///
/// The class name must be specified in `ClassType::NAME`, and it must be
/// unique across the entire application. Good practice here is similarly to
/// include your crate name in the prefix.
///
/// The class is guaranteed to have been created and registered with the
/// Objective-C runtime after the [`ClassType::class`] function has been
/// called.
///
/// The macro will generate a `dealloc` method for you, which will call any
/// `Drop` impl the class may have.
///
/// [`declare::Ivar`]: crate::declare::Ivar
/// [`declare::IvarType`]: crate::declare::IvarType
/// [`ClassType::class`]: crate::ClassType::class
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
/// or `#[method_id(my:selector:)]` attribute, similar to the
/// [`extern_methods!`] macro.
///
/// If the `#[method_id(...)]` attribute is used, the return type must be
/// `Option<Id<T, O>>` or `Id<T, O>`. Additionally, if the selector is in the
/// "init"-family, the "self"/"this" argument must be `Allocated<Self>`.
///
/// Putting other attributes on the method such as `cfg`, `allow`, `doc`,
/// `deprecated` and so on is supported. However, note that `cfg_attr` may not
/// work correctly, due to implementation difficulty - if you have a concrete
/// use-case, please [open an issue], then we can discuss it.
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
/// [open an issue]: https://github.com/madsmtm/objc2/issues/new
/// [`msg_send!`]: crate::msg_send
/// [`runtime::Bool`]: crate::runtime::Bool
///
///
/// ## Protocol definitions
///
/// You can specify protocols that the class should implement, along with any
/// required/optional methods for said protocols.
///
/// The protocol must have been previously defined with [`extern_protocol!`].
///
/// The methods work exactly as normal, they're only put "under" the protocol
/// definition to make things easier to read.
///
/// Putting attributes on the `impl` item such as `cfg`, `allow`, `doc`,
/// `deprecated` and so on is supported.
///
/// [`extern_protocol!`]: crate::extern_protocol
///
///
/// # Panics
///
/// The implemented `ClassType::class` method may panic in a few cases, such
/// as if:
/// - A class with the specified name already exists.
/// - One of the class' instance variables already exist on a superclass.
/// - Debug assertions are enabled, and an overriden method's signature is not
///   equal to the one on the superclass.
/// - The `verify` feature and debug assertions are enabled, and the required
///   protocol methods are not implemented.
/// - And possibly more similar cases.
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
/// `unsafe impl P for T { ... }` requires that all required methods of the
/// specified protocol is implemented, and that any extra requirements
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
/// use objc2::declare::{Ivar, IvarDrop, IvarEncode};
/// use objc2::rc::{Id, Owned, Shared};
/// use objc2::runtime::{NSObject, NSObjectProtocol, NSZone};
/// use objc2::{
///     declare_class, extern_protocol, msg_send, msg_send_id, ClassType,
///     ProtocolType,
/// };
///
/// // Declare the NSCopying protocol so that we can implement it.
/// //
/// // In practice, you wouldn't have to do this, since it is done for you in
/// // `icrate`.
/// extern_protocol!(
///     unsafe trait NSCopying {
///         #[method(copyWithZone:)]
///         fn copy_with_zone(&self, _zone: *const NSZone) -> *mut Self;
///     }
///
///     unsafe impl ProtocolType for dyn NSCopying {
///         const NAME: &'static str = "NSCopying";
///     }
/// );
///
///
/// declare_class!(
///     struct MyCustomObject {
///         foo: IvarEncode<u8, "_foo">,
///         pub bar: IvarEncode<c_int, "_bar">,
///         object: IvarDrop<Id<NSObject, Shared>, "_object">,
///     }
///
///     mod ivars;
///
///     unsafe impl ClassType for MyCustomObject {
///         type Super = NSObject;
///         const NAME: &'static str = "MyCustomObject";
///     }
///
///     unsafe impl MyCustomObject {
///         #[method(initWithFoo:)]
///         fn init_with(this: &mut Self, foo: u8) -> Option<&mut Self> {
///             let this: Option<&mut Self> = unsafe {
///                 msg_send![super(this), init]
///             };
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
///                 Ivar::write(&mut this.object, Id::into_shared(NSObject::new()));
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
///         #[method_id(object)]
///         fn __get_object(&self) -> Id<NSObject, Shared> {
///             self.object.clone()
///         }
///
///         #[method(myClassMethod)]
///         fn __my_class_method() -> bool {
///             true
///         }
///     }
///
///     unsafe impl NSCopying for MyCustomObject {
///         #[method(copyWithZone:)]
///         fn copy_with_zone(&self, _zone: *const NSZone) -> *mut Self {
///             let mut obj = Self::new(*self.foo);
///             *obj.bar = *self.bar;
///             obj.autorelease_return()
///         }
///
///         // If we have tried to add other methods here, or had forgotten
///         // to implement the method, we would have gotten an error with the
///         // `verify` feature enabled.
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
///     pub fn get_object(&self) -> Id<NSObject, Shared> {
///         unsafe { msg_send_id![self, object] }
///     }
///
///     pub fn my_class_method() -> bool {
///         unsafe { msg_send![Self::class(), myClassMethod] }
///     }
/// }
///
/// // TODO: `NSCopying` from `icrate` works a bit differently
/// // unsafe impl icrate::Foundation::NSCopying for MyCustomObject {
/// //     type Ownership = Owned;
/// //     type Output = Self;
/// // }
///
/// fn main() {
///     let obj = MyCustomObject::new(3);
///     assert_eq!(*obj.foo, 3);
///     assert_eq!(*obj.bar, 42);
///     assert!(obj.object.is_kind_of::<NSObject>());
///
///     let obj: Id<MyCustomObject, Shared> = unsafe {
///          msg_send_id![&obj, copy]
///     }; // Or obj.copy() with `icrate`
///
///     assert_eq!(obj.get_foo(), 3);
///     assert!(obj.get_object().is_kind_of::<NSObject>());
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
/// - (NSObject*)object;
/// + (BOOL)myClassMethod;
///
/// @end
///
///
/// @implementation MyCustomObject {
///     // Private ivar
///     uint8_t foo;
///     NSObject* _Nonnull object;
/// }
///
/// - (instancetype)initWithFoo:(uint8_t)foo_arg {
///     self = [super init];
///     if (self) {
///         self->foo = foo_arg;
///         self->bar = 42;
///         self->object = [NSObject new];
///     }
///     return self;
/// }
///
/// - (uint8_t)foo {
///     return self->foo; // Or just `foo`
/// }
///
/// - (NSObject*)object {
///     return self->object;
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
#[doc(alias = "@interface")]
#[doc(alias = "@implementation")]
#[macro_export]
macro_rules! declare_class {
    // With ivar helper
    {
        $(#[$m:meta])*
        $v:vis struct $name:ident {
            $($fields:tt)*
        }

        $ivar_helper_module_v:vis mod $ivar_helper_module:ident;

        unsafe impl ClassType for $for:ty {
            $(#[inherits($($inheritance_rest:ty),+)])?
            type Super = $superclass:ty;

            const NAME: &'static str = $name_const:literal;
        }

        $($methods:tt)*
    } => {
        $crate::__emit_struct_and_ivars! {
            ($(#[$m])*)
            ($v)
            ($name)
            ($ivar_helper_module_v mod $ivar_helper_module)
            ($($fields)*)
            (
                // Superclasses are deallocated by calling `[super dealloc]`.
                __inner: $crate::__macro_helpers::ManuallyDrop<$superclass>,
            )
        }

        $crate::__inner_declare_class! {
            ($ivar_helper_module)

            unsafe impl ClassType for $for {
                $(#[inherits($($inheritance_rest),+)])?
                type Super = $superclass;

                const NAME: &'static str = $name_const;
            }

            $($methods)*
        }
    };

    // No ivar helper
    {
        $(#[$m:meta])*
        $v:vis struct $name:ident {
            $($fields:tt)*
        }

        unsafe impl ClassType for $for:ty {
            $(#[inherits($($inheritance_rest:ty),+)])?
            type Super = $superclass:ty;

            const NAME: &'static str = $name_const:literal;
        }

        $($methods:tt)*
    } => {
        $crate::__emit_struct_and_ivars! {
            ($(#[$m])*)
            ($v)
            ($name)
            ()
            ($($fields)*)
            (
                // Superclasses are deallocated by calling `[super dealloc]`.
                __inner: $crate::__macro_helpers::ManuallyDrop<$superclass>,
            )
        }

        $crate::__inner_declare_class! {
            ()

            unsafe impl ClassType for $for {
                $(#[inherits($($inheritance_rest),+)])?
                type Super = $superclass;

                const NAME: &'static str = $name_const;
            }

            $($methods)*
        }
    };

    // Allow declaring class with no instance variables
    {
        $(#[$m:meta])*
        $v:vis struct $name:ident;

        unsafe impl ClassType for $for:ty {
            $(#[inherits($($inheritance_rest:ty),+)])?
            type Super = $superclass:ty;

            const NAME: &'static str = $name_const:literal;
        }

        $($methods:tt)*
    } => {
        $crate::__emit_struct_and_ivars! {
            ($(#[$m])*)
            ($v)
            ($name)
            ()
            ()
            (
                // Superclasses are deallocated by calling `[super dealloc]`.
                __inner: $crate::__macro_helpers::ManuallyDrop<$superclass>,
            )
        }

        $crate::__inner_declare_class! {
            ()

            unsafe impl ClassType for $for {
                $(#[inherits($($inheritance_rest),+)])?
                type Super = $superclass;

                const NAME: &'static str = $name_const;
            }

            $($methods)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __inner_declare_class {
    {
        ($($ivar_helper_module:ident)?)

        unsafe impl ClassType for $for:ty {
            $(#[inherits($($inheritance_rest:ty),+)])?
            type Super = $superclass:ty;

            const NAME: &'static str = $name_const:literal;
        }

        $($methods:tt)*
    } => {
        $crate::__extern_class_impl_traits! {
            // SAFETY: Upheld by caller
            unsafe impl () for $for {
                INHERITS = [$superclass, $($($inheritance_rest,)+)? $crate::runtime::Object];
            }
        }

        // Creation
        unsafe impl ClassType for $for {
            type Super = $superclass;
            const NAME: &'static $crate::__macro_helpers::str = $name_const;

            fn class() -> &'static $crate::runtime::Class {
                // TODO: Use `core::cell::LazyCell`
                static REGISTER_CLASS: $crate::__macro_helpers::Once = $crate::__macro_helpers::Once::new();

                REGISTER_CLASS.call_once(|| {
                    let __objc2_superclass = <$superclass as $crate::ClassType>::class();
                    let mut __objc2_builder = $crate::declare::ClassBuilder::new(
                        <Self as ClassType>::NAME,
                        __objc2_superclass,
                    ).unwrap_or_else(|| {
                        $crate::__macro_helpers::panic!(
                            "could not create new class {}. Perhaps a class with that name already exists?",
                            <Self as ClassType>::NAME,
                        )
                    });

                    $($ivar_helper_module::__objc2_declare_ivars(&mut __objc2_builder);)?

                    // See the following links for more details:
                    // - <https://clang.llvm.org/docs/AutomaticReferenceCounting.html#dealloc>
                    // - <https://developer.apple.com/documentation/objectivec/nsobject/1571947-dealloc>
                    // - <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/MemoryMgmt/Articles/mmRules.html#//apple_ref/doc/uid/20000994-SW2>
                    unsafe extern "C" fn __objc2_dealloc(__objc2_self: *mut $for, __objc2_cmd: $crate::runtime::Sel) {
                        // SAFETY: Ivars are explicitly designed to always
                        // be valid to drop, and since this is the
                        // `dealloc` method, we know the ivars are never
                        // going to be touched again.
                        //
                        // This also runs any `Drop` impl that the type may
                        // have.
                        unsafe { $crate::__macro_helpers::drop_in_place(__objc2_self) };

                        // The superclass' "marker" that this stores is
                        // wrapped in `ManuallyDrop`, instead we drop it by
                        // calling the superclass' `dealloc` method.
                        //
                        // Note: ARC does this automatically, which means
                        // most Objective-C code in the wild don't contain
                        // this; but we _are_ ARC, so we must do this.
                        unsafe {
                            $crate::MessageReceiver::__send_super_message_static(
                                __objc2_self,
                                __objc2_cmd, // Reuse the selector
                                (), // No arguments
                            )
                        }
                    }

                    if $crate::__macro_helpers::needs_drop::<Self>() {
                        unsafe {
                            __objc2_builder.add_method(
                                $crate::sel!(dealloc),
                                __objc2_dealloc as unsafe extern "C" fn(_, _),
                            );
                        }
                    }

                    // Implement protocols and methods
                    $crate::__declare_class_register_methods! {
                        (__objc2_builder)
                        $($methods)*
                    }

                    let _cls = __objc2_builder.register();
                });

                // We just registered the class, so it should be available
                $crate::runtime::Class::get(<Self as ClassType>::NAME).unwrap()
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

        // Methods
        $crate::__declare_class_methods! {
            $($methods)*
        }
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
    // Base-case
    () => {};
    // With protocol
    (
        $(#[$m:meta])*
        unsafe impl $protocol:ident for $for:ty {
            $($methods:tt)*
        }

        $($rest:tt)*
    ) => {
        // SAFETY: Upheld by caller
        $(#[$m])*
        unsafe impl $protocol for $for {}

        $(#[$m])*
        impl $for {
            $crate::__declare_class_rewrite_methods! {
                ($crate::__declare_class_method_out)
                ()

                $($methods)*
            }
        }

        $crate::__declare_class_methods!{
            $($rest)*
        }
    };
    // Without protocol
    (
        $(#[$m:meta])*
        unsafe impl $for:ty {
            $($methods:tt)*
        }

        $($rest:tt)*
    ) => {
        $(#[$m])*
        impl $for {
            $crate::__declare_class_rewrite_methods! {
                ($crate::__declare_class_method_out)
                ()

                $($methods)*
            }
        }

        $crate::__declare_class_methods! {
            $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_class_register_methods {
    // Base-case
    (
        ($builder:ident)
    ) => {};

    // With protocol
    (
        ($builder:ident)

        $(#[$($m:tt)*])*
        unsafe impl $protocol:ident for $for:ty {
            $($methods:tt)*
        }

        $($rest:tt)*
    ) => {
        $crate::__extract_and_apply_cfg_attributes! {
            @($(#[$($m)*])*)
            @(
                // Implement protocol
                #[allow(unused_mut)]
                let mut __objc2_protocol_builder = $builder.__add_protocol_methods(
                    <dyn $protocol as $crate::ProtocolType>::protocol()
                );

                // In case the user's function is marked `deprecated`
                #[allow(deprecated)]
                // In case the user did not specify any methods
                #[allow(unused_unsafe)]
                // SAFETY: Upheld by caller
                unsafe {
                    $crate::__declare_class_rewrite_methods! {
                        ($crate::__declare_class_register_out)
                        (__objc2_protocol_builder)

                        $($methods)*
                    }
                }

                // Finished declaring protocol; get error message if any
                __objc2_protocol_builder.__finish();
            )
        }

        $crate::__declare_class_register_methods! {
            ($builder)
            $($rest)*
        }
    };

    // Without protocol
    (
        ($builder:ident)

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
                        ($crate::__declare_class_register_out)
                        ($builder)

                        $($methods)*
                    }
                }
            )
        }

        $crate::__declare_class_register_methods! {
            ($builder)
            $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_class_rewrite_methods {
    {
        ($out_macro:path)
        ($($macro_arg:tt)*)
    } => {};

    // Unsafe variant
    {
        ($out_macro:path)
        ($($macro_arg:tt)*)

        $(#[$($m:tt)*])*
        unsafe fn $name:ident($($args:tt)*) $(-> $ret:ty)? $body:block

        $($rest:tt)*
    } => {
        $crate::__rewrite_self_arg! {
            ($($args)*)

            ($crate::__extract_custom_attributes)
            ($(#[$($m)*])*)
            ($name)

            ($out_macro)
            ($($macro_arg)*)
            (unsafe)
            ($name)
            ($($ret)?)
            ($body)
        }

        $crate::__declare_class_rewrite_methods! {
            ($out_macro)
            ($($macro_arg)*)

            $($rest)*
        }
    };

    // Safe variant
    {
        ($out_macro:path)
        ($($macro_arg:tt)*)

        $(#[$($m:tt)*])*
        fn $name:ident($($args:tt)*) $(-> $ret:ty)? $body:block

        $($rest:tt)*
    } => {
        $crate::__rewrite_self_arg! {
            ($($args)*)

            ($crate::__extract_custom_attributes)
            ($(#[$($m)*])*)
            ($name)

            ($out_macro)
            ($($macro_arg)*)
            ()
            ($name)
            ($($ret)?)
            ($body)
        }

        $crate::__declare_class_rewrite_methods! {
            ($out_macro)
            ($($macro_arg)*)

            $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_class_method_out {
    {
        ()
        ($($qualifiers:tt)*)
        ($name:ident)
        ($($ret:ty)?)
        ($body:block)

        ($builder_method:ident)
        ($receiver:expr)
        ($receiver_ty:ty)
        ($($args_prefix:tt)*)
        ($($args_rest:tt)*)

        ($($m_method:tt)*)
        ($($m_optional:tt)*)
        ($($m_checked:tt)*)
    } => {
        $crate::__declare_class_rewrite_args! {
            ($($args_rest)*)
            ()
            ()

            ($crate::__declare_class_method_out_inner)

            ($($qualifiers)*)
            ($name)
            ($($ret)?)
            ($body)

            ($builder_method)
            ($receiver)
            ($receiver_ty)
            ($($args_prefix)*)

            ($($m_method)*)
            ($($m_optional)*)
            ($($m_checked)*)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_class_rewrite_args {
    // Convert _
    {
        (_ : $param_ty:ty $(, $($rest_args:tt)*)?)
        ($($args_converted:tt)*)
        ($($body_prefix:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__declare_class_rewrite_args! {
            ($($($rest_args)*)?)
            ($($args_converted)* _ : <$param_ty as $crate::encode::EncodeConvert>::__Inner,)
            ($($body_prefix)*)

            ($out_macro)
            $($macro_args)*
        }
    };
    // Convert mut
    {
        (mut $param:ident : $param_ty:ty $(, $($rest_args:tt)*)?)
        ($($args_converted:tt)*)
        ($($body_prefix:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__declare_class_rewrite_args! {
            ($($($rest_args)*)?)
            ($($args_converted)* $param : <$param_ty as $crate::encode::EncodeConvert>::__Inner,)
            (
                $($body_prefix)*
                let mut $param = <$param_ty as $crate::encode::EncodeConvert>::__from_inner($param);
            )

            ($out_macro)
            $($macro_args)*
        }
    };
    // Convert
    {
        ($param:ident : $param_ty:ty $(, $($rest_args:tt)*)?)
        ($($args_converted:tt)*)
        ($($body_prefix:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__declare_class_rewrite_args! {
            ($($($rest_args)*)?)
            ($($args_converted)* $param : <$param_ty as $crate::encode::EncodeConvert>::__Inner,)
            (
                $($body_prefix)*
                let $param = <$param_ty as $crate::encode::EncodeConvert>::__from_inner($param);
            )

            ($out_macro)
            $($macro_args)*
        }
    };
    // Output result
    {
        ()
        ($($args_converted:tt)*)
        ($($body_prefix:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            ($($args_converted)*)
            ($($body_prefix)*)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_class_method_out_inner {
    // #[method(...)]
    {
        ($($qualifiers:tt)*)
        ($name:ident)
        ($($ret:ty)?)
        ($body:block)

        ($__builder_method:ident)
        ($__receiver:expr)
        ($__receiver_ty:ty)
        ($($args_prefix:tt)*)

        (#[method($($__sel:tt)*)])
        ($($__m_optional:tt)*)
        ($($m_checked:tt)*)

        ($($args_converted:tt)*)
        ($($body_prefix:tt)*)
    } => {
        $($m_checked)*
        $($qualifiers)* extern "C" fn $name(
            $($args_prefix)*
            $($args_converted)*
        ) $(-> <$ret as $crate::encode::EncodeConvert>::__Inner)? {
            $($body_prefix)*
            $crate::__convert_result! {
                $body $(; $ret)?
            }
        }
    };

    // #[method_id(...)]
    {
        ($($qualifiers:tt)*)
        ($name:ident)
        ($ret:ty)
        ($body:block)

        ($__builder_method:ident)
        ($__receiver:expr)
        ($receiver_ty:ty)
        ($($args_prefix:tt)*)

        (#[method_id($($sel:tt)*)])
        ($($__m_optional:tt)*)
        ($($m_checked:tt)*)

        ($($args_converted:tt)*)
        ($($body_prefix:tt)*)
    } => {
        $($m_checked)*
        $($qualifiers)* extern "C" fn $name(
            $($args_prefix)*
            $($args_converted)*
        ) -> $crate::declare::__IdReturnValue {
            $($body_prefix)*

            let __objc2_result = $body;

            #[allow(unreachable_code)]
            <$crate::__macro_helpers::RetainSemantics<{
                $crate::__macro_helpers::retain_semantics(
                    $crate::__sel_helper! {
                        @()
                        $($sel)*
                    }
                )
            }> as $crate::__macro_helpers::MessageRecieveId<
                $receiver_ty,
                $ret,
            >>::into_return(__objc2_result)
        }
    };

    {
        ($($qualifiers:tt)*)
        ($name:ident)
        ()
        ($body:block)

        ($__builder_method:ident)
        ($__receiver:expr)
        ($__receiver_ty:ty)
        ($($args_prefix:tt)*)

        (#[method_id($($sel:tt)*)])
        ($($__m_optional:tt)*)
        ($($m_checked:tt)*)

        ($($args_converted:tt)*)
        ($($body_prefix:tt)*)
    } => {
        $($m_checked)*
        $($qualifiers)* extern "C" fn $name() {
            compile_error!("`#[method_id(...)]` must have a return type")
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __convert_result {
    ($body:block) => {
        $body
    };
    ($body:block; $ret:ty) => {
        let __objc2_result = $body;
        #[allow(unreachable_code)]
        <$ret as $crate::encode::EncodeConvert>::__into_inner(__objc2_result)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_class_register_out {
    // #[method(dealloc)]
    {
        ($builder:ident)
        ($($qualifiers:tt)*)
        ($name:ident)
        ($($__ret:ty)?)
        ($__body:block)

        ($builder_method:ident)
        ($__receiver:expr)
        ($__receiver_ty:ty)
        ($($__args_prefix:tt)*)
        ($($args_rest:tt)*)

        (#[method(dealloc)])
        () // No optional
        ($($m_checked:tt)*)
    } => {
        $crate::__extract_and_apply_cfg_attributes! {
            @($($m_checked)*)
            @($crate::__macro_helpers::compile_error!(
                "`#[method(dealloc)]` is not supported. Implement `Drop` for the type instead"
            ))
        }
    };

    // #[method(...)]
    {
        ($builder:ident)
        ($($qualifiers:tt)*)
        ($name:ident)
        ($($__ret:ty)?)
        ($__body:block)

        ($builder_method:ident)
        ($__receiver:expr)
        ($__receiver_ty:ty)
        ($($__args_prefix:tt)*)
        ($($args_rest:tt)*)

        (#[method($($sel:tt)*)])
        () // No optional
        ($($m_checked:tt)*)
    } => {
        $crate::__extract_and_apply_cfg_attributes! {
            @($($m_checked)*)
            @(
                $builder.$builder_method(
                    $crate::sel!($($sel)*),
                    Self::$name as $crate::__fn_ptr! {
                        ($($qualifiers)*)
                        (_, _,)
                        $($args_rest)*
                    },
                );
            )
        }
    };

    // #[method_id(...)]
    {
        ($builder:ident)
        ($($qualifiers:tt)*)
        ($name:ident)
        ($($__ret:ty)?)
        ($__body:block)

        ($builder_method:ident)
        ($__receiver:expr)
        ($__receiver_ty:ty)
        ($($__args_prefix:tt)*)
        ($($args_rest:tt)*)

        (#[method_id($($sel:tt)*)])
        () // No optional
        ($($m_checked:tt)*)
    } => {
        $crate::__extract_and_apply_cfg_attributes! {
            @($($m_checked)*)
            @(
                $builder.$builder_method(
                    $crate::__get_method_id_sel!($($sel)*),
                    Self::$name as $crate::__fn_ptr! {
                        ($($qualifiers)*)
                        (_, _,)
                        $($args_rest)*
                    },
                );
            )
        }
    };

    // #[optional]
    {
        ($builder:ident)
        ($($qualifiers:tt)*)
        ($name:ident)
        ($($__ret:ty)?)
        ($__body:block)

        ($builder_method:ident)
        ($__receiver:expr)
        ($__receiver_ty:ty)
        ($($__args_prefix:tt)*)
        ($($args_rest:tt)*)

        ($($m_method:tt)*)
        ($($m_optional:tt)*)
        ($($m_checked:tt)*)
    } => {
        compile_error!("`#[optional]` is only supported in `extern_protocol!`")
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __get_method_id_sel {
    (alloc) => {
        compile_error!(concat!(
            "`#[method_id(alloc)]` is not supported. ",
            "Use `#[method(alloc)]` and do the memory management yourself",
        ))
    };
    (retain) => {
        compile_error!(concat!(
            "`#[method_id(retain)]` is not supported. ",
            "Use `#[method(retain)]` and do the memory management yourself",
        ))
    };
    (release) => {
        compile_error!(concat!(
            "`#[method_id(release)]` is not supported. ",
            "Use `#[method(release)]` and do the memory management yourself",
        ))
    };
    (autorelease) => {
        compile_error!(concat!(
            "`#[method_id(autorelease)]` is not supported. ",
            "Use `#[method(autorelease)]` and do the memory management yourself",
        ))
    };
    (dealloc) => {
        compile_error!(concat!(
            "`#[method_id(dealloc)]` is not supported. ",
            "Add an instance variable with a `Drop` impl to the class instead",
        ))
    };
    ($($t:tt)*) => {
        $crate::sel!($($t)*)
    };
}

/// Create function pointer type with inferred arguments.
#[doc(hidden)]
#[macro_export]
macro_rules! __fn_ptr {
    (
        ($($qualifiers:tt)*)
        ($($output:tt)*)
        $(,)?
    ) => {
        $($qualifiers)* extern "C" fn($($output)*) -> _
    };
    (
        ($($qualifiers:tt)*)
        ($($output:tt)*)
        _ : $param_ty:ty $(, $($rest:tt)*)?
    ) => {
        $crate::__fn_ptr! {
            ($($qualifiers)*)
            ($($output)* _,)
            $($($rest)*)?
        }
    };
    (
        ($($qualifiers:tt)*)
        ($($output:tt)*)
        mut $param:ident : $param_ty:ty $(, $($rest:tt)*)?
    ) => {
        $crate::__fn_ptr! {
            ($($qualifiers)*)
            ($($output)* _,)
            $($($rest)*)?
        }
    };
    (
        ($($qualifiers:tt)*)
        ($($output:tt)*)
        $param:ident : $param_ty:ty $(, $($rest:tt)*)?
    ) => {
        $crate::__fn_ptr! {
            ($($qualifiers)*)
            ($($output)* _,)
            $($($rest)*)?
        }
    };
}
