#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use fluent_templates::{static_loader, Loader};
use utils::evals::*;
// use serde_json::Value;
use std::str::FromStr;
use unic_langid::LanguageIdentifier;
mod utils;

const STYLE: &str = asset!("./assets/tailwind.css");

static_loader! {
    static LOCALES = {
        locales: "./lang",
        fallback_language: "en-US",
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

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

    rsx! {
        // Lang JS from component
        Lang_settings {}
        head::Link { rel: "stylesheet", href: STYLE }
        // Note: For development use only. Remove before production.
        Script { src: "https://cdn.tailwindcss.com" }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let lang: Signal<String> = use_context();
    let nav = navigator();
    if &lang() as &str != "en" {
        nav.push(Route::HomeLang { lang: lang() });
    }
    rsx! {
        HomeContent {}
    }
}

#[component]
fn HomeLang(lang: String) -> Element {
    let lang: Signal<String> = use_context();
    let nav = navigator();
    if &lang() as &str == "en" {
        nav.push(Route::Home {});
    }
    rsx! {
        HomeContent {}
    }
}

#[component]
fn HomeContent() -> Element {
    let lang: Signal<String> = use_context();
    let lang_id = &LanguageIdentifier::from_str(&lang() as &str).unwrap();
    rsx! {
        div { class: "p-4 text-2xl",
            h1 { class: " font-bold ", {LOCALES.lookup(lang_id, "hello-world")} }
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

#[component]
fn NavBar() -> Element {
    let mut lang: Signal<String> = use_context();
    let lang_code = vec!["en", "de", "es", "ar"];

    rsx! {
        nav { class: "px-2 py-4 shadow-lg",
            ul { class: "flex flex-row w-full",
                for code in lang_code {
                    li { class: "ring-1 bg-blue-200 px-2 mx-2 rounded-lg",
                        match code {
                            "en" => rsx!{
                                Link {
                                    onclick: move |_| {
                                        lang.set(code.to_string());
                                        // JS from fn
                                        let eval = Button_Lang();
                                        eval.send(code.into()).unwrap();
                                    },
                                    to: Route::Home {},
                                    "{code}"
                                },
                            },
                            _ => rsx!{
                                Link {
                                    onclick: move |_| {
                                        lang.set(code.to_string());
                                        // JS from fn
                                        let eval = Button_Lang();
                                        eval.send(code.into()).unwrap();
                                    },
                                    to: Route::HomeLang {
                                        lang: code.to_string(),
                                    },
                                    "{code}"
                                }
                            }
                        }
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}
