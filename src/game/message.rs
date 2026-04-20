use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum Message {
    #[default]
    Neutral,
    Correct(ArrayVec<u8, 2>),
    Incorrect(u8, u8),
    Undone,
    Restarted,
    Won,
    Lost,
}

impl Message {
    pub fn emoji(&self) -> &str {
        match self {
            Message::Correct(_) => "😺",
            Message::Incorrect(_, _) => "😾",
            Message::Won => "😸",
            Message::Lost => "😿",
            _ => "🐱",
        }
    }
}