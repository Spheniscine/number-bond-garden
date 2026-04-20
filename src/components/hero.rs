use dioxus::prelude::*;
use crate::{components::{BoardComponent, Help}, game::{Difficulty, GameState, ScreenState}};

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

            if st.screen_state == ScreenState::Game {
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
                    style: "position: absolute; top: 130rem; display: flex; flex-direction: row; font-size: 5rem; color: #fff; place-items: center;",
                    
                    div {
                        style: "font-family: 'Noto Color Emoji'; font-size: 8rem;",
                        "🐱",
                    }

                    div {
                        class: "speech",
                        "Good luck and have fun!"
                    }
                }

                div {
                    style: "position: absolute; top: 143rem; display: flex; flex-direction: row;",
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
                    style: "position: absolute; top: 156rem; display: flex; flex-direction: row;",
                    div {
                        class: "button",
                        style: "width: 40rem;",
                        onclick: move |_| { state.write().dim_blocked ^= true; },
                        "Dim Blocked: {dim_blocked}",
                    },
                    div {
                        class: "button",
                        style: "width: 40rem;",
                        onclick: move |_| { state.write().screen_state = ScreenState::Help; },
                        "Help",
                    },
                }

                p {
                    style: "position: absolute; bottom: 1.5rem; font-size: 3rem; color: #fff;",
                    "© OnlineMathLearning.com"
                },
            } else if st.screen_state == ScreenState::Help {
                Help { 
                    state: state.clone(),
                }
            }
            
        }
    }
}
