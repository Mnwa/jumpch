//! Jumpch: Jump Consistent Hashing for Rust
//!
//! A tiny, fast, and allocation‑free implementation of the Jump Consistent Hash algorithm.
//! Use it to map arbitrary keys (strings, numbers, structs) to a stable bucket index when the
//! number of buckets changes over time (e.g., sharding, partitioning, load balancing).
//!
//! Key points
//! - Deterministic: the same key always maps to the same bucket for a fixed number of slots.
//! - Minimal memory and very fast.
//! - Great for distributed systems and caches where nodes/partitions change.
//!
//! Quick example
//! ```rust
//! use std::collections::hash_map::DefaultHasher;
//! use std::hash::{Hash, Hasher};
//! use jumpch::JumpHasher;
//!
//! let mut hasher: JumpHasher<DefaultHasher> = JumpHasher::new(1000);
//! "some-key".hash(&mut hasher);
//! let bucket = hasher.finish(); // in range 0..1000
//! println!("bucket = {}", bucket);
//! ```
//!
//! Low‑level function
//! ```rust
//! let bucket = jumpch::hash(123456u64, 1000u32);
//! assert!(bucket < 1000);
//! ```

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

/// A `Hasher` adapter that turns any standard hasher into a Jump Consistent Hash bucket picker.
///
/// Jump Consistent Hashing maps keys to an integer bucket index in a way that minimizes
/// remapping when the number of buckets (slots) changes. `JumpHasher` implements the
/// standard `Hasher` trait so you can reuse regular `Hash` implementations and still
/// retrieve a stable bucket index with `finish()`.
///
/// Notes
/// - `finish()` returns a `u64`, but its value is always in the range `0..slots` and fits
///   into `u32` because the algorithm’s output is a bucket index.
/// - The mapping is deterministic for the same key and number of slots.
///
/// Example
/// ```rust
/// use std::collections::hash_map::DefaultHasher;
/// use std::hash::{Hash, Hasher};
/// use jumpch::JumpHasher;
///
/// let mut hasher: JumpHasher<DefaultHasher> = JumpHasher::new(1000);
/// "test".hash(&mut hasher);
/// let bucket = hasher.finish();
/// assert!(bucket < 1000);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct JumpHasher<H = DefaultHasher> {
    slots: Slots,
    hasher: H,
}

impl<H: Hasher> JumpHasher<H> {
    /// Create a new `JumpHasher` with a custom underlying hasher.
    ///
    /// Parameters
    /// - `slots`: Number of buckets (> 0). Accepts `u32` directly or a [`Slots`] wrapper.
    /// - `hasher`: The underlying `Hasher` that receives the key bytes.
    ///
    /// Example
    /// ```rust
    /// use std::collections::hash_map::DefaultHasher;
    /// use jumpch::JumpHasher;
    ///
    /// let hasher = JumpHasher::new_with_hasher(1000u32, DefaultHasher::new());
    /// ```
    pub fn new_with_hasher<S: Into<Slots>>(slots: S, hasher: H) -> Self {
        Self {
            slots: slots.into(),
            hasher,
        }
    }
}

impl<H: Hasher + Default> JumpHasher<H> {
    /// Create a new `JumpHasher` using the type’s `Default` hasher.
    ///
    /// Example
    /// ```rust
    /// use std::collections::hash_map::DefaultHasher;
    /// use jumpch::JumpHasher;
    ///
    /// let _hasher = JumpHasher::<DefaultHasher>::new(1000u32);
    /// ```
    pub fn new<S: Into<Slots>>(slots: S) -> Self {
        Self::new_with_hasher(slots, H::default())
    }
}

impl<H: Hasher> Hasher for JumpHasher<H> {
    fn finish(&self) -> u64 {
        hash(self.hasher.finish(), self.slots) as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        self.hasher.write(bytes)
    }
}

/// Wrapper type for the number of buckets (also called “slots”).
///
/// This is a thin newtype over `u32` used to make APIs more expressive. You can
/// pass a plain `u32` wherever a `Slots` is expected via `Into<Slots>`.
///
/// Invariants
/// - Value must be greater than 0.
#[derive(Copy, Clone, Debug)]
pub struct Slots(u32);

impl From<u32> for Slots {
    /// Creates `Slots` from a positive `u32`.
    ///
    /// Panics
    /// - If `value == 0`.
    fn from(value: u32) -> Self {
        assert!(value > 0, "slots must be greater than 0");
        Self(value)
    }
}

/// Computes the Jump Consistent Hash bucket index for a given `key` and number of `slots`.
///
/// Properties
/// - Deterministic: same input key and slot count map to the same bucket.
/// - Output range: `0..slots` (i.e., less than the number of slots).
/// - No memory allocations.
///
/// Example
/// ```rust
/// let bucket = jumpch::hash(123456u64, 1000u32);
/// assert!(bucket < 1000);
/// ```
pub fn hash<S: Into<Slots>>(mut key: u64, slots: S) -> u32 {
    let slots = slots.into();
    let (mut b, mut j) = (-1i64, 0i64);
    while j < slots.0 as i64 {
        b = j;
        key = key.wrapping_mul(2862933555777941757).wrapping_add(1);
        j = ((b.wrapping_add(1) as f64) * (((1u64 << 31) as f64) / (((key >> 33) + 1) as f64)))
            as i64;
    }
    b as u32
}

#[cfg(test)]
mod tests {
    use crate::JumpHasher;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[test]
    fn test_struct() {
        #[derive(Hash)]
        struct Test(i32, String);

        let test = Test(123456, "test".to_string());
        check_range(&test);
    }

    #[test]
    fn test_str() {
        let test = "test 1";
        check_range(&test);
    }

    #[test]
    fn test_int() {
        let test = 123456;
        check_range(&test);
    }

    fn check_range<H: Hash>(test: &H) {
        for slots in 1..1000 {
            check_algorithm(slots, test);
        }
    }

    fn check_algorithm<H: Hash>(slots: u32, test: H) {
        let mut hasher: JumpHasher = JumpHasher::new(slots);
        test.hash(&mut hasher);
        let hash = hasher.finish();

        for i in (hash.wrapping_add(1) as u32)..=slots {
            let mut hasher = JumpHasher::<DefaultHasher>::new(i);

            test.hash(&mut hasher);

            assert_eq!(hasher.finish(), hash)
        }
    }
}
