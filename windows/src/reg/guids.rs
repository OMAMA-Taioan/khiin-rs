#![allow(non_upper_case_globals)]

use windows::core::GUID;

////////////////
// Windows GUIDs
////////////////

// IClassFactory
// {00000001-0000-0000-c000-000000000046}
pub const IID_IClassFactory: GUID =
    GUID::from_u128(0x00000001_0000_0000_c000_000000000046);

// ITfTextInputProcessor
// {aa80e7f7-2021-11d2-93e0-0060b067b86e}
pub const IID_ITfTextInputProcessor: GUID =
    GUID::from_u128(0xaa80e7f7_2021_11d2_93e0_0060b067b86e);

// ITfInputProcessorProfiles
// {1f02b6c5-7842-4ee6-8a0b-9a24183a95ca}
// pub const IID_ITfInputProcessorProfiles: GUID =
//     GUID::from_u128(0x1f02b6c5_7842_4ee6_8a0b_9a24183a95ca);

//////////////
// Khiin GUIDs
//////////////

// KhiinClassFactory
// {829893f6-728d-11ec-8c6e-e0d46491b35a}
pub const IID_KhiinTextService: GUID =
    GUID::from_u128(0x829893f6_728d_11ec_8c6e_e0d46491b35a);

// LanguageProfile
// {829893f7-728d-11ec-8c6e-e0d46491b35a}
pub const LanguageProfile: GUID =
    GUID::from_u128(0x829893f7_728d_11ec_8c6e_e0d46491b35a);
