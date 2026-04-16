use dioxus::prelude::*;
use hexx::{Hex, storage::HexagonalMap, Vec2};

use crate::{components::HexGrid, game::Board};

#[component]
pub fn Help() -> Element {
    let origin = Vec2 { x: 20., y: 14. };
    let scale = 7.;

    let mut board = Board {
        inner: HexagonalMap::new(Hex::ORIGIN, 3, |_| None)
    };
    board[Hex::ORIGIN] = Some(3);
    board[Hex { x: 0, y: 1}] = Some(7);
    
    rsx! {
        div {
            style: "overflow: hidden; position: absolute; height: 40rem; width: 40rem; border: 0.5rem solid #fff",

            HexGrid { 
                board,
                origin,
                scale,
                dim_blocked: true,
            },
        }
    }
}