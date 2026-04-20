use dioxus::prelude::*;

use crate::components::Hero;

mod utils;
mod game;
mod components;

const FAVICON: Asset = asset!("/assets/favicon.ico");
// const HEADER_SVG: Asset = asset!("/assets/header.svg");

// string inclusion is used to prevent FOUC;

const _RAND_RECOMPILE: u64 = 0x4a2a5cf9126cd711; // comment and uncomment to force recompilation
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
            href: "https://fonts.googleapis.com/css2?family=Noto+Color+Emoji&family=Noto+Sans:ital,wght@0,100..900;1,100..900&display=swap",
            rel: "stylesheet",
        }

        document::Link { rel: "icon", href: FAVICON }
        document::Style { {MAIN_CSS} }
        // document::Style { {TAILWIND_CSS} }
        Hero {}

    }
}