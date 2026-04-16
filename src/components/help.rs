use dioxus::prelude::*;
use hexx::{Hex, Vec2, hex, storage::HexagonalMap};

use crate::{components::HexGrid, game::Board};

#[component]
pub fn Help() -> Element {
    let view_scale: f32 = 40.;
    let hex_scale: f32 = 7.;
    let hex_aspect_ratio: f32 = 0.8660254;
    let origin = Vec2::splat(view_scale / 2.) - Vec2 { x: 0., y: hex_scale * hex_aspect_ratio };

    let mut board1 = Board {
        inner: HexagonalMap::new(Hex::ORIGIN, 3, |_| None)
    };
    board1[Hex::ORIGIN] = Some(3);
    board1[Hex { x: 0, y: 1}] = Some(7);

    let mut board2 = board1.clone();
    
    rsx! {
        div {
            style: "padding: 2rem; display: flex; flex-direction: column; align-items: center;",

            div {
                style: "display: flex; flex-direction: row;",

                div {
                    style: "overflow: hidden; position: relative; top: 2rem; height: {view_scale}rem; width: {view_scale}rem;",

                    HexGrid { 
                        board: board1,
                        origin,
                        scale: hex_scale,
                        dim_blocked: true,
                    },
                },

                div {
                    style: "width: 5rem;"
                }

                div {
                    style: "overflow: hidden; position: relative; top: 2rem; height: {view_scale}rem; width: {view_scale}rem;",

                    HexGrid { 
                        board: board2,
                        origin,
                        scale: hex_scale,
                        dim_blocked: true,
                        selected: Hex::ORIGIN,
                    },
                },
            }

            div {
                style: "position: relative; top: 4rem; font-size: 4.5rem; color: #fff; text-align: center;",
                "Your goal is to clear the board. Select a free orb, then match it with another orb, such that their numbers add up to ten, to remove both orbs from the board."
            }
        }
        
    }
}