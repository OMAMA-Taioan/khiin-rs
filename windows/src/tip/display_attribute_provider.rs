use windows::core::implement;
use windows::core::Result;
use windows::Win32::UI::TextServices::IEnumTfDisplayAttributeInfo;
use windows::Win32::UI::TextServices::ITfDisplayAttributeInfo;
use windows::Win32::UI::TextServices::ITfDisplayAttributeProvider;
use windows::Win32::UI::TextServices::ITfDisplayAttributeProvider_Impl;

#[implement(ITfDisplayAttributeProvider)]
struct DisplayAttributeProvider;

impl ITfDisplayAttributeProvider_Impl for DisplayAttributeProvider {
    fn EnumDisplayAttributeInfo(&self) -> Result<IEnumTfDisplayAttributeInfo> {
        todo!()
    }

    fn GetDisplayAttributeInfo(
        &self,
        guid: *const windows::core::GUID,
    ) -> Result<ITfDisplayAttributeInfo> {
        todo!()
    }
}
