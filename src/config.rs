use serde::Deserialize;
use windows::core::Result;
use windows::Win32::Foundation::HMODULE;

use khiin_protos::config::AppConfig;

#[derive(Deserialize, Default)]
pub struct WinConfig {
    pub engine: Option<EngineConfig>,
}

#[derive(Deserialize, Default)]
pub struct EngineConfig {
    pub input_mode: Option<String>,
}

pub fn load_from_file(module: HMODULE) -> Result<AppConfig> {
    Ok(AppConfig::default())
}
