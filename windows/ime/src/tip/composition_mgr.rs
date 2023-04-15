use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::ManuallyDrop;
use std::sync::Arc;

use khiin_protos::command::CommandType;
use khiin_protos::command::Preedit;
use khiin_protos::command::SegmentStatus;
use windows::core::ComInterface;
use windows::core::Error;
use windows::core::Result;
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::Foundation::FALSE;
use windows::Win32::System::Com::VARIANT;
use windows::Win32::System::Com::VT_I4;
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

use crate::utils::SegmentData;
use crate::utils::ToWidePreedit;

use super::text_service::TF_INVALID_GUIDATOM;

pub struct CompositionMgr {
    composition: RefCell<Option<ITfComposition>>,
}

impl CompositionMgr {
    pub fn new() -> Result<Self> {
        Ok(Self {
            composition: RefCell::new(None),
        })
    }

    pub fn reset(&self) -> Result<()> {
        self.composition.replace(None);
        Ok(())
    }

    pub fn composition(&self) -> Result<ITfComposition> {
        self.composition
            .try_borrow()
            .map_err(|_| Error::from(E_FAIL))?
            .clone()
            .ok_or(Error::from(E_FAIL))
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
            self.new_composition(ec, context.clone(), sink)?;
        }

        let comp = self.composition()?;

        if command.response.committed
            || command.request.type_.enum_value_or_default()
                == CommandType::CMD_COMMIT
        {
            self.commit_composition(
                ec,
                comp,
                context,
                &command.response.preedit,
            )?;
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
            comp_range.SetText(ec, TF_ST_CORRECTION, &display)?;

            let prop =
                context.GetProperty(&GUID_PROP_ATTRIBUTE)?;
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
                let atom = attr_atoms
                    .get(&status)
                    .unwrap_or(&TF_INVALID_GUIDATOM)
                    .clone();
                (*variant.Anonymous.Anonymous).vt = VT_I4;
                (*variant.Anonymous.Anonymous).Anonymous.lVal = atom as i32;
                prop.SetValue(ec, &seg_range, &variant)?;
            }

            // TODO segment attrs

            let curs_range = comp_range.Clone()?;
            curs_range.Collapse(ec, TF_ANCHOR_START)?;
            let mut shifted: i32 = 0;
            curs_range.ShiftEnd(
                ec,
                preedit.caret,
                &mut shifted,
                std::ptr::null(),
            )?;
            curs_range.ShiftStart(
                ec,
                preedit.caret,
                &mut shifted,
                std::ptr::null(),
            )?;
            self.set_selection(ec, context, curs_range, TF_AE_END)?;

            Ok(())
        }
    }

    fn commit_composition(
        &mut self,
        ec: u32,
        composition: ITfComposition,
        context: ITfContext,
        preedit: &Preedit,
    ) -> Result<()> {
        unsafe {
            let preedit = preedit.widen();
            let range = composition.GetRange()?;
            let end_range = range.Clone()?;
            let mut shifted: i32 = 0;
            end_range.ShiftStart(
                ec,
                preedit.display.len() as i32,
                &mut shifted,
                std::ptr::null(),
            )?;
            end_range.Collapse(ec, TF_ANCHOR_START)?;
            composition.ShiftStart(ec, &end_range)?;
            self.set_selection(ec, context, end_range, TF_AE_END)?;
            self.cleanup(ec, composition)?;
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
