//! Fuzz hashing collection operations with interior mutability.
//!
//! This is explicitly not done with any form of oracle, since while this is
//! not language-level undefined behaviour, the behaviour is not specified,
//! and will "corrupt the collection", and may behave differently on different
//! versions of Foundation:
//! <https://developer.apple.com/library/archive/documentation/General/Conceptual/CocoaEncyclopedia/ObjectMutability/ObjectMutability.html#//apple_ref/doc/uid/TP40010810-CH5-SW69>
#![cfg_attr(not(feature = "afl"), no_main)]
use std::cell::Cell;
use std::hint::black_box;

use arbitrary::Arbitrary;
use objc2::rc::{autoreleasepool, Id, Retained};
use objc2::runtime::AnyObject;
use objc2::{declare_class, msg_send_id, AllocAnyThread, ClassType, DeclaredClass, Message};
use objc2_foundation::{
    CopyingHelper, NSCopying, NSMutableDictionary, NSMutableSet, NSObject, NSObjectProtocol,
    NSUInteger, NSZone,
};

/// Index into the global "keys" array.
type KeyIndex = u8;

/// The operations that the fuzzer can do on a set and the keys within.
#[derive(Arbitrary, Debug)]
enum Operation {
    /// count
    Count,
    /// member: / objectForKey:
    Get(KeyIndex),
    /// objectEnumerator / keyEnumerator
    Enumerate,
    /// addObject: / setObject:forKey:
    Add(KeyIndex),
    /// removeObject: / removeObjectForKey:
    Remove(KeyIndex),

    /// Set the hash value of a key.
    SetHash(KeyIndex, NSUInteger),
    /// Set which other key masks this key is equal to.
    SetEqualToMask(KeyIndex, u8),
}

struct KeyIvars {
    index: KeyIndex,
    hash: Cell<usize>,
    equal_to_mask: Cell<u8>,
}

declare_class!(
    struct Key;

    unsafe impl ClassType for Key {
        type Super = NSObject;
        const NAME: &'static str = "Key";
    }

    impl DeclaredClass for Key {
        type Ivars = KeyIvars;
    }

    unsafe impl NSObjectProtocol for Key {
        #[method(isEqual:)]
        fn is_equal(&self, other: &AnyObject) -> bool {
            assert_eq!(other.class(), Self::class());
            let other: *const AnyObject = other;
            let other: *const Self = other.cast();
            // SAFETY: Just checked that the object is of this class
            let other: &Self = unsafe { &*other };

            (other.ivars().index & self.ivars().equal_to_mask.get()) != 0
        }

        #[method(hash)]
        fn hash_(&self) -> NSUInteger {
            self.ivars().hash.get()
        }
    }

    unsafe impl NSCopying for Key {
        #[method_id(copyWithZone:)]
        fn copy_with_zone(&self, _zone: *mut NSZone) -> Retained<Self> {
            self.retain()
        }
    }
);

unsafe impl CopyingHelper for Key {
    type Result = Self;
}

impl Key {
    fn new(index: KeyIndex) -> Retained<Self> {
        let key = Key::alloc().set_ivars(KeyIvars {
            index,
            hash: Cell::new(0),
            equal_to_mask: Cell::new(0),
        });
        unsafe { msg_send_id![super(key), init] }
    }

    fn validate(&self) {
        black_box(self.ivars().index);
        black_box(self.ivars().hash.get());
    }
}

fn run(ops: Vec<Operation>) {
    let keys: Vec<_> = (0..=KeyIndex::MAX).map(Key::new).collect();
    let key = |idx: KeyIndex| -> &Key { &keys[idx as usize] };

    let set: Id<NSMutableSet<Key>> = NSMutableSet::new();
    let dict: Id<NSMutableDictionary<Key, NSObject>> = NSMutableDictionary::new();

    for op in ops {
        autoreleasepool(|_| match op {
            Operation::Count => {
                set.count();
                dict.count();
            }
            Operation::Get(idx) => {
                set.member(key(idx));
                dict.objectForKey(key(idx));
            }
            Operation::Enumerate => {
                for key in unsafe { set.objectEnumerator() } {
                    key.validate();
                }
                for key in &set {
                    key.validate();
                }
                for key in unsafe { dict.keyEnumerator() } {
                    key.validate();
                }
            }
            Operation::Add(idx) => {
                set.addObject(key(idx));
                dict.insert(key(idx), &NSObject::new());
            }
            Operation::Remove(idx) => {
                set.removeObject(key(idx));
                dict.removeObjectForKey(key(idx));
            }
            Operation::SetHash(idx, hash) => {
                key(idx).ivars().hash.set(hash);
            }
            Operation::SetEqualToMask(idx, equal_to_mask) => {
                key(idx).ivars().equal_to_mask.set(equal_to_mask);
            }
        });
    }
}

#[cfg(not(feature = "afl"))]
libfuzzer_sys::fuzz_target!(|ops: Vec<Operation>| run(ops));

#[cfg(feature = "afl")]
fn main() {
    afl::fuzz!(|ops: Vec<Operation>| {
        run(ops);
    });
}
