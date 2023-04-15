use windows::core::AsImpl;
use windows::core::ComInterface;
use windows::core::Result;
use windows::core::GUID;
use windows::Win32::UI::Input::KeyboardAndMouse::MOD_ALT;
use windows::Win32::UI::Input::KeyboardAndMouse::MOD_CONTROL;
use windows::Win32::UI::Input::KeyboardAndMouse::MOD_SHIFT;
use windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_OEM_3;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_SPACE;
use windows::Win32::UI::TextServices::ITfKeystrokeMgr;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;
use windows::Win32::UI::TextServices::TF_PRESERVEDKEY;

use crate::reg::guids::GUID_PRESERVED_KEY_FULL_WIDTH_SPACE;
use crate::reg::guids::GUID_PRESERVED_KEY_ON_OFF;
use crate::reg::guids::GUID_PRESERVED_KEY_SWITCH_MODE;
use crate::utils::WinString;

static VK_BACKTICK: VIRTUAL_KEY = VK_OEM_3;

pub struct PreservedKey {
    guid: GUID,
    preserved_key: TF_PRESERVEDKEY,
    desc: &'static str,
}

static PREKEY_ON_OFF: PreservedKey = PreservedKey {
    guid: GUID_PRESERVED_KEY_ON_OFF,
    preserved_key: TF_PRESERVEDKEY {
        uVKey: VK_BACKTICK.0 as u32,
        uModifiers: MOD_ALT.0 as u32,
    },
    desc: "Direct input",
};

static PREKEY_SWITCH_MODE: PreservedKey = PreservedKey {
    guid: GUID_PRESERVED_KEY_SWITCH_MODE,
    preserved_key: TF_PRESERVEDKEY {
        uVKey: VK_BACKTICK.0 as u32,
        uModifiers: MOD_CONTROL.0 as u32,
    },
    desc: "Switch mode",
};

static PREKEY_FULL_WIDTH_SPACE: PreservedKey = PreservedKey {
    guid: GUID_PRESERVED_KEY_FULL_WIDTH_SPACE,
    preserved_key: TF_PRESERVEDKEY {
        uVKey: VK_SPACE.0 as u32,
        uModifiers: MOD_SHIFT.0 as u32,
    },
    desc: "Direct input",
};

pub struct PreservedKeyMgr {
    tip: ITfTextInputProcessor,
}

impl PreservedKeyMgr {
    pub fn new(tip: ITfTextInputProcessor) -> Self {
        Self { tip }
    }

    fn keystroke_mgr(&self) -> Result<ITfKeystrokeMgr> {
        let service = self.tip.as_impl();
        service.threadmgr().cast()
    }

    fn preserve_key(&self, pk: PreservedKey) -> Result<()> {
        let service = self.tip.as_impl();
        let desc: &str = &pk.desc;
        let pchdesc = desc.to_utf16_nul();
        unsafe {
            self.keystroke_mgr()?.PreserveKey(
                service.clientid()?,
                &pk.guid,
                &pk.preserved_key,
                &pchdesc,
            )
        }
    }
}
