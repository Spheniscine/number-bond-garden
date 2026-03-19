use dioxus::prelude::*;

#[component]
pub fn HexBox(style: Option<String>) -> Element {
    rsx! {
        div {
            class: "hexagon",
            style,
        }
    }
}