use yew::prelude::*;
use crate::data::CONTACT_DATA;

#[function_component(Contact)]
pub fn contact() -> Html {
    let data = CONTACT_DATA;

    html! {
        <section id="contact" class="section">
            <div class="container">
                <h2 class="section-title">{ "Get In Touch" }</h2>
                <div style="text-align: center; max-width: 600px; margin: 0 auto;">
                    <p style="color: var(--text-secondary); margin-bottom: 2rem;">
                        { data.description }
                    </p>
                    <div style="display: flex; justify-content: center; gap: 2rem; flex-wrap: wrap;">
                        { data.links.iter().map(|link| {
                            html! {
                                <div>
                                    <h3 style="font-size: 1.25rem; margin-bottom: 0.5rem;">{ link.platform }</h3>
                                    <a href={link.url} target={if link.platform != "Email" { "_blank" } else { "" }}>
                                        { link.display_text }
                                    </a>
                                </div>
                            }
                        }).collect::<Html>() }
                    </div>
                </div>
            </div>
        </section>
    }
}