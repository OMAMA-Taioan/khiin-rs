use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ColorScheme {
    #[default]
    Auto,
    Light,
    Dark,
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct CandidateSettings {
    pub colors: ColorScheme,
    pub font_size: u8,
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct AppSettings {
    pub candidates: CandidateSettings,
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct SettingsManager {
    pub filename: PathBuf,
    pub settings: AppSettings,
}

impl SettingsManager {
    pub fn load_from_file(filename: &PathBuf) -> Self {
        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        if let Ok(settings) = toml::from_str::<AppSettings>(&contents) {
            SettingsManager {
                settings,
                filename: filename.clone(),
            }
        } else {
            SettingsManager {
                settings: AppSettings::default(),
                filename: filename.clone(),
            }
        }
    }

    pub fn set_font_size(&self, size: u8) {
        log::debug!("Setting font size to: {}", size);
    }
}

impl From<JsValue> for AppSettings {
    fn from(value: JsValue) -> Self {
        serde_wasm_bindgen::from_value(value).unwrap_or_else(|err| {
            log::debug!("Error deserializing to Settings object: {:?}", err);
            Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_deserializes() {
        let settings: AppSettings = toml::from_str(
            r#"
            [candidates]
            colors = "auto"
            font_size = 24
        "#,
        )
        .unwrap();

        assert_eq!(settings.candidates.colors, ColorScheme::Auto);
        assert_eq!(settings.candidates.font_size, 24);
    }
}
