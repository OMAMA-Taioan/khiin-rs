use khiin_protos::command::{SegmentStatus, Preedit};

use crate::utils::win::WinString;

pub struct SegmentData {
    pub start: u32,
    pub stop: u32,
    pub status: SegmentStatus,
}

pub struct WPreedit {
    pub caret: i32,
    pub display: Vec<u16>,
    pub segments: Vec<SegmentData>,
}

impl WPreedit {
    pub fn new(preedit: &Preedit) -> Self {
        let caret = preedit.caret;
        let mut segments: Vec<SegmentData> = Vec::new();
        let mut display: Vec<u16> = Vec::new();

        for (idx, segment) in preedit.segments.iter().enumerate() {
            let wstring = segment.value.to_utf16();
            let size = wstring.len() as u32;
            display.extend(wstring);

            segments.push(SegmentData {
                start: idx as u32,
                stop: idx as u32 + size,
                status: segment.status.enum_value_or_default(),
            })
        }

        Self {
            caret,
            display,
            segments,
        }
    }
}

pub trait ToWidePreedit {
    fn widen(&self) -> WPreedit;
}

impl ToWidePreedit for Preedit {
    fn widen(&self) -> WPreedit {
        WPreedit::new(self)
    }
}
