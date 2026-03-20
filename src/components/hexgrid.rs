use dioxus::prelude::*;
use hexx::{Hex, HexBounds, HexLayout, HexOrientation, Vec2};

use crate::components::HexBox;

#[component]
pub fn HexGrid() -> Element {
    let radius = 4;
    let bounds = HexBounds::new(Hex::ORIGIN, radius);

    let layout = HexLayout {
        orientation: HexOrientation::Flat,
        origin: Vec2 { x: 50., y: 56. },
        scale: Vec2 { x: 7., y: 7. },
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
            }
        }
    }
}