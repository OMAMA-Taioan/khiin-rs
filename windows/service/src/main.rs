mod engine_handler;
mod server;

use anyhow::Result;
use interprocess::local_socket::tokio::LocalSocketListener;
use interprocess::local_socket::NameTypeSupport;

#[tokio::main]
pub async fn main() -> Result<()> {
    log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    // get args
    let args: Vec<String> = std::env::args().collect();
    let mut suffix = "sock".to_string();
    if args.len() > 1 {
        suffix.push_str(".");
        suffix.push_str(&args[1]);
    }
    let name = {
        use NameTypeSupport::*;
        match NameTypeSupport::query() {
            OnlyPaths => format!("/tmp/khiin.{}", suffix),
            OnlyNamespaced | Both => format!("@khiin.{}", suffix),
        }
    };

    if let Ok(listener) = LocalSocketListener::bind(name.clone()) {
        log::debug!("Begin listening on: {}", name);
        server::run(listener, tokio::signal::ctrl_c()).await?;
    }

    Ok(())
}
