use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::ManuallyDrop;
use std::sync::Arc;

use khiin_protos::command::CommandType;
use khiin_protos::command::Preedit;
use khiin_protos::command::SegmentStatus;
use windows::core::Interface;
use windows::core::Result;
use windows::core::VARIANT;
use windows::Win32::Foundation::FALSE;
use windows::Win32::System::Variant::VT_I4;
use windows::Win32::UI::TextServices::ITfComposition;
use windows::Win32::UI::TextServices::ITfCompositionSink;
use windows::Win32::UI::TextServices::ITfContext;
use windows::Win32::UI::TextServices::ITfContextComposition;
use windows::Win32::UI::TextServices::ITfInsertAtSelection;
use windows::Win32::UI::TextServices::ITfRange;
use windows::Win32::UI::TextServices::TfActiveSelEnd;
use windows::Win32::UI::TextServices::GUID_PROP_ATTRIBUTE;
use windows::Win32::UI::TextServices::TF_AE_END;
use windows::Win32::UI::TextServices::TF_AE_NONE;
use windows::Win32::UI::TextServices::TF_ANCHOR_START;
use windows::Win32::UI::TextServices::TF_IAS_QUERYONLY;
use windows::Win32::UI::TextServices::TF_SELECTION;
use windows::Win32::UI::TextServices::TF_SELECTIONSTYLE;
use windows::Win32::UI::TextServices::TF_ST_CORRECTION;

use khiin_protos::command::Command;

use crate::fail;
use crate::tip::text_service::TF_INVALID_GUIDATOM;
use crate::utils::SegmentData;
use crate::utils::ToWidePreedit;

pub struct CompositionMgr {
    composition: RefCell<Option<ITfComposition>>,
    is_manual_mode: RefCell<bool>,
}

impl CompositionMgr {
    pub fn new() -> Result<Self> {
        Ok(Self {
            composition: RefCell::new(None),
            is_manual_mode: RefCell::new(false),
        })
    }

    pub fn composing(&self) -> bool {
        self.composition.borrow().is_some()
    }

    pub fn reset(&self) -> Result<()> {
        self.composition.replace(None);
        Ok(())
    }

    pub fn refresh_input_mode(&self, is_manual: bool) {
        self.is_manual_mode.replace(is_manual);
    }

    pub fn composition(&self) -> Result<ITfComposition> {
        self.composition
            .try_borrow()
            .map_err(|_| fail!())?
            .clone()
            .ok_or(fail!())
    }

    pub fn commit_all(
        &mut self,
        ec: u32,
        context: ITfContext,
        preedit: &Preedit,
    ) -> Result<()> {
        let comp = self.composition()?;
        let display = String::from_utf16_lossy(&preedit.widen().display);
        self.commit_composition(ec, comp, context, preedit, &display)?;
        Ok(())
    }

    pub fn notify_command(
        &mut self,
        ec: u32,
        context: ITfContext,
        sink: ITfCompositionSink,
        command: Arc<Command>,
        attr_atoms: &HashMap<SegmentStatus, u32>,
    ) -> Result<()> {
        if self.composition().is_err() {
            self.new_composition(ec, context.clone(), sink.clone())?;
        }
        // to check composition is still valid
        unsafe {
            if self.composition()?.GetRange().is_err() {
                log::debug!("Composition is invalid, creating new one");
                self.reset()?;
                self.new_composition(ec, context.clone(), sink)?;
            }
        }

        let comp = self.composition()?;

        if command.request.type_.enum_value_or_default()
            == CommandType::CMD_COMMIT
        {
            self.commit_composition(
                ec,
                comp,
                context,
                &command.response.preedit,
                &command.response.committed_text,
            )?;
        } else if command.response.committed {
            self.commit_composition(
                ec,
                comp.clone(),
                context.clone(),
                &command.response.preedit,
                &command.response.committed_text,
            )?;
            if command.response.preedit.caret > 0 {
                self.do_composition(
                    ec,
                    comp,
                    context,
                    &command.response.preedit,
                    attr_atoms,
                )?;
            }
        } else {
            self.do_composition(
                ec,
                comp,
                context,
                &command.response.preedit,
                attr_atoms,
            )?;
        }

        Ok(())
    }

