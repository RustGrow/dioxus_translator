#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use fluent_templates::{static_loader, Loader};
use std::{collections::HashMap, str::FromStr};
use unic_langid::{langid, LanguageIdentifier};

const US_ENGLISH: LanguageIdentifier = langid!("en-US");
const SPANISH: LanguageIdentifier = langid!("es");
const GERMAN: LanguageIdentifier = langid!("de");

static_loader! {
    static LOCALES = {
        locales: "./assets/lang",
        fallback_language: "en-US",
        // Removes unicode isolating marks around arguments, you typically
        // should only set to false when testing.
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    // Default English
    #[route("/")]
    Home {},
    #[route("/:lang/")]
    HomeLang { lang: String },
    // http://site.com/de/market/2024-09-09-post-name-slug/
    // #[route("/:category/:slug/")] // Default English
    // Blog {
    //     category: String,
    //     slug: String, //2024-09-09-post-name-slug
    // },
    // #[route("/:lang/:category/:slug/")]
    // BlogLang {
    //     lang: String,
    //     category: String,
    //     slug: String, //2024-09-09-post-name-slug
    // },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    use_context_provider(|| Signal::new("en".to_string()));

    rsx! {
        Router::<Route> {}
        script { src: asset!("https://cdn.tailwindcss.com") }
        // for manganis
        head::Link { rel: "stylesheet", href: asset!("./assets/tailwind.css") }
    }
}

#[component]
fn Home() -> Element {
    let lang: Signal<String> = use_context();

    println!("US - {}", LOCALES.lookup(&US_ENGLISH, "hello-world"));
    println!("SPANISH - {}", LOCALES.lookup(&SPANISH, "hello-world"));
    println!("GERMAN - {}", LOCALES.lookup(&GERMAN, "hello-world"));

    rsx! {
        div { class: "p-4", "Homepage with language {lang}" }
        // p { {LOCALES.lookup(&US_ENGLISH, "hello-world")} }
        // p { {LOCALES.lookup(&LanguageIdentifier::from_str(&lang.read() as &str), "hello-world")} }
        p { class: "p-4",
            {LOCALES.lookup(&LanguageIdentifier::from_str(&lang() as &str ).unwrap(), "hello-world")}
        }
        div {
            // h1 { class: " p-4", "Change language to {lang}" }
            div { class: "p-4", Languages {} }
        }
    }
}

#[component]
fn HomeLang(lang: String) -> Element {
    rsx! {
        Home {}
    }
}

#[component]
fn Languages() -> Element {
    let mut lang: Signal<String> = use_context();

    // let change_to_english = move |_| i18n.set_language(langid!("en"));
    // let change_to_spanish = move |_| i18n.set_language(langid!("es-ES"));
    let lang_code = vec!["en", "de", "es"];

    rsx! {
        ul { class: "flex flex-row space-x-5",
            for code in lang_code {
                li { class: " ring-1 bg-blue-200 px-2 py-0 rounded-lg",
                    match code {
                        "en" => rsx!{
                            Link {
                                onclick: move |_| lang.set(code.to_string()),
                                to: Route::Home {},
                                "{code}"
                            },
                        },
                        _ => rsx!{
                            Link {
                                onclick: move |_| lang.set(code.to_string()),
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
}

#[component]
fn Blog(lang: String) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
    }
}

#[component]
fn BlogLang(lang: String) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
    }
}
