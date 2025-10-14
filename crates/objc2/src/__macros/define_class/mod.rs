mod checks;
mod ivars;
mod output_impls;
mod register_impls;

pub use self::checks::*;
pub use self::ivars::*;

/// Create a new Objective-C class.
///
/// This is useful in many cases since Objective-C frameworks tend to favour a
/// design pattern using "delegates", where to hook into a piece of
/// functionality in a class, you implement that class' delegate protocol in
/// a custom class.
///
/// This macro is the declarative way of creating classes, in contrast with
/// [`ClassBuilder`], which allows creating classes in an imperative fashion.
/// It is highly recommended that you use this macro though, since it contains
/// a lot of extra debug assertions and niceties that help ensure the
/// soundness of your code.
///
/// The class is guaranteed to have been created and registered with the
/// Objective-C runtime after the [`ClassType::class`] function has been
/// called.
///
/// See [Apple's documentation] on defining classes for a more in-depth
/// introduction.
///
/// [Apple's documentation]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ProgrammingWithObjectiveC/DefiningClasses/DefiningClasses.html
/// [`ClassBuilder`]: crate::runtime::ClassBuilder
/// [`ClassType::class`]: crate::ClassType::class
///
///
/// # Specification
///
/// This macro consists of the following parts:
/// - The type definition, along with special attributes.
/// - Any number of inherent implementations.
/// - Any number of protocol implementations.
///
/// With the syntax generally resembling a combination of that in
/// [`extern_class!`] and [`extern_methods!`].
///
/// This macro creates an opaque struct with implementations in [a similar
/// manner as the `extern_class!` macro][ec_spec]. Additionally, it implements
/// the [`DefinedClass`] trait, as well as any protocols specified in the
/// protocol implementations.
///
/// If the type implements [`Drop`], the macro will generate a `dealloc`
/// method for you, which will call `drop` automatically.
///
/// The macro does not support generic types.
///
/// [`extern_class!`]: crate::extern_class
/// [`extern_methods!`]: crate::extern_methods
/// [ec_spec]: crate::extern_class#specification
/// [`DefinedClass`]: crate::DefinedClass
///
///
/// ## Attributes
///
/// You can add most normal attributes to the class, including `#[cfg(...)]`,
/// `#[allow(...)]` and doc comments.
///
/// Exceptions and special attributes are noted below.
///
///
/// ### `#[unsafe(super(...))]` (required)
///
/// Same [as in `extern_class!`](crate::extern_class#unsafesuper-required).
///
///
/// ### `#[thread_kind = ...]` (optional)
///
/// Same [as in `extern_class!`](crate::extern_class#thread_kind---optional).
///
///
/// ### `#[name = "..."]` (optional)
///
/// Specify the runtime-name for the class. Must be unique across the entire
/// application. This is useful if the name of a class is used elsewhere, such
/// as when defining a delegate that needs to be named in e.g. `Info.plist`.
///
/// If not set, this will default to:
/// ```ignore
/// concat!(module_path!(), "::", $class, env!("CARGO_PKG_VERSION"));
/// ```
///
/// E.g. for example `"my_crate::my_module::MyClass0.1.0"`.
///
/// If you're developing a library, it is recommended that you do not set
/// this, and instead rely on the default naming, since that usually works
/// better with users having multiple SemVer-incompatible versions of your
/// library in the same binary.
///
/// If the name is auto-generated, the class will also be allowed to be used
/// across multiple shared dynamic libraries in the same process.
///
///
/// ### `#[ivars = ...]` (optional)
///
/// Controls [the instance variables] of the class; this is the intended way
/// to specify the data your class stores. If you don't set this attribute,
/// the macro will default to [`()`][unit].
///
/// It is recommended that you wrap your instance variables in [`Cell`],
/// [`RefCell`], atomics or other similar interior mutability abstractions to
/// allow mutating your instance variables. See [the docs on interior
/// mutability][interior_mutability] for further details.
///
/// Beware that if you want to use the class' inherited initializers (such as
/// `init`), you must override the subclass' designated initializers, and
/// initialize your ivars properly in there.
///
/// [the instance variables]: crate::DefinedClass::Ivars
/// [`Cell`]: core::cell::Cell
/// [`RefCell`]: core::cell::RefCell
/// [interior_mutability]: crate::topics::interior_mutability
///
///
/// ### `#[derive(...)]`
///
/// This is overridden, and only works with [`PartialEq`], [`Eq`], [`Hash`]
/// and [`Debug`].
///
/// The implementations delegate to the superclass' implementation, so if you
/// want to change how they work, you should override the [`isEqual:`] and
/// [`hash`] methods instead.
///
/// The `Debug` implementation currently also debug print your ivars, but that
/// may change in the future. Prefer to override [`description`] (and
/// potentially [`debugDescription`]) instead.
///
/// [`Hash`]: std::hash::Hash
/// [`Debug`]: std::fmt::Debug
/// [`isEqual:`]: crate::runtime::NSObjectProtocol::isEqual
/// [`hash`]: crate::runtime::NSObjectProtocol::hash
/// [`description`]: crate::runtime::NSObjectProtocol::description
/// [`debugDescription`]: crate::runtime::NSObjectProtocol::debugDescription
///
///
/// ### `#[cfg_attr(..., ...)]`
///
/// Same [as in `extern_class!`](crate::extern_class#cfg_attr-).
///
///
/// ### `#[repr(...)]`
///
/// Same [as in `extern_class!`](crate::extern_class#repr).
///
///
/// ## Inherent method definitions
///
/// Within the `impl` block you can define two types of functions;
/// ["associated functions"] and ["methods"]. These are then mapped to the
/// Objective-C equivalents "class methods" and "instance methods". In
/// particular, if you use `self` or the special name `this` (or `_this`),
/// your method will be registered as an instance method, and if you don't it
/// will be registered as a class method.
///
/// On instance methods, you can freely choose between different types of
/// receivers, e.g. `&self`, `self: *const Self`, `this: *const Self`, and so
/// on. Note that using `&mut self` is not possible, if you need mutation of
/// your class' instance variables, consider using [`Cell`] or similar
/// instead.
///
/// The desired selector can be specified using the
/// `#[unsafe(method(my:selector:))]` or `#[unsafe(method_id(my:selector:))]`
/// attributes, similar to the [`extern_methods!`] macro.
///
/// If the `#[unsafe(method_id(...))]` attribute is used, the return type must
/// be `Option<Retained<T>>` or `Retained<T>`. Additionally, if the selector
/// is in the "init"-family, the `self`/`this` parameter must be
/// `Allocated<Self>`.
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
/// If the parameter or return type is [`bool`], a conversion is performed to
/// make it behave similarly to the Objective-C `BOOL`. Use [`runtime::Bool`]
/// if you want to control this manually.
///
/// Note that `&mut Retained<_>` and other such out parameters are not yet
/// supported, and may generate a panic at runtime.
///
/// ["associated functions"]: https://doc.rust-lang.org/reference/items/associated-items.html#methods
/// ["methods"]: https://doc.rust-lang.org/reference/items/associated-items.html#methods
/// [`Cell`]: core::cell::Cell
/// [open an issue]: https://github.com/madsmtm/objc2/issues/new
/// [`msg_send!`]: crate::msg_send
/// [`runtime::Bool`]: crate::runtime::Bool
///
///
/// ## Protocol implementations
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
/// - Debug assertions are enabled, and an overridden method's signature is not
///   equal to the one on the superclass.
/// - Debug assertions are enabled, and the protocol's required methods are not
///   implemented.
///
/// And possibly more similar cases in the future.
///
///
/// # Safety
///
/// Using this macro requires writing a lot of `unsafe` markers:
///
/// When writing `#[unsafe(super(...))]`, you must ensure that:
/// - Any invariants that the superclass [`ClassType::Super`] may have must be
///   upheld.
/// - If your type implements `Drop`, the implementation must abide by the
///   following rules:
///   - It must not call any overridden methods.
///   - It must not `retain` the object past the lifetime of the drop.
///   - It must not `retain` in the same scope that `&mut self` is active.
///   - TODO: And probably a few more. [Open an issue] if you would like
///     guidance on whether your implementation is correct.
///
/// `#[unsafe(method(...))]` asserts that the types match those that are
/// expected when the method is invoked from Objective-C. Note that unlike
/// with [`extern_methods!`], there are no safe-guards here; you can write
/// `i8`, but if Objective-C thinks it's an `u32`, it will cause UB when
/// called!
///
/// `unsafe impl P for T { ... }` requires that all required methods of the
/// specified protocol is implemented, and that any extra requirements
/// (implicit or explicit) that the protocol has are upheld.
///
/// [`ClassType::Super`]: crate::ClassType::Super
/// [Open an issue]: https://github.com/madsmtm/objc2/issues/new
///
///
/// # Thread safety
///
/// The Objective-C runtime is thread-safe, so any classes you create yourself
/// will automatically be [`Send`] and [`Sync`] (via auto traits) provided
/// that:
/// 1. The superclass is thread-safe, or is [`NSObject`].
/// 2. The ivars are thread-safe.
/// 3. The thread kind is not [`MainThreadOnly`].
///
/// Note though that in many cases, [the frameworks] you will be interacting
/// with will not be thread-safe, and so in many cases it will make sense to
/// [use interior mutability] in your custom classes.
///
/// [`NSObject`]: crate::runtime::NSObject
/// [`MainThreadOnly`]: crate::MainThreadOnly
/// [the frameworks]: crate::topics::frameworks_list
/// [use interior mutability]: crate::topics::interior_mutability
///
///
/// # Examples
///
/// Define a class `MyCustomObject` that inherits `NSObject`, has a few
/// instance variables and methods, and implements the `NSCopying` protocol.
///
/// ```
/// use std::ffi::c_int;
///
/// # use objc2::runtime::{NSObject, NSObjectProtocol, NSZone};
/// # objc2::extern_protocol!(
/// #     unsafe trait NSCopying {
/// #         #[unsafe(method(copy))]
/// #         fn copy(&self) -> Retained<Self>;
/// #         #[unsafe(method(copyWithZone:))]
/// #         unsafe fn copyWithZone(&self, zone: *mut NSZone) -> Retained<Self>;
/// #     }
/// # );
/// # #[cfg(requires_foundation)]
/// use objc2_foundation::{CopyingHelper, NSCopying, NSObject, NSObjectProtocol, NSZone};
/// use objc2::rc::{Allocated, Retained};
/// use objc2::{
///     define_class, extern_methods, extern_protocol, msg_send, AnyThread,
///     ClassType, DefinedClass, ProtocolType,
/// };
///
/// #[derive(Clone)]
/// struct Ivars {
///     foo: u8,
///     bar: c_int,
///     object: Retained<NSObject>,
/// }
///
/// define_class!(
///     // SAFETY:
///     // - The superclass NSObject does not have any subclassing requirements.
///     // - `MyCustomObject` does not implement `Drop`.
///     #[unsafe(super(NSObject))]
///
///     // If we were implementing delegate methods like `NSApplicationDelegate`,
///     // we would specify the object to only be usable on the main thread:
///     // #[thread_kind = MainThreadOnly]
///
///     // If we needed to refer to the class from elsewhere, we'd give it a
///     // name here explicitly.
///     // #[name = "MyCustomObject"]
///
///     // Specify the instance variables this class has.
///     #[ivars = Ivars]
///     struct MyCustomObject;
///
///     impl MyCustomObject {
///         #[unsafe(method(foo))]
///         fn __get_foo(&self) -> u8 {
///             self.ivars().foo
///         }
///
///         #[unsafe(method_id(object))]
///         fn __get_object(&self) -> Retained<NSObject> {
///             self.ivars().object.clone()
///         }
///
///         #[unsafe(method(myClassMethod))]
///         fn __my_class_method() -> bool {
///             true
///         }
///     }
///
///     unsafe impl NSObjectProtocol for MyCustomObject {}
///
///     unsafe impl NSCopying for MyCustomObject {
///         #[unsafe(method_id(copyWithZone:))]
///         fn copyWithZone(&self, _zone: *const NSZone) -> Retained<Self> {
///             let new = Self::alloc().set_ivars(self.ivars().clone());
///             unsafe { msg_send![super(new), init] }
///         }
///
///         // If we have tried to add other methods here, or had forgotten
///         // to implement the method, we would have gotten an error.
///     }
/// );
///
/// // Specially required for `NSCopying`, but otherwise not needed.
/// # #[cfg(requires_foundation)]
/// unsafe impl CopyingHelper for MyCustomObject {
///     type Result = Self;
/// }
///
/// // Add creation method.
/// impl MyCustomObject {
///     fn new(foo: u8) -> Retained<Self> {
///         // Initialize instance variables.
///         let this = Self::alloc().set_ivars(Ivars {
///             foo,
///             bar: 42,
///             object: NSObject::new(),
///         });
///         // Call `NSObject`'s `init` method.
///         unsafe { msg_send![super(this), init] }
///     }
/// }
///
/// // Make an interface to the methods we defined.
/// impl MyCustomObject {
///     extern_methods!(
///         #[unsafe(method(foo))]
///         pub fn get_foo(&self) -> u8;
///
///         #[unsafe(method(object))]
///         pub fn get_object(&self) -> Retained<NSObject>;
///
///         #[unsafe(method(myClassMethod))]
///         pub fn my_class_method() -> bool;
///     );
/// }
///
/// # // Intentionally use `fn main` for clarity
/// fn main() {
///     let obj = MyCustomObject::new(3);
///     assert_eq!(obj.ivars().foo, 3);
///     assert_eq!(obj.ivars().bar, 42);
///     assert!(obj.ivars().object.isKindOfClass(NSObject::class()));
///
///     let obj = obj.copy();
///
///     assert_eq!(obj.get_foo(), 3);
///     assert!(obj.get_object().isKindOfClass(NSObject::class()));
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
/// @interface MyCustomObject: NSObject <NSCopying>
/// - (instancetype)initWithFoo:(uint8_t)foo;
/// - (uint8_t)foo;
/// - (NSObject*)object;
/// + (BOOL)myClassMethod;
/// @end
///
///
/// @implementation MyCustomObject {
///     // Instance variables
///     uint8_t foo;
///     int bar;
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
///     return self->foo;
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
///     MyCustomObject* new = [[MyCustomObject alloc] initWithFoo: self->foo];
///     new->bar = self->bar;
///     new->obj = self->obj;
///     return new;
/// }
///
/// @end
/// ```
#[doc(alias = "@interface")]
#[doc(alias = "@implementation")]
#[macro_export]
macro_rules! define_class {
    {
        // The following special attributes are supported:
        // - #[unsafe(super($($superclasses:path),*))]
        // - #[unsafe(super = $superclass:path)]
        // - #[thread_kind = $thread_kind:path]
        // - #[name = $name:literal]
        // - #[ivars = $ivars:path]
        $(#[$($attrs:tt)*])*
        $v:vis struct $class:ident;

        // unsafe impl Protocol for $class { ... }
        // impl $class { ... }
        $($impls:tt)*
    } => {
        // Struct and various impls.
        $crate::__extract_attributes! {
            ($(#[$($attrs)*])*)

            ($crate::__define_class_inner)
            ($v)
            ($class)
            // We duplicate the impls here, since we need them to create a
            // thunk that is inserted into the class' method table.
            ($($impls)*)
        }

        // Methods.
        $crate::__define_class_output_impls! {
            // TODO: Add $class here and verify that the same item is being
            // implemented everywhere.
            $($impls)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! declare_class {
    {
        $(#[$m:meta])*
        $v:vis struct $name:ident;

        unsafe impl ClassType for $for_class:ty {
            $(#[inherits($($inheritance_rest:ty),+)])?
            type Super = $superclass:ty;

            type Mutability = $mutability:ty;

            const NAME: &'static str = $name_const:expr;
        }

        impl DefinedClass for $for_defined:ty {
            $(type Ivars = $ivars:ty;)?
        }

        $($impls:tt)*
    } => {
        // For slightly better diagnostics
        $(#[$m])*
        $v struct $name;

        $crate::__macros::compile_error!("declare_class! has been renamed to define_class!, and the syntax has changed")
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_class_inner {
    (
        ($v:vis)
        ($class:ident)
        ($($impls:tt)*)

        ($($safety:tt $superclass:path $(, $superclasses:path)* $(,)?)?)
        ($($($thread_kind:tt)+)?)
        ($($name:tt)*)
        ($($ivars:tt)*)
        ($($derives:tt)*)
        ($($attr_struct:tt)*)
        ($($attr_impl:tt)*)
    ) => {
        // Ensure that the type has the same layout as the superclass.
        // #[repr(transparent)] doesn't work because the superclass is a ZST.
        #[repr(C)]
        $($attr_struct)*
        $v struct $class {
            // Superclasses are deallocated by calling `[super dealloc]`.
            //
            // Auto traits are taken from `__SubclassingType` (which is
            // usually the super class).
            __superclass: $crate::__macros::ManuallyDrop<$crate::__fallback_if_not_set! {
                ($(<$superclass as $crate::ClassType>::__SubclassingType)?)
                // For better diagnostics, see also __extern_class_inner!
                ($crate::runtime::NSObject)
            }>,
            __phantom: $crate::__macros::PhantomData<(
                // Include ivars for auto traits.
                <Self as $crate::DefinedClass>::Ivars,
                // Translate thread kind to appropriate auto traits.
                $crate::__macros::ThreadKindAutoTraits<<Self as $crate::ClassType>::ThreadKind>,
            )>,
        }

        $crate::__extern_class_impl_traits! {
            ($($attr_impl)*)
            (unsafe impl)
            ($class)
            ($($superclass, $($superclasses,)*)? $crate::runtime::AnyObject)
        }

        $crate::__define_class_derives! {
            ($($attr_impl)*)
            ($class)
            ($($derives)*)
        }

        // Anonymous block to hide the shared statics
        $($attr_impl)*
        const _: () = {
            // Add #[export_name] here to ensure that all class names are
            // unique, at least within the resulting linker invocation (though
            // not across dylibs). The Objective-C runtime will ensure this as
            // well, but this will allow ensuring it at link time.
            #[export_name = $crate::__macros::concat!(
                "__CLASS_",
                $crate::__define_class_name!($class, $($name)*),
            )]
            static __OBJC2_CLASS: $crate::__macros::SyncUnsafeCell<
                $crate::__macros::MaybeUninit<&'static $crate::runtime::AnyClass>
            > = $crate::__macros::SyncUnsafeCell::new($crate::__macros::MaybeUninit::uninit());

            #[export_name = $crate::__macros::concat!(
                "__IVAR_OFFSET_",
                $crate::__define_class_name!($class, $($name)*),
            )]
            static __OBJC2_IVAR_OFFSET: $crate::__macros::SyncUnsafeCell<
                $crate::__macros::MaybeUninit<$crate::__macros::isize>
            > = $crate::__macros::SyncUnsafeCell::new($crate::__macros::MaybeUninit::uninit());

            #[export_name = $crate::__macros::concat!(
                "__DROP_FLAG_OFFSET_",
                $crate::__define_class_name!($class, $($name)*),
            )]
            static __OBJC2_DROP_FLAG_OFFSET: $crate::__macros::SyncUnsafeCell<
                $crate::__macros::MaybeUninit<$crate::__macros::isize>
            > = $crate::__macros::SyncUnsafeCell::new($crate::__macros::MaybeUninit::uninit());

            // Creation
            unsafe impl $crate::ClassType for $class {
                type Super = $crate::__fallback_if_not_set! {
                    ($($superclass)?)
                    // For better diagnostics, see also __extern_class_inner!
                    ($crate::runtime::NSObject)
                };

                type ThreadKind = $crate::__fallback_if_not_set! {
                    ($(dyn ($($thread_kind)+))?)
                    // Default to the super class' thread kind
                    (<<Self as $crate::ClassType>::Super as $crate::ClassType>::ThreadKind)
                };

                const NAME: &'static $crate::__macros::str = $crate::__define_class_name!($class, $($name)*);

                fn class() -> &'static $crate::runtime::AnyClass {
                    let _ = <Self as $crate::__macros::ValidThreadKind<Self::ThreadKind>>::check;
                    let _ = <Self as $crate::__macros::MainThreadOnlyDoesNotImplSendSync<_>>::check;

                    const C_NAME: &'static $crate::__macros::CStr = $crate::__macros::class_c_name(
                        $crate::__macros::concat!($crate::__define_class_name!($class, $($name)*), "\0")
                    );

                    // TODO: Use `std::sync::OnceLock`
                    #[export_name = $crate::__macros::concat!(
                        "__REGISTER_CLASS_",
                        $crate::__define_class_name!($class, $($name)*),
                    )]
                    static REGISTER_CLASS: $crate::__macros::Once = $crate::__macros::Once::new();

                    REGISTER_CLASS.call_once(|| {
                        let (__objc2_cls, __objc2_ivar_offset, __objc2_drop_flag_offset) = $crate::__macros::define_class::<Self>(
                            C_NAME,
                            $crate::__define_class_name_is_auto_generated!($($name)*),
                            |mut __objc2_builder| {
                                // Implement protocols and methods
                                $crate::__define_class_register_impls! {
                                    (__objc2_builder)
                                    $($impls)*
                                }
                            }
                        );

                        // SAFETY: Modification is ensured by `Once` to happen
                        // before any access to the variables.
                        unsafe {
                            __OBJC2_CLASS.get().write($crate::__macros::MaybeUninit::new(__objc2_cls));
                            if <Self as $crate::__macros::DefinedIvarsHelper>::HAS_IVARS {
                                __OBJC2_IVAR_OFFSET.get().write($crate::__macros::MaybeUninit::new(__objc2_ivar_offset));
                            }
                            if <Self as $crate::__macros::DefinedIvarsHelper>::HAS_DROP_FLAG {
                                __OBJC2_DROP_FLAG_OFFSET.get().write($crate::__macros::MaybeUninit::new(__objc2_drop_flag_offset));
                            }
                        }
                    });

                    // SAFETY: We just registered the class, so is now available
                    unsafe { __OBJC2_CLASS.get().read().assume_init() }
                }

                #[inline]
                fn as_super(&self) -> &Self::Super {
                    &*self.__superclass
                }

                const __INNER: () = ();

                type __SubclassingType = Self;
            }

            impl $crate::DefinedClass for $class {
                type Ivars = $crate::__fallback_if_not_set!(
                    ($($ivars)?)
                    // Default ivars to unit.
                    (())
                );

                #[inline]
                fn __ivars_offset() -> $crate::__macros::isize {
                    // Only access ivar offset if we have an ivar.
                    //
                    // This makes the offset not be included in the final
                    // executable if it's not needed.
                    if <Self as $crate::__macros::DefinedIvarsHelper>::HAS_IVARS {
                        // SAFETY: Accessing the offset is guaranteed to only be
                        // done after the class has been initialized.
                        unsafe { __OBJC2_IVAR_OFFSET.get().read().assume_init() }
                    } else {
                        // Fall back to an offset of zero.
                        //
                        // This is fine, since any reads here will only be via zero-sized
                        // ivars, where the actual pointer doesn't matter.
                        0
                    }
                }

                #[inline]
                fn __drop_flag_offset() -> $crate::__macros::isize {
                    if <Self as $crate::__macros::DefinedIvarsHelper>::HAS_DROP_FLAG {
                        // SAFETY: Same as above.
                        unsafe { __OBJC2_DROP_FLAG_OFFSET.get().read().assume_init() }
                    } else {
                        // Fall back to an offset of zero.
                        //
                        // This is fine, since the drop flag is never actually used in the
                        // cases where it was not added.
                        0
                    }
                }

                // SAFETY: The offsets are implemented correctly
                const __UNSAFE_OFFSETS_CORRECT: () = ();
            }
        };

        // SAFETY: This macro only allows non-generic classes and non-generic
        // classes are always valid downcast targets.
        $($attr_impl)*
        unsafe impl $crate::DowncastTarget for $class {}

        $($attr_impl)*
        $crate::__extern_class_check_super_unsafe!($($safety $superclass)?);
    };
}

/// Mirror of [`crate::__extern_class_derives`].
#[doc(hidden)]
#[macro_export]
macro_rules! __define_class_derives {
    // Base case
    (
        ($($attr_impl:tt)*)
        ($for:path)
        ($(,)*)
    ) => {};

    // Debug
    (
        ($($attr_impl:tt)*)
        ($for:path)
        (
            $(,)*
            Debug
            $($rest:tt)*
        )
    ) => {
        $($attr_impl)*
        #[automatically_derived]
        impl $crate::__macros::fmt::Debug for $for {
            fn fmt(&self, f: &mut $crate::__macros::fmt::Formatter<'_>) -> $crate::__macros::fmt::Result {
                f.debug_struct($crate::__macros::stringify!($for))
                    .field("super", &**self.__superclass)
                    .field("ivars", <Self as $crate::DefinedClass>::ivars(self))
                    .finish()
            }
        }

        $crate::__define_class_derives! {
            ($($attr_impl)*)
            ($for)
            ($($rest)*)
        }
    };

    // PartialEq
    (
        ($($attr_impl:tt)*)
        ($for:path)
        (
            $(,)*
            PartialEq
            $($rest:tt)*
        )
    ) => {
        $($attr_impl)*
        #[automatically_derived]
        impl $crate::__macros::PartialEq for $for {
            #[inline]
            fn eq(&self, other: &Self) -> $crate::__macros::bool {
                // Delegate to the superclass (referential equality)
                $crate::__macros::PartialEq::eq(&self.__superclass, &other.__superclass)
            }
        }

        $crate::__define_class_derives! {
            ($($attr_impl)*)
            ($for)
            ($($rest)*)
        }
    };

    // Eq
    (
        ($($attr_impl:tt)*)
        ($for:path)
        (
            $(,)*
            Eq
            $($rest:tt)*
        )
    ) => {
        $($attr_impl)*
        #[automatically_derived]
        impl $crate::__macros::Eq for $for {}

        $crate::__define_class_derives! {
            ($($attr_impl)*)
            ($for)
            ($($rest)*)
        }
    };

    // Hash
    (
        ($($attr_impl:tt)*)
        ($for:path)
        (
            $(,)*
            Hash
            $($rest:tt)*
        )
    ) => {
        $($attr_impl)*
        #[automatically_derived]
        impl $crate::__macros::Hash for $for {
            #[inline]
            fn hash<H: $crate::__macros::Hasher>(&self, state: &mut H) {
                // Delegate to the superclass (which hashes the reference)
                $crate::__macros::Hash::hash(&self.__superclass, state)
            }
        }

        $crate::__define_class_derives! {
            ($($attr_impl)*)
            ($for)
            ($($rest)*)
        }
    };

    // Unhandled derive
    (
        ($($attr_impl:tt)*)
        ($for:path)
        (
            $(,)*
            $derive:path
            $(, $($rest:tt)*)?
        )
    ) => {
        const _: () = {
            // For better diagnostics.
            #[derive($derive)]
            struct Derive;
        };
        $crate::__macros::compile_error!($crate::__macros::stringify!(
            #[derive($derive)] is not supported in define_class!
        ));

        $crate::__define_class_derives! {
            ($($attr_impl)*)
            ($for)
            ($($($rest)*)?)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_class_name_is_auto_generated {
    ($name:expr) => {
        false
    };
    () => {
        // If no custom name is specified.
        true
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_class_name {
    ($class:ident, $($name:tt)+) => {
        $($name)+
    };
    ($class:ident,) => {
        // The user didn't specify a name; generate a reasonable one.
        $crate::__macros::concat!(
            // Module path includes crate name when in library.
            $crate::__macros::module_path!(),
            "::",
            $crate::__macros::stringify!($class),
            $crate::__macros::env!("CARGO_PKG_VERSION"),
        )
    };
}
