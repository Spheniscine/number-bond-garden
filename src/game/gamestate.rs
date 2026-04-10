use serde::{Deserialize, Serialize};

use crate::game::{Board, Difficulty, ThreadRng};


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameState {
    pub difficulty: Difficulty,
    pub board: Board,
    pub dim_blocked: bool,
}

impl GameState {
    pub fn generate(difficulty: Difficulty) -> Self {
        let rng = &mut ThreadRng;
        Self {
            difficulty,
            board: Board::generate(rng, difficulty),
            dim_blocked: true,
        }
    }

    pub fn change_difficulty(&mut self) {
        self.difficulty = match self.difficulty {
            Difficulty::Normal => Difficulty::Hard,
            Difficulty::Hard => Difficulty::Normal,
        };
        self.board = Board::generate(&mut ThreadRng, self.difficulty);
    }

    pub fn new_game(&mut self) {
        self.board = Board::generate(&mut ThreadRng, self.difficulty);
    }
}