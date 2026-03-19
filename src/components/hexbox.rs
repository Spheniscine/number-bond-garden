use dioxus::prelude::*;

#[component]
pub fn HexBox() -> Element {
    rsx! {
        div {
            style: "display: flex; justify-content: center; align-items: center; height: 20rem; aspect-ratio: 1.15470053838;",
            div {
                class: "hexagon",
                style: "height: 96%; width: 96%",
            }
        }
        
    }
}