use serde::{Deserialize, Serialize};

use crate::game::{Board, ThreadRng};


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameState {
    pub board: Board,
}

impl GameState {
    pub fn test_gen() -> Self {
        let rng = &mut ThreadRng;
        Self {
            board: Board::test_gen(rng)
        }
    }
}