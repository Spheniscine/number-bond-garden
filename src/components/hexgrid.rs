use dioxus::prelude::*;
use hexx::{Hex, HexBounds, HexLayout, HexOrientation, Vec2};
use rand::RngExt;

use crate::{components::HexBox, game::BOARD_RADIUS};

#[component]
pub fn HexGrid(
    #[props(default = BOARD_RADIUS)]
    radius: u32,
) -> Element {
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

    let rng = &mut rand::rng();
    let mut test_orbs = std::iter::from_fn(|| {
        let x = rng.random_range(0u8 ..= 10);
        Some(Some(x).take_if(|x| *x != 0))
    });

    rsx! {
        for (v, hex) in boxes {
            HexBox {
                pos: v,
                size,
                hex,
                content: test_orbs.next().flatten()
            }
        }
    }
}