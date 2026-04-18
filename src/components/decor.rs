use dioxus::prelude::*;
use hexx::Vec2;


const POINTER: &str = "👆";
const LOCK: &str = "🔒";

/// special "decorations" to be placed on a HexGrid, for the help screen
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decor {
    Pointer,
    Lock,
    Highlight,
}

#[component]
pub fn DecorComponent(
    pos: Vec2,
    scale: Vec2,
    decor: Decor,
) -> Element {
    if decor == Decor::Pointer {
        rsx! {
            div {
                style: "position: absolute; left: {pos.x}rem; top: {pos.y}rem;
                font-family: 'Noto Color Emoji'; font-size: {scale.y * 0.5}rem;",
                "👆"
            }
        }
    } else {
        rsx!{}
    }
}