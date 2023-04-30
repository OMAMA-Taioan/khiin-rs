use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Default, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ColorScheme {
    #[default]
    Auto,
    Light,
    Dark,
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct AppSettings {
    pub colors: ColorScheme,
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct Settings {
    pub app: AppSettings,
}

impl Settings {
    pub fn load_from_file(filename: &PathBuf) -> Option<Settings> {
        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        toml::from_str(&contents).unwrap()
    }
}

impl From<JsValue> for Settings {
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
        let settings: Settings = toml::from_str(
            r#"
            [app]
            colors = "auto"
        "#,
        )
        .unwrap();

        assert_eq!(settings.app.colors, ColorScheme::Auto);
    }
}