    fn new_composition(
        &mut self,
        ec: u32,
        context: ITfContext,
        sink: ITfCompositionSink,
    ) -> Result<ITfComposition> {
        let insert_sel: ITfInsertAtSelection = context.cast()?;
        let insert_pos = unsafe {
            insert_sel.InsertTextAtSelection(ec, TF_IAS_QUERYONLY, &[])?
        };
        let ctx_comp: ITfContextComposition = context.cast()?;
        let comp =
            unsafe { ctx_comp.StartComposition(ec, &insert_pos, &sink)? };
        self.composition.replace(Some(comp.clone()));
        self.set_selection(ec, context, insert_pos, TF_AE_NONE)?;
        Ok(comp)
    }

    fn get_range_text(range: &ITfRange, ec: u32) -> Result<String> {
        unsafe {
            let mut buf = [0u16; 1024]; // 暫存 1024 個字
            let mut fetched = 0;
            range.GetText(ec, 0, &mut buf, &mut fetched)?;

            Ok(String::from_utf16_lossy(&buf[..fetched as usize]))
        }
    }

    fn do_composition(
        &mut self,
        ec: u32,
        composition: ITfComposition,
        context: ITfContext,
        preedit: &Preedit,
        attr_atoms: &HashMap<SegmentStatus, u32>,
    ) -> Result<()> {
        unsafe {
            let preedit = preedit.widen();
            let range = composition.GetRange()?;
            let is_empty = range.IsEmpty(ec)?;
            if is_empty == FALSE {
                range.SetText(ec, TF_ST_CORRECTION, &[])?;
            }

            let comp_range = composition.GetRange()?;
            let display = preedit.display.clone();
            let caret = display.len() as i32;
            comp_range.SetText(ec, TF_ST_CORRECTION, &display)?;

            let prop = context.GetProperty(&GUID_PROP_ATTRIBUTE)?;
            for segment in preedit.segments {
                let SegmentData {
                    start,
                    stop,
                    status,
                } = segment;
                let seg_range = comp_range.Clone()?;
                seg_range.Collapse(ec, TF_ANCHOR_START)?;
                let mut shifted: i32 = 0;
                seg_range.ShiftEnd(ec, stop, &mut shifted, std::ptr::null())?;
                seg_range.ShiftStart(
                    ec,
                    start,
                    &mut shifted,
                    std::ptr::null(),
                )?;
                let mut variant = VARIANT::default();
                let &(mut var) = variant.as_raw();
                let atom = attr_atoms
                    .get(&status)
                    .unwrap_or(&TF_INVALID_GUIDATOM)
                    .clone();
                var.Anonymous.Anonymous.vt = VT_I4.0;
                var.Anonymous.Anonymous.Anonymous.lVal = atom as i32;
                prop.SetValue(ec, &seg_range, &variant)?;
            }

            // TODO segment attrs
            // let attr_range = comp_range.Clone()?;
            // let mut variant = VARIANT::default();
            // let &(mut var) = variant.as_raw();
            // let atom = attr_atoms
            //     .get(&SegmentStatus::SS_COMPOSING)
            //     .unwrap_or(&TF_INVALID_GUIDATOM)
            //     .clone();
            // var.Anonymous.Anonymous.vt = VT_I4.0;
            // var.Anonymous.Anonymous.Anonymous.lVal = atom as i32;
            // prop.SetValue(ec, &attr_range, &variant)?;

            let curs_range = comp_range.Clone()?;
            curs_range.Collapse(ec, TF_ANCHOR_START)?;
            let mut shifted: i32 = 0;
            curs_range.ShiftEnd(ec, caret, &mut shifted, std::ptr::null())?;
            log::debug!("do_composition ShiftEnd: {}", shifted);
            curs_range.ShiftStart(ec, caret, &mut shifted, std::ptr::null())?;
            log::debug!("do_composition ShiftStart: {}", shifted);
            self.set_selection(ec, context, curs_range, TF_AE_END)?;

            Ok(())
        }
    }

