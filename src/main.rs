use dioxus::prelude::*;

use crate::{components::BoardComponent, game::{Difficulty, GameState}};

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
    // tracing::info!("{:?}", game::Board::_pattern_stats(1_000_000, &mut game::ThreadRng));

    rsx! {
        document::Link {
            rel: "preconnect",
            href: "https://fonts.googleapis.com",
        }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "anonymous",
        }
        document::Link {
            href: "https://fonts.googleapis.com/css2?family=Noto+Sans:ital,wght@0,100..900;1,100..900&display=swap",
            rel: "stylesheet",
        }

        document::Link { rel: "icon", href: FAVICON }
        document::Style { {MAIN_CSS} }
        // document::Style { {TAILWIND_CSS} }
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    let mut state = use_signal(|| {GameState::generate(Difficulty::Normal)});
    // tracing::info!("Number of free orbs: {:?}", state().board.count_free());
    let st = state();
    let dim_blocked = if st.dim_blocked {"On"} else {"Off"};

    let button_class = |enabled: bool| {
        if enabled {"button"} else {"button-disabled"}
    };

    rsx! {
        div {
            id: "hero",
            class: "select-none",

            div {
                style: "display: flex; flex-direction: row;",
                div {
                    class: "button",
                    style: "width: 50rem;",
                    onclick: move |_| { state.write().change_difficulty(); },
                    "Difficulty: {st.difficulty}",
                },
                div {
                    class: "button",
                    style: "width: 30rem;",
                    onclick: move |_| { state.write().new_game(); },
                    "New Game",
                },
            },

            BoardComponent {
                state: state.clone(),
            },

            div {
                style: "position: absolute; top: 130rem; display: flex; flex-direction: row;",
                div {
                    class: button_class(!st.undo_stack.is_empty()),
                    style: "width: 40rem;",
                    onclick: move |_| { state.write().undo() },
                    "Undo",
                },
                div {
                    class: button_class(!st.undo_stack.is_empty()),
                    style: "width: 40rem;",
                    onclick: move |_| { state.write().restart() },
                    "Restart",
                },
            }

            div {
                style: "position: absolute; top: 143rem; display: flex; flex-direction: row;",
                div {
                    class: "button",
                    style: "width: 40rem;",
                    onclick: move |_| { state.write().dim_blocked ^= true; },
                    "Dim Blocked: {dim_blocked}",
                },
                div {
                    class: "button-disabled",
                    style: "width: 40rem;",
                    "Help",
                },
            }
        }
    }
}
