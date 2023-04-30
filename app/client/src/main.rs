mod app;

use gloo_console::log;

use app::App;

fn main() {
    log!("Starting app");
    yew::Renderer::<App>::new().render();
}
