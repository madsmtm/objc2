#![cfg(feature = "Foundation_NSDictionary")]
use alloc::vec::Vec;
use core::cmp::min;
use core::fmt;
use core::mem;
use core::ops::Index;
use core::panic::{RefUnwindSafe, UnwindSafe};
use core::ptr::{self, NonNull};

use objc2::rc::{DefaultId, SliceId};
use objc2::{msg_send, msg_send_id};

use crate::common::*;
use crate::Foundation::{self, NSDictionary};

// TODO: SAFETY
// Approximately same as `NSArray<T, Shared>`
unsafe impl<K: Message + Sync + Send, V: Message + Sync + Send> Sync for NSDictionary<K, V> {}
unsafe impl<K: Message + Sync + Send, V: Message + Sync + Send> Send for NSDictionary<K, V> {}

#[cfg(feature = "Foundation_NSMutableDictionary")]
unsafe impl<K: Message + Sync + Send, V: Message + Sync + Send> Sync
    for Foundation::NSMutableDictionary<K, V>
{
}
#[cfg(feature = "Foundation_NSMutableDictionary")]
unsafe impl<K: Message + Sync + Send, V: Message + Sync + Send> Send
    for Foundation::NSMutableDictionary<K, V>
{
}

// Approximately same as `NSArray<T, Shared>`
impl<K: Message + RefUnwindSafe, V: Message + RefUnwindSafe> UnwindSafe for NSDictionary<K, V> {}
impl<K: Message + RefUnwindSafe, V: Message + RefUnwindSafe> RefUnwindSafe for NSDictionary<K, V> {}

#[cfg(feature = "Foundation_NSMutableDictionary")]
impl<K: Message + RefUnwindSafe, V: Message + RefUnwindSafe> UnwindSafe
    for Foundation::NSMutableDictionary<K, V>
{
}
// impl<K: Message + RefUnwindSafe, V: Message + RefUnwindSafe> RefUnwindSafe for NSMutableDictionary<K, V> {}

extern_methods!(
    unsafe impl<K: Message, V: Message> NSDictionary<K, V> {
        #[method_id(new)]
        pub fn new() -> Id<Self, Shared>;

        pub fn len(&self) -> usize {
            self.count()
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        #[doc(alias = "objectForKey:")]
        #[method(objectForKey:)]
        pub fn get(&self, key: &K) -> Option<&V>;

        #[doc(alias = "getObjects:andKeys:")]
        pub fn keys(&self) -> Vec<&K> {
            let len = self.len();
            let mut keys: Vec<NonNull<K>> = Vec::with_capacity(len);
            unsafe {
                #[allow(deprecated)]
                self.getObjects_andKeys(ptr::null_mut(), keys.as_mut_ptr());
                keys.set_len(len);
                mem::transmute(keys)
            }
        }

        #[doc(alias = "getObjects:andKeys:")]
        pub fn values(&self) -> Vec<&V> {
            let len = self.len();
            let mut vals: Vec<NonNull<V>> = Vec::with_capacity(len);
            unsafe {
                #[allow(deprecated)]
                self.getObjects_andKeys(vals.as_mut_ptr(), ptr::null_mut());
                vals.set_len(len);
                mem::transmute(vals)
            }
        }

        #[doc(alias = "getObjects:andKeys:")]
        pub fn keys_and_objects(&self) -> (Vec<&K>, Vec<&V>) {
            let len = self.len();
            let mut keys: Vec<NonNull<K>> = Vec::with_capacity(len);
            let mut objs: Vec<NonNull<V>> = Vec::with_capacity(len);
            unsafe {
                #[allow(deprecated)]
                self.getObjects_andKeys(objs.as_mut_ptr(), keys.as_mut_ptr());
                keys.set_len(len);
                objs.set_len(len);
                (mem::transmute(keys), mem::transmute(objs))
            }
        }

        #[doc(alias = "keyEnumerator")]
        #[cfg(feature = "Foundation_NSEnumerator")]
        pub fn iter_keys(&self) -> Foundation::NSEnumerator2<'_, K> {
            unsafe {
                let result = msg_send![self, keyEnumerator];
                Foundation::NSEnumerator2::from_ptr(result)
            }
        }

        #[doc(alias = "objectEnumerator")]
        #[cfg(feature = "Foundation_NSEnumerator")]
        pub fn iter_values(&self) -> Foundation::NSEnumerator2<'_, V> {
            unsafe {
                let result = msg_send![self, objectEnumerator];
                Foundation::NSEnumerator2::from_ptr(result)
            }
        }

        pub fn from_keys_and_objects<T>(keys: &[&T], vals: Vec<Id<V, Owned>>) -> Id<Self, Shared>
        where
            T: Foundation::NSCopying<Output = K>,
        {
            let vals = vals.as_slice_ref();

            let count = min(keys.len(), vals.len());
            unsafe {
                msg_send_id![
                    Self::alloc(),
                    initWithObjects: vals.as_ptr(),
                    forKeys: keys.as_ptr(),
                    count: count,
                ]
            }
        }
    }
);

impl<K: Message, V: Message> DefaultId for NSDictionary<K, V> {
    type Ownership = Shared;

    #[inline]
    fn default_id() -> Id<Self, Self::Ownership> {
        Self::new()
    }
}

unsafe impl<K: Message, V: Message> Foundation::NSFastEnumeration2 for NSDictionary<K, V> {
    type Item = K;
}

impl<'a, K: Message, V: Message> Index<&'a K> for NSDictionary<K, V> {
    type Output = V;

    fn index<'s>(&'s self, index: &'a K) -> &'s V {
        self.get(index).unwrap()
    }
}

#[cfg(feature = "Foundation_NSEnumerator")]
impl<K: fmt::Debug + Message, V: fmt::Debug + Message> fmt::Debug for NSDictionary<K, V> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = self.iter_keys().zip(self.iter_values());
        f.debug_map().entries(iter).finish()
    }
}
