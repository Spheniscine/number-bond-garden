use dioxus::prelude::*;
use crate::{components::{BoardComponent, EMOJI_MAP, Emoji, Help, LocalStorage, Speech}, game::{Difficulty, GameState, ScreenState}};

#[component]
pub fn Hero() -> Element {
    let mut state = use_signal(|| {
        if let Some(mut state) = LocalStorage.load_game_state() {
            state.selected = None;
            state.screen_state = ScreenState::Game;
            return state;
        }
        GameState::generate(Difficulty::Normal)
    });

    let confetti_counter = use_memo(move || {
        state.read().num_wins
    });
    use_effect(move || {
        let _ = confetti_counter.read();
        document::eval("confetti();");
    });

    
    let st = state();

    // tracing::info!("Number of free orbs: {:?}", state().board.count_free());
    
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

                div {
                    style: "position: absolute; top: 16rem; left: 3rem; font-size: 4rem; color: #fff; 
                    padding-left: 1.5rem; padding-right: 1.5rem; padding-bottom: 1rem; width: 16rem;
                    background-color: #052; border-radius: 1rem; text-align: center;",

                    span {
                        style: "font-size: 3rem",
                        "SCORE",
                    }

                    br {}

                    "{st.score} / {st.difficulty.num_orbs()}"
                }

                div {
                    style: "position: absolute; top: 16rem; right: 3rem; font-size: 4rem; color: #fff; 
                    padding-left: 1.5rem; padding-right: 1.5rem; padding-bottom: 1rem; width: 16rem;
                    background-color: #052; border-radius: 1rem; text-align: center;",

                    span {
                        style: "font-size: 3rem",
                        "WINS",
                    }

                    br {}

                    "{st.num_wins}"
                }

                BoardComponent {
                    state: state.clone(),
                },

                div {
                    style: "position: absolute; top: 130rem; display: flex; flex-direction: row; font-size: 5rem; color: #fff; place-items: center;",
                    
                    div {
                        style: "font-family: 'Noto Color Emoji'; font-size: 8rem;",
                        Emoji { 
                            text: "{st.message.emoji()}"
                        }
                    }

                    div {
                        class: "speech",
                        Speech { message: st.message.clone(), }
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
                        "Reset",
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
            
            div {
                id: "preloaded-images",

                for asset in EMOJI_MAP.values() {
                    img {
                        src: *asset,
                        width: 1,
                        height: 1,
                    }
                }
            },
        }
    }
}
