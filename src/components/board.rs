use dioxus::prelude::*;
use hexx::Vec2;

use crate::{components::HexGrid, game::GameState};

#[component]
pub fn BoardComponent(
    state: Signal<GameState>,
) -> Element {
    let origin = Vec2 { x: 50., y: 72. };
    let st = state();
    let scale = 49. / (1. + 1.5 * st.board.inner.bounds().radius as f32);
    let onclick= move |hex| {
        state.write().click_hex(hex);
    };

    rsx! {
        HexGrid { 
            board: st.board,
            origin,
            scale,
            dim_blocked: st.dim_blocked,
            selected: st.selected,
            onclick,
        }
    }
}