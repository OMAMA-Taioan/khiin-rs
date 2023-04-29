use std::collections::HashMap;
use std::sync::Mutex;

use fluent_bundle::FluentValue;
use fluent_templates::LanguageIdentifier;
use fluent_templates::Loader;
use once_cell::sync::Lazy;
use unic_langid::langid;

static ENGLISH: LanguageIdentifier = langid!("en-US");
static TAILO: Lazy<LanguageIdentifier> = Lazy::new(|| langid!("und-Latn-TW"));
static HANLO: Lazy<LanguageIdentifier> = Lazy::new(|| langid!("und-TW"));
static CURRENT_LOCALE: Lazy<Mutex<&LanguageIdentifier>> =
    Lazy::new(|| Mutex::new(&ENGLISH));

fluent_templates::static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
    };
}

pub enum Locales {
    English,
    Tailo,
    Hanlo,
}

pub fn set_locale(locale: Locales) {
    let locale = match locale {
        Locales::English => &ENGLISH,
        Locales::Tailo => &TAILO,
        Locales::Hanlo => &HANLO,
    };

    let mut curr_locale = CURRENT_LOCALE.lock().unwrap();
    *curr_locale = locale;
}

pub fn t(key: &str) -> String {
    let locale = CURRENT_LOCALE.lock().unwrap();
    LOCALES.lookup(&locale, key).unwrap_or(key.to_string())
}

pub fn t_args<T: AsRef<str>>(
    key: &str,
    args: &HashMap<T, FluentValue>,
) -> String {
    let locale = CURRENT_LOCALE.lock().unwrap();
    LOCALES
        .lookup_with_args(&locale, key, args)
        .unwrap_or(key.to_string())
}