    pub fn commit_text(
        &mut self,
        ec: u32,
        context: ITfContext,
        committed_text: &String,
    ) -> Result<()> {
        log::debug!("Committing text");
        let composition = self.composition()?;
        unsafe {
            let range = composition.GetRange()?;
            let is_empty = range.IsEmpty(ec)?;
            if is_empty == FALSE {
                range.SetText(ec, TF_ST_CORRECTION, &[])?;
            }

            let comp_range = composition.GetRange()?;
            let committed_text_utf16: Vec<u16> =
                committed_text.encode_utf16().collect();
            comp_range.SetText(ec, TF_ST_CORRECTION, &committed_text_utf16)?;

            let end_range = range.Clone()?;
            let mut shifted: i32 = 0;
            let len = committed_text.len() as i32;
            log::debug!("Committing with len: {}", len);
            end_range.ShiftStart(ec, len, &mut shifted, std::ptr::null())?;
            end_range.Collapse(ec, TF_ANCHOR_START)?;
            composition.ShiftStart(ec, &end_range)?;
            self.set_selection(ec, context, end_range, TF_AE_END)?;
            self.cleanup(ec, composition)?;
        }

        Ok(())
    }

    fn commit_composition(
        &mut self,
        ec: u32,
        composition: ITfComposition,
        context: ITfContext,
        preedit: &Preedit,
        committed_text: &String,
    ) -> Result<()> {
        log::debug!("Committing composition");
        if self.is_manual_mode.borrow().clone() {
            unsafe {
                let preedit = preedit.widen();
                let range = composition.GetRange()?;
                let end_range = range.Clone()?;
                let mut shifted: i32 = 0;
                let len = preedit.display.len() as i32;
                log::debug!("Committing with len: {}", len);
                end_range.ShiftStart(
                    ec,
                    len,
                    &mut shifted,
                    std::ptr::null(),
                )?;
                end_range.Collapse(ec, TF_ANCHOR_START)?;
                composition.ShiftStart(ec, &end_range)?;
                self.set_selection(ec, context, end_range, TF_AE_END)?;
                self.cleanup(ec, composition)?;
            }
        } else {
            unsafe {
                let range = composition.GetRange()?;
                let is_empty = range.IsEmpty(ec)?;
                if is_empty == FALSE {
                    range.SetText(ec, TF_ST_CORRECTION, &[])?;
                }

                let comp_range = composition.GetRange()?;
                let committed_text_utf16: Vec<u16> =
                    committed_text.encode_utf16().collect();
                comp_range.SetText(
                    ec,
                    TF_ST_CORRECTION,
                    &committed_text_utf16,
                )?;

                let end_range = range.Clone()?;
                let mut shifted: i32 = 0;
                let len = committed_text.len() as i32;
                log::debug!("Committing with len: {}", len);
                end_range.ShiftStart(
                    ec,
                    len,
                    &mut shifted,
                    std::ptr::null(),
                )?;
                end_range.Collapse(ec, TF_ANCHOR_START)?;
                composition.ShiftStart(ec, &end_range)?;
                self.set_selection(ec, context, end_range, TF_AE_END)?;
                if preedit.widen().display.len() == 0 {
                    self.cleanup(ec, composition)?;
                }
            }
        }

        Ok(())
    }

    pub fn cancel_composition(&self, ec: u32) -> Result<()> {
        match self.composition() {
            Ok(comp) => self.cleanup(ec, comp),
            _ => Ok(()),
        }
    }

    fn cleanup(&self, ec: u32, composition: ITfComposition) -> Result<()> {
        unsafe {
            composition.EndComposition(ec)?;
        }
        self.composition.replace(None);
        Ok(())
    }

    fn set_selection(
        &mut self,
        ec: u32,
        context: ITfContext,
        range: ITfRange,
        ase: TfActiveSelEnd,
    ) -> Result<()> {
        let sel = [TF_SELECTION {
            range: ManuallyDrop::new(Some(range)),
            style: TF_SELECTIONSTYLE {
                ase,
                fInterimChar: FALSE,
            },
        }];
        unsafe { context.SetSelection(ec, &sel) }
    }
}
