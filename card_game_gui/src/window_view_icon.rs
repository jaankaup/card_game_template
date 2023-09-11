use dioxus::prelude::*;

const TEXT_STYLE: &str = r#"
    display: block;
    width: 180px;
    height: 20px;
    color: white;
    border: 1px solid black;
    margin: 1px;
    padding: 1px;
   "#;

#[inline_props]
pub fn WindowViewIcon(cx: Scope, text: String) -> Element {

    let color = use_state(&cx, || "darkblue");
    cx.render(rsx! {
        button {
            align_self: "end",
            style: "{TEXT_STYLE}",
            background: {*color.get()},
            onmousedown: move |_| {color.set("blue");},
            onmouseup: move |_| {color.set("darkblue");},
            text.to_owned(),
        }
    })
}
