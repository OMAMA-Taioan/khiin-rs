use std::collections::HashMap;
use std::fmt::format;
use std::sync::Mutex;

use once_cell::sync::Lazy;

static ALL_TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut m = HashMap::new();
        // en
        m.insert("en.continuous", "Continuous Mode");
        m.insert("en.basic", "Basic Mode");
        m.insert("en.manual", "Continuous Mode");
        m.insert("en.direct", "Alphanumeric (Direct input)");
        m.insert("en.settings", "Khíín Settings");

        // oan-hanl
        m.insert("oan-hanl.continuous", "連續打字");
        m.insert("oan-hanl.basic", "隋个隋个打");
        m.insert("oan-hanl.manual", "加忌打");
        m.insert("oan-hanl.direct", "干焦打 ABC");
        m.insert("oan-hanl.settings", "起引打字法設置");
        m
    });

static CURRENT_LOCALE: Lazy<Mutex<String>> =
    Lazy::new(|| Mutex::new(String::from("en")));

/// Get I18n text by locale and key
pub fn translate(locale: &str, key: &str) -> String {
    let key = {
        let res = format(format_args!("{0}.{1}", locale, key));
        res
    };

    match ALL_TRANSLATIONS.get(key.as_str()) {
        Some(value) => value.to_string(),
        None => key.to_string(),
    }
}

pub fn set_locale(locale: &str) {
    let mut current_locale = CURRENT_LOCALE.lock().unwrap();
    *current_locale = locale.to_string();
}

pub fn t(key: &str) -> String {
    let locale = CURRENT_LOCALE.lock().unwrap();
    translate(&locale[..], key)
}
