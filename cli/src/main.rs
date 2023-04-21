mod app;
mod engine_ctrl;
mod keys;

use anyhow::Result;

fn main() -> Result<()> {
    let mut stdout = std::io::stdout();
    app::run(&mut stdout)
}
