#![recursion_limit = "256"]

mod app;
mod positions;
mod text_input;

use app::App;

fn main() {
    yew::start_app::<App>();
}
