#[cfg(not(windows))]
pub(crate) mod cli;
#[cfg(not(windows))]
pub(crate) mod engine_ctrl;
#[cfg(not(windows))]
pub(crate) mod keys;

#[cfg(not(windows))]
fn main() -> anyhow::Result<()> {
    env_logger::init();
    let ret = cli::run();
    println!("{}", termion::cursor::Show);
    ret
}

#[cfg(windows)]
fn main() {
    println!("Not implemented on windows.")
}
