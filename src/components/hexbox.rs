use dioxus::prelude::*;
use hexx::{Hex, Vec2};

use crate::components::orb::Orb;

#[component]
pub fn HexBox(
    /// Position coordinates in rems (canvas width = 100rems).
    pos: Vec2, 
    /// Size of bounding box in rems (canvas width = 100rems).
    size: Vec2,
    hex: Hex,
    content: Option<u8>,
    dimmed: bool,
    selected: bool,
    onclick: EventHandler<MouseEvent>
) -> Element {

    rsx! {
        div {
            style: "position: absolute; left: {pos.x}rem; top: {pos.y}rem;
            display: grid; place-items: center; 
            width: {size.x}rem; height: {size.y}rem;",
            div {
                class: "hexagon",
                style: "height: 96%; width: 96%; display: grid; place-items: center;",
                
                if let Some(content) = content {
                    Orb {
                        content,
                        size_y: size.y,
                        dimmed, selected, onclick,
                    }
                }
            }
        }
        
    }
}