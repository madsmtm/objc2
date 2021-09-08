/**
Gets a reference to a [`Class`] from the given name.

# Panics

Panics if no class with the given name can be found.

To check for a class that may not exist, use [`Class::get`].

[`Class`]: crate::runtime::Class
[`Class::get`]: crate::runtime::Class::get

# Examples

``` no_run
# use objc2::class;
let cls = class!(NSObject);
```
*/
#[macro_export]
macro_rules! class {
    ($name:ident) => {{
        static CLASS: $crate::__CachedClass = $crate::__CachedClass::new();
        let name = concat!(stringify!($name), '\0');
        #[allow(unused_unsafe)]
        let cls = unsafe { CLASS.get(name) };
        match cls {
            Some(cls) => cls,
            None => panic!("Class with name {} could not be found", stringify!($name)),
        }
    }};
}

/**
Registers a selector with the Objective-C runtime.

Returns a [`Sel`].

[`Sel`]: crate::runtime::Sel

# Examples

```
# use objc2::sel;
let sel = sel!(description);
let sel = sel!(setObject:forKey:);
```
*/
#[macro_export]
macro_rules! sel {
    ($name:ident) => ({
        static SEL: $crate::__CachedSel = $crate::__CachedSel::new();
        let name = concat!(stringify!($name), '\0');
        #[allow(unused_unsafe)]
        unsafe { SEL.get(name) }
    });
    ($($name:ident :)+) => ({
        static SEL: $crate::__CachedSel = $crate::__CachedSel::new();
        let name = concat!($(stringify!($name), ':'),+, '\0');
        #[allow(unused_unsafe)]
        unsafe { SEL.get(name) }
    });
}

/**
Sends a message to an object.

The first argument can be any type that dereferences to a type that implements
[`Message`], like a reference, a pointer, or an `objc2_id::Id` to an object.

The syntax is similar to the message syntax in Objective-C.

Variadic arguments are not currently supported.

[`Message`]: crate::Message

# Panics

Panics if the `exception` feature is enabled and the Objective-C method throws
an exception.

And panics if the `verify_message` feature is enabled and the Objective-C
method's argument's encoding does not match the encoding of the given arguments.

# Examples

``` no_run
# use objc2::msg_send;
# use objc2::runtime::Object;
# unsafe {
let obj: *mut Object;
# let obj: *mut Object = 0 as *mut Object;
let description: *const Object = msg_send![obj, description];
let _: () = msg_send![obj, setArg1:1 arg2:2];
# }
```
*/
#[macro_export]
macro_rules! msg_send {
    (super($obj:expr, $superclass:expr), $name:ident) => ({
        let sel = $crate::sel!($name);
        let result;
        match $crate::__send_super_message(&*$obj, $superclass, sel, ()) {
            Err(s) => panic!("{}", s),
            Ok(r) => result = r,
        }
        result
    });
    (super($obj:expr, $superclass:expr), $($name:ident : $arg:expr)+) => ({
        let sel = $crate::sel!($($name:)+);
        let result;
        match $crate::__send_super_message(&*$obj, $superclass, sel, ($($arg,)*)) {
            Err(s) => panic!("{}", s),
            Ok(r) => result = r,
        }
        result
    });
    ($obj:expr, $name:ident) => ({
        let sel = $crate::sel!($name);
        let result;
        match $crate::__send_message(&*$obj, sel, ()) {
            Err(s) => panic!("{}", s),
            Ok(r) => result = r,
        }
        result
    });
    ($obj:expr, $($name:ident : $arg:expr)+) => ({
        let sel = $crate::sel!($($name:)+);
        let result;
        match $crate::__send_message(&*$obj, sel, ($($arg,)*)) {
            Err(s) => panic!("{}", s),
            Ok(r) => result = r,
        }
        result
    });
}
