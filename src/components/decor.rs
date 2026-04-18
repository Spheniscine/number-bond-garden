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
    match decor {
        Decor::Pointer => rsx! {
            div {
                style: "position: absolute; left: {pos.x}rem; top: {pos.y}rem;
                font-family: 'Noto Color Emoji'; font-size: {scale.y * 0.5}rem;",
                "👆",
            }
        },
        Decor::Lock => {
            let pos = pos - scale / 2. + Vec2 { x: scale.x * 0.015, y: scale.y * -0.03, };
            rsx! {
                div {
                    style: "position: absolute; left: {pos.x}rem; top: {pos.y}rem; 
                    display: grid; place-items: center; 
                    width: {scale.x}rem; height: {scale.y}rem;
                    font-family: 'Noto Color Emoji'; font-size: {scale.y * 0.5}rem;",
                    "🔒",
                }
            }
        }
        Decor::Highlight => {
            let pos = pos - scale / 2.;
            rsx! {
                div {
                    style: "position: absolute; left: {pos.x}rem; top: {pos.y}rem; 
                    display: grid; place-items: center;
                    width: {scale.x}rem; height: {scale.y}rem;",
                    div {
                        class: "hexagon",
                        style: "height: 80%; width: 80%; background: #ff0; opacity: 0.5;"
                    }
                }
            }
        } 
    }
}