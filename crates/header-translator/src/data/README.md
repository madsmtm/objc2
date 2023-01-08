# Data enrichment

The top-level `translation-config.toml` file describes various tweaks that we need to do because our header translation is incomplete in some areas.

However, even if our header translation was perfect, we still need a way to enrich the generated data, since C headers have no way to describe which methods are safe, or mutable, and which are not!

You can also view it as that, over time, we need to _decrease_ the amount of stuff in `translation-config.toml`, but we need to _increase_ the amount of stuff in this module.


## Example

```rust
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
