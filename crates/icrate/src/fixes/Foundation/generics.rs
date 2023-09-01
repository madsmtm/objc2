#![allow(dead_code)]
use core::panic::{RefUnwindSafe, UnwindSafe};

use crate::common::*;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy, Debug)]
#[repr(transparent)]
struct UnsafeIgnoreAutoTraits<T: ?Sized>(T);
// SAFETY: Moved to the name of the struct
unsafe impl<T: ?Sized> Send for UnsafeIgnoreAutoTraits<T> {}
// SAFETY: Moved to the name of the struct
unsafe impl<T: ?Sized> Sync for UnsafeIgnoreAutoTraits<T> {}
impl<T: ?Sized> RefUnwindSafe for UnsafeIgnoreAutoTraits<T> {}
impl<T: ?Sized> UnwindSafe for UnsafeIgnoreAutoTraits<T> {}
// TODO
// impl<T: ?Sized> Unpin for UnsafeIgnoreAutoTraits<T> {}

__inner_extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSArray")]
    pub struct NSArray<ObjectType: Message = AnyObject> {
        // SAFETY: Auto traits specified below.
        __superclass: UnsafeIgnoreAutoTraits<NSObject>,
        /// `NSArray` and `NSMutableArray` have `Id`-like storage.
        ///
        /// This fairly trivially true for `NSMutableArray<T: IsMutable>` and
        /// `NSArray<T: IsIdCloneable>`, but let's explore the two other cases
        /// a bit:
        ///
        ///
        /// # `NSMutableArray<T: IsIdCloneable>`
        ///
        /// Must have `Arc`-like auto traits, since `NSMutableArray<T>` is
        /// `NSMutableCopying` in that case (which is a shallow clone).
        ///
        /// E.g. if we had `NSMutableArray<T: Send>: Send`, `T` would be
        /// accessible from multiple threads, therefore we require `T: Sync`:
        /// ```ignore
        /// std::thread::scope(|s| {
        ///     let obj: Id<NSMutableArray<T>>;
        ///     let obj2 = obj.mutableCopy();
        ///     s.spawn(move || {
        ///         let obj = obj;
        ///     });
        ///     s.spawn(move || {
        ///         let obj2 = obj2;
        ///     });
        /// })
        /// ```
        ///
        /// Similarly, if we had `NSMutableArray<T: Sync>: Sync`, `T` would be
        /// dropable from other threads, therefore we require `T: Send`:
        /// ```ignore
        /// let obj: Id<NSMutableArray<T>>;
        /// std::thread::spawn(|| {
        ///     let obj2 = obj.mutableCopy();
        ///
        ///     // Do some long-lived work, the original `obj` is dropped in
        ///     // the original thread in the meantime.
        ///
        ///     // And now, the destructor is run on the wrong thread.
        ///     drop(obj2);
        /// });
        /// // (note: example does not work due to lifetime issues, but the
        /// // idea still stands).
        /// ```
        ///
        ///
        /// # `NSArray<T: IsMutable>`
        ///
        /// This could safely be `Arc`-like, but for consistency with
        /// `NSMutableArray` (esp. since `get_mut` is available on `NSArray`
        /// and not `NSMutableArray`), we make it `Box`-like.
        ///
        /// This may seem wrong since arrays are unconditionally immutable,
        /// e.g. `ClassType::Mutability` is always
        /// `ImmutableWithMutableSubclass`.
        ///
        /// But since we already require `T: IsCloneable` on `NSCopying`, and
        /// already prevent other forms of `&NSArray<T> -> Id<NSArray<T>>`,
        /// this is actually fine, since `Id<NSArray<T>>: Send | Sync`
        /// requires `NSArray<T>: Send + Sync` (and hence `T: Send + Sync`).
        __inner: PhantomData<Id<ObjectType>>,
    }

    #[cfg(feature = "Foundation_NSArray")]
    unsafe impl<ObjectType: Message> ClassType for NSArray<ObjectType> {
        type Super = NSObject;
        type Mutability = ImmutableWithMutableSubclass<NSMutableArray<ObjectType>>;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass.0
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass.0
        }
    }
);

__inner_extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSArray")]
    pub struct NSMutableArray<ObjectType: Message = AnyObject> {
        // Inherit auto traits from superclass.
        __superclass: NSArray<ObjectType>,
    }

    #[cfg(feature = "Foundation_NSArray")]
    unsafe impl<ObjectType: Message> ClassType for NSMutableArray<ObjectType> {
        #[inherits(NSObject)]
        type Super = NSArray<ObjectType>;
        type Mutability = MutableWithImmutableSuperclass<NSArray<ObjectType>>;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

__inner_extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSDictionary")]
    pub struct NSDictionary<KeyType: Message = AnyObject, ObjectType: Message = AnyObject> {
        // SAFETY: Auto traits specified below.
        __superclass: UnsafeIgnoreAutoTraits<NSObject>,
        // Same as if the dictionary was implemented with:
        // `(NSArray<KeyType>, NSArray<ObjectType>)`
        __inner: PhantomData<(Id<KeyType>, Id<ObjectType>)>,
    }

    #[cfg(feature = "Foundation_NSDictionary")]
    unsafe impl<KeyType: Message, ObjectType: Message> ClassType for NSDictionary<KeyType, ObjectType> {
        type Super = NSObject;
        type Mutability = ImmutableWithMutableSubclass<NSMutableDictionary<KeyType, ObjectType>>;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass.0
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass.0
        }
    }
);

