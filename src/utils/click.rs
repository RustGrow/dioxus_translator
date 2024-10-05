use crate::model::app_state::ApplicationData;
use dioxus::prelude::*;
// use gloo_events;

pub fn close_elements() {
    let mut data = use_context::<ApplicationData>();
    (data.show_lang_menu).set(false);
}

// pub fn outside_click() {
//     let mut data = use_context::<ApplicationData>();

//     let document = web_sys::window().unwrap().document().unwrap();
//     let _ = gloo_events::EventListener::new(&document, "click", move |_| {
//         (data.show_lang_menu).set(false);
//     });
// }
pub fn use_outside_click<F>(callback: F)
where
    F: FnMut(&web_sys::Event) + Clone + 'static,
{
    let mut event_listener = use_signal(|| None);

    use_effect(move || {
        let document = web_sys::window().unwrap().document().unwrap();
        let mut value = callback.clone();
        event_listener.set(Some(gloo_events::EventListener::new(
            &document,
            "click",
            move |e| {
                value(&e);
            },
        )))
    });
}

// origin
// fn use_outside_click<F>(callback: F)
// where
//     F: FnMut(&web_sys::Event) + Clone + 'static,
// {
//     let mut event_listener = use_signal(|| None);

//     use_effect(move || {
//         event_listener.set(Some(gloo_events::EventListener::new(
//             &document,
//             "click",
//             move |e| {
//                 callback(&e);
//             },
//         )))
//     });
// }
