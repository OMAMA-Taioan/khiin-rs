pub(crate) mod cli;
pub(crate) mod engine_ctrl;
pub(crate) mod keys;

fn main() -> anyhow::Result<()> {
    cli::run()
}
