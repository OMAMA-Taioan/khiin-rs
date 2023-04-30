use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::routes::AppLink;
use crate::app::routes::Route;

#[function_component(SideMenu)]
pub fn side_menu() -> Html {
    let route = use_route().unwrap_or(Route::Error404);

    html! {
        <aside class="side-menu">
            <p class="menu-label">
                {"General"}
            </p>
            <ul class="menu-list">
                <li>
                    <AppLink to={Route::Home}
                        classes={classes!(
                            (route == Route::Home).then(|| Some("active"))
                        )}
                    >
                        {"Getting Started"}
                    </AppLink>
                </li>
            </ul>
            <p class="menu-label">
                {"Settings"}
            </p>
            <ul class="menu-list">
                <li>
                    <AppLink to={Route::Appearance}
                        classes={classes!(
                            (route == Route::Appearance).then(|| Some("active"))
                        )}
                    >
                        {"Appearance"}
                    </AppLink>
                </li>
                <li>
                    <AppLink to={Route::Input}
                        classes={classes!(
                            (route == Route::Input).then(|| Some("active"))
                        )}
                    >
                        {"Input"}
                    </AppLink>
                </li>
                <li>
                    <AppLink to={Route::UserDictionary}
                        classes={classes!(
                            (route == Route::UserDictionary)
                                .then(|| Some("active"))
                        )}
                    >
                        {"Custom Dictionary"}
                    </AppLink>
                </li>
            </ul>
        </aside>
    }
}
