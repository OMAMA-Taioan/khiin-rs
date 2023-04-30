mod locales;
mod macros;
mod menu;
mod routes;
mod screens;
mod wasm;

use yew::prelude::*;
use yew_router::prelude::*;

use menu::SideMenu;

use crate::app::locales::set_locale;
use crate::app::locales::Locales;
use crate::app::routes::switch_route;
use crate::app::routes::Route;

#[function_component(App)]
pub fn app() -> Html {
    set_locale(Locales::Tailo);

    html! {
        <BrowserRouter>
            <main class="app-container">
                <SideMenu />
                <Switch<Route> render={switch_route} />
            </main>
        </BrowserRouter>
    }
}
