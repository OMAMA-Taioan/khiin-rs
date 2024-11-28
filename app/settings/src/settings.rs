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
const INPUT_MODE_DEFAULT: &str = "classic";
const TONE_MODE_DEFAULT: &str = "telex";
const OUTPUT_MODE_DEFAULT: &str = "lomaji";
const T2_DEFAULT: char = 's';
const T3_DEFAULT: char = 'f';
const T5_DEFAULT: char = 'l';
const T6_DEFAULT: char = 'x';
const T7_DEFAULT: char = 'j';
const T8_DEFAULT: char = 'j';
const T9_DEFAULT: char = 'w';
const KHIN_DEFAULT: char = 'v';
const HYPHON_DEFAULT: char = 'd';
const DONE_DEFAULT: char = 'r';

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct InputSettings {
    #[serde(default = "default_input_mode")]
    pub input_mode: String,
    #[serde(default = "default_tone_mode")]
    pub tone_mode: String,
    #[serde(default = "default_output_mode")]
    pub output_mode: String,
    #[serde(default = "default_t2")]
    pub t2: char,
    #[serde(default = "default_t3")]
    pub t3: char,
    #[serde(default = "default_t5")]
    pub t5: char,
    #[serde(default = "default_t6")]
    pub t6: char,
    #[serde(default = "default_t7")]
    pub t7: char,
    #[serde(default = "default_t8")]
    pub t8: char,
    #[serde(default = "default_t9")]
    pub t9: char,
    #[serde(default = "default_khin")]
    pub khin: char,
    #[serde(default = "default_hyphon")]
    pub hyphon: char,
    #[serde(default = "default_done")]
    pub done: char,
}

fn default_input_mode() -> String {
    INPUT_MODE_DEFAULT.to_string()
}

fn default_tone_mode() -> String {
    TONE_MODE_DEFAULT.to_string()
}

fn default_output_mode() -> String {
    OUTPUT_MODE_DEFAULT.to_string()
}

fn default_t2() -> char {
    T2_DEFAULT
}

fn default_t3() -> char {
    T3_DEFAULT
}

fn default_t5() -> char {
    T5_DEFAULT
}

fn default_t6() -> char {
    T6_DEFAULT
}

fn default_t7() -> char {
    T7_DEFAULT
}

fn default_t8() -> char {
    T8_DEFAULT
}

fn default_t9() -> char {
    T9_DEFAULT
}

fn default_khin() -> char {
    KHIN_DEFAULT
}

fn default_hyphon() -> char {
    HYPHON_DEFAULT
}

fn default_done() -> char {
    DONE_DEFAULT
}

impl Default for InputSettings {
    fn default() -> Self {
        Self {
            input_mode: INPUT_MODE_DEFAULT.to_string(),
            tone_mode: TONE_MODE_DEFAULT.to_string(),
            output_mode: OUTPUT_MODE_DEFAULT.to_string(),
            t2: T2_DEFAULT,
            t3: T3_DEFAULT,
            t5: T5_DEFAULT,
            t6: T6_DEFAULT,
            t7: T7_DEFAULT,
            t8: T8_DEFAULT,
            t9: T9_DEFAULT,
            khin: KHIN_DEFAULT,
            hyphon: HYPHON_DEFAULT,
            done: DONE_DEFAULT,
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct AppSettings {
    #[serde(default)]
    pub candidates: CandidateSettings,
    #[serde(default)]
    pub input_settings: InputSettings,
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct SettingsManager {
    pub filename: PathBuf,
    pub settings: AppSettings,
}

impl SettingsManager {
    pub fn load_from_file(filename: &PathBuf) -> Self {
        if let Ok(mut file) = File::open(filename) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
    
            if let Ok(mut settings) = toml::from_str::<AppSettings>(&contents) {
                if settings.input_settings.input_mode == "auto" {
                    settings.input_settings.input_mode = INPUT_MODE_DEFAULT.to_string();
                }
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
                .create(true)
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

            [input_settings]
            input_mode = "manual"
            tone_mode = "numeric"
            output_mode = "lomaji"
            t3 = "c"
        "#,
        )
        .unwrap();

        assert_eq!(settings.candidates.colors, ColorScheme::Auto);
        assert_eq!(settings.candidates.font_size, 24);
        assert_eq!(settings.input_settings.input_mode, "manual");
        assert_eq!(settings.input_settings.tone_mode, "numeric");
        assert_eq!(settings.input_settings.output_mode, "lomaji");
        assert_eq!(settings.input_settings.t2, 's');
        assert_eq!(settings.input_settings.t3, 'c');
    }
}
