use yew::prelude::*;

mod components;

use components::{Header, Home, Projects};

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="min-h-screen bg-gray-100">
            <Header />
            <main class="container mx-auto px-4 py-8">
                <Home />
                <Projects />
            </main>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}