use gloo_console::log;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::app::wasm::set_font_size;

#[function_component]
pub fn AppearanceSettings() -> Html {
    let font_size_value_handle = use_state(|| String::from("16"));
    let font_size_value = (*font_size_value_handle).clone();

    let font_size_onchange = {
        let handle = font_size_value_handle.clone();

        Callback::from(move |e: Event| {
            let target = e.target();
            let input =
                target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let value = input.value();
                log!("Changed value: ", &value);
                if let Ok(size) = input.value().parse::<u8>() {
                    handle.set(input.value());
                    spawn_local(async move {
                        log!("Setting font size");
                        set_font_size(size).await;
                    });
                }
            }
        })
    };

    html! {
        <div class="settings-screen">
            <h1>{"Candidate Window"}</h1>
            <label for="font-size-input">
                {"Font Size: "}
                {font_size_value.clone()}
                <input
                    id="font-size-input"
                    type="range"
                    min="14"
                    max="32"
                    value={font_size_value}
                    onchange={font_size_onchange}
                />
            </label>
        </div>
    }
}
