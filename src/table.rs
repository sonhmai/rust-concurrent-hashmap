use crossbeam::epoch::Atomic;
use crate::bin_entry::BinEntry;

/// The underlying Table of the hashmap storing the bins of hash to list of bin entries.
/// Each bin entry is a key-value pair whose key is hashed to the has value of the bin.
///
/// Each bin in the table is a list of nodes `BinEntry`.
/// Table accesses require atomic reads, writers, and CASes.
pub struct Table<K, V> {
    bins: Atomic<BinEntry<K, V>>,
}