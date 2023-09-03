use dioxus_html::input_data::keyboard_types::Code;
use dioxus_desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
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
    background-color: red;
    position: relative;
    align-items: left;
    height: 50px;
    width: 100%;
    position: absolute;
   "#;

const THE_REST_CONTAINER_STYLE: &str = r#"
    margin: 0;
    padding: 0;
    top: 250px;
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

struct CssThings {
    main_css_style: String,
    header_css_style: String,
    the_rest_css_style: String,
}

fn loadCss() -> CssThings  {
    let main_css_file = loadFile("css/main_css_file.css").unwrap();
    let header_css_file = loadFile("css/header_css_file.css").unwrap();
    let the_rest_css_file = loadFile("css/the_rest_css_file.css").unwrap();
    CssThings {
        main_css_style: main_css_file,
        header_css_style: header_css_file,
        the_rest_css_style: the_rest_css_file,
    }
}


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
    
    let config = Config::new().with_window(WindowBuilder::default().with_title("Mtg booster generator")
                                           .with_inner_size(dioxus_desktop::LogicalSize::new(1000, 1000))
                                          );  
    
    dioxus_desktop::launch_with_props(MainView, initial_props, config);
}

/// The main component of the application.
#[allow(non_snake_case)]
fn MainView(cx: Scope<StartupProps>) -> Element {

    let text = use_state(cx, || vec!["erkki".to_string(), "jooseppi".to_string(), "exit".to_string()]); 
    let cards = use_state(cx, || cx.props.mtg_cards.clone());
    let mainStyle = use_state(cx, || MAIN_CONTAINER_STYLE.to_string());
    let css_things = use_state(cx, || loadCss());
    let handle_key_down_event = move |evt: KeyboardEvent| 
        match evt.code() {
            Code::Space => {
                println!("Reloading css file.");
                css_things.set(loadCss());
            }
            _ => {  },
        };

    cx.render(rsx! {

        // Top level layout.
        body {
            tabindex: "0",
            autofocus: "true",
            style: "{(*css_things.get()).main_css_style}",
            onkeydown: handle_key_down_event,
            onkeypress: handle_key_down_event,
            onkeyup: handle_key_down_event,
            onclick: move |_| { println!("Jeejee"); },
                
            div {
                style: "{(*css_things.get()).header_css_style}",
                onkeydown: handle_key_down_event,
            }
            div {
                style: "{(*css_things.get()).the_rest_css_style}",
                onkeydown: handle_key_down_event,
                "{cx.props.application_name}",
                td {
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

            }
        }
    })
}
