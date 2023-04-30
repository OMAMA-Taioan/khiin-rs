use yew::prelude::*;
use yew_router::prelude::*;

use super::screens::AppearanceSettings;
use super::screens::Home;
use super::screens::InputSettings;
use super::screens::UserDictionary;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/appearance")]
    Appearance,
    #[at("/input")]
    Input,
    #[at("/user_dictionary")]
    UserDictionary,
    #[at("/404")]
    Error404,
}

pub fn switch_route(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Appearance => html! { <AppearanceSettings /> },
        Route::Input => html! { <InputSettings /> },
        Route::UserDictionary => html! { <UserDictionary /> },
        Route::Error404 => html! { <p>{"Page not found"}</p> },
    }
}

pub type AppLink = Link<Route>;
