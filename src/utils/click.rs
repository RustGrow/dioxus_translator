use crate::model::app_state::ApplicationData;
use dioxus::prelude::*;

pub fn close_elements() {
    let mut data = use_context::<ApplicationData>();
    (data.show_lang_menu).set(false);
}
