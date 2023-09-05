use dioxus::prelude::*;
use crate::resource_loader::MtgCard;

/// Properties that are given to the main application.
#[derive(PartialEq, Props)]
pub struct StartupProps {
    pub application_name: String,
    pub mtg_cards: Vec<MtgCard>,
    pub errors: String,
}

const HEADER_STYLE: &str = r#"
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: row;
    color: pink;
    background-color: blue;
    align-items: left;
    width: 100%;
   "#;

const TEXT_STYLE: &str = r#"
    display: block;
    width: 180px;
    height: 20px;
    color: white;
    align-items: left;
    border: 1px solid black;
    margin: 1px;
    padding: 1px;
   "#;


const CARD_STYLE: &str = r#"
    background-color: yellow;
    position: absolute;
    width: 100px;
    height: 200px;
   "#;

const SYMBOL_STYLE: &str = r#"
    background-color: yellow;
    width: 20px;
    height: 20px;
   "#;

#[inline_props]
pub fn HeaderButton(cx: Scope, text: String) -> Element {

    let color = use_state(&cx, || "darkblue");
    cx.render(rsx! {
        button {
            style: "{TEXT_STYLE}",
            background: {*color.get()},
            onmousedown: move |_| {color.set("blue");},
            onmouseup: move |_| {color.set("darkblue");},
            text.to_owned(),
        }
    })
}

/// The main application header. 
#[allow(non_snake_case)]
#[inline_props]
pub fn HeaderComponent<'a>(cx: Scope<'a>, some_text: &'a Vec<String>) -> Element {

    cx.render(rsx! {
        header {
            style: "{HEADER_STYLE}",
            some_text.iter().map(|x| rsx!{ p { style: "{TEXT_STYLE}", (*x).to_owned()} } )
        }
    })
}

pub struct Card {
    pub pos_x: u32,
    pub pos_y: u32,
}

/// A test object (card).
#[allow(non_snake_case)]
#[inline_props]
pub fn CardComponent<'a>(cx: Scope<'a>, card: &'a Card) -> Element {

    cx.render(rsx! {
        div {
            style: "{CARD_STYLE}",
            position: "fixed",
            top: "{card.pos_y}",
            left: "{card.pos_x}",
        }
    })
}
