use dioxus::prelude::*;

/// Properties that are given to the main application.
#[derive(PartialEq, Props)]
pub struct StartupProps {
    pub application_name: String,
}
