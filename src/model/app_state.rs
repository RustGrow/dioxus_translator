use dioxus::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct ApplicationData {
    pub lang_code: Signal<String>,
}

impl ApplicationData {
    pub fn new() -> Self {
        Self {
            lang_code: Signal::new("en".to_string()),
        }
    }
}
