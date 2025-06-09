// src/components/footer.rs
use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <div class="container">
                <p>
                    { "© 2025 " }
                    <a href="https://false.foundation" target="_blank" rel="noopener noreferrer">
                        { "FalseFoundation" }
                    </a>
                    { ". All rights reserved." }
                </p>
            </div>
        </footer>
    }
}