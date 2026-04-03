use dioxus::prelude::*;
use hexx::{Hex, Vec2};

use crate::game::ORB_COLORS;

#[component]
pub fn HexBox(
    /// Position coordinates in rems (canvas width = 100rems).
    pos: Vec2, 
    /// Size of bounding box in rems (canvas width = 100rems).
    size: Vec2,
    hex: Hex,
    content: Option<u8>,
) -> Element {
    let colors_and_content = content.map(|x| {
        (ORB_COLORS[x as usize], x)
    });

    rsx! {
        div {
            style: "position: absolute; left: {pos.x}rem; top: {pos.y}rem;
            display: grid; place-items: center; 
            width: {size.x}rem; height: {size.y}rem; font-size: {size.y * 0.5}rem",
            div {
                class: "hexagon",
                style: "height: 96%; width: 96%; display: grid; place-items: center; font-family: KaTeX_Main;",
                
                if let Some(((bg_color, text_color), content)) = colors_and_content {
                    div {
                        // class: if (hex.x, hex.y) == (0, 0) {"selected"} else {""},
                        style: "height: 80%; aspect-ratio: 1; border-radius: 50%; 
                        background-color: {bg_color}; color: {text_color}; display: grid; place-items: center;",
                        "{content}",
                    }
                }
            }
        }
        
    }
}