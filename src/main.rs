#![allow(non_snake_case)]
mod constants;
mod model;
mod ui;
mod utils;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use fluent_templates::Loader;
use model::app_state::ApplicationData;
use ui::nav_bar::NavBar;
// use serde_json::Value;
use constants::{LOCALES, STYLE};
use std::str::FromStr;
use unic_langid::LanguageIdentifier;
use utils::evals::*;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // Wrap Home in a Navbar Layout
    #[layout(NavBar)]
        // Default English
        #[route("/")]
        Home {},
        #[route("/:lang/")]
        HomeLang { lang: String },
    // And the regular page layout
    #[end_layout]

    // Finally, we need to handle the 404 page
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    launch(App);
}

fn App() -> Element {
    use_context_provider(|| Signal::new("en".to_string()));
    use_context_provider(ApplicationData::new);
    rsx! {
        LangSettings {}
        head::Link { rel: "stylesheet", href: STYLE }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    // let lang: Signal<String> = use_context();
    let data = use_context::<ApplicationData>();
    let nav = navigator();
    if &(data.lang_code)() as &str != "en" {
        nav.push(Route::HomeLang {
            lang: (data.lang_code)(),
        });
    }
    rsx! {
        HomeContent {}
    }
}

#[component]
fn HomeLang(lang: String) -> Element {
    // let lang: Signal<String> = use_context();
    let data = use_context::<ApplicationData>();
    let nav = navigator();
    if &(data.lang_code)() as &str == "en" {
        nav.push(Route::Home {});
    }
    rsx! {
        HomeContent {}
    }
}

#[component]
fn HomeContent() -> Element {
    let mut data = use_context::<ApplicationData>();
    // let lang: Signal<String> = use_context();
    let lang_id = &LanguageIdentifier::from_str(&(data.lang_code)() as &str).unwrap();
    rsx! {
        div {
            class: "p-4 text-2xl h-screen",
            onclick: move |_| { (data.show_lang_menu).set(false) },
            h1 { class: "font-bold", {LOCALES.lookup(lang_id, "hello-world")} }
            div { {LOCALES.lookup(lang_id, "homepage")} }
            p { {LOCALES.lookup(lang_id, "dioxus")} }
        }
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattempted to navigate to: {route:?}" }
    }
}
