use crate::node::Node;

/// Each key-value mapping is held in a BinEntry.
///
/// Types of BinEntry
/// - Most nodes are of type `BinEntry::Node` with hash, key, value, and a `next` field.
/// - some nodes are of type `BinEntry::Moved`; these "forwarding nodes" are placed at the
/// heads of bins during resizing.
///
/// Java version also has other special node types, not implemented yet.
/// These special nodes are either uncommon or transient.
/// TODO: TreeNode, ReservationNode
pub enum BinEntry<K, V> {
    Node(Node<K, V>),
    Moved
}

