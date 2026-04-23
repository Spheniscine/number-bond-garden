use dioxus::prelude::*;

#[component]
pub fn Math(
    tex: String,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let html = katex::render(tex.as_str()).unwrap();
    rsx! {
        span {
            dangerous_inner_html: html,
            ..attributes,
        }
    }
}