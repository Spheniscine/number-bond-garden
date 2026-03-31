use dioxus::prelude::*;

use crate::{components::HexGrid, game::GameState};

mod utils;
mod game;
mod components;

const FAVICON: Asset = asset!("/assets/favicon.ico");
// const HEADER_SVG: Asset = asset!("/assets/header.svg");

// string inclusion is used to prevent FOUC;

// const _RAND_RECOMPILE: u64 = 0x4a2a5cf9126cd711; // comment and uncomment to force recompilation
const MAIN_CSS: &str = const_css_minify::minify!("../assets/main.css");
// const TAILWIND_CSS: &str = const_css_minify::minify!("../assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Style { {MAIN_CSS} }
        // document::Style { {TAILWIND_CSS} }
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    let state = GameState::test_gen();

    rsx! {
        div {
            id: "hero",
            class: "select-none",

            HexGrid {
                board: state.board
            }
            // "a",
            // img { src: HEADER_SVG, id: "header" }
            // div { id: "links",
            //     a { href: "https://dioxuslabs.com/learn/0.7/", "📚 Learn Dioxus" }
            //     a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
            //     a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
            //     a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
            //     a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
            //     a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            // }
        }
    }
}
