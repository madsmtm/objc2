# Chapter 1 - What is Objective-C?

Objective-C is a superset of C that has been the standard programming language on Apple platforms like macOS, iOS, iPadOS, tvOS and watchOS for many years. While it has since been superseded by Swift, most of the core interfaces, libraries and frameworks that are in use on Apple systems are still written in Objective-C, and hence it is still useful for us to learn a bit about it.

Objective-C provides object-oriented capabilities, that work similarly to other object-oriented languages; objects contain variables and methods, as defined by classes, and the methods are customizable by subclasses.

Additionally, everything in Objective-C is dynamic, allowing objects to be introspected and customized at runtime (as an example, it is possible, although discouraged, to change the methods of an object at runtime).


## Reference-counting

Every object in Objective-C is allocated and stored on the heap, and as such, is only accesible behind pointers. To make sharing and ownership easier, ...

means that objects are not accesible directly, but instead must either be held as `&NSObject` or `Id<NSObject>`

`rc::Id` / reference-counting / `ClassType::retain`


## `Deref` and superclasses

Objective-C (and Swift) has single parent inheritance, which means that every class has a superclass, except for the few "root" classes like `NSObject` and `NSProxy`.

"instance" is used to  "class" is used to describe the thing that "instances" are an instance of.

Rust, however, has no notion of inheritance, so to avoid . This may be slightly confusing sometimes, but was considered to be the best option amongst evils.

The documentation should show the methods available on a class, even through it's `Deref`-chain. The `Deref` / superclass chain always ends at `runtime::Object`, so that it is easily possible to call methods which accept any object.

```rust
fn my_method(obj: &NSObject) {
    // Do something with the object
}

let string: &NSString;
// Because of the `&`, deref-coercion kicks in, and the string is converted to
// `&NSObject` before being passed to the method.
my_method(&string);
```

This only works in select cases, though, so sometimes you will have to explicitly get the superclass of the object, with either (possibly repeated) calls to `Id::into_super` or to `ClassType::as_super`.

