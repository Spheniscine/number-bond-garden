use dioxus::prelude::*;

use crate::game::ORB_COLORS;

#[component]
pub fn Orb(
    content: u8,
    size_y: f32,
    #[props(default = false)]
    dimmed: bool,
    #[props(default = false)]
    selected: bool,
    #[props(default)]
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let (bg_color, text_color) = ORB_COLORS[content as usize];

    rsx! {
        div {
            class: if dimmed {"dimmed"},
            class: if selected {"selected"},
            style: "height: {size_y * 0.8}rem; aspect-ratio: 1; border-radius: 50%; 
            background-color: {bg_color}; color: {text_color}; display: grid; place-items: center;
            font-size: {size_y * 0.5}rem; font-family: KaTeX_Main;",
            onclick,
            "{content}",
        }
    }
}