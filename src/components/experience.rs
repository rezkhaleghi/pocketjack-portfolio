use yew::prelude::*;
use crate::data::EXPERIENCE_DATA;

#[function_component(Experience)]
pub fn experience() -> Html {
    let experiences = EXPERIENCE_DATA.experiences;

    html! {
        <section id="experience" class="section">
            <div class="container">
                <h2 class="section-title">{ "Experience" }</h2>
                <div class="timeline" role="list">
                    { for experiences.iter().enumerate().map(|(index, exp)| {
                        html! {
                            <div class={classes!("timeline-entry", if index % 2 == 0 { "left" } else { "right" })} role="listitem" aria-label={format!("{} at {}, {}", exp.title, exp.company, exp.date)}>
                                <div class="timeline-marker"></div>
                                <img src={exp.image} alt={format!("{} logo", exp.company)} class="timeline-image" aria-hidden="true" />
                                <div class="timeline-content">
                                    <div class="experience-header">
                                        <h3 class="job-title">{ exp.title }</h3>
                                        <span class="job-date">{ exp.date }</span>
                                    </div>
                                    <div class="company-name">{ exp.company }</div>
                                    <p class="job-description">{ exp.description }</p>
                                    <div class="timeline-footer">
                                        <div class="technologies">
                                            <span class="tech-label">{ "Technologies:" }</span>
                                            <span>{ exp.technologies }</span>
                                        </div>
                                        { if !exp.links.is_empty() {
                                            html! {
                                                <div class="timeline-links">
                                                    { for exp.links.iter().map(|link| {
                                                        html! {
                                                            <a href={link.url} class="timeline-link" aria-label={link.text}>{ link.text }</a>
                                                        }
                                                    })}
                                                </div>
                                            }
                                        } else {
                                            html! {}
                                        }}
                                    </div>
                                </div>
                            </div>
                        }
                    })}
                </div>
            </div>
        </section>
    }
}