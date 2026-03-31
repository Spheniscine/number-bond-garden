use hexx::{Hex, storage::HexagonalMap};
use rand::{RngExt, seq::SliceRandom};
use serde::{Deserialize, Serialize};

use crate::game::{BOARD_RADIUS, Board, NUM_ORBS};


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameState {
    pub board: Board,
}

impl GameState {
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

        Self {
            board: Board { inner: board }
        }
    }
}