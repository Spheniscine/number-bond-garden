use serde::{Deserialize, Serialize};

use crate::game::{Board, Difficulty, ThreadRng};


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameState {
    pub difficulty: Difficulty,
    pub board: Board,
}

impl GameState {
    pub fn generate(difficulty: Difficulty) -> Self {
        let rng = &mut ThreadRng;
        Self {
            difficulty,
            board: Board::generate(rng, difficulty)
        }
    }

    pub fn change_difficulty(&mut self) {
        let difficulty = match self.difficulty {
            Difficulty::Normal => Difficulty::Hard,
            Difficulty::Hard => Difficulty::Normal,
        };
        *self = Self::generate(difficulty);
    }

    pub fn new_game(&mut self) {
        *self = Self::generate(self.difficulty);
    }
}