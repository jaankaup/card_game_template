use dioxus_desktop::Config;
use dioxus::prelude::*;
use card_game_gui::components::{StartupProps}; 

fn main() {

    // Create initial data to the application.
    let initial_props = StartupProps {
        application_name: "My application".to_owned(),    
    };

    dioxus_desktop::launch_with_props(Application, initial_props, Config::new());
}

/// The main component of the application.
#[allow(non_snake_case)]
fn Application(cx: Scope<StartupProps>) -> Element {

    let app_name = use_state(cx, || cx.props.application_name.to_owned()); 
    cx.render(rsx! {

            h1 {
                "{app_name.get()}"
            }
    })
}
