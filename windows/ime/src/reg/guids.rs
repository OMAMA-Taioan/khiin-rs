#![allow(non_upper_case_globals)]

use windows::core::GUID;

// KhiinClassFactory
// {829893f6-728d-11ec-8c6e-e0d46491b35a}
pub const IID_KhiinTextService: GUID =
    GUID::from_u128(0x829893f6_728d_11ec_8c6e_e0d46491b35a);

// LanguageProfile
// {829893f7-728d-11ec-8c6e-e0d46491b35a}
pub const LanguageProfile: GUID =
    GUID::from_u128(0x829893f7_728d_11ec_8c6e_e0d46491b35a);

// DisplayAttribute: Input
// 829893f8-728d-11ec-8c6e-e0d46491b35a
pub const GUID_DISPLAY_ATTRIBUTE_INPUT: GUID =
    GUID::from_u128(0x829893f8_728d_11ec_8c6e_e0d46491b35a);

// DisplayAttribute: Converted
// 829893f9-728d-11ec-8c6e-e0d46491b35a
pub const GUID_DISPLAY_ATTRIBUTE_CONVERTED: GUID =
    GUID::from_u128(0x829893f9_728d_11ec_8c6e_e0d46491b35a);

// DisplayAttribute: Focused
// 829893fb-728d-11ec-8c6e-e0d46491b35a
pub const GUID_DISPLAY_ATTRIBUTE_FOCUSED: GUID =
    GUID::from_u128(0x829893fb_728d_11ec_8c6e_e0d46491b35a);

// 829893fa-728d-11ec-8c6e-e0d46491b35a
pub const GUID_CANDIDATE_WINDOW: GUID =
    GUID::from_u128(0x829893fa_728d_11ec_8c6e_e0d46491b35a);

// 829893fc-728d-11ec-8c6e-e0d46491b35a
pub const GUID_CONFIG_CHANGED_COMPARTMENT: GUID =
    GUID::from_u128(0x829893fc_728d_11ec_8c6e_e0d46491b35a);

// 829893fd-728d-11ec-8c6e-e0d46491b35a
pub const GUID_PRESERVED_KEY_ON_OFF: GUID =
    GUID::from_u128(0x829893fd_728d_11ec_8c6e_e0d46491b35a);

// 829893fe-728d-11ec-8c6e-e0d46491b35a
pub const GUID_PRESERVED_KEY_SWITCH_MODE: GUID =
    GUID::from_u128(0x829893fe_728d_11ec_8c6e_e0d46491b35a);

// 829893ff-728d-11ec-8c6e-e0d46491b35a
pub const GUID_PRESERVED_KEY_FULL_WIDTH_SPACE: GUID =
    GUID::from_u128(0x829893ff_728d_11ec_8c6e_e0d46491b35a);

pub const GUID_RESET_USERDATA_COMPARTMENT: GUID =
    GUID::from_u128(0x82989400_728d_11ec_8c6e_e0d46491b35a);
