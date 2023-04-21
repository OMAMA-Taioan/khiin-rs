use std::cell::RefCell;

use windows::Win32::Foundation::HWND;

use crate::app::pages::PageHandler;
use crate::app::resource::*;

#[derive(Default)]
pub struct InputPage {
    handle: RefCell<HWND>,
}

impl PageHandler for InputPage {
    fn handle(&self) -> HWND {
        *self.handle.borrow()
    }

    fn set_handle(&self, handle: HWND) {
        self.handle.replace(handle);
    }

    fn initialize(&self) -> isize {
        self.set_labels(vec![
            IDL_INPUTMODE,
            IDC_INPUTMODE_BASIC,
            IDC_INPUTMODE_CONTINUOUS,
            IDC_INPUTMODE_PRO,
            IDL_INPUTMODE_HOTKEY,
            IDL_ON_OFF_HOTKEY,
            IDL_DEFAULT_PUNCTUATION,
            IDL_TONE_KEYS,
            IDL_DOTTED_O_KEY,
            IDL_NASAL_KEY,
            IDL_HYPHEN_KEY,
            IDC_OPTION_AUTOKHIN,
            IDC_OPTION_DOTTED_KHIN,
            IDC_OPTION_EASY_CH,
            IDC_OPTION_UPPERCASE_NASAL,
        ]);

        self.init_combobox(
            IDC_INPUTMODE_KEY_COMBO,
            vec![
                IDS_INPUTMODE_KEY_CTRL_PERIOD,
                IDS_INPUTMODE_KEY_CTRL_BACKTICK,
            ],
            0,
        );

        self.init_combobox(
            IDC_ON_OFF_HOTKEY_COMBO,
            vec![IDS_ON_OFF_HOTKEY_ALTBACKTICK, IDS_ON_OFF_HOTKEY_SHIFT],
            0,
        );

        self.init_combobox(
            IDC_PUNCTUATION_COMBO,
            vec![IDS_PUNCT_FULL_WIDTH, IDS_PUNCT_HALF_WIDTH],
            0,
        );

        self.init_combobox(
            IDC_TONE_KEYS_COMBO,
            vec![IDS_TONE_KEYS_NUMERIC, IDS_TONE_KEYS_TELEX],
            0,
        );

        self.init_combobox(
            IDC_NASAL_KEY_COMBO,
            vec![IDS_NASAL_NN, IDS_NASAL_V],
            0,
        );

        self.init_combobox(
            IDC_DOTTED_O_KEY_COMBO,
            vec![IDS_DOTTED_O_OU, IDS_DOTTED_O_OO],
            0,
        );

        self.init_combobox(
            IDC_HYPHEN_KEY_COMBO,
            vec![IDS_HYPHEN_KEY_HYPHEN, IDS_HYPHEN_KEY_V],
            0,
        );

        0
    }
}
