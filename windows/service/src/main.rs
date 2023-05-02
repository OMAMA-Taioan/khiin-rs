mod server;


use anyhow::Result;
use interprocess::local_socket::tokio::LocalSocketListener;
use interprocess::local_socket::NameTypeSupport;

#[tokio::main]
pub async fn main() -> Result<()> {
    log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    let name = {
        use NameTypeSupport::*;
        match NameTypeSupport::query() {
            OnlyPaths => "/tmp/khiin.sock",
            OnlyNamespaced | Both => "@khiin.sock",
        }
    };

    if let Ok(listener) = LocalSocketListener::bind(name) {
        log::debug!("Begin listening on: {}", name);
        server::run(listener, tokio::signal::ctrl_c()).await;
    }

    Ok(())
}
