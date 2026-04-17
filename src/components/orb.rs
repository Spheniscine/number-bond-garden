use dioxus::prelude::*;

use crate::game::{ORB_COLORS, ORB_SCALE};

#[component]
pub fn Orb(
    content: u8,
    size_y: f32,
    #[props(default = false)]
    dimmed: bool,
    #[props(default = false)]
    selected: bool,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let (bg_color, text_color) = ORB_COLORS[content as usize];
    let dimmed = if dimmed {"dimmed"} else {""};
    let selected = if selected {"selected"} else {""};

    let onclick = move |e| if let Some(f) = onclick { f.call(e) };

    rsx! {
        div {
            class: "{dimmed} {selected}",
            style: "height: {size_y * ORB_SCALE}rem; aspect-ratio: 1; border-radius: 50%; 
            background-color: {bg_color}; color: {text_color}; display: grid; place-items: center;
            font-size: {size_y * 0.5}rem",
            onclick,
            "{content}",
        }
    }
}