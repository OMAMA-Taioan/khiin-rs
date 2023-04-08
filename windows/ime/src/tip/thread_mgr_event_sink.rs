use windows::core::implement;
use windows::core::Result;
use windows::Win32::UI::TextServices::ITfContext;
use windows::Win32::UI::TextServices::ITfDocumentMgr;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;
use windows::Win32::UI::TextServices::ITfThreadMgrEventSink;
use windows::Win32::UI::TextServices::ITfThreadMgrEventSink_Impl;


#[implement(ITfThreadMgrEventSink)]
pub struct ThreadMgrEventSink {
    _service: ITfTextInputProcessor,
}

impl ThreadMgrEventSink {
    pub fn new(service: ITfTextInputProcessor) -> Self {
        Self {
            _service: service,
        }
    }
}

impl ITfThreadMgrEventSink_Impl for ThreadMgrEventSink {
    fn OnInitDocumentMgr(&self, _pdim: Option<&ITfDocumentMgr>) -> Result<()> {
        Ok(())
    }

    fn OnUninitDocumentMgr(&self, _pdim: Option<&ITfDocumentMgr>) -> Result<()> {
        Ok(())
    }

    fn OnSetFocus(
        &self,
        _pdimfocus: Option<&ITfDocumentMgr>,
        _pdimprevfocus: Option<&ITfDocumentMgr>,
    ) -> Result<()> {
        Ok(())
    }

    fn OnPushContext(&self, _pic: Option<&ITfContext>) -> Result<()> {
        Ok(())
    }

    fn OnPopContext(&self, _pic: Option<&ITfContext>) -> Result<()> {
        Ok(())
    }
}
