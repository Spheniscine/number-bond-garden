use arrayvec::ArrayVec;
use hexx::{Hex, storage::HexStore};
use serde::{Deserialize, Serialize};

use crate::game::{Board, Difficulty, ThreadRng};

pub type Move = ArrayVec<(Hex, u8), 2>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameState {
    pub difficulty: Difficulty,
    pub board: Board,
    pub dim_blocked: bool,
    pub selected: Option<Hex>,
    pub undo_stack: Vec<Move>,
}

impl GameState {
    pub fn generate(difficulty: Difficulty) -> Self {
        let rng = &mut ThreadRng;
        Self {
            difficulty,
            board: Board::generate(rng, difficulty),
            dim_blocked: true,
            selected: None,
            undo_stack: vec![],
        }
    }

    pub fn change_difficulty(&mut self) {
        self.difficulty = match self.difficulty {
            Difficulty::Normal => Difficulty::Hard,
            Difficulty::Hard => Difficulty::Normal,
        };
        self.new_game();
    }

    pub fn new_game(&mut self) {
        self.board = Board::generate(&mut ThreadRng, self.difficulty);
        self.selected = None;
        self.undo_stack.clear();
    }

    pub fn click_hex(&mut self, hex: Hex) {
        let Some(&Some(a)) = self.board.inner.get(hex) else { return };
        let b = self.selected.map(|hex| self.board.inner.get(hex).copied()).flatten().flatten();

        if let Some(b) = b {
            let bhex = self.selected.unwrap();
            if bhex == hex {
                self.selected = None;
                return;
            }
            if a + b != 10 { return; }
            let mut mv = Move::new();
            mv.push((hex, a));
            mv.push((bhex, b));
            self.undo_stack.push(mv);
            self.board[hex] = None;
            self.board[bhex] = None;
            self.selected = None;
        } else {
            if a == 10 {
                let mut mv = Move::new();
                mv.push((hex, a));
                self.undo_stack.push(mv);
                self.board[hex] = None;
                self.selected = None;
            } else {
                self.selected = Some(hex);
            }
        }
    }
}