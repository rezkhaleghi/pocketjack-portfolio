// src/components/contact.rs
use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <section id="contact" class="section">
            <div class="container">
                <h2 class="section-title">{ "Get In Touch" }</h2>
                <div style="text-align: center; max-width: 600px; margin: 0 auto;">
                    <p style="margin-bottom: 2rem; color: var(--text-secondary);">
                        { "Interested in working together or have questions about my work? Feel free to reach out through any of the channels below." }
                    </p>
                    <div style="display: flex; justify-content: center; gap: 2rem; flex-wrap: wrap;">
                        <div>
                            <h3 style="font-size: 1.25rem; margin-bottom: 0.5rem;">{ "Email" }</h3>
                            <a href="mailto:rezaxkhaleghi@gmail.com">{ "rezaxkhaleghi@gmail.com" }</a>
                        </div>
                        <div>
                            <h3 style="font-size: 1.25rem; margin-bottom: 0.5rem;">{ "GitHub" }</h3>
                            <a href="https://github.com/rezkhaleghi" target="_blank">{ "github.com/rezkhaleghi" }</a>
                        </div>
                        <div>
                            <h3 style="font-size: 1.25rem; margin-bottom: 0.5rem;">{ "LinkedIn" }</h3>
                            <a href="https://linkedin.com/in/rezaxkhaleghi" target="_blank">{ "linkedin.com/in/rezaxkhaleghi" }</a>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}