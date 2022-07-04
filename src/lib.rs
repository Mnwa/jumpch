use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

/// `JumpHasher` is the wrapper for `Jump Consistent Hash` that implementing `Hasher` trait.
/// `MUST TO KNOW`: This implementation finishes never return value more than `u32`.
///
/// Example:
/// ```rust
/// use std::collections::hash_map::DefaultHasher;
/// use std::hash::{Hash, Hasher};
/// use jumpch::JumpHasher;
///
/// let mut hasher: JumpHasher<DefaultHasher> = JumpHasher::new(1000);
///
/// 123456i32.hash(&mut hasher);
///
/// assert_eq!(hasher.finish(), 179)
/// ```
#[derive(Copy, Clone, Debug)]
pub struct JumpHasher<H = DefaultHasher> {
    slots: u32,
    hasher: H,
}

impl<H: Hasher> JumpHasher<H> {
    /// Create new JumpHasher with custom hasher
    /// ```rust
    /// use std::collections::hash_map::DefaultHasher;
    /// use jumpch::JumpHasher;
    ///
    /// let hasher = JumpHasher::new_with_hasher(1000, DefaultHasher::new());
    pub fn new_with_hasher(slots: u32, hasher: H) -> Self {
        Self { slots, hasher }
    }
}

impl<H: Hasher + Default> JumpHasher<H> {
    /// Create new JumpHasher with default hasher
    /// ```rust
    /// use std::collections::hash_map::DefaultHasher;
    /// use jumpch::JumpHasher;
    ///
    /// let hasher = JumpHasher::new(1000);
    pub fn new(slots: u32) -> Self {
        Self {
            slots,
            hasher: H::default(),
        }
    }
}

impl<H: Hasher> Hasher for JumpHasher<H> {
    fn finish(&self) -> u64 {
        hash(self.hasher.finish(), self.slots as i64) as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        self.hasher.write(bytes)
    }
}

/// The base realization of `Jump Consistent Hash` algorithm.
/// Usage example:
/// ```rust
/// use jumpch::hash;
///
/// assert_eq!(hash(123456, 1000), 176)
/// ```
pub fn hash(mut key: u64, slots: i64) -> u32 {
    let (mut b, mut j) = (-1i64, 0i64);
    while j < slots {
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
    fn get_slot_1() {
        let test = 123456;
        check_range(test);
    }

    #[test]
    fn get_slot_2() {
        let test = 654321;
        check_range(test);
    }

    #[test]
    fn get_slot_3() {
        let test = 123456;
        check_range(test);
    }

    fn check_range(test: usize) {
        for slots in 0..1000 {
            check_algorithm(slots, test);
        }
    }

    fn check_algorithm(slots: u32, test: usize) {
        let mut hasher: JumpHasher<DefaultHasher> = JumpHasher::new(slots);
        test.hash(&mut hasher);
        let hash = hasher.finish();

        for i in (hash.wrapping_add(1) as u32)..slots.max(1) {
            let mut hasher = JumpHasher::<DefaultHasher>::new(i);

            test.hash(&mut hasher);

            assert_eq!(hasher.finish(), hash)
        }
    }
}
