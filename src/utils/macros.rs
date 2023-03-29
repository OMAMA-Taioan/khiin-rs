#[macro_export]
macro_rules! winerr {
    ($s:ident) => {
        {
            Err(Error::from($s))
        }
    };
}

#[macro_export]
macro_rules! pcwstr {
    ($s:expr) => {{
        let s: &str = $s;
        windows::core::PCWSTR(windows::core::HSTRING::from(s).as_ptr())
    }};
}

#[macro_export]
macro_rules! check_win32error {
    ($result:ident) => {
        if $result.is_ok() {
            Ok(())
        } else {
            Err(Error::from($result.to_hresult()))
        }
    };
    ($result:ident,$return:ident) => {
        if $result.is_ok() {
            Ok($return)
        } else {
            Err(Error::from($result.to_hresult()))
        }
    };
}
