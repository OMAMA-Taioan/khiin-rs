use std::collections::HashMap;
use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::resource::*;
use crate::collection;

#[derive(Clone, Copy)]
pub enum Locale {
    EN,
    OAN,
}

static TRANSLATIONS_EN: Lazy<HashMap<u16, &'static str>> =
    Lazy::new(|| {
        collection!(
            IDS_WINDOW_CAPTION =>             "Khíín Taiwanese IME Settings",
            IDD_APPEARANCETAB =>              "Display",
            IDL_COLOR =>                      "Colors:",
            IDS_LIGHT_THEME =>                "Light",
            IDS_DARK_THEME =>                 "Dark",
            IDL_CANDIDATE_SIZE =>             "Candidate font size:",
            IDL_CANDIDATE_SIZE_S =>           "Smaller",
            IDL_CANDIDATE_SIZE_L =>           "Larger",
            IDL_DISPLAY_LANGUAGE =>           "Display language (介面語言):",
            IDS_DISPLAY_LANGUAGE_EN =>        "English",
            IDS_DISPLAY_LANGUAGE_HANLO =>     "漢羅台 (Hanlo Taiwanese)",
            IDS_DISPLAY_LANGUAGE_LO =>        "Lô-jī Tâi (Romanized Taiwanese)",
            IDL_EDIT_TRY =>                   "Try it:",
        )
    });

static TRANSLATIONS_OAN: Lazy<HashMap<u16, &'static str>> =
    Lazy::new(|| {
        collection!(
            IDS_WINDOW_CAPTION =>             "起引台語打字法設置",
            IDD_APPEARANCETAB =>              "外皮",
            IDL_COLOR =>                      "色水：",
            IDS_LIGHT_THEME =>                "白底",
            IDS_DARK_THEME =>                 "烏底",
            IDL_CANDIDATE_SIZE =>             "揀字大細：",
            IDL_CANDIDATE_SIZE_S =>           "Khah 細",
            IDL_CANDIDATE_SIZE_L =>           "Khah 大",
            IDL_EDIT_TRY =>                   "打看覓仔：",
            IDL_DISPLAY_LANGUAGE =>           "介面語言 (Display Language)：",
            IDS_DISPLAY_LANGUAGE_EN =>        "英語 (English)",
            IDS_DISPLAY_LANGUAGE_HANLO =>     "漢羅台",
            IDS_DISPLAY_LANGUAGE_LO =>        "Lô-jī Tâi",
        )
    });

static CURRENT_LOCALE: Lazy<Mutex<Locale>> =
    Lazy::new(|| Mutex::new(Locale::EN));

pub fn translate(locale: Locale, key: u16) -> String {
    let string_set = match locale {
        Locale::EN => &TRANSLATIONS_EN,
        Locale::OAN => &TRANSLATIONS_OAN,
    };

    match string_set.get(&key) {
        Some(value) => value.to_string(),
        None => key.to_string(),
    }
}

pub fn set_locale(locale: Locale) {
    let mut current_locale = CURRENT_LOCALE.lock().unwrap();
    *current_locale = locale;
}

pub fn t(key: u16) -> String {
    let locale = CURRENT_LOCALE.lock().unwrap();
    translate(*locale, key)
}

