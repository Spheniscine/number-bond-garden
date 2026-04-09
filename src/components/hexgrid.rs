use dioxus::prelude::*;
use hexx::{HexLayout, HexOrientation, Vec2};
use crate::{components::HexBox, game::Board};

#[component]
pub fn HexGrid(
    board: Board
) -> Element {
    let bounds = board.inner.bounds();

    let scale = 49. / (1. + 1.5 * board.inner.bounds().radius as f32);

    let layout = HexLayout {
        orientation: HexOrientation::Flat,
        origin: Vec2 { x: 50., y: 70. },
        scale: Vec2 { x: scale, y: scale },
    };

    let size = layout.rect_size();
    let half_size = size / 2.;
    let boxes = bounds.all_coords().map(|hex| {
        let p = layout.hex_to_world_pos(hex);
        (p - half_size, hex)
    });

    rsx! {
        for (v, hex) in boxes {
            HexBox {
                pos: v,
                size,
                hex,
                content: board[hex]
            }
        }
    }
}