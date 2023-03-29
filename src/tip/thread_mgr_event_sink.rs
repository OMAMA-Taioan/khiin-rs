use windows::core::implement;
use windows::core::Result;
use windows::Win32::UI::TextServices::ITfContext;
use windows::Win32::UI::TextServices::ITfDocumentMgr;
use windows::Win32::UI::TextServices::ITfThreadMgrEventSink;
use windows::Win32::UI::TextServices::ITfThreadMgrEventSink_Impl;

#[implement(ITfThreadMgrEventSink)]
struct ThreadMgrEventSink;

impl ITfThreadMgrEventSink_Impl for ThreadMgrEventSink {
    fn OnInitDocumentMgr(&self, pdim: Option<&ITfDocumentMgr>) -> Result<()> {
        todo!()
    }

    fn OnUninitDocumentMgr(&self, pdim: Option<&ITfDocumentMgr>) -> Result<()> {
        todo!()
    }

    fn OnSetFocus(
        &self,
        pdimfocus: Option<&ITfDocumentMgr>,
        pdimprevfocus: Option<&ITfDocumentMgr>,
    ) -> Result<()> {
        todo!()
    }

    fn OnPushContext(&self, pic: Option<&ITfContext>) -> Result<()> {
        todo!()
    }

    fn OnPopContext(&self, pic: Option<&ITfContext>) -> Result<()> {
        todo!()
    }
}
