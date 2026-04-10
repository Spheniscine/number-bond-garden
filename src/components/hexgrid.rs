use dioxus::prelude::*;
use hexx::{Hex, HexLayout, HexOrientation, Vec2};
use crate::{components::HexBox, game::Board};

#[component]
pub fn HexGrid(
    board: Board,
    origin: Vec2,
    scale: f32,
    dim_blocked: bool,
    selected: Option<Hex>,
    onclick: Option<EventHandler<Hex>>,
) -> Element {
    let bounds = board.inner.bounds();

    let layout = HexLayout {
        orientation: HexOrientation::Flat,
        origin,
        scale: Vec2 { x: scale, y: scale },
    };

    let size = layout.rect_size();
    let half_size = size / 2.;
    let boxes = bounds.all_coords().map(|hex| {
        let p = layout.hex_to_world_pos(hex);
        (p - half_size, hex)
    });

    let onclick = |hex: Hex| {
        move |_| {
            if let Some(onclick) = onclick {
                onclick.call(hex)
            }
        }
    };

    rsx! {
        for (v, hex) in boxes {
            HexBox {
                pos: v,
                size,
                hex,
                content: board[hex],
                dimmed: dim_blocked && !board.is_free(hex),
                selected: selected == Some(hex),
                onclick: onclick(hex),
            }
        }
    }
}