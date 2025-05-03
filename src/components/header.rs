use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <div class="container">
                <nav>
                    <a href="#" class="logo">{ "PJ" }</a>
                    <div class="nav-links">
                        <a href="#about">{ "About" }</a>
                        <a href="#projects">{ "Projects" }</a>
                        <a href="#music">{ "Music" }</a>
                        <a href="#contact">{ "Contact" }</a>
                    </div>
                </nav>
            </div>
        </header>
    }
}