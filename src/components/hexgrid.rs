use dioxus::prelude::*;
use hexx::{EdgeDirection, GridEdge, Hex, HexBounds, HexLayout, HexOrientation, Vec2};

use crate::components::HexBox;

#[component]
pub fn HexGrid() -> Element {
    let radius = 4;
    let bounds = HexBounds::new(Hex::ORIGIN, radius);

    let layout = HexLayout {
        orientation: HexOrientation::Flat,
        origin: Vec2 { x: 50., y: -47. },
        scale: Vec2 { x: 7., y: 7. },
    };

    let size = layout.rect_size();
    let boxes = bounds.all_coords().map(|hex| {
        let [p, q] = layout.edge_coordinates(GridEdge { origin: hex, direction: EdgeDirection::FLAT_NORTH_WEST });
        Vec2 { x: p.x, y: q.y }
    });

    rsx! {
        for v in boxes {
            HexBox {
                pos: v,
                size,
            }
        }
    }
}