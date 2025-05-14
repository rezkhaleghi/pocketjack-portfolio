// main.rs
use yew::prelude::*;

mod components;

use components::app::App;

#[function_component(Main)]
fn main_component() -> Html {
    html! {
        <App />
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}