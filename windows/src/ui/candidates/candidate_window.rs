use std::cell::RefCell;
use std::rc::Rc;

use windows::core::AsImpl;
use windows::core::Result;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;

use crate::ui::window::WindowData;

pub struct CandidateWindow {
    tip: ITfTextInputProcessor,
    window: Rc<RefCell<WindowData>>,
}

impl CandidateWindow {
    pub(crate) fn new(tip: ITfTextInputProcessor) -> Result<Self> {
        let service = tip.as_impl();
        let factory = service.render_factory.clone();
        let window = WindowData::new(factory)?;

        Ok(Self {
            tip,
            window: Rc::new(RefCell::new(window)),
        })
    }
}
