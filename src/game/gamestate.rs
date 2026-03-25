use hexx::storage::HexagonalMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct GameState {
    pub board: HexagonalMap<Option<u8>>,
}