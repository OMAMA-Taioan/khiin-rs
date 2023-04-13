// Until someone finds a better way, this file
// needs to be kept in sync with res/resource.h

use windows::core::PCWSTR;

pub static IDM_MANIFEST: u16 = 1;
pub static IDS_TEXT_SERVICE_DISPLAY_NAME: u16 = 101;
pub static IDI_MAINICON: u16 = 102;
pub static IDI_MODE_ALPHA: u16 = 103;
pub static IDI_MODE_ALPHA_W: u16 = 104;
pub static IDI_MODE_CONTINUOUS: u16 = 105;
pub static IDI_MODE_CONTINUOUS_W: u16 = 106;
pub static IDI_MODE_PRO: u16 = 107;
pub static IDI_MODE_PRO_W: u16 = 108;
pub static IDI_MODE_BASIC: u16 = 109;
pub static IDI_MODE_BASIC_W: u16 = 110;
pub static IDI_SETTINGS: u16 = 111;
pub static IDI_SETTINGS_W: u16 = 112;
pub static IDR_POPUP_MENU: u16 = 113;
pub static IDS_CONTINUOUS_MODE: u16 = 1000;
pub static IDS_BASIC_MODE: u16 = 1001;
pub static IDS_MANUAL_MODE: u16 = 1002;
pub static IDS_DIRECT_MODE: u16 = 1003;
pub static IDS_OPEN_SETTINGS: u16 = 1004;

#[inline]
pub fn make_int_resource(rid: u16) -> PCWSTR {
    PCWSTR(rid as *mut u16)
}
