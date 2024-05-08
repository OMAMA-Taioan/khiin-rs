use std::marker::PhantomData;

use windows::core::Interface;
use windows::core::IUnknown;
use windows::core::Result;
use windows::Win32::UI::TextServices::ITfSource;
use windows::Win32::UI::TextServices::TF_INVALID_COOKIE;

pub struct SinkMgr<T: Interface> {
    _marker: PhantomData<T>,
    source: Option<ITfSource>,
    cookie: u32,
}

impl<T: Interface> SinkMgr<T> {
    pub fn new() -> Self {
        Self {
            _marker: Default::default(),
            source: None,
            cookie: TF_INVALID_COOKIE,
        }
    }

    pub fn advise(&mut self, has_source: IUnknown, sink: T) -> Result<()> {
        let source: ITfSource = has_source.cast()?;
        let punk: IUnknown = sink.cast()?;
        let cookie = unsafe { source.AdviseSink(&T::IID, &punk)? };
        self.source = Some(source);
        self.cookie = cookie;
        Ok(())
    }

    pub fn unadvise(&mut self) -> Result<()> {
        if self.cookie == TF_INVALID_COOKIE {
            return Ok(());
        }
        if let Some(source) = self.source.clone() {
            unsafe { source.UnadviseSink(self.cookie)? };
        }

        self.source = None;
        self.cookie = TF_INVALID_COOKIE;
        Ok(())
    }
}
