#![allow(dead_code)]
use core::ptr::NonNull;

use objc2::mutability::IsIdCloneable;
use objc2::rc::Id;
use objc2::Message;

pub(crate) fn id_ptr_cast<T: ?Sized>(objects: *mut Id<T>) -> *mut NonNull<T> {
    // SAFETY: `Id<T>` has the same memory layout as `NonNull<T>`, and
    // stronger guarantees.
    objects.cast()
}

fn ref_ptr_cast<T: ?Sized>(objects: *mut &T) -> *mut NonNull<T> {
    // SAFETY: `&T` has the same memory layout as `NonNull<T>`, and stronger
    // guarantees.
    objects.cast()
}

pub(crate) fn ref_ptr_cast_const<T: ?Sized>(objects: *const &T) -> *mut NonNull<T> {
    ref_ptr_cast(objects as _)
}

pub(crate) fn id_ptr_cast_const<T: ?Sized>(objects: *const Id<T>) -> *mut NonNull<T> {
    id_ptr_cast(objects as _)
}

// Foundation collection types store their items in such a way that they can
// give out references to their data without having to autorelease it first:
// <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/MemoryMgmt/Articles/mmPractical.html#//apple_ref/doc/uid/TP40004447-SW12>
//
// Hence functions can safely return references that are not bound to a pool
// (provided they are not modified outside `&mut`, which we implement them not
// to be).
//
// ---
//
// A way to verify the safety of the collection interfaces is to imagine their
// storage as an equivalent Rust type, and then try to write a (possibly
// inefficient) implementation of the method with that type.
//
// In table form:
// | Objective-C                    | Equivalent Rust storage  |
// | ------------------------------ | ------------------------ |
// | Id<NSArray<T>>                 | Arc<[Id<T>]>             |
// | Id<NSSet<T>>                   | Arc<[Id<T>]>             |
// | Id<NSDictionary<K, V>>         | Arc<[(Id<K>, Id<V>)]>    |
// | Id<NSMutableArray<T>>          | Vec<Id<T>>               |
// | Id<NSMutableSet<T>>            | Vec<Id<T>>               |
// | Id<NSMutableDictionary<K, V>>  | Vec<(Id<K>, Id<V>)>      |
// | ------------------------------ | ------------------------ |
// | &NSArray<T>                    | &[Id<T>]                 |
// | &NSDictionary<K, V>            | &[(Id<K>, Id<V>)]        |
// | &mut NSArray<T>                | &mut [Id<T>]             |
// | &mut NSMutableArray<T>         | &mut Vec<Id<T>>          |
// | &mut NSDictionary<K, V>        | &mut [(Id<K>, Id<V>)]    |
// | &mut NSMutableDictionary<K, V> | &mut Vec<(Id<K>, Id<V>)> |

/// We should be able to access `&Id<T>` from `&self` in collection types,
/// though that is not directly possible since we don't have raw access to the
/// internal allocation; so instead we use this helper function to allow
/// roughly the same functionality.
///
///
/// # Safety
///
/// The object must be stored inside a collection.
#[inline]
pub(crate) unsafe fn collection_retain_id<T>(obj: &T) -> Id<T>
where
    T: Message + IsIdCloneable,
{
    // SAFETY: We're allowed to access `&Id<T>` from `&self` in collections,
    // and since `T: IsIdCloneable`, we can convert that to `Id<T>`.
    unsafe { Id::retain(obj as *const T as *mut T).unwrap_unchecked() }
}

/// The mutable variants give us the extra benefit that we may get `Id<T>`
/// out, provided we've removed it from the collection first.
///
///
/// # Safety
///
/// The object must have just been removed from the mutable collection, or the
/// collection must be consumed right after this.
//
// TODO: Take a pointer here to avoid assuming `T` is immutable - this only
// works now because `T` is usually `UnsafeCell` anyway.
#[inline]
pub(crate) unsafe fn mutable_collection_retain_removed_id<T>(obj: &T) -> Id<T>
where
    T: Message,
{
    // SAFETY: The collection had mutable ownership over the object, and since
    // the object will never again be accessed from the collection, we can
    // convert it to `Id<T>`.
    unsafe { Id::retain(obj as *const T as *mut T).unwrap_unchecked() }
}
