use super::lang_drop::LangDropDown;
use crate::constants::LANG_CODES;
use crate::utils::evals::*;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    let mut lang: Signal<String> = use_context();

    rsx! {
        nav { class: "px-2 py-4 shadow-lg flex flex-row justify-around",
            ul { class: "flex flex-row w-full items-center",
                for code in LANG_CODES {
                    li { class: "ring-1 bg-blue-200 px-2 mx-2 rounded-lg",
                        match code {
                            "en" => rsx!{
                                Link {
                                    onclick: move |_| {
                                        lang.set(code.to_string());
                                        let eval = ButtonLang();
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
                                        let eval = ButtonLang();
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
            LangDropDown {}
        }
        Outlet::<Route> {}
    }
}
