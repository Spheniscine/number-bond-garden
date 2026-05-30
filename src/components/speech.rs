use dioxus::prelude::*;

use crate::{components::Math, game::Message};
use std::fmt::Write;

#[component]
pub fn Speech(
    message: Message,
) -> Element {
    match message {
        Message::Neutral => rsx! {
            "Good luck and have fun!"
        },
        Message::Correct(mv) => {
            let mut tex = String::new();
            if mv.len() >= 1 { write!(&mut tex, "{} ", mv[0]).ok(); }
            if mv.len() >= 2 { write!(&mut tex, "+ {} ", mv[1]).ok(); }
            tex.push_str("= 10");

            rsx! { Math { tex } }
        }
        Message::Incorrect(a, b) => rsx! {
            Math {
                tex: "{a} + {b} = {a + b} \\; \\color{{yellow}}{{\\ne 10}}"
            }
        },
        Message::Undone => rsx! {
            "Undid your last move."
        },
        Message::Restarted => rsx! {
            "Game reset."
        },
        Message::Won => rsx! {
            "You cleared the board!"
        },
        Message::Lost => rsx! {
            "No more moves."
        },
    }
}