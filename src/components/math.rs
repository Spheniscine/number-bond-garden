use dioxus::prelude::*;

#[component]
pub fn Math(tex: String, style: Option<String>) -> Element {
    let html = katex::render(tex.as_str()).unwrap();
    rsx! {
        span {
            style,
            dangerous_inner_html: html
        }
    }
}