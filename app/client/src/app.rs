mod locales;
mod macros;
mod menu;
mod routes;
mod screens;
mod wasm;

use gloo_console::log;
use khiin_settings::ColorScheme;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use menu::SideMenu;

use crate::app::locales::set_locale;
use crate::app::locales::Locales;
use crate::app::routes::switch_route;
use crate::app::routes::Route;
use crate::app::wasm::load_settings;

#[function_component(App)]
pub fn app() -> Html {
    set_locale(Locales::Tailo);

    let settings = use_state_eq(|| JsValue::default());

    {
        let settings = settings.clone();
        use_effect(move || {
            spawn_local(async move {
                log!("In thread");
                let result = load_settings().await;
                if result.candidates.colors == ColorScheme::Light {
                    log!("Recieved response OK");
                }
            });
        });
    }

    html! {
        <BrowserRouter>
            <main class="app-container">
                <SideMenu />
                <div class="content-wrapper">
                    <Switch<Route> render={switch_route} />
                </div>
            </main>
        </BrowserRouter>
    }
}
