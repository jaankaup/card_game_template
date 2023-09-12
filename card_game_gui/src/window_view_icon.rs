use dioxus::prelude::*;
use crate::components::MainWindowState;
use crate::components::MainWindowStates;

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
//pub fn WindowViewIcon<'a>(cx: Scope, text: String, is_clicked: EventHandler<'a, MouseEvent>) -> Element<'a> {
pub fn WindowViewIcon(cx: Scope, text: String, window_state: MainWindowState) -> Element {

    let color = use_state(&cx, || "darkred");
    let shared_window_state = use_shared_state::<MainWindowStates>(cx).unwrap();

    cx.render(rsx! {
        button {
            align_self: "end",
            style: "{TEXT_STYLE}",
            onclick: move |e| { shared_window_state.write().0 = *window_state; },
            background: if shared_window_state.read().0 == *window_state { "red" } else { "darkred" },
            // onmousedown: move |_| {color.set("blue");},
            // onmouseup: move |_| {color.set("darkblue");},
            text.to_owned(),
        }
    })
}
