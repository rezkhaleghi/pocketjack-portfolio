use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <section id="about" class="section">
            <div class="container">
                <h2 class="section-title">{ "About Me" }</h2>
                <div style="max-width: 800px; margin: 0 auto; text-align: center;">
                    <p style="margin-bottom: 1.5rem;">
                        { "Hello! I'm Reza, also known as PocketJack. I'm a passionate software engineer and musician with a love for creating both functional applications and expressive music." }
                    </p>
                    <p style="margin-bottom: 1.5rem;">
                        { "My technical journey has led me through various technologies including Rust, which I'm currently using with the Yew framework. I believe in clean, efficient code that solves real problems." }
                    </p>
                    <p>
                        { "When I'm not coding, you'll find me creating music, exploring new sounds, and pushing the boundaries of my creative expression." }
                    </p>
                </div>
            </div>
        </section>
    }
}