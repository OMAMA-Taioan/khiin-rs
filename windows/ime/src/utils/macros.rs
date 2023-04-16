#[macro_export]
macro_rules! winerr {
    ($s:ident) => {{
        Err(windows::core::Error::from($s))
    }};
}

#[macro_export]
macro_rules! check_win32error {
    ($result:ident) => {
        if $result.is_ok() {
            Ok(())
        } else {
            Err(windows::core::Error::from($result.to_hresult()))
        }
    };
    ($result:ident,$return:ident) => {
        if $result.is_ok() {
            Ok($return)
        } else {
            Err(windows::core::Error::from($result.to_hresult()))
        }
    };
}

#[macro_export]
macro_rules! trace {
    () => {
        log::debug!("{}", stdext::function_name!());
    };
}
