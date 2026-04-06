use std::ops::RangeInclusive;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Difficulty {
    Normal, Hard
}

impl Difficulty {
    pub fn board_radius(&self) -> u32 {
        match self {
            Difficulty::Normal => 4,
            Difficulty::Hard => 5,
        }
    }

    pub fn num_dupes(&self) -> usize {
        // must be even
        match self {
            Difficulty::Normal => 4,
            Difficulty::Hard => 6,
        }
    }

    pub fn num_orbs(&self) -> usize {
        9 * self.num_dupes() + 1
    }

    pub fn initial_free_orb_range(&self) -> RangeInclusive<usize> {
        4 ..= 13
    }
}