__inner_extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSDictionary")]
    pub struct NSMutableDictionary<KeyType: Message = AnyObject, ObjectType: Message = AnyObject> {
        // Inherit auto traits from superclass.
        __superclass: NSDictionary<KeyType, ObjectType>,
    }

    #[cfg(feature = "Foundation_NSDictionary")]
    unsafe impl<KeyType: Message, ObjectType: Message> ClassType
        for NSMutableDictionary<KeyType, ObjectType>
    {
        #[inherits(NSObject)]
        type Super = NSDictionary<KeyType, ObjectType>;
        type Mutability = MutableWithImmutableSuperclass<NSDictionary<KeyType, ObjectType>>;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

__inner_extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSSet")]
    pub struct NSSet<ObjectType: Message = AnyObject> {
        // SAFETY: Auto traits specified below.
        __superclass: UnsafeIgnoreAutoTraits<NSObject>,
        // Same as if the set was implemented as `NSArray<ObjectType>`.
        __inner: PhantomData<Id<ObjectType>>,
    }

    #[cfg(feature = "Foundation_NSSet")]
    unsafe impl<ObjectType: Message> ClassType for NSSet<ObjectType> {
        type Super = NSObject;
        type Mutability = ImmutableWithMutableSubclass<NSMutableSet<ObjectType>>;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass.0
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass.0
        }
    }
);

__inner_extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSSet")]
    pub struct NSMutableSet<ObjectType: Message = AnyObject> {
        // Inherit auto traits from superclass.
        __superclass: NSSet<ObjectType>,
    }

    #[cfg(feature = "Foundation_NSSet")]
    unsafe impl<ObjectType: Message> ClassType for NSMutableSet<ObjectType> {
        #[inherits(NSObject)]
        type Super = NSSet<ObjectType>;
        type Mutability = MutableWithImmutableSuperclass<NSSet<ObjectType>>;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSCountedSet")]
    pub struct NSCountedSet<ObjectType: Message = AnyObject> {
        // Inherit auto traits from superclass.
        __superclass: NSMutableSet<ObjectType>,
    }

    #[cfg(feature = "Foundation_NSCountedSet")]
    unsafe impl<ObjectType: Message> ClassType for NSCountedSet<ObjectType> {
        #[inherits(NSSet<ObjectType>, NSObject)]
        type Super = NSMutableSet<ObjectType>;
        type Mutability = Mutable;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

__inner_extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSOrderedSet")]
    pub struct NSOrderedSet<ObjectType: Message = AnyObject> {
        // SAFETY: Auto traits specified below.
        __superclass: UnsafeIgnoreAutoTraits<NSObject>,
        // Same as if the set was implemented with `NSArray<ObjectType>`.
        __inner: PhantomData<Id<ObjectType>>,
    }

    #[cfg(feature = "Foundation_NSOrderedSet")]
    unsafe impl<ObjectType: Message> ClassType for NSOrderedSet<ObjectType> {
        type Super = NSObject;
        type Mutability = ImmutableWithMutableSubclass<NSMutableOrderedSet<ObjectType>>;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass.0
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass.0
        }
    }
);

__inner_extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSOrderedSet")]
    pub struct NSMutableOrderedSet<ObjectType: Message = AnyObject> {
        // Inherit auto traits from superclass.
        __superclass: NSOrderedSet<ObjectType>,
    }

    #[cfg(feature = "Foundation_NSOrderedSet")]
    unsafe impl<ObjectType: Message> ClassType for NSMutableOrderedSet<ObjectType> {
        #[inherits(NSObject)]
        type Super = NSOrderedSet<ObjectType>;
        type Mutability = MutableWithImmutableSuperclass<NSOrderedSet<ObjectType>>;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSEnumerator")]
    pub struct NSEnumerator<ObjectType: Message = AnyObject> {
        // SAFETY: Auto traits specified below.
        __superclass: UnsafeIgnoreAutoTraits<NSObject>,
        // Enumerators are basically the same as if we were storing
        // `NSMutableArray<ObjectType>`, and removed an element from that on
        // each iteration.
        //
        // This is only true because everything in this file has that type of
        // ownership; if something didn't, and instead had e.g. `Arc<T>`-like
        // ownership, we'd have to modify the ownership of enumerators to
        // match with that (if we wanted to safely use enumerators returned by
        // that type).
        __inner: PhantomData<Id<ObjectType>>,
    }

    #[cfg(feature = "Foundation_NSEnumerator")]
    unsafe impl<ObjectType: Message> ClassType for NSEnumerator<ObjectType> {
        type Super = NSObject;
        type Mutability = Mutable;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass.0
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass.0
        }
    }
);
