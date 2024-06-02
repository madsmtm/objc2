# The Model-View-Controller design pattern

The [Model-View-Controller (MVC) design pattern][mvc-doc] is quite prominent in Cocoa application development, and you may end up needing to incoporate parts of it in your application. You can use the following as a rough guideline for how to do so.

**Model**: Use plain Rust structures inside [`Cell<T>`] or [`RefCell<T>`] so that you can mutate them behind shared references (or possibly `Rc<RefCell<T>>`, depending on if you need to share data between multiple controllers).

**View**: Use the built-in views (`NSView` or `UIView`). If you need to register a delegate on these, use the controller as the delegate.

**Controller**: Use the [`declare_class!`] macro to create a new object. Use [`mutability::MainThreadOnly`] as the [`ClassType::Mutability`], so that you can implement view delegate protocols.

[mvc-doc]: https://developer.apple.com/library/archive/documentation/General/Conceptual/CocoaEncyclopedia/Model-View-Controller/Model-View-Controller.html
[`Cell<T>`]: core::cell::Cell
[`RefCell<T>`]: core::cell::RefCell
[`declare_class!`]: crate::declare_class
[`mutability::MainThreadOnly`]: crate::mutability::MainThreadOnly
[`ClassType::Mutability`]: crate::ClassType::Mutability
