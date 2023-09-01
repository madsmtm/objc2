//! TODO
#![doc = include_str!("./intro.md")]

pub mod chapter_1 {
    #![doc = include_str!("./chapter_1.md")]
    pub use super::chapter_2 as next;
}

pub mod chapter_2 {
    #![doc = include_str!("./chapter_2.md")]
    pub use super::chapter_1 as prev;
    pub use super::chapter_3 as next;
}

pub mod chapter_3 {
    #![doc = include_str!("./chapter_3.md")]
    pub use super::chapter_2 as prev;
    pub use super::chapter_4 as next;
}

pub mod chapter_4 {
    #![doc = include_str!("./chapter_4.md")]
    pub use super::chapter_3 as prev;
}

/*
How is things translated?
- Class -> Opaque struct
    - Pointer to class -> Option<&MyClass> or &MyClass
- Protocol -> Trait
- Property getter -> Method
- Property setter -> Method
- Class method -> Associated function
- Instance method -> Method
    - Properties vs. methods

Object creation

Mutability/ownership:
- ...
- `ClassType::alloc` / init methods vs. `new` methods
- `rc::Allocated`
- `MainThreadMarker`/`MainThreadMarker::alloc`

Declaring classes yourself:
- "Some of these objects are immediately usable, such as basic data types like strings and numbers, or user interface elements like buttons and table views. Some are designed for you to customize with your own code to behave in the way you require. The app development process involves deciding how best to customize and combine the objects provided by the underlying frameworks with your own objects to give your app its unique set of features and functionality."
- `ClassType` vs. `Message`
- Instance variables
- Name must be unique
- Methods:
    - New methods
    - Overriding methods
    - Implementing protocols
- `declare_class!`

Autoreleased objects:
- `rc::autoreleasepool`
- Autoreleasing in declared classes

Advanced topics:
- `rc::WeakId` and `retain`ing vs copying
- `runtime::Class`
- `runtime::ProtocolObject`
- Type-encodings
- Exceptions???
- Message sending
- "literals" / ns_string! macro (and others in the future).


What now?

If you want to learn more about Objective-C and Swift first, some great starting points are the [Programming with Objective-C][objc-book] book and [The Swift Programming Language][swift-book] book.

[objc-book]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ProgrammingWithObjectiveC/Introduction/Introduction.html
[swift-book]: https://docs.swift.org/swift-book/documentation/the-swift-programming-language

*/
