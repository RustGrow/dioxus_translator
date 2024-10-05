use super::lang_drop::LangDropDown;
use crate::constants::LANG_CODES;
use crate::model::app_state::ApplicationData;
use crate::utils::click::close_elements;
use crate::utils::evals::*;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    // let mut lang: Signal<String> = use_context();
    let mut data = use_context::<ApplicationData>();

    rsx! {
        nav { class: "px-2 py-4 shadow-lg flex flex-row justify-around",
            // onclick: move |_| {
            //     close_elements();
            // },
            ul { class: "flex flex-row w-full items-center",
                for code in LANG_CODES {
                    li { class: "ring-1 bg-blue-200 px-2 mx-2 rounded-lg",
                        match code {
                            "en" => rsx!{
                                Link {
                                    onclick: move |_| {
                                        (data.lang_code).set(code.to_string());
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
                                        (data.lang_code).set(code.to_string());
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
            div {
                class: "bg-blue-200 ring-2 mx-40 px-4 py-2 rounded-lg",
                onclick: move |ev| {
                    ev.stop_propagation();
                },
                "stop_propagation"
            }
            LangDropDown {}
        }
        Outlet::<Route> {}
    }
}
