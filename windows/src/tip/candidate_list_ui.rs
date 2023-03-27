use windows::Win32::Foundation::BOOL;
use windows::Win32::UI::TextServices::ITfDocumentMgr;
use windows::core::BSTR;
use windows::core::GUID;
use windows::core::implement;
use windows::core::Result;
use windows::Win32::UI::TextServices::ITfCandidateListUIElement;
use windows::Win32::UI::TextServices::ITfCandidateListUIElementBehavior;
use windows::Win32::UI::TextServices::ITfCandidateListUIElementBehavior_Impl;
use windows::Win32::UI::TextServices::ITfCandidateListUIElement_Impl;
use windows::Win32::UI::TextServices::ITfUIElement;
use windows::Win32::UI::TextServices::ITfUIElement_Impl;

#[implement(
    ITfUIElement,
    ITfCandidateListUIElement,
    ITfCandidateListUIElementBehavior
)]
struct CandidateListUI;

impl ITfUIElement_Impl for CandidateListUI {
    fn GetDescription(&self) -> Result<BSTR> {
        todo!()
    }

    fn GetGUID(&self) -> Result<GUID> {
        todo!()
    }

    fn Show(
        &self,
        bshow: BOOL,
    ) -> Result<()> {
        todo!()
    }

    fn IsShown(&self) -> Result<BOOL> {
        todo!()
    }
}

impl ITfCandidateListUIElement_Impl for CandidateListUI {
    fn GetUpdatedFlags(&self) -> Result<u32> {
        todo!()
    }

    fn GetDocumentMgr(
        &self,
    ) -> Result<ITfDocumentMgr> {
        todo!()
    }

    fn GetCount(&self) -> Result<u32> {
        todo!()
    }

    fn GetSelection(&self) -> Result<u32> {
        todo!()
    }

    fn GetString(&self, uindex: u32) -> Result<BSTR> {
        todo!()
    }

    fn GetPageIndex(
        &self,
        pindex: *mut u32,
        usize: u32,
        pupagecnt: *mut u32,
    ) -> Result<()> {
        todo!()
    }

    fn SetPageIndex(&self, pindex: *const u32, upagecnt: u32) -> Result<()> {
        todo!()
    }

    fn GetCurrentPage(&self) -> Result<u32> {
        todo!()
    }
}

impl ITfCandidateListUIElementBehavior_Impl for CandidateListUI {
    fn SetSelection(&self, nindex: u32) -> Result<()> {
        todo!()
    }

    fn Finalize(&self) -> Result<()> {
        todo!()
    }

    fn Abort(&self) -> Result<()> {
        todo!()
    }
}
