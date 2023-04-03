use std::mem::ManuallyDrop;
use std::sync::Arc;

use khiin_protos::command::Preedit;
use windows::Win32::UI::TextServices::TF_AE_END;
use windows::Win32::UI::TextServices::TF_ANCHOR_START;
use windows::Win32::UI::TextServices::TF_ST_CORRECTION;
use windows::core::ComInterface;
use windows::core::Result;
use windows::Win32::Foundation::FALSE;
use windows::Win32::UI::TextServices::ITfComposition;
use windows::Win32::UI::TextServices::ITfCompositionSink;
use windows::Win32::UI::TextServices::ITfContext;
use windows::Win32::UI::TextServices::ITfContextComposition;
use windows::Win32::UI::TextServices::ITfInsertAtSelection;
use windows::Win32::UI::TextServices::ITfRange;
use windows::Win32::UI::TextServices::TfActiveSelEnd;
use windows::Win32::UI::TextServices::TF_AE_NONE;
use windows::Win32::UI::TextServices::TF_IAS_QUERYONLY;
use windows::Win32::UI::TextServices::TF_SELECTION;
use windows::Win32::UI::TextServices::TF_SELECTIONSTYLE;

use khiin_protos::command::Command;

use crate::utils::wpreedit::ToWidePreedit;

pub struct CompositionMgr {
    composition: Option<ITfComposition>,
}

impl CompositionMgr {
    pub fn new() -> Result<Self> {
        Ok(Self { composition: None })
    }

    pub fn notify_command(
        &mut self,
        ec: u32,
        context: ITfContext,
        sink: ITfCompositionSink,
        command: Arc<Command>,
    ) -> Result<()> {
        if self.composition.is_none() {
            self.new_composition(ec, context.clone(), sink)?;
        }

        if let Some(comp) = self.composition.clone() {
            self.do_composition(ec, comp, context, &command.response.preedit)?;
        }

        Ok(())
    }

    fn new_composition(
        &mut self,
        ec: u32,
        context: ITfContext,
        sink: ITfCompositionSink,
    ) -> Result<()> {
        let insert_sel: ITfInsertAtSelection = context.cast()?;
        let insert_pos = unsafe {
            insert_sel.InsertTextAtSelection(ec, TF_IAS_QUERYONLY, &[])?
        };
        let ctx_comp: ITfContextComposition = context.cast()?;
        let comp =
            unsafe { ctx_comp.StartComposition(ec, &insert_pos, &sink)? };
        self.composition = Some(comp);
        self.set_selection(ec, context, insert_pos, TF_AE_NONE)?;
        Ok(())
    }

    fn do_composition(
        &self,
        ec: u32,
        composition: ITfComposition,
        context: ITfContext,
        preedit: &Preedit,
    ) -> Result<()> {
        unsafe {
            let preedit = preedit.widen();
            let range = composition.GetRange()?;
            let is_empty = range.IsEmpty(ec)?;
            if is_empty == FALSE {
                range.SetText(ec, TF_ST_CORRECTION, &[])?;
            }

            let range = composition.GetRange()?;
            range.SetText(ec, TF_ST_CORRECTION, &preedit.display);

            // TODO segment attrs

            let curs_range = range.Clone()?;
            curs_range.Collapse(ec, TF_ANCHOR_START);
            let mut shifted: i32 = 0;
            curs_range.ShiftEnd(ec, preedit.caret, &mut shifted, std::ptr::null())?;
            curs_range.ShiftStart(ec, preedit.caret, &mut shifted, std::ptr::null())?;
            self.set_selection(ec, context, curs_range, TF_AE_END)?;

            Ok(())
        }
    }

    fn set_selection(
        &self,
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
