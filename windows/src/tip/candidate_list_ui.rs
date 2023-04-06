use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use khiin_protos::command::CandidateList;
use khiin_protos::command::EditState::ES_COMPOSING;
use windows::core::implement;
use windows::core::AsImpl;
use windows::core::ComInterface;
use windows::core::Result;
use windows::core::BSTR;
use windows::core::GUID;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::TRUE;
use windows::Win32::UI::TextServices::ITfCandidateListUIElement;
use windows::Win32::UI::TextServices::ITfCandidateListUIElementBehavior;
use windows::Win32::UI::TextServices::ITfCandidateListUIElementBehavior_Impl;
use windows::Win32::UI::TextServices::ITfCandidateListUIElement_Impl;
use windows::Win32::UI::TextServices::ITfContext;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;
use windows::Win32::UI::TextServices::ITfUIElementMgr;
// use windows::Win32::UI::TextServices::ITfIntegratableCandidateListUIElement;
// use windows::Win32::UI::TextServices::ITfIntegratableCandidateListUIElement_Impl;
use windows::Win32::UI::TextServices::ITfDocumentMgr;
use windows::Win32::UI::TextServices::ITfUIElement;
use windows::Win32::UI::TextServices::ITfUIElement_Impl;

use khiin_protos::command::Command;

use crate::geometry::Rect;
use crate::ui::candidates::CandidateWindow;
use crate::ui::candidates::Pager;
use crate::utils::ArcLock;

#[implement(
    ITfUIElement,
    ITfCandidateListUIElement,
    ITfCandidateListUIElementBehavior,
    // ITfIntegratableCandidateListUIElement,
)]
pub struct CandidateListUI {
    tip: ITfTextInputProcessor,
    element_id: ArcLock<u32>,
    popup: Arc<CandidateWindow>,
    pager: Rc<RefCell<Pager>>,
}

impl CandidateListUI {
    pub fn new(tip: ITfTextInputProcessor) -> Result<ITfUIElement> {
        let elem: ITfUIElement = Self {
            tip: tip.clone(),
            element_id: ArcLock::new(0),
            popup: Arc::new(CandidateWindow::new(tip)?),
            pager: Rc::new(RefCell::new(Pager::default())),
        }.into();

        let this = elem.as_impl();
        this.begin_ui_elem(elem.clone())?;
        Ok(elem)
    }

    pub fn notify_command(
        &self,
        context: ITfContext,
        command: Arc<Command>,
        rect: Rect<i32>,
    ) -> Result<()> {
        self.pager.replace(Pager::new(command.clone()));
        let res = &command.response;
        let edit_state = res.edit_state.enum_value_or_default();
        let focused = res.candidate_list.focused;

        if edit_state == ES_COMPOSING {
            // self.pager.set_candidate_list(command)?;
        } else {
            // self.pager.set_focus(focused)?;
        }

        self.update_ui_elem()?;
        Ok(())
    }

    fn ui_elem_mgr(&self) -> Result<ITfUIElementMgr> {
        let service = self.tip.as_impl();
        service.threadmgr().cast()
    }

    fn begin_ui_elem(&self, elem: ITfUIElement) -> Result<()> {
        let ui_elem_mgr = self.ui_elem_mgr()?;
        let mut showable = TRUE;
        let mut elementid = 0u32;

        unsafe {
            ui_elem_mgr.BeginUIElement(
                &elem,
                &mut showable,
                &mut elementid,
            )?;
            self.element_id.set(elementid)
        }
    }

    fn update_ui_elem(&self) -> Result<()> {
        let ui_elem_mgr = self.ui_elem_mgr()?;
        unsafe { ui_elem_mgr.UpdateUIElement(self.element_id.get()?) }
    }

    pub fn end_ui_elem(&self) -> Result<()> {
        let ui_elem_mgr = self.ui_elem_mgr()?;
        unsafe { ui_elem_mgr.EndUIElement(self.element_id.get()?) }
    }
}

impl ITfUIElement_Impl for CandidateListUI {
    fn GetDescription(&self) -> Result<BSTR> {
        todo!()
    }

    fn GetGUID(&self) -> Result<GUID> {
        todo!()
    }

    fn Show(&self, bshow: BOOL) -> Result<()> {
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

    fn GetDocumentMgr(&self) -> Result<ITfDocumentMgr> {
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

// TODO
// impl ITfIntegratableCandidateListUIElement_Impl for CandidateListUI {
//     fn SetIntegrationStyle(&self,guidintegrationstyle: &GUID) -> Result<()> {
//         todo!()
//     }

//     fn GetSelectionStyle(&self) -> Result<TfIntegratableCandidateListSelectionStyle> {
//         todo!()
//     }

//     fn OnKeyDown(&self, wparam: WPARAM, lparam: LPARAM) -> Result<BOOL> {
//         todo!()
//     }

//     fn ShowCandidateNumbers(&self) -> Result<BOOL> {
//         todo!()
//     }

//     fn FinalizeExactCompositionString(&self) -> Result<()> {
//         todo!()
//     }
// }
