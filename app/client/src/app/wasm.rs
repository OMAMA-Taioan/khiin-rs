use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use khiin_settings::AppSettings;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub async fn load_settings() -> AppSettings {
    let result = invoke("load_settings", Default::default()).await;
    result.into()
}

#[derive(Serialize, Deserialize)]
struct FontSizeArgs {
    size: u8,
}

pub async fn set_font_size(size: u8) {
    let args = to_value(&FontSizeArgs { size }).unwrap();
    invoke("set_font_size", args).await;
}
