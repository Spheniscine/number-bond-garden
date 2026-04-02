use std::ops::{Index, IndexMut};

use hexx::{Hex, storage::{HexStore, HexagonalMap}};
use rand::{RngExt, seq::SliceRandom};
use serde::{Deserialize, Serialize};

use crate::game::{BOARD_RADIUS, NUM_ORBS};

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
    pub fn test_gen() -> Self {
        let mut pattern = HexagonalMap::new(Hex::ORIGIN, BOARD_RADIUS, |_| false);
        let rng = &mut rand::rng();

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

        // TODO: randomly fill for now, doesn't yet guarantee valid boards
        let mut board = HexagonalMap::new(Hex::ORIGIN, BOARD_RADIUS, |i| {
            if pattern[i] { Some(rng.random_range(1..=9)) } else { None }
        });
        board[Hex::ORIGIN] = Some(10);

        Board { inner: board }
    }
}