use khiin_protos::command::Preedit;
use khiin_protos::command::SegmentStatus;

pub struct SegmentData {
    pub start: i32,
    pub stop: i32,
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

        let (segments, display): (Vec<SegmentData>, Vec<Vec<u16>>) =
            preedit.segments.iter().enumerate().map(|(i, s)| {
                let wstring: Vec<u16> = s.value.encode_utf16().collect();
                let size = wstring.len() as i32;
                let seg_data = SegmentData {
                    start: i as i32,
                    stop: i as i32 + size,
                    status: s.status.enum_value_or_default(),
                };
                (seg_data, wstring)
            }).unzip();
        
        let display: Vec<u16> = display.into_iter().flatten().collect();

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
