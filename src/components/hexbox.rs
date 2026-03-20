use dioxus::prelude::*;
use hexx::{Hex, Vec2};

#[component]
pub fn HexBox(
    /// Position coordinates in rems (canvas width = 100rems).
    pos: Vec2, 
    /// Size of bounding box in rems (canvas width = 100rems).
    size: Vec2,
    hex: Hex,
) -> Element {
    rsx! {
        div {
            style: "position: absolute; left: {pos.x}rem; top: {pos.y}rem;
            display: grid; place-items: center; 
            width: {size.x}rem; height: {size.y}rem;",
            div {
                class: "hexagon",
                style: "height: 96%; width: 96%; display: grid; place-items: center; font-size: 4rem; color: #000",
                "{hex.x}, {hex.y}",
            }
        }
        
    }
}