use std::hash::{BuildHasher, Hash, Hasher, RandomState};
use std::sync::atomic::AtomicUsize;
use crossbeam::epoch::Atomic;
use crate::table::Table;

pub struct HashMap<K, V> {
    table: Atomic<Table<K, V>>,
    count: AtomicUsize,
    build_hasher: RandomState,
}

impl<K, V> HashMap<K, V>
where
    K: Hash,
{
    pub fn new() -> Self {
        Self {
            table: Atomic::null(),
            count: AtomicUsize::new(0),
            build_hasher: RandomState::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.put(key, value);
        None
    }

    fn put(&self, key: K, value: V) -> Option<V> {
        let h = self.hash(&key);
        None
    }

    fn hash(&self, key: &K) -> u64 {
        let mut hasher = self.build_hasher.build_hasher();
        key.hash(&mut hasher);
        hasher.finish()
    }
}