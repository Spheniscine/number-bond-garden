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
        Self {
            board: Board::test_gen()
        }
    }
}