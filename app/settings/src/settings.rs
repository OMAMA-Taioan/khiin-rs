use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ColorScheme {
    #[default]
    Auto,
    Light,
    Dark,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct CandidateSettings {
    #[serde(default)]
    pub colors: ColorScheme,
    #[serde(default)]
    pub font_size: u8,
}

impl Default for CandidateSettings {
    fn default() -> Self {
        Self {
            colors: Default::default(),
            font_size: 16,
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct AppSettings {
    #[serde(default)]
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

    pub fn save_to_file(&self) -> Result<()> {
        if let Ok(str) = toml::to_string(&self.settings) {
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .truncate(true)
                .open(&self.filename)
                .unwrap();
            let result = file.write_all(str.as_bytes());

            if result.is_err() {
                println!("Error: {}", result.unwrap_err());
                return Err(anyhow!("failed"));
            }

            Ok(())
        } else {
            Err(anyhow!("Unable to save file"))
        }
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
