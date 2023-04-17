use std::cell::RefCell;
use std::rc::Rc;
use std::slice::from_raw_parts_mut;
use std::sync::Arc;

use khiin_protos::command::EditState::ES_COMPOSING;
use windows::core::implement;
use windows::core::AsImpl;
use windows::core::ComInterface;
use windows::core::Error;
use windows::core::Result;
use windows::core::BSTR;
use windows::core::GUID;
use windows::Win32::Foundation::SysAllocString;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::E_INVALIDARG;
use windows::Win32::Foundation::E_NOTIMPL;
use windows::Win32::Foundation::TRUE;
use windows::Win32::UI::Input::KeyboardAndMouse::GetFocus;
use windows::Win32::UI::TextServices::ITfCandidateListUIElement;
use windows::Win32::UI::TextServices::ITfCandidateListUIElementBehavior;
use windows::Win32::UI::TextServices::ITfCandidateListUIElementBehavior_Impl;
use windows::Win32::UI::TextServices::ITfCandidateListUIElement_Impl;
use windows::Win32::UI::TextServices::ITfContext;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;
use windows::Win32::UI::TextServices::ITfUIElementMgr;
use windows::Win32::UI::TextServices::TF_CLUIE_COUNT;
use windows::Win32::UI::TextServices::TF_CLUIE_CURRENTPAGE;
use windows::Win32::UI::TextServices::TF_CLUIE_PAGEINDEX;
use windows::Win32::UI::TextServices::TF_CLUIE_SELECTION;
use windows::Win32::UI::TextServices::TF_CLUIE_STRING;
// use windows::Win32::UI::TextServices::ITfIntegratableCandidateListUIElement;
// use windows::Win32::UI::TextServices::ITfIntegratableCandidateListUIElement_Impl;
use windows::Win32::UI::TextServices::ITfDocumentMgr;
use windows::Win32::UI::TextServices::ITfUIElement;
use windows::Win32::UI::TextServices::ITfUIElement_Impl;

use khiin_protos::command::Command;

use crate::fail;
use crate::geometry::Rect;
use crate::reg::guids::GUID_CANDIDATE_WINDOW;
use crate::ui::candidates::CandidateWindow;
use crate::ui::candidates::Pager;
use crate::ui::window::WindowHandler;
use crate::ui::wndproc::Wndproc;
use crate::utils::ArcLock;
use crate::utils::ToPcwstr;
use crate::winerr;

static CANDIDATE_WINDOW_UI_DESCRIPTION: &str = "Khiin Candidate Window UI";

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
    context: RefCell<Option<ITfContext>>,
    command: RefCell<Arc<Command>>,
    prev_command: RefCell<Arc<Command>>,
    popup_registered: RefCell<bool>,
}

impl CandidateListUI {
    pub fn new(tip: ITfTextInputProcessor) -> Result<ITfUIElement> {
        let cmd = Arc::new(Command::default());
        let elem: ITfUIElement = Self {
            tip: tip.clone(),
            element_id: ArcLock::new(0),
            popup: Arc::new(CandidateWindow::new(tip)?),
            pager: Rc::new(RefCell::new(Pager::default())),
            context: RefCell::new(None),
            command: RefCell::new(cmd.clone()),
            prev_command: RefCell::new(cmd),
            popup_registered: RefCell::new(false),
        }
        .into();

        let this = elem.as_impl();
        this.begin_ui_elem(elem.clone())?;
        Ok(elem)
    }

    pub fn shutdown(&self) -> Result<()> {
        self.context.replace(None);
        self.popup.hide()?;
        self.popup.destroy()?;
        self.popup_registered.replace(false);
        self.end_ui_elem()
    }

    fn create_candidate_window(&self, context: ITfContext) -> Result<()> {
        if !*self.popup_registered.borrow() {
            unsafe {
                let context_view = context.GetActiveView()?;
                let parent = context_view.GetWnd().unwrap_or(GetFocus());
                CandidateWindow::create(self.popup.clone(), parent)?;
                self.popup_registered.replace(true);
            }
        }

        Ok(())
    }

