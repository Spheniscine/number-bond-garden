use serde::{Deserialize, Serialize};

use crate::game::Board;


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