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
pub const IID_DISPLAY_ATTRIBUTE_INPUT: GUID =
    GUID::from_u128(0x829893f8_728d_11ec_8c6e_e0d46491b35a);

// DisplayAttribute: Converted
// 829893f9-728d-11ec-8c6e-e0d46491b35a
pub const IID_DISPLAY_ATTRIBUTE_CONVERTED: GUID =
    GUID::from_u128(0x829893f9_728d_11ec_8c6e_e0d46491b35a);

// DisplayAttribute: Focused
// 829893fb-728d-11ec-8c6e-e0d46491b35a
pub const IID_DISPLAY_ATTRIBUTE_FOCUSED: GUID =
    GUID::from_u128(0x829893fb_728d_11ec_8c6e_e0d46491b35a);
