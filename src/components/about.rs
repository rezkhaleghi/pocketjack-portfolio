use yew::prelude::*;
use crate::data::ABOUT_DATA;

#[function_component(About)]
pub fn about() -> Html {
    let data = ABOUT_DATA;

    html! {
        <section id="about" class="section">
            <div class="container">
                <h2 class="section-title">{ "About Me" }</h2>
                <div style="max-width: 800px; margin: 0 auto; text-align: center;">
                    { data.sections.iter().map(|section| {
                        html! {
                            <>
                                <p>{ section.emoji }</p>
                                <p style="margin-bottom: 1.5rem;">
                                    { section.text }
                                </p>
                            </>
                        }
                    }).collect::<Html>() }
                </div>
            </div>
        </section>
    }
}