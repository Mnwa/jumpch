# Jump Consistent Hash algorithm
[![](https://docs.rs/jumpch/badge.svg)](https://docs.rs/jumpch/)
[![](https://img.shields.io/crates/v/jumpch.svg)](https://crates.io/crates/jumpch)
[![](https://img.shields.io/crates/d/jumpch.svg)](https://crates.io/crates/jumpch)

> Jump Consistent Hashing is a fast, minimal memory, consistent hash algorithm. In comparison to the algorithm of Karger et al., jump
consistent hash requires no storage, is faster, and does a better job of evenly dividing the key
space among the buckets and of evenly dividing the workload when the number of buckets
changes.

## Usage
```rust
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use jumpch::JumpHasher;

fn main () {
    let mut hasher: JumpHasher<DefaultHasher> = JumpHasher::new(1000);

    "test".hash(&mut hasher);

    assert_eq!(hasher.finish(), 677)
}
```

```rust
use jumpch::hash;

fn main () {
    assert_eq!(hash(123456, 1000), 176)
}
```

## Contributing
Any PR's and issues are welcome.

## License
This lib is distributed by `MIT` and `Apache 2.0` licenses. Just choose what's you want.