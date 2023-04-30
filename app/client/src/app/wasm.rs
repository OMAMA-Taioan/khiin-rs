use wasm_bindgen::prelude::*;

use khiin_settings::Settings;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub async fn invoke_load_settings() -> Settings {
    let result = invoke("load_settings", Default::default()).await;
    result.into()
}
