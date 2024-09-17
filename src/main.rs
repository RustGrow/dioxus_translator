#![allow(non_snake_case)]
mod constants;
mod domain;
mod repo;
mod utils;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use fluent_templates::{static_loader, Loader};
use redb::{AccessGuard, Database, Error, ReadableTable, TableDefinition};
use repo::db::*;
use serde_json::Value;
use std::{collections::HashMap, str::FromStr};
use unic_langid::{langid, LanguageIdentifier};
use utils::lang;

static_loader! {
    static LOCALES = {
        locales: "./lang",
        fallback_language: "en-US",
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
    #[route("/:category/:slug/")] // Default English
    Blog {
        category: String,
        slug: String, //2024-09-09-post-name-slug
    },
    #[route("/:lang/:category/:slug/")]
    BlogLang {
        lang: String,
        category: String,
        slug: String, //2024-09-09-post-name-slug
    },
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    // let db = Database::create("my_db.redb").expect("Error");
    launch(App);
}

fn App() -> Element {
    use_context_provider(|| Signal::new("en".to_string()));
    let mut lang: Signal<String> = use_context();

    let _ = use_resource(move || async move {
        let mut eval = eval(
            r#"
                // Function to set lang and dir attributes for the <html> tag
                function setHtmlLanguageAndDirection(lang, dir) {
                    var htmlElement = document.documentElement;
                    htmlElement.lang = lang;
                    if (dir) {
                        htmlElement.dir = dir;
                    } else {
                        htmlElement.removeAttribute('dir');
                    }
                }

                let lang = localStorage.getItem("lang");
                let browserLang = navigator.language || navigator.userLanguage;
                let langCode = browserLang.substring(0, 2);

                if (langCode === "zh") {
                    if (browserLang === "zh-CN" || browserLang === "zh-SG") {
                        langCode = "zh-Hans"; // Simplified Chinese
                    } else if (browserLang === "zh-TW" || browserLang === "zh-HK" || browserLang === "zh-MO") {
                        langCode = "zh-Hant"; // Traditional Chinese
                    }
                }

                // If there is no language value in local storage, set it
                if (!lang) {
                    localStorage.setItem("lang", langCode);
                    lang = langCode;
                }

                // Set language and direction in HTML
                if (lang === "ar" || lang === "he") {
                    setHtmlLanguageAndDirection(lang, 'rtl');
                } else {
                    setHtmlLanguageAndDirection(lang, null);
                }
                // let arr = [lang, langCode];
                dioxus.send(lang);
                "#,
        );
        let js_lang = eval.recv().await.unwrap();
        *lang.write() = String::from(js_lang.as_str().unwrap());

        // if js_lang == Value::Null {
        //     // Get lang from browser lang
        //     // *lang.write() = String::from(js_lang[1].as_str().unwrap());
        //     // *lang.write() = String::from("de");
        // } else {
        //     // Get lang from browser storage
        //     *lang.write() = String::from(js_lang.as_str().unwrap());
        // }
    });
    info!("Lang is {}", lang());

    rsx! {
        Router::<Route> {}
        script { src: asset!("https://cdn.tailwindcss.com") }
        head::Link { rel: "stylesheet", href: asset!("./assets/tailwind.css") }
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
        div { class: "p-4", "Homepage with language {lang}" }
        p { class: "p-4", {LOCALES.lookup(lang_id, "hello-world")} }
        div {
            div { class: "p-4", Languages {} }
        }
    }
}

// http://site.com/market/2024-09-09-post-name-slug/
#[component]
fn Blog(category: String, slug: String) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
    }
}

// http://site.com/de/market/2024-09-09-post-name-slug/
#[component]
fn BlogLang(lang: String, category: String, slug: String) -> Element {
    rsx! {
        Blog { category, slug }
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
fn Languages() -> Element {
    let mut lang: Signal<String> = use_context();

    let lang_code = vec!["en", "de", "es", "ar"];

    rsx! {
        ul { class: "flex flex-row space-x-5",
            for code in lang_code {
                li { class: " ring-1 bg-blue-200 px-2 py-0 rounded-lg",
                    match code {
                        "en" => rsx!{
                            Link {
                                onclick: move |_| {
                                    lang.set(code.to_string());
                    
                                    let eval = eval(r#"
                                    // Function to set lang and dir attributes for the <html> tag
                                    function setHtmlLanguageAndDirection(lang, dir) {
                                        var htmlElement = document.documentElement;
                                        htmlElement.lang = lang;
                                        if (dir) {
                                            htmlElement.dir = dir;
                                        } else {
                                            htmlElement.removeAttribute('dir');
                                        }
                                    }
                    
                                    let lang = await dioxus.recv();
                                    localStorage.setItem("lang", lang);
                                    
                                    // Set language and direction in HTML
                                    if (lang === "ar" || lang === "he") {
                                        setHtmlLanguageAndDirection(lang, 'rtl');
                                    } else {
                                        setHtmlLanguageAndDirection(lang, null);
                                    }
                                    
                                    "#);
                    
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
                    
                                    let eval = eval(r#"
                                    // Function to set lang and dir attributes for the <html> tag
                                    function setHtmlLanguageAndDirection(lang, dir) {
                                        var htmlElement = document.documentElement;
                                        htmlElement.lang = lang;
                                        if (dir) {
                                            htmlElement.dir = dir;
                                        } else {
                                            htmlElement.removeAttribute('dir');
                                        }
                                    }
                    
                                    let lang = await dioxus.recv();
                                    localStorage.setItem("lang", lang);
                                    
                                    // Set language and direction in HTML
                                    if (lang === "ar" || lang === "he") {
                                        setHtmlLanguageAndDirection(lang, 'rtl');
                                    } else {
                                        setHtmlLanguageAndDirection(lang, null);
                                    }
                                    
                                    "#);
                    
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
}
