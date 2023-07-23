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
    background-color: blue;
   "#;

const CARD_STYLE: &str = r#"
    background-color: yellow;
    position: fixed;
    width: 100px;
    height: 200px;
   "#;

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
