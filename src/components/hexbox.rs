use dioxus::prelude::*;
use hexx::Vec2;

#[component]
pub fn HexBox(
    /// Position coordinates in rems (canvas width = 100rems). The Y axis should be negative.
    pos: Vec2, 
    /// Size of bounding box in rems (canvas width = 100rems).
    size: Vec2
) -> Element {
    rsx! {
        div {
            style: "position: absolute; left: {pos.x}rem; top: {-pos.y}rem;
            display: flex; justify-content: center; align-items: center;
            width: {size.x}rem; height: {size.y}rem;",
            div {
                class: "hexagon",
                style: "height: 96%; width: 96%",
            }
        }
        
    }
}