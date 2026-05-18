use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasherDefault, Hasher};

use crate::grid::Position;

// FxHash-style hasher: rotate-xor-mul keeps small i32 keys cheap to hash
// while still distributing well — default SipHash is ~3x slower for this.
const K: u64 = 0x517c_c1b7_2722_0a95;

#[derive(Default)]
pub struct FxHasher {
    hash: u64,
}

impl Hasher for FxHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.hash = (self.hash.rotate_left(5) ^ u64::from(b)).wrapping_mul(K);
        }
    }

    #[inline]
    fn write_u32(&mut self, n: u32) {
        self.hash = (self.hash.rotate_left(5) ^ u64::from(n)).wrapping_mul(K);
    }

    #[inline]
    fn write_i32(&mut self, n: i32) {
        self.write_u32(n as u32);
    }

    #[inline]
    fn write_u64(&mut self, n: u64) {
        self.hash = (self.hash.rotate_left(5) ^ n).wrapping_mul(K);
    }
}

pub type BuildPosHasher = BuildHasherDefault<FxHasher>;
pub type LiveSet = HashSet<Position, BuildPosHasher>;
pub type PosMap<V> = HashMap<Position, V, BuildPosHasher>;
