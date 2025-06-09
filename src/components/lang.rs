use yew::prelude::*;
use crate::data::LANGUAGE_DATA;
use crate::models::Language;

#[function_component(Languages)]
pub fn languages() -> Html {
    let create_lang_card = |lang: &Language| {
        let proficiency = lang.proficiency;
        let radius = 65.0; // Increased to fill more of the 140px card
        let circumference = 2.0 * std::f64::consts::PI * radius;
        let stroke_dasharray = circumference;
        let stroke_dashoffset = circumference * (1.0 - proficiency / 100.0);

        html! {
            <div class="lang-card" role="listitem" aria-label={format!("{} - {}% proficiency", lang.name, lang.proficiency)}>
                <svg class="lang-ring" width="140" height="140" viewBox="0 0 140 140" preserveAspectRatio="xMidYMid meet">
                    <circle
                        class="lang-ring-bg"
                        cx="70"
                        cy="70"
                        r={radius.to_string()}
                        fill="none"
                        stroke="var(--border)"
                        stroke-width="10"
                    />
                    <circle
                        class="lang-ring-progress"
                        cx="70"
                        cy="70"
                        r={radius.to_string()}
                        fill="none"
                        stroke="var(--accent)"
                        stroke-width="10"
                        stroke-dasharray={stroke_dasharray.to_string()}
                        stroke-dashoffset={stroke_dashoffset.to_string()}
                        transform="rotate(-90 70 70)"
                    />
                </svg>
                <div class="lang-content">
                    <div class="lang-icon">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="28"
                            height="28"
                            viewBox={lang.icon.view_box.to_string()}
                            fill="none"
                            stroke="var(--accent)"
                            stroke-width="2"
                            stroke-linecap="square"
                            stroke-linejoin="miter"
                        >
                            { for lang.icon.paths.iter().map(|&path| html! { <path d={path.to_string()}></path> }) }
                        </svg>
                    </div>
                    <div class="lang-name">{ lang.name }</div>
                </div>
            </div>
        }
    };

    html! {
        <section id="languages" class="section">
            <div class="container">
                <h2 class="section-title">{ "Languages" }</h2>
                <div class="lang-section">
                    <div class="lang-grid" role="list" aria-label="List of languages">
                        { for LANGUAGE_DATA.iter().map(|lang| create_lang_card(lang)) }
                    </div>
                </div>
            </div>
        </section>
    }
}