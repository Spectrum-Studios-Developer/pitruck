use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[inline(always)]
pub fn hash_name(name: &str) -> u64 {
    let mut h = DefaultHasher::new();
    name.hash(&mut h);
    h.finish()
}