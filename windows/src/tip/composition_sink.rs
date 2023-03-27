use core::option::Option;

use windows::core::implement;
use windows::core::Result;
use windows::Win32::UI::TextServices::ITfComposition;
use windows::Win32::UI::TextServices::ITfCompositionSink;
use windows::Win32::UI::TextServices::ITfCompositionSink_Impl;

#[implement(ITfCompositionSink)]
struct CompositionSink;

impl ITfCompositionSink_Impl for CompositionSink {
    fn OnCompositionTerminated(
        &self,
        ecwrite: u32,
        pcomposition: Option<&ITfComposition>,
    ) -> Result<()> {
        todo!()
    }
}