    pub fn notify_command(
        &self,
        context: ITfContext,
        command: Arc<Command>,
        rect: Rect<i32>,
    ) -> Result<()> {
        self.context.replace(Some(context.clone()));
        self.create_candidate_window(context)?;

        let res = &command.response;
        let edit_state = res.edit_state.enum_value_or_default();
        let focused_id = res.candidate_list.focused;

        if edit_state == ES_COMPOSING {
            self.pager.replace(Pager::new(command.clone()));
        } else {
            self.pager
                .try_borrow()
                .map_err(|_| fail!())?
                .set_focus(focused_id)?;
        }

        self.popup.show(
            self.pager.try_borrow().map_err(|_| fail!())?.get_page(),
            rect,
        )?;

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
            ui_elem_mgr.BeginUIElement(&elem, &mut showable, &mut elementid)?;
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

    fn is_candidate_list_modified(&self) -> bool {
        let prev_command = self.prev_command.borrow().clone();
        let command = self.command.borrow().clone();
        let lhs = &prev_command.response.candidate_list.candidates;
        let rhs = &command.response.candidate_list.candidates;

        if lhs.len() != rhs.len() {
            return true;
        }

        for a in lhs.iter() {
            for b in rhs.iter() {
                if a.value != b.value {
                    return true;
                }
            }
        }

        self.prev_command.replace(command);

        false
    }
}

impl ITfUIElement_Impl for CandidateListUI {
    fn GetDescription(&self) -> Result<BSTR> {
        let pcwstr = CANDIDATE_WINDOW_UI_DESCRIPTION.to_pcwstr();
        let bstr = unsafe { SysAllocString(*pcwstr) };
        Ok(bstr)
    }

    fn GetGUID(&self) -> Result<GUID> {
        Ok(GUID_CANDIDATE_WINDOW)
    }

    fn Show(&self, bshow: BOOL) -> Result<()> {
        match bshow.0 {
            0 => self.popup.hide(),
            _ => self.popup.show(
                self.pager.try_borrow().map_err(|_| fail!())?.get_page(),
                Rect::<i32>::default(),
            ),
        }
    }

    fn IsShown(&self) -> Result<BOOL> {
        Ok(BOOL::from(self.popup.is_showing()?))
    }
}

impl ITfCandidateListUIElement_Impl for CandidateListUI {
    fn GetUpdatedFlags(&self) -> Result<u32> {
        let mut flags = 0;

        if self.is_candidate_list_modified() {
            flags |= TF_CLUIE_STRING | TF_CLUIE_COUNT;
        }

        flags |= TF_CLUIE_CURRENTPAGE | TF_CLUIE_PAGEINDEX | TF_CLUIE_SELECTION;

        Ok(flags)
    }

    fn GetDocumentMgr(&self) -> Result<ITfDocumentMgr> {
        unsafe {
            self.context
                .borrow()
                .clone()
                .ok_or(fail!())?
                .GetDocumentMgr()
        }
    }

    fn GetCount(&self) -> Result<u32> {
        Ok(self
            .pager
            .try_borrow()
            .map_err(|_| fail!())?
            .candidate_count() as u32)
    }

    fn GetSelection(&self) -> Result<u32> {
        Ok(self
            .command
            .try_borrow()
            .map_err(|_| fail!())?
            .response
            .candidate_list
            .focused as u32)
    }

    fn GetString(&self, uindex: u32) -> Result<BSTR> {
        let candidate = self
            .command
            .try_borrow()
            .map_err(|_| fail!())?
            .response
            .candidate_list
            .candidates
            .get(uindex as usize)
            .ok_or(fail!())?
            .value
            .as_str()
            .to_pcwstr();

        let bstr = unsafe { SysAllocString(*candidate) };
        Ok(bstr)
    }

    fn GetPageIndex(
        &self,
        index: *mut u32,
        index_buf_size: u32,
        page_count: *mut u32,
    ) -> Result<()> {
        let page_count = unsafe { &mut *page_count };

        *page_count =
            self.pager.try_borrow().map_err(|_| fail!())?.page_count() as u32;

        if index_buf_size < *page_count {
            return Err(Error::from(E_INVALIDARG));
        }

        let index: &mut [u32] =
            unsafe { from_raw_parts_mut(index, index_buf_size as usize) };
        let max_page_size = self
            .pager
            .try_borrow()
            .map_err(|_| fail!())?
            .max_page_size();

        for i in 0..*page_count {
            index[i as usize] = i * max_page_size as u32;
        }

        Ok(())
    }

    fn SetPageIndex(&self, pindex: *const u32, upagecnt: u32) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }

    fn GetCurrentPage(&self) -> Result<u32> {
        Ok(self.pager.try_borrow().map_err(|_| fail!())?.current_page() as u32)
    }
}

impl ITfCandidateListUIElementBehavior_Impl for CandidateListUI {
    fn SetSelection(&self, nindex: u32) -> Result<()> {
        if nindex
            >= self
                .pager
                .try_borrow()
                .map_err(|_| fail!())?
                .candidate_count() as u32
        {
            winerr!(E_INVALIDARG)
        } else {
            self.tip.as_impl().on_candidate_selected(nindex)
        }
    }

    fn Finalize(&self) -> Result<()> {
        self.tip.as_impl().commit_composition()
    }

    fn Abort(&self) -> Result<()> {
        self.tip.as_impl().reset()
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
