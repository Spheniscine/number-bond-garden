use std::ops::{Index, IndexMut};

use hexx::{Hex, storage::{HexStore, HexagonalMap}};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Board {
    pub inner: HexagonalMap<Option<u8>>
}

impl Index<Hex> for Board {
    type Output = Option<u8>;

    fn index(&self, index: Hex) -> &Self::Output {
        &self.inner[index]
    }
}

impl IndexMut<Hex> for Board {
    fn index_mut(&mut self, index: Hex) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.inner.bounds() == other.inner.bounds() && self.inner.values().eq(other.inner.values())
    }
}
impl Eq for Board {}