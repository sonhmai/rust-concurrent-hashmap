# Module core::hash

Trait
1. BuildHasher: trait for creating instances of Hasher
2. Hash: hashable type
3. Hasher: hashing an arbitrary stream of bytes


### std::collections::hash_map::`RandomState`

1. is the default state for HashMap types.
2. A particular instance `RandomState` will create the same instances of `Hasher`
3. but the hashers created by two `different RandomState` instances are `unlikely` to produce the same result for the same values.

```rust
use std::collections::HashMap;
use std::hash::RandomState;;

let s = RandomState::new();
let mut map = HashMap::with_hasher(s);
map.insert(1, 2);
```

### trait core::hash::BuildHasher

```rust
use std::hash::{BuildHasher, Hasher, RandomState};

let s = RandomState::new();
let mut hasher_1 = s.build_hasher();
let mut hasher_2 = s.build_hasher();

hasher_1.write_u32(8128);
hasher_2.write_u32(8128);

assert_eq!(hasher_1.finish(), hasher_2.finish());
```