# Safety and soundness in the framework crates

There is a huge number of things that can violate the memory-safety guarantees of Rust, especially when calling into foreign code such as we're doing. We would like to gain confidence that the framework crates are **sound**, such that programs written using these will not be subject to memory-safety issues.

To examine this problem, I find it useful to consider the following taxonomy of memory safety[^rust_different_safety] (borrowed from [Apple's security blog][apple-memorysafety]):

1. **Temporal or lifetime safety** means that all memory accesses to an object occur during the lifetime of that object’s allocation, between when the object’s memory is allocated and when it is freed. An access to the object outside of this window is unsafe and called a Use-After-Free (UAF); double-free violations are a particular variant of UAF.
2. **Spatial or bounds safety** notes that a memory allocation has a particular size, and it’s incorrect to access any memory outside of the intended bounds of the allocation. Violations of this property are called Out-of-Bounds (OOB) accesses.
3. **Type safety** means that when a memory allocation represents a particular object with specific rules about how the object can be used, those rules can’t unexpectedly change — in other words, that the allocation is typed. Violations of this property are called type confusions.
4. **Initialization safety** denotes that a program is responsible for properly initializing newly allocated memory before using it, as the allocation might otherwise contain unexpected data. Violations of this property often lead to issues called information disclosures, but can sometimes lead to more serious memory safety issues, such as type confusions or UAFs.
5. **Thread safety** is concerned with how modern software concurrently accesses memory. If concurrent accesses to an allocation aren’t properly synchronized, then the objects contained in the allocation might reach an incorrect state and break their invariants. Violations of this property are typically called data races.

I examine each of these below, and discuss how the `objc2` project maps Apple's frameworks to the Rust type-system in a way that avoids these issues.

[apple-memorysafety]: https://security.apple.com/blog/towards-the-next-generation-of-xnu-memory-safety#memorysafety
[^rust_different_safety]: These are rarely discussed in Rust-land, because so many of these are upheld by the language by default. Instead, the Rust reference contains [an extensive list of undefined behaviour](https://doc.rust-lang.org/reference/behavior-considered-undefined.html). But we're dealing with Objective-C code, and so we must consider the properties that apply to that.


## Type safety

The framework crates map each class and protocol to separate types with the [`extern_class!`][crate::extern_class] and [`extern_protocol!`][crate::extern_protocol] macros, and are thereby strongly typed. The headers sometimes uses `id` (the bottom type for all Objective-C objects, equivalent to `dyn Any`), which we correctly map as [`AnyObject`][crate::runtime::AnyObject], but note that Apple's APIs in practice often do not actually check the type of any argument passed here before use, see [#562]. Therefore, APIs using this are marked `unsafe`.

Note that the fact that [classes `Deref` to their superclasses][crate::topics::frameworks_deref] introduces unique challenges that are not otherwise present in Rust (Rust only deals with subtyping in lifetimes, whereas this introduces subtyping everywhere). These concerns are handled conservatively by various functionality in `objc2`, see for example [`DowncastTarget`][crate::DowncastTarget].

Enums are mapped as a newtype `struct MyEnum(pub underlying_int);` with associated constants for the enumeration variants. Type confusion between enums, while often erroneous, is by itself not a soundness concern, since enums in C are "open", and consumers should e.g. always handle the default case in a switch on these. For this reason, we make the inner field `pub`. In a select few cases the enum is marked as "closed" in the header, in those cases we map it as `enum MyEnum { ... }`.

Most type definitions are mapped as simple type alias. Some typedefs, such as those used in CoreFoundation or Dispatch objects, are imbued with extra semantics, and thus we map those as opaque newtypes instead.

Finally, when mapping methods/functions, we ensure that the types we pass over the FFI boundary is correct, see [the docs on layered safety][crate::topics::layered_safety] for details on this.

[#562]: https://github.com/madsmtm/objc2/issues/562


## Initialization safety

Objective-C objects can be allocated with [`<T as AnyThread>::alloc()`][crate::AnyThread::alloc], which returns a wrapper [`Allocated<T>`][crate::rc::Allocated] over the object that was allocated. You cannot do anything with this type except `Drop` it or pass it onwards to an `init` function, thereby ensuring that objects are always initialized (and only initialized once) before use.

For custom classes, initialization is complicated by objects being in semi-initialized states before calling their superclasses. Here we use a helper type [`PartialInit`][crate::rc::PartialInit] to mirror [Swift's two-phase initialization][swift-init], see the [`define_class!`][crate::define_class] macro for details.

[swift-init]: https://docs.swift.org/swift-book/documentation/the-swift-programming-language/initialization/#Two-Phase-Initialization


## Temporal safety (lifetime safety)

Objective-C avoids temporal safety issues by pervasively reference-counting objects.

In the generated framework crates, this means that functions/methods take `&`-references to objects, and returns reference-counted smart pointers [`Retained<T>`][crate::rc::Retained]. This guarantees that objects are alive for the duration of a function call (the callee is responsible for retaining the object if it needs it beyond that), and that returned objects are kept alive even if e.g. an array that it was part of goes out of scope and is deallocated.

A subproblem of this is iterator invalidation, and we can similarly handle this by only returning `Retained<T>` objects when iterating.

Note that objects may be contained in structs, which forces us to use a raw pointer. These cases are quite rare though, so we can just mark APIs that use those as `unsafe`.

Note also that some properties are not retained by their wrapper object (`@property(unsafe_retained)` in Objective-C, effectively the same as a struct containing `*const T`). When mapping those properties, we return a pointer instead (TODO(breaking), not yet), which puts the burden of proof that the object is still alive onto the user.

All of this requires special memory management rules that we uphold with the [`msg_send!`][crate::msg_send] / [`extern_methods!`][crate::extern_methods] macros. Objective-C also has a mechanism for making objects alive inside a specific scope. We provide the [`autoreleasepool`][crate::rc::autoreleasepool] function for doing this, though note that it is not possible in Rust to safely bound the lifetime of objects to the pool (see [#540] for details).

[#540]: https://github.com/madsmtm/objc2/issues/540


## Spatial safety (bounds safety)

Foundation collections like `NSArray`, `NSDictionary` etc. are internally bounds checked, and throws an exception on out of bounds accesses. Other APIs might not be, so those will have to be marked as `unsafe`.


## Thread safety

Objective-C runtime primitives such as reference-counting are atomic, so are synthethized property setter/getters by default (see also [#757]).

Unfortunately, exactly because all objects are reference-counted, they necessarily also heavily employ interior mutability. This means that it is up to each individual object to declare whether they are thread-safe or not (and generally you must assume an object is not thread-safe if it is not marked as such). See [this article][ThreadSafetySummary] for an overview and further discussion.

In `objc2`, we handle this by marking `Retained<T>` as `Send + Sync` only if `T: Send + Sync` (and similarly for the other generic containers). In the framework crates, if the object is marked with the special `NS_SWIFT_SENDABLE` macro in their header, we implement `Send` and `Sync` for it, otherwise we conservatively do not.

One odd case where you might something should be thread-safe, but it isn't, is `NSString`. It is an immutable class, so you'd think it would be thread-safe, however, since `NSMutableString` `Deref`s to it, we cannot statically know whether the object is actually a `NSMutableString` that is referenced elsewhere. Objective-C APIs often solve this by copying the string before storing it (`@property(copy)`).

A final special case is that a large number of objects are only safe to use from the main thread. In the headers, this is marked with `NS_SWIFT_UI_ACTOR`, which we map to the [`MainThreadOnly`][crate::MainThreadOnly] trait. Along with the [`MainThreadMarker`][crate::MainThreadMarker], which is `!Send + !Sync` and can only be constructed on the main thread, we statically ensure that such objects are only constructible and usable from the main thread.

[#757]: https://github.com/madsmtm/objc2/issues/757
[ThreadSafetySummary]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Multithreading/ThreadSafetySummary/ThreadSafetySummary.html


## Arbitrary safety requirements

There's a missing piece of the puzzle that Apple didn't include in their taxonomy: C functions / Objective-C methods may have arbitrary additional safety requirements in their documentation. I weakly suspect that Apple's omission here is deliberate, to further Swift's position as a memory safe language, but I digress, the point is that these requirements cannot be checked automatically.

As a contrived example, consider a function/method that takes any integer except `42`. In Rust we could write:

```rust
/// # Safety
///
/// The given number must not be `42`.
pub unsafe fn takes_anything_but_42(num: i32) {
    if num == 42 {
        // SAFETY: Upheld by caller.
        unsafe { std::hint::unreachable_unchecked() }
    }
}
```

But since C / Objective-C signatures do not have the `unsafe` keyword, there's no way to ascertain whether such a requirement exists purely by looking at the signature, we must read the documentation.

In practice, the additional requirements are often for practical reasons inherent in how they work. A few examples:
- `NSAutoreleasePool` may violate temporal safety if misused.
- `unlock`ing an `NSLock` must be paired with a previous `lock` on the same thread.
- `NSWindow` is not safe to initialize because of a legacy feature that you have to manually disable that over-releases the window when closed.
- `CFSetCreateMutableCopy` takes a `capacity` parameter which must "must be greater than or equal to the count of the set which is to be copied, or the behavior is undefined".


## Marking APIs as safe

The tool I use for generating the framework crates (dubbed `header-translator`, works similarly to `bindgen`) has enough information to check most of the above properties at generation time, and **can be configured to automatically mark methods and functions as safe** if a method signature is not known to have any safety requirements.

The motivation for doing this automatically is to avoid users becoming safety blind. TODO create blog post about this, in short: too much `unsafe` in codebase -> the function of the marker is eroded -> users just ignore `unsafe`, thus marking automatically safe ultimately leads to fewer safety issues.

There are (to my knowledge) only two things the `header-translator` cannot properly detect: unchecked bounds requirements, and additional requirements only present in the documentation. As such, there is still some manual labour involved in reviewing each framework for these issues, though the work involved in that should be relatively minor (though by no means trivial).

That said, I'm only human. If you think you've found an API that is incorrectly marked safe or other such soundness bugs in the framework crates, feel free to [leave a comment on issue #782][#782] or [open a new one][new-issue].

[#782]: https://github.com/madsmtm/objc2/issues/782
[new-issue]: https://github.com/madsmtm/objc2/issues/new
