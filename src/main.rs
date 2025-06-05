use yew::prelude::*;

mod components;
mod pages;
mod models;
mod data;
use pages::home::Home;

#[function_component(Main)]
fn main_component() -> Html {
    html! {
        <Home />
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}