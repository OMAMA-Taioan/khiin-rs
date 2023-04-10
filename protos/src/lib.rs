#[cfg(windows)]
macro_rules! sep {
    () => {
        r"\"
    };
}

#[cfg(not(windows))]
macro_rules! sep {
    () => {
        r"/"
    };
}

include!(concat!(env!("OUT_DIR"), sep!(), "protos", sep!(), "mod.rs"));
