mod app;
mod locales;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
