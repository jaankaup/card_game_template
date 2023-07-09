use dioxus_desktop::Config;
use dioxus::prelude::*;
use card_game_gui::components::{StartupProps, HeaderComponent, CardComponent, Card};
use card_game_gui::meta_components::{UIDisplay};


const MAIN_CONTAINER_STYLE: &str = r#"
    display: contents;
    position: relative;
    color: red;
    background-color: rgb(235,235, 255);
    position: relative;
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
    background-color: rgb(235,235, 255);
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
    background-color: rgb(35,235, 255);
    align-items: left;
    border: 1px solid red;
    position: absolute;
    width: 100%;
    height: 100%;
    bottom: 0;
   "#;

fn main() {
    println!("{:?}", UIDisplay::INLINE_BLOCK);

    // Create initial data to the application.
    let initial_props = StartupProps {
        application_name: "My application".to_owned(),    
    };

    dioxus_desktop::launch_with_props(MainView, initial_props, Config::new());
}

/// The main component of the application.
#[allow(non_snake_case)]
fn MainView(cx: Scope<StartupProps>) -> Element {

    let app_name = use_state(cx, || cx.props.application_name.to_owned()); 
    let text = use_state(cx, || vec!["erkki".to_string(), "jooseppi".to_string(), "exit".to_string()]); 

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
                "{app_name.get()}",
                CardComponent { card: &Card { pos_x: 550, pos_y: 135, } },
            }
        }
    })
}
