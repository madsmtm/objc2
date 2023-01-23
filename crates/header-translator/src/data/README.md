# Data enrichment

The top-level `translation-config.toml` file describes various tweaks that we need to do because our header translation is incomplete in some areas.

However, even if our header translation was perfect, we still need a way to enrich the generated data, since C headers have no way to describe which methods are safe, or mutable, and which are not!

You can also view it as that, over time, we need to _decrease_ the amount of stuff in `translation-config.toml`, but we need to _increase_ the amount of stuff in this module.


## What is required for a method to be safe?

This is a longer discussion, but the following guidelines will get you far. Do
not be afraid of opening an issue or PR that discusses the safety of a
specific API!

1. The method must not take a raw pointer; one could trivially pass
  `ptr::invalid()` and cause UB with that.
2. Any extra requirements that the method states in its documentation must be
  upheld. For example, a method may declare in its documentation that some
  property must be something specific in relation to another property. Since
  we don't know whether this is upheld, the method is not safe.

  Note: This is the hardest part; determining for sure if a given method is
  safe or not!
  - For `AppKit`, `UIKit` and similar UI frameworks, we're working on a
    system to make relevant types main-thread only in [#359]. Until then,
    this safety requirement is _not_ considered upheld.
3. If the method can throw an exception if provided with invalid inputs, it is
  not safe. Consider declaring a helper method that checks the preconditions
  first!
4. Beware of `Mutable` classes (e.g. `NSMutableString`); these usually need to
  be passed as `&mut T`, or operate on `&mut self`.

Note: It is _not_ considered a breaking change in `icrate` for a method to be
marked safe, so such an improvement can be made in a minor version!

[#359]: https://github.com/madsmtm/objc2/pull/359


## Example

```rust , ignore
data! {
    // Everywhere that the class is returned, it is as
    // `Id<MyMutableClass, Owned>`.
    //
    // Specifying this is _not_ unsafe, but every method that returns this
    // class must be checked for safety with it in mind.
    class MyMutableClass: Owned {
        // The class method `new` is safe.
        unsafe +new;
        // The method `init` is safe.
        unsafe -init;
        // The method `mutate` takes &mut self, and is safe.
        unsafe mut -mutate;
        // The method `mutateUnchecked` takes &mut self, but is not safe.
        mut -mutateUnchecked;
    }

    // Declare the function "foo" as safe
    unsafe fn foo;
}
```

Note how we use `unsafe` to specify that a method is safe - this is so that there is one "instance" of `unsafe` to point back to, in case something turned out to be unsound (or stated differently, every instance of `unsafe` in here needs to be reviewed for soundness).
