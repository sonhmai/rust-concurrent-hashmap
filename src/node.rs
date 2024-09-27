use crossbeam::epoch::Atomic;

pub(crate) struct Node<K, V> {
    key: K,
    value: Atomic<V>,
}