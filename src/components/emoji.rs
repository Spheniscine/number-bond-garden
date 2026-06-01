use dioxus::prelude::*;
use phf::phf_map;

pub static EMOJI_MAP: phf::Map<&'static str, Asset> = phf_map! {
    "😺" => asset!("/assets/emoji/emoji_u1f63a.svg"),
    "😾" => asset!("/assets/emoji/emoji_u1f63e.svg"),
    "😸" => asset!("/assets/emoji/emoji_u1f638.svg"),
    "😿" => asset!("/assets/emoji/emoji_u1f63f.svg"),
    "🐱" => asset!("/assets/emoji/emoji_u1f431.svg"),
    "🔒" => asset!("/assets/emoji/emoji_u1f512.svg"),
    "👆" => asset!("/assets/emoji/emoji_u1f446.svg"),
};

#[component]
pub fn Emoji(text: String) -> Element {
    if let Some(asset) = EMOJI_MAP.get(&text) {
        rsx! {
            img {
                style: "height: 1.175em; vertical-align: middle;",
                src: *asset,
                draggable: false,
                alt: text,
            }
        }
    } else {
        tracing::error!("No emoji asset loaded for string '{text}'");
        rsx! {
            "ERROR"
        }
    }
    
}