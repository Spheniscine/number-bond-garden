use std::ops::{Index, IndexMut};

use hexx::{Hex, storage::{HexStore, HexagonalMap}};
use rand::{Rng, RngExt, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use shash::SHash;

use crate::game::{BOARD_RADIUS, INITIAL_FREE_ORB_RANGE, NUM_DUPES, NUM_ORBS};

type IndexSet<T> = indexmap::IndexSet<T, SHash>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Board {
    pub inner: HexagonalMap<Option<u8>>
}

impl Index<Hex> for Board {
    type Output = Option<u8>;

    fn index(&self, index: Hex) -> &Self::Output {
        &self.inner[index]
    }
}

impl IndexMut<Hex> for Board {
    fn index_mut(&mut self, index: Hex) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.inner.bounds() == other.inner.bounds() && self.inner.values().eq(other.inner.values())
    }
}
impl Eq for Board {}

impl Board {
    pub fn generate_pattern(rng: &mut impl Rng) -> HexagonalMap<bool> {
        let mut pattern = HexagonalMap::new(Hex::ORIGIN, BOARD_RADIUS, |_| false);

        if rng.random::<bool>() {
            // 2-way symmetry
            const _: () = assert!(NUM_ORBS % 2 == 1);

            // For 2-way symmetry: limit hexes picked to those where (x, y) > (0, 0)
            let mut pool = pattern.bounds().all_coords().filter(|hex| {
                (hex.x, hex.y) > (0, 0)
            }).collect::<Vec<_>>();

            for &mut x in pool.partial_shuffle(rng, NUM_ORBS / 2).0 {
                pattern[x] = true;
                pattern[-x] = true;
            }
        } else {
            // 3-way symmetry
            const _: () = assert!(NUM_ORBS % 3 == 1);

            // For 3-way symmetry: limit hexes picked to those where x >= 0 && y < 0
            let mut pool = pattern.bounds().all_coords().filter(|hex| {
                hex.x >= 0 && hex.y < 0
            }).collect::<Vec<_>>();

            for &mut mut x in pool.partial_shuffle(rng, NUM_ORBS / 3).0 {
                for _ in 0..3 {
                    pattern[x] = true;
                    x = x.clockwise().clockwise();
                }
            }
        }

        pattern[Hex::ORIGIN] = true;
        pattern
    }

    fn is_free_inner(hex: Hex, is_filled: impl Fn(Hex) -> bool) -> bool {
        if !is_filled(hex) { return false; }
        let neighbors_filled = hex.all_neighbors().map(|nb| is_filled(nb));
        (0..6).any(|i| {
            (i .. i+3).all(|j| !neighbors_filled[j % 6])
        })
    }

    fn is_free_pattern(pattern: &HexagonalMap<bool>, hex: Hex) -> bool {
        Self::is_free_inner(hex, |hex| pattern.get(hex).copied().unwrap_or(false))
    }

    pub fn is_free(&self, hex: Hex) -> bool {
        Self::is_free_inner(hex, |hex| self.inner.get(hex).copied().flatten().is_some())
    }

    pub fn _pattern_stats(n: i32, rng: &mut impl Rng) -> [i32; NUM_ORBS + 1] {
        let mut ans = [0; NUM_ORBS + 1];
        
        for _ in 0..n {
            let pattern = Self::generate_pattern(rng);
            let active_count = pattern.bounds().all_coords().filter(|&hex| {
                Self::is_free_pattern(&pattern, hex)
            }).count();
            ans[active_count] += 1;
        }

        ans
    }

    fn free_hexes_in_pattern(pattern: &HexagonalMap<bool>) -> IndexSet<Hex> {
        pattern.bounds().all_coords().filter(|&hex| {
            Self::is_free_pattern(&pattern, hex)
        }).collect()
    }

    pub fn test_gen(rng: &mut impl Rng) -> Self {
        'gen: loop {
            let mut pattern = Self::generate_pattern(rng);
            let mut free = Self::free_hexes_in_pattern(&pattern);
            if !INITIAL_FREE_ORB_RANGE.contains(&free.len()) { continue; }

            const _: () = assert!(NUM_DUPES % 2 == 0);
            let mut pool = std::iter::repeat((1u8 ..= 4).map(|i| [i, 10 - i]))
                .take(NUM_DUPES).flatten()
                .chain(
                    std::iter::repeat([5, 5]).take(NUM_DUPES / 2)
                ).collect::<Vec<_>>();
            
            pool.shuffle(rng);

            let mut board = HexagonalMap::new(Hex::ORIGIN, BOARD_RADIUS, |_| { None });
            loop {
                if free.swap_remove(&Hex::ORIGIN) {
                    pattern[Hex::ORIGIN] = false;
                    for hex in Hex::ORIGIN.all_neighbors() {
                        if Self::is_free_pattern(&pattern, hex) { free.insert(hex); }
                    }
                }

                let Some(orbs) = pool.pop() else {break};

                if free.len() < 2 {
                    tracing::info!("Shouldn't happen: free orbs < 2 when generating orbs");
                    continue 'gen;
                }

                let a = free.swap_remove_index(rng.random_range(0 .. free.len())).unwrap();
                let b = free.swap_remove_index(rng.random_range(0 .. free.len())).unwrap();
                board[a] = Some(orbs[0]);
                board[b] = Some(orbs[1]);

                for a in [a, b] {
                    pattern[a] = false;
                }
                for a in [a, b] {
                    for hex in a.all_neighbors() {
                        if Self::is_free_pattern(&pattern, hex) { free.insert(hex); }
                    }
                }
            }

            if !pool.is_empty() {
                tracing::info!("missing? {:?}", &pool);
            }

            board[Hex::ORIGIN] = Some(10);

            return Board { inner: board };
        }
    }

    pub fn count_free(&self) -> usize {
        self.inner.bounds().all_coords().filter(|&hex| {
            self.is_free(hex)
        }).count()
    }
}