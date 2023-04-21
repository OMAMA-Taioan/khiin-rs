use std::collections::HashMap;
use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::collection;
use crate::app::resource::*;

#[derive(Clone, Copy)]
pub enum Locale {
    EN,
    OAN,
}

static TRANSLATIONS_EN: Lazy<HashMap<u16, &'static str>> = Lazy::new(|| {
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
        IDD_INPUTTAB =>                   "Input",
        IDL_INPUTMODE =>                  "Input Mode",
        IDC_INPUTMODE_CONTINUOUS =>       "Continuous: just keep typing",
        IDC_INPUTMODE_BASIC =>            "Basic: one word at a time",
        IDC_INPUTMODE_PRO =>              "Manual: no assistance from the IME",
        IDL_INPUTMODE_HOTKEY =>            "Switch mode:",
        IDS_INPUTMODE_KEY_CTRL_PERIOD =>   "ctrl + .",
        IDS_INPUTMODE_KEY_CTRL_BACKTICK => "ctrl + ` (~)",
        IDL_ON_OFF_HOTKEY =>              "IME quick on/off:",
        IDS_ON_OFF_HOTKEY_ALTBACKTICK =>  "alt + ` (~)",
        IDS_ON_OFF_HOTKEY_SHIFT =>        "shift",
        IDL_TONE_KEYS =>                  "Tone keys:",
        IDS_TONE_KEYS_NUMERIC =>          "Numeric: 2 3 5 7 8 9 0",
        IDS_TONE_KEYS_TELEX =>            "Telex: s f l j j w q",
        IDL_DOTTED_O_KEY =>               "Input o͘ :",
        IDS_DOTTED_O_OU =>                "ou",
        IDS_DOTTED_O_OO =>                "oo",
        IDS_DOTTED_O_Y =>                 "y",
        IDL_NASAL_KEY =>                  "Input ⁿ :",
        IDS_NASAL_NN =>                   "nn",
        IDS_NASAL_V =>                    "v",
        IDC_OPTION_UPPERCASE_NASAL =>     "Use upper ᴺ",
        IDC_OPTION_DOTTED_KHIN =>         "Convert -- to · (khin dot)",
        IDC_OPTION_AUTOKHIN =>            "Auto khin following syllables",
        IDC_OPTION_EASY_CH =>             "EZ ch (type c for ch)",
        IDL_DEFAULT_PUNCTUATION =>        "Default Punctuation",
        IDS_PUNCT_FULL_WIDTH =>           "Full width (。、！)",
        IDS_PUNCT_HALF_WIDTH =>           "Half width (. , !)",
        IDL_HYPHEN_KEY =>                 "Input -:",
        IDS_HYPHEN_KEY_HYPHEN =>          "-",
        IDS_HYPHEN_KEY_V =>               "v",
        IDD_DICTIONARYTAB =>              "Dictionary",
        IDL_RESET_USERDATA =>             "Clear input history:\n\nKhíín uses your typing history to improve candidate prediction.\n\nWARNING: clearing this history cannot be undone!",
        IDC_RESET_USERDATA_BTN =>         "Clear Now",
        IDL_RESET_USERDATA_BTN_DONE =>    "Cleared!",
        IDL_EDIT_USERDICT =>              "Custom dictionary:\n\n • One entry per line.\n • Format as <input output> (space between).\n • Input must be letters or numbers.\n • Output can be any words or symbols.",
        IDC_EDIT_USEDICT_BTN =>           "Open File",

        IDD_ABOUTTAB =>                   "About",
        IDL_KHIIN_VERSION =>              "Khíín PJH v0.1.0",
        IDL_KHIIN_COPYRIGHT =>            "Released under the MIT license",
    )
});

static TRANSLATIONS_OAN: Lazy<HashMap<u16, &'static str>> = Lazy::new(|| {
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
        IDD_INPUTTAB =>                    "打字",
        IDL_INPUTMODE =>                   "打字模式",
        IDC_INPUTMODE_CONTINUOUS =>        "自：電腦自動切語詞",
        IDC_INPUTMODE_BASIC =>             "揀：我切語詞、電腦鬥揀字",
        IDC_INPUTMODE_PRO =>               "手：電腦無鬥相共",
        IDL_INPUTMODE_HOTKEY =>            "換打字模式：",
        IDS_INPUTMODE_KEY_CTRL_PERIOD =>   "ctrl + .",
        IDS_INPUTMODE_KEY_CTRL_BACKTICK => "ctrl + ` (~)",
        IDL_ON_OFF_HOTKEY =>               "打字法切掉．点灱：",
        IDS_ON_OFF_HOTKEY_ALTBACKTICK =>   "alt + ` (~)",
        IDS_ON_OFF_HOTKEY_SHIFT =>         "shift",
        IDL_TONE_KEYS =>                   "調号：",
        IDS_TONE_KEYS_NUMERIC =>           "打數字： 2 3 5 7 8 9 0",
        IDS_TONE_KEYS_TELEX =>             "Telex： s f l j j w q",
        IDL_DOTTED_O_KEY =>               "打「o͘」：",
        IDS_DOTTED_O_OU =>                "ou",
        IDS_DOTTED_O_OO =>                "oo",
        IDS_DOTTED_O_Y =>                 "y",
        IDL_NASAL_KEY =>                  "打「ⁿ」：",
        IDS_NASAL_NN =>                   "nn",
        IDS_NASAL_V =>                    "v",
        IDC_OPTION_UPPERCASE_NASAL =>     "使用大本「ᴺ」",
        IDC_OPTION_DOTTED_KHIN =>         "「--」換「·」：打双連劃共換做輕点",
        IDC_OPTION_AUTOKHIN =>            "打輕了後、自動共後者變輕",
        IDC_OPTION_EASY_CH =>             "「c」換「ch」：打 c 自動加一个 h",
        IDL_DEFAULT_PUNCTUATION =>        "標点符号：",
        IDS_PUNCT_FULL_WIDTH =>           "全 (漢字式 。、！)",
        IDS_PUNCT_HALF_WIDTH =>           "半 (羅字式 . , !)",
        IDL_HYPHEN_KEY =>                 "連劃「-」：",
        IDS_HYPHEN_KEY_HYPHEN =>          "-",
        IDS_HYPHEN_KEY_V =>               "v",
        IDD_DICTIONARYTAB =>              "詞庫",
        IDL_RESET_USERDATA =>             "清使用者打字統計。注！袂使 UNDO。",
        IDC_RESET_USERDATA_BTN =>         "Liâm-mi 清掉",
        IDL_RESET_USERDATA_BTN_DONE =>    "清好啞",
        IDL_EDIT_USERDICT =>              "使用者 ka-kī ê 資料庫",
        IDC_EDIT_USEDICT_BTN =>           "編輯",
        IDD_ABOUTTAB =>                   "起引",
        IDL_KHIIN_VERSION =>              "起引打字法 v0.1.0",
        IDL_KHIIN_COPYRIGHT =>            "Released under the MIT license",
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
