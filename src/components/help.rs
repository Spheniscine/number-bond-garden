use dioxus::prelude::*;
use hexx::{Hex, HexLayout, HexOrientation, Vec2, storage::HexagonalMap};

use crate::{components::{Decor, HexGrid, orb::Orb}, game::{Board, GameState, HEX_ASPECT_RATIO, ScreenState}};

#[component]
pub fn HelpPair(
    a: u8,
    b: Option<u8>,
) -> Element {
    let size_y = 12. * HEX_ASPECT_RATIO;
    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center;
            font-size: {size_y * 0.5}rem; font-family: KaTeX_Main; color: #fff",

            Orb {
                content: a,
                size_y,
            }
            
            if let Some(b) = b {
                "+",

                Orb {
                    content: b,
                    size_y,
                }
            }
        }
    }
}

#[component]
pub fn Help(
    state: Signal<GameState>
) -> Element {
    let view_scale: f32 = 35.;
    let hex_scale: f32 = 6.;
    let hex_aspect_ratio: f32 = HEX_ASPECT_RATIO;
    let origin1 = Vec2::splat(view_scale / 2.) - Vec2 { x: 0., y: hex_scale * hex_aspect_ratio };

    let mut board1 = Board {
        inner: HexagonalMap::new(Hex::ORIGIN, 3, |_| None)
    };
    board1[Hex::ORIGIN] = Some(4);
    board1[Hex { x: 0, y: 1 }] = Some(6);
    let decors1 = vec![
        (Hex::ORIGIN, Decor::Pointer),
    ];

    let board2 = board1.clone();
    let origin2 = origin1;
    let decors2 = vec![
        (Hex { x: 0, y: 1 }, Decor::Pointer),
    ];

    let mut board3 = Board {
        inner: HexagonalMap::new(Hex::ORIGIN, 3, |_| None)
    };
    board3[Hex::ORIGIN] = Some(1);
    board3[Hex { x: -1, y: 0 }] = Some(9);
    board3[Hex { x: 0, y: 1 }] = Some(9);
    board3[Hex { x: 1, y: 0 }] = Some(9);
    let decors3 = vec![
        (Hex::ORIGIN, Decor::Lock),
        (Hex { x: 0, y: -1 }, Decor::Highlight),
        (Hex { x: 1, y: -1 }, Decor::Highlight),
        (Hex { x: -1, y: 1 }, Decor::Highlight),
    ];
    

    let origin3 = Vec2::splat(view_scale / 2.);

    let mut board4 = Board {
        inner: HexagonalMap::new(Hex::ORIGIN, 4, |_| None)
    };
    let layout4 = HexLayout {
        orientation: HexOrientation::Flat,
        origin: Vec2::ZERO,
        scale: Vec2::splat(hex_scale),
    };

    let origin4 = Vec2::splat(view_scale / 2.) - layout4.hex_to_world_pos(Hex { x: 2, y: -4 });
    board4[Hex { x: 2, y: -4 }] = Some(1);
    board4[Hex { x: 1, y: -3 }] = Some(9);
    board4[Hex { x: 2, y: -3 }] = Some(9);
    board4[Hex { x: 3, y: -4 }] = Some(9);
    let decors4 = vec![
        (Hex { x: 2, y: -5 }, Decor::Highlight),
        (Hex { x: 3, y: -5 }, Decor::Highlight),
        (Hex { x: 1, y: -4 }, Decor::Highlight),
    ];
    
    rsx! {
        div {
            style: "padding: 2rem; display: flex; flex-direction: column; align-items: center;",

            div {
                style: "display: flex; flex-direction: row; padding-top: 1rem;",

                div {
                    style: "overflow: hidden; position: relative; height: {view_scale}rem; width: {view_scale}rem; border: 0.5rem solid #fff;",

                    HexGrid { 
                        board: board1,
                        origin: origin1,
                        scale: hex_scale,
                        dim_blocked: true,
                        decors: decors1,
                    },
                },

                div {
                    style: "width: 7rem;"
                }

                div {
                    style: "overflow: hidden; position: relative; height: {view_scale}rem; width: {view_scale}rem; border: 0.5rem solid #fff;",

                    HexGrid { 
                        board: board2,
                        origin: origin2,
                        scale: hex_scale,
                        dim_blocked: true,
                        selected: Hex::ORIGIN,
                        decors: decors2,
                    },
                },
            }

            div {
                style: "position: relative; padding-top: 2.5rem; font-size: 4rem; color: #fff; text-align: center;",
                "Your goal is to clear the board.", br {},
                "Select a free orb, then select another free orb, such that their numbers add up to ten, to remove both orbs from the board."
            }

            div {
                style: "display: flex; flex-direction: row; padding-top: 2.5rem;",

                div {
                    style: "overflow: hidden; position: relative; height: {view_scale}rem; width: {view_scale}rem; border: 0.5rem solid #fff;",

                    HexGrid { 
                        board: board3,
                        origin: origin3,
                        scale: hex_scale,
                        dim_blocked: true,
                        decors: decors3,
                    },
                },

                div {
                    style: "width: 7rem;"
                }

                div {
                    style: "overflow: hidden; position: relative; height: {view_scale}rem; width: {view_scale}rem; border: 0.5rem solid #fff;",

                    HexGrid { 
                        board: board4,
                        origin: origin4,
                        scale: hex_scale,
                        dim_blocked: true,
                        selected: Hex { x: 2, y: -4 },
                        decors: decors4,
                    },
                },
            }

            div {
                style: "position: relative; padding-top: 2.5rem; font-size: 4rem; color: #fff; text-align: center;",
                "An orb is free only if it has three ",
                b { 
                    style: "color: #ff0",
                    "contiguous" 
                },
                " empty spaces next to it. Luckily, spaces off the board count as empty spaces.",
            }

            div {
                style: "position: relative; padding-top: 2.5rem; display: flex; flex-direction: row; place-items: center;",
                for a in 1..=5 {
                    HelpPair { 
                        a, b: 10 - a,
                    }
                    div { style: "width: 4rem;" }
                }
                HelpPair {
                    a: 10,
                }
            }

            div {
                style: "position: relative; padding-top: 2.5rem; font-size: 4rem; color: #fff; text-align: center;",
                "These are the possible matches. The ",
                span {
                    style: "font-family: KaTeX_Main; font-size: 115%;",
                    "10",
                },
                
                " orb is special and can be removed once it is free.",
            }

            div {
                style: "position: relative; padding-top: 2rem;",
                div {
                    class: "button",
                    style: "width: 40rem;",
                    onclick: move |_| { state.write().screen_state = ScreenState::Game; },
                    "Back to game",
                },
            }
        }
        
    }
}