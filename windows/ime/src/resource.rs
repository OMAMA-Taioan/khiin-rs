// Until someone finds a better way, this file
// needs to be kept in sync with res/resource.h

use windows::core::PCWSTR;

pub static IDM_MANIFEST: u16 =                    1;
pub static IDS_TEXT_SERVICE_DISPLAY_NAME: u16 =   101;
pub static IDI_MAINICON: u16 =                    102;
pub static IDI_MODE_ALPHA: u16 =                  103;
pub static IDI_MODE_ALPHA_W: u16 =                104;
pub static IDI_MODE_CONTINUOUS: u16 =             105;
pub static IDI_MODE_CONTINUOUS_W: u16 =           106;
pub static IDI_MODE_PRO: u16 =                    107;
pub static IDI_MODE_PRO_W: u16 =                  108;
pub static IDI_MODE_BASIC: u16 =                  109;
pub static IDI_MODE_BASIC_W: u16 =                110;
pub static IDI_SETTINGS: u16 =                    111;
pub static IDI_SETTINGS_W: u16 =                  112;
pub static IDR_POPUP_MENU: u16 =                  113;
pub static IDS_CONTINUOUS_MODE: u16 =             1000;
pub static IDS_BASIC_MODE: u16 =                  1001;
pub static IDS_MANUAL_MODE: u16 =                 1002;
pub static IDS_DIRECT_MODE: u16 =                 1003;
pub static IDS_OPEN_SETTINGS: u16 =               1004;

// Settings
pub static IDD_ABOUTTAB: u16 =                    2000;
pub static IDS_LIGHT_THEME: u16 =                 2001;
pub static IDS_DARK_THEME: u16 =                  2002;
pub static IDS_DISPLAY_LANGUAGE_EN: u16 =         2003;
pub static IDS_DISPLAY_LANGUAGE_HANLO: u16 =      2004;
pub static IDS_DISPLAY_LANGUAGE_LO: u16 =         2005;
pub static IDS_WINDOW_CAPTION: u16 =              2006;
pub static IDD_APPEARANCETAB: u16 =               2007;
pub static IDL_COLOR: u16 =                       2008;
pub static IDC_COMBOBOX_THEME_COLOR: u16 =        2009;
pub static IDL_CANDIDATE_SIZE: u16 =              2010;
pub static IDC_CANDIDATE_SIZE: u16 =              2011;
pub static IDL_CANDIDATE_SIZE_S: u16 =            2012;
pub static IDL_CANDIDATE_SIZE_L: u16 =            2013;
pub static IDC_EDIT_TRY: u16 =                    2014;
pub static IDL_EDIT_TRY: u16 =                    2015;
pub static IDC_DISPLAY_LANGUAGE: u16 =            2016;
pub static IDL_DISPLAY_LANGUAGE: u16 =            2017;
pub static IDD_INPUTTAB: u16 =                    2018;
pub static IDL_INPUTMODE: u16 =                   2019;
pub static IDC_INPUTMODE_CONTINUOUS: u16 =        2020;
pub static IDC_INPUTMODE_BASIC: u16 =             2021;
pub static IDC_INPUTMODE_PRO: u16 =               2022;
pub static IDL_INPUTMODE_HOTKEY: u16 =            2023;
pub static IDC_INPUTMODE_KEY_COMBO: u16 =         2024;
pub static IDS_INPUTMODE_KEY_CTRL_PERIOD: u16 =   2025;
pub static IDS_INPUTMODE_KEY_CTRL_BACKTICK: u16 = 2026;
pub static IDL_ON_OFF_HOTKEY: u16 =               2027;
pub static IDC_ON_OFF_HOTKEY_COMBO: u16 =         2028;
pub static IDS_ON_OFF_HOTKEY_SHIFT: u16 =         2029;
pub static IDS_ON_OFF_HOTKEY_ALTBACKTICK: u16 =   2030;
pub static IDL_DEFAULT_PUNCTUATION: u16 =         2031;
pub static IDC_PUNCTUATION_COMBO: u16 =           2032;
pub static IDS_PUNCT_FULL_WIDTH: u16 =            2033;
pub static IDS_PUNCT_HALF_WIDTH: u16 =            2034;
pub static IDL_TONE_KEYS: u16 =                   2035;
pub static IDC_TONE_KEYS_COMBO: u16 =             2036;
pub static IDS_TONE_KEYS_NUMERIC: u16 =           2037;
pub static IDS_TONE_KEYS_TELEX: u16 =             2038;
pub static IDL_DOTTED_O_KEY: u16 =                2039;
pub static IDC_DOTTED_O_KEY_COMBO: u16 =          2040;
pub static IDS_DOTTED_O_OU: u16 =                 2041;
pub static IDS_DOTTED_O_OO: u16 =                 2042;
pub static IDS_DOTTED_O_Y: u16 =                  2043;
pub static IDL_NASAL_KEY: u16 =                   2044;
pub static IDC_NASAL_KEY_COMBO: u16 =             2045;
pub static IDS_NASAL_NN: u16 =                    2046;
pub static IDS_NASAL_V: u16 =                     2047;
pub static IDC_OPTION_DOTTED_KHIN: u16 =          2048;
pub static IDC_OPTION_AUTOKHIN: u16 =             2049;
pub static IDC_OPTION_EASY_CH: u16 =              2050;
pub static IDC_OPTION_UPPERCASE_NASAL: u16 =      2051;
pub static IDL_HYPHEN_KEY: u16 =                  2052;
pub static IDC_HYPHEN_KEY_COMBO: u16 =            2053;
pub static IDS_HYPHEN_KEY_HYPHEN: u16 =           2054;
pub static IDS_HYPHEN_KEY_V: u16 =                2055;
pub static IDD_DICTIONARYTAB: u16 =               2056;
pub static IDL_RESET_USERDATA: u16 =              2057;
pub static IDC_RESET_USERDATA_BTN: u16 =          2058;
pub static IDL_RESET_USERDATA_BTN_DONE: u16 =     2059;
pub static IDL_EDIT_USERDICT: u16 =               2060;
pub static IDC_EDIT_USEDICT_BTN: u16 =            2061;

#[inline]
pub fn make_int_resource(rid: u16) -> PCWSTR {
    PCWSTR(rid as *mut u16)
}
