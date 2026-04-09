use std::{fmt::Display, ops::RangeInclusive};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Difficulty {
    Normal, Hard
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Difficulty::Normal => "Normal",
            Difficulty::Hard => "Hard",
        };
        f.write_str(s)?;
        Ok(())
    }
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