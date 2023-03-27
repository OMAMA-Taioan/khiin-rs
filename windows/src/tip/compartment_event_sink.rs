use windows::core::implement;
use windows::core::Result;
use windows::core::GUID;
use windows::Win32::UI::TextServices::ITfCompartmentEventSink;
use windows::Win32::UI::TextServices::ITfCompartmentEventSink_Impl;

#[implement(ITfCompartmentEventSink)]
struct CompartmentEventSink;

impl ITfCompartmentEventSink_Impl for CompartmentEventSink {
    fn OnChange(&self, rguid: *const GUID) -> Result<()> {
        todo!()
    }
}
