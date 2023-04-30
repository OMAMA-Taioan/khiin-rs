use yew::prelude::*;

use crate::app::locales::t;
use crate::app::locales::t_args;
use crate::args;

#[function_component]
pub fn Home() -> Html {
    html! {
        <div class="content">
            <div class="row">
                <a href="https://khiin.app" target="_blank">
                    <img src="public/app-icon.png" class="logo link" alt="Khiin logo"/>
                </a>
            </div>

            <h1>{t("khiin-name")}</h1>

            <p>
                {t_args("greeting", &args!(
                    "name" => "Ta̍k-ke".into()
                ))}
            </p>
        </div>
    }
}
