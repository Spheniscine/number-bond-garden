use arrayvec::ArrayVec;
use hexx::{Hex, storage::HexStore};
use serde::{Deserialize, Serialize};

use crate::{components::LocalStorage, game::{Board, Difficulty, Message, ThreadRng}};

pub type Move = ArrayVec<(Hex, u8), 2>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScreenState {
    Game, Help
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameState {
    pub difficulty: Difficulty,
    pub board: Board,
    pub dim_blocked: bool,
    pub selected: Option<Hex>,
    pub undo_stack: Vec<Move>,
    pub screen_state: ScreenState,
    pub already_won: bool,
    pub score: usize,
    pub num_wins: i32,
    pub message: Message,
}

impl GameState {
    pub fn generate(difficulty: Difficulty) -> Self {
        let rng = &mut ThreadRng;
        let res = Self {
            difficulty,
            board: Board::generate(rng, difficulty),
            dim_blocked: true,
            selected: None,
            undo_stack: vec![],
            screen_state: ScreenState::Game,
            already_won: false,
            score: 0,
            num_wins: 0,
            message: Message::Neutral,
        };
        LocalStorage.save_game_state(&res);
        res
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
        self.already_won = false;
        self.score = 0;
        self.message = Message::Neutral;
        LocalStorage.save_game_state(&self);
    }

    pub fn click_hex(&mut self, hex: Hex) {
        if self.message == Message::Lost { return; }
        if !self.board.is_free(hex) { return; }
        let Some(&Some(a)) = self.board.inner.get(hex) else { return };
        let b = self.selected.map(|hex| self.board.inner.get(hex).copied()).flatten().flatten();

        if let Some(b) = b {
            let bhex = self.selected.unwrap();
            if bhex == hex {
                self.selected = None;
                return;
            }
            if a + b != 10 { 
                self.message = Message::Incorrect(b, a);
                return;
            }
            let mut mv = Move::new();
            mv.push((bhex, b));
            mv.push((hex, a));
            self.do_move(mv);
        } else {
            if a == 10 {
                let mut mv = Move::new();
                mv.push((hex, a));
                self.do_move(mv);
            } else {
                self.selected = Some(hex);
            }
        }
    }

    fn do_move(&mut self, mv: Move) {
        self.message = Message::Correct(mv.iter().map(|m| m.1).collect());
        for &(hex, _) in &mv {
            self.board[hex] = None;
        }
        self.selected = None;
        self.score += mv.len();
        self.undo_stack.push(mv);
        self.check_game_end();
        LocalStorage.save_game_state(&self);
    }

    fn check_game_end(&mut self) {
        if self.score == self.difficulty.num_orbs() { // check for win
            if !self.already_won {
                self.num_wins += 1;
                self.already_won = true;
            }
            self.message = Message::Won;
        } else { // check for loss
            let mut free = [false; 10];
            for (hex, &content) in self.board.inner.iter() {
                if !self.board.is_free(hex) { continue; }
                if let Some(x) = content {
                    if x == 10 { return; }
                    if free[(10 - x) as usize] { return; }
                    free[x as usize] = true;
                }
            }
            self.message = Message::Lost;
        }
    }

    // pub fn is_won(&self) -> bool {
    //     self.message == Message::Won
    // }

    pub fn undo(&mut self) {
        self._undo();
        LocalStorage.save_game_state(&self);
    }

    fn _undo(&mut self) {
        if let Some(mv) = self.undo_stack.pop() {
            self.score -= mv.len();
            for (hex, val) in mv {
                self.board[hex] = Some(val);
            }
            self.selected = None;
            self.message = Message::Undone;
        }
    }

    pub fn restart(&mut self) {
        if self.undo_stack.is_empty() { return; }
        while !self.undo_stack.is_empty() {
            self._undo();
        }
        self.message = Message::Restarted;
        LocalStorage.save_game_state(&self);
    }
}