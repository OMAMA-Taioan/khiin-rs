use windows::core::implement;
use windows::core::Result;
use windows::Win32::UI::TextServices::IEnumTfDisplayAttributeInfo;
use windows::Win32::UI::TextServices::IEnumTfDisplayAttributeInfo_Impl;
use windows::Win32::UI::TextServices::ITfDisplayAttributeInfo;

#[implement(IEnumTfDisplayAttributeInfo)]
struct DisplayAttributeInfoEnum;

impl IEnumTfDisplayAttributeInfo_Impl for DisplayAttributeInfoEnum {
    fn Clone(&self) -> Result<IEnumTfDisplayAttributeInfo> {
        todo!()
    }

    fn Next(
        &self,
        ulcount: u32,
        rginfo: *mut Option<ITfDisplayAttributeInfo>,
        pcfetched: *mut u32,
    ) -> Result<()> {
        todo!()
    }

    fn Reset(&self) -> Result<()> {
        todo!()
    }

    fn Skip(&self, ulcount: u32) -> Result<()> {
        todo!()
    }
}
