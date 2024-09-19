use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[component]
pub fn LangSettings() -> Element {
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

            const supportedLangs = {
                "en": true,
                "de": true,
                "es": true,
                "ar": true
            };

            // The code checks if the browser language is in the list of supported languages,
            // and if not, sets English as the default language.
            if (!supportedLangs[langCode]) {
                langCode = "en";
            }

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

            dioxus.send(lang);

            // How to get arr from JS to Dioxus
            // let arr = [lang, langCode];
            // dioxus.send(arr);
            
            "#,
        );
        let js_lang = eval.recv().await.unwrap();
        *lang.write() = String::from(js_lang.as_str().unwrap());

        // Working with array from JS
        // if js_lang[0] == Value::Null {
        //     // Get lang from browser lang
        //     // *lang.write() = String::from(js_lang[1].as_str().unwrap());
        //     // *lang.write() = String::from("de");
        // } else {
        //     // Get lang from browser storage
        //     *lang.write() = String::from(js_lang[0].as_str().unwrap());
        // }
    });
    info!("Lang is {}", lang());
    rsx! {}
}

pub fn ButtonLang() -> UseEval {
    eval(
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

        let lang = await dioxus.recv();
        localStorage.setItem("lang", lang);
        
        // Set language and direction in HTML
        if (lang === "ar" || lang === "he") {
            setHtmlLanguageAndDirection(lang, 'rtl');
        } else {
            setHtmlLanguageAndDirection(lang, null);
        }
        
        "#,
    )
}
