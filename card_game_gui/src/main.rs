use dioxus_html::input_data::keyboard_types::Code;
use dioxus_desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
use dioxus::prelude::*;
use card_game_gui::components::{StartupProps, HeaderComponent, CardComponent, Card, HeaderButton};
use card_game_gui::meta_components::{UIDisplay};
use card_game_gui::misc::loadFile;
use card_game_gui::xml_parser::{CardDataFiles, FileToInclude};
use card_game_gui::resource_loader::load_mtg_cards;
use card_game_gui::mana_symbol::{ManaSymbol, ManaType};
use hard_xml::XmlRead;
use card_game_gui::resource_loader::MtgCard;

struct CssThings {
    main_css_style: String,
    header_css_style: String,
    the_rest_css_style: String,
    header_button_css_style: String,
}

fn loadCss() -> CssThings  {
    let main_css_file = loadFile("css/main_css_file.css").unwrap();
    let header_css_file = loadFile("css/header_css_file.css").unwrap();
    let the_rest_css_file = loadFile("css/the_rest_css_file.css").unwrap();
    let header_button_css_file = loadFile("css/header_button_css_file.css").unwrap();
    CssThings {
        main_css_style: main_css_file,
        header_css_style: header_css_file,
        the_rest_css_style: the_rest_css_file,
        header_button_css_style: header_button_css_file,
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
                                           .with_inner_size(dioxus_desktop::LogicalSize::new(1920, 1080))
                                          );  
    
    dioxus_desktop::launch_with_props(MainView, initial_props, config);
}

/// The main component of the application.
#[allow(non_snake_case)]
fn MainView(cx: Scope<StartupProps>) -> Element {

    let text = use_state(cx, || vec!["erkki".to_string(), "jooseppi".to_string(), "exit".to_string()]); 
    let cards = use_state(cx, || cx.props.mtg_cards.clone());
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
        div {
            tabindex: "0",
            autofocus: "true",
            style: "{(*css_things.get()).main_css_style}",
            onkeypress: handle_key_down_event,
            onclick: move |_| { println!("Jeejee"); },
                
            div {
                style: "{(*css_things.get()).header_css_style}",
                onkeydown: handle_key_down_event,
                HeaderButton {
                    text: "Load deck".to_string(),                    
                },
                HeaderButton {
                    text: "Save deck".to_string(),                    
                },
                HeaderButton {
                    text: "Buy boosters".to_string(),                    
                },
                HeaderButton {
                    text: "Generate random deck".to_string(),                    
                },
                HeaderButton {
                    text: "Clear".to_string(),                    
                },
            }
            div {
                style: "{(*css_things.get()).the_rest_css_style}",
                onkeydown: handle_key_down_event,
                td {
                    // CardComponent {
                    //     card: &Card {
                    //         pos_x: 400,                            
                    //         pos_y: 300,                            
                    //     }
                    // },
                    // CardComponent {
                    //     card: &Card {
                    //         pos_x: 600,                            
                    //         pos_y: 500,                            
                    //     }
                    // },
                    // img {
                    //     src: "circle-heat.svg",
                    // }
                    ManaSymbol { mana_type: ManaType::Plains,},
                    ManaSymbol { mana_type: ManaType::Mountain,},
                    ManaSymbol { mana_type: ManaType::Forest,},
                    ManaSymbol { mana_type: ManaType::Island,},
                    ManaSymbol { mana_type: ManaType::Swamp,},
                }
            }
        }
    })
}
