# Jump Consistent Hash for Rust (jumpch)
[![](https://docs.rs/jumpch/badge.svg)](https://docs.rs/jumpch/)
[![](https://img.shields.io/crates/v/jumpch.svg)](https://crates.io/crates/jumpch)
[![](https://img.shields.io/crates/d/jumpch.svg)](https://crates.io/crates/jumpch)

Jump Consistent Hashing is a fast, minimal‑memory, consistent hashing algorithm. Compared to the
classic algorithm by Karger et al., Jump Consistent Hash requires no storage, runs faster, and better
evens out keys across buckets while minimizing remapping when the number of buckets changes.

This crate provides a tiny, dependency‑free implementation suitable for sharding, partitioning, load
balancing, and cache key distribution.

- Deterministic mapping from key → bucket index
- O(1) time, O(1) space
- No allocations

## Install
Add this to your Cargo.toml:

```toml
[dependencies]
jumpch = "*"
```

## Quick start
High‑level adapter implementing `Hasher`:

```rust
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use jumpch::JumpHasher;

fn main() {
    let mut hasher: JumpHasher<DefaultHasher> = JumpHasher::new(1000);
    "test".hash(&mut hasher);
    let bucket = hasher.finish(); // 0..1000
    assert!(bucket < 1000);
}
```

Low‑level function if you already have a 64‑bit key:

```rust
fn main() {
    let bucket = jumpch::hash(123456u64, 1000u32);
    assert!(bucket < 1000);
}
```

## When to use Jump Consistent Hashing
- Distributing keys across shards/partitions
- Client‑side load balancing across N backends
- Cache clustering with minimal key movement when nodes change

## API overview
- `JumpHasher<H>`: a `Hasher` adapter that yields the bucket index via `finish()`.
- `hash(key, slots) -> u32`: compute the bucket directly.
- `Slots(u32)`: newtype wrapper; panics on `0`.

## FAQ
- What range does the result fall into?
  The bucket index is always `< slots`.
- Is the output stable?
  Yes, for a fixed number of `slots`, the same key always maps to the same bucket.

## Contributing
PRs and issues are welcome.

## License
Dual‑licensed under MIT and Apache‑2.0; choose either license.