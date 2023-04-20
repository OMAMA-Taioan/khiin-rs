pub(crate) mod cli;
pub(crate) mod engine_ctrl;
pub(crate) mod keys;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let ret = cli::run();
    println!("{}", termion::cursor::Show);
    ret
}
