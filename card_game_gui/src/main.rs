use dioxus_html::input_data::keyboard_types::Code;
use dioxus_desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
use dioxus::prelude::*;
use card_game_gui::components::{
    StartupProps,
    HeaderComponent,
    CardComponent,
    Card,
    HeaderButton,
    MainWindowState,
    MainWindowStates,
};
use card_game_gui::meta_components::{UIDisplay};
use card_game_gui::misc::loadFile;
use card_game_gui::xml_parser::{CardDataFiles, FileToInclude};
use card_game_gui::resource_loader::load_mtg_cards;
use card_game_gui::mana_symbol::{ManaSymbol, ManaType};
use hard_xml::XmlRead;
// use card_game_gui::resource_loader::MtgCard;
use card_game_gui::mtg_card::{MtgCard, CardImage};
use card_game_gui::window_view_icon::WindowViewIcon;

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

    // Window state. TODO: use ref?
    use_shared_state_provider(cx, || MainWindowStates(MainWindowState::DeckView)); 
    let shared_window_state = use_shared_state::<MainWindowStates>(cx).unwrap();

    let cards = use_state(cx, || cx.props.mtg_cards.clone());
    let css_things = use_state(cx, || loadCss());
    let white_checked = use_ref(cx, || false);
    let black_checked = use_ref(cx, || false);
    let red_checked = use_ref(cx, || false);
    let blue_checked = use_ref(cx, || false);
    let green_checked = use_ref(cx, || false);

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
                    text: "Generate random deck".to_string(),
                },
                HeaderButton {
                    text: "Clear".to_string(),
                },

                if shared_window_state.read().0 == MainWindowState::BoosterShop {
                    render! {
                        HeaderButton {
                            text: "Buy boosters".to_string(),
                        },
                    }
                }
                if shared_window_state.read().0 == MainWindowState::DeckView {
                    render! {
                        ManaSymbol { mana_type: ManaType::Plains, on_clicked: move |event: bool | { white_checked.set(event) } },
                        ManaSymbol { mana_type: ManaType::Mountain, on_clicked: move |event: bool | { red_checked.set(event) } },
                        ManaSymbol { mana_type: ManaType::Forest, on_clicked: move |event: bool | { green_checked.set(event) } },
                        ManaSymbol { mana_type: ManaType::Island, on_clicked: move |event: bool | { blue_checked.set(event) } },
                        ManaSymbol { mana_type: ManaType::Swamp, on_clicked: move |event: bool | { black_checked.set(event) } },
                    }
                }


                // Window state icons.
                div {
                    display: "flex",
                    flex_flow: "row-reverse",
                    background_color: "yellow",
                    width: "100%",
                    WindowViewIcon {
                        text: "Deck view".to_string(),
                        window_state: MainWindowState::DeckView,
                        // is_clicked: |evt: MouseEvent| { *shared_window_state.write = MainWindowState::DeckView; },
                        // is_selected: shared_window_state.read().0 == MainWindowState::DeckView,
                    },
                    WindowViewIcon {
                        text: "Random deck parameters".to_string(),
                        window_state: MainWindowState::RandomParametrs,
                        // is_clicked: |evt: MouseEvent| { *shared_window_state.write() = MainWindowState::RandomParametrs; },
                        // is_selected: shared_window_state.read().0 == MainWindowState::RandomParametrs,
                    },
                    WindowViewIcon {
                        text: "Booster shop view".to_string(),
                        window_state: MainWindowState::BoosterShop,
                        // is_clicked: |evt: MouseEvent| { *shared_window_state.write() = MainWindowState::BoosterShop; },
                        // is_selected: shared_window_state.read().0 == MainWindowState::BoosterShop,
                    },
                }
            }
            div {
                style: "{(*css_things.get()).the_rest_css_style}",
                onkeydown: handle_key_down_event,
                td {
                    for i in 0..100 {
                        CardImage { mtg_card: &cards.get()[i] } 
                    }
                }
            }
        }
    })
}
