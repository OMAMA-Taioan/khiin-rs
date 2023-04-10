// https://github.com/microsoft/windows-rs/issues/973#issuecomment-1363481060

use windows::core::PCWSTR;

pub struct Pcwstr {
    text: PCWSTR,
    // this is here to allow it to get dropped at the same time as the PCWSTR
    #[allow(unused)]
    _container: Vec<u16>,
}

impl std::ops::Deref for Pcwstr {
    type Target = PCWSTR;

    fn deref(&self) -> &Self::Target {
        &self.text
    }
}

pub trait ToPcwstr {
    fn to_pcwstr(&self) -> Pcwstr;
}

impl ToPcwstr for &str {
    fn to_pcwstr(&self) -> Pcwstr {
        if self.is_empty() {
            return Pcwstr {
                text: PCWSTR::null(),
                _container: Vec::new(),
            }
        }

        // do not drop when scope ends, by moving it into struct
        let mut text = self.encode_utf16().collect::<Vec<_>>();
        text.push(0);

        Pcwstr {
            text: PCWSTR::from_raw(text.as_ptr()),
            _container: text,
        }
    }
}

impl ToPcwstr for String {
    fn to_pcwstr(&self) -> Pcwstr {
        self.as_str().to_pcwstr()
    }
}
