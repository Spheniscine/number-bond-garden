use serde::{Deserialize, Serialize};

use crate::game::{Board, Difficulty, ThreadRng};


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameState {
    pub difficulty: Difficulty,
    pub board: Board,
}

impl GameState {
    pub fn test_gen(difficulty: Difficulty) -> Self {
        let rng = &mut ThreadRng;
        Self {
            difficulty,
            board: Board::generate(rng, difficulty)
        }
    }
}