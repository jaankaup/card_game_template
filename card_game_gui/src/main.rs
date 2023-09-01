use dioxus_desktop::Config;
use dioxus::prelude::*;
use card_game_gui::components::{StartupProps, HeaderComponent, CardComponent, Card};
use card_game_gui::meta_components::{UIDisplay};
use card_game_gui::misc::loadFile;
use card_game_gui::xml_parser::{CardDataFiles, FileToInclude};
use card_game_gui::resource_loader::load_mtg_cards;
use hard_xml::XmlRead;
use card_game_gui::resource_loader::MtgCard;

const MAIN_CONTAINER_STYLE: &str = r#"
    display: flex;
    position: relative;
    color: red;
    background-color: orange;
    align-items: left;
    height: 100%;
    width: 100%;
    margin: 0;
    padding: 0;
   "#;

const HEADER_CONTAINER_STYLE: &str = r#"
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    color: red;
    background-color: rgb(235,235, 55);
    position: relative;
    align-items: left;
    height: 50px;
    width: 100%;
    position: absolute;
   "#;

const THE_REST_CONTAINER_STYLE: &str = r#"
    margin: 0;
    padding: 0;
    top: 50px;
    display: flex;
    flex-direction: column;
    color: black;
    background-color: black;
    align-items: left;
    border: 1px solid red;
    position: relative;
    width: 100%;
    height: 100%;
    bottom: 0;
   "#;

fn main() {
    println!("{:?}", UIDisplay::INLINE_BLOCK);

    let mut errors = String::new();
    let mtg_cards: Vec<MtgCard>;

    let card_load_result = load_mtg_cards();

    match card_load_result {
        Ok(cards) => { mtg_cards = cards; },
        Err(err) => { errors = err; mtg_cards = Vec::<MtgCard>::new(); },
    }

    let initial_props = StartupProps {
        application_name: "Mtg booster generator".to_string(),
        mtg_cards: mtg_cards,
        errors: errors,
    };
    
    dioxus_desktop::launch_with_props(MainView, initial_props, Config::new());
}

/// The main component of the application.
#[allow(non_snake_case)]
fn MainView(cx: Scope<StartupProps>) -> Element {

    let text = use_state(cx, || vec!["erkki".to_string(), "jooseppi".to_string(), "exit".to_string()]); 
    let cards = use_state(cx, || cx.props.mtg_cards.clone());

    cx.render(rsx! {

        // Top level layout.
        div {
            style: "{MAIN_CONTAINER_STYLE}", 
            div {
                style: "{HEADER_CONTAINER_STYLE}", 
                HeaderComponent { some_text: text.get() },
            }
            div {
                style: "{THE_REST_CONTAINER_STYLE}", 
                "{cx.props.application_name}",
                td {
                    // map! {
                    //     cards.iter().enumerate().map(|(i, card)| {
                    //         CardComponent { card: card }
                    //     })
                    // }
                    tr { "hekotus" },
                    tr { "hekotus2" },
                    tr { "hekotus2" },
                    tr { "hekotus2" },
                    tr { "hekotus2" },
                    tr { "hekotus2" },
                    tr { "hekotus2" },
                    tr { "hekotus2" },
                    tr { "hekotus2" },
                    tr { "hekotus2" },
                }

                // CardComponent { card: &Card { pos_x: 550, pos_y: 135, } },
            }
        }
    })
}
