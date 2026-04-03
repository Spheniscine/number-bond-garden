use std::{cell::Cell, convert::Infallible, hash::{BuildHasher, Hash, Hasher, RandomState}};
use rand::TryRng;
use web_time::Instant;

fn generate_seed() -> u64 {
    let state = RandomState::new();
    let mut hasher = state.build_hasher();
    Instant::now().hash(&mut hasher);
    hasher.finish()
}

// RNG from https://github.com/tkaitchuck/Mwc256XXA64
#[derive(Debug, Clone)]
struct InnerRng {
    state: Cell<[u64; 4]>
}
impl InnerRng {
    fn from_seed(s0: u64, s1: u64, s2: u64) -> Self { 
        let res = Self { state: Cell::new([s0, s1, s2 << 2 | 1, 0x14057B7EF767814F]) };
        for _ in 0..6 { res.gen_64(); }
        res
    }
    fn new() -> Self { Self::from_seed(generate_seed(), generate_seed(), generate_seed()) }
    fn gen_64(&self) -> u64 {
        let [x1, x2, x3, c] = self.state.get();
        let t = (x3 as u128).wrapping_mul(0xfeb3_4465_7c0a_f413);
        let (low, hi) = (t as u64, (t >> 64) as u64);
        let res = (x3 ^ x2).wrapping_add(x1 ^ hi);
        let (x0, b) = low.overflowing_add(c);
        self.state.set([x0, x1, x2, hi.wrapping_add(b as u64)]);
        res
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ThreadRng;

impl TryRng for ThreadRng {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        self.try_next_u64().map(|x| x as u32)
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        thread_local! {
            static RNG: InnerRng = InnerRng::new();
        }
        Ok(RNG.with(|rng| rng.gen_64()))
    }

    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        rand_core::utils::fill_bytes_via_next_word(dst, || self.try_next_u64())
    }
}