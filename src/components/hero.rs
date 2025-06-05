use yew::prelude::*;
use crate::data::HERO_DATA;
use crate::models::SocialLink;

#[function_component(Hero)]
pub fn hero() -> Html {
    let data = HERO_DATA;

    html! {
        <section class="hero">
            <div class="container">
                <div class="hero">
                    <div class="hero-content">
                        <h1>{ &data.name }</h1>
                        <p class="subtitle">{ &data.subtitle }</p>
                        <p class="hero-description">{ &data.description }</p>
                        <div class="social-buttons-container">
                            <div class="social-buttons-title">{ "Connect With Me" }</div>
                            <div class="social-buttons">
                                { data.social_links.iter().map(|link: &SocialLink| {
                                    html! {
                                        <a
                                            href={link.url.to_string()}
                                            target="_blank"
                                            class={format!("social-button social-{}", link.platform)}
                                            title={link.title.to_string()}
                                        >
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                width={link.svg.width.to_string()}
                                                height={link.svg.height.to_string()}
                                                viewBox={link.svg.view_box.to_string()}
                                                fill="none"
                                                stroke="currentColor"
                                                stroke-width="2"
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                            >
                                                { link.svg.paths.iter().map(|path| {
                                                    html! { <path d={path.to_string()}></path> }
                                                }).collect::<Html>() }
                                            </svg>
                                        </a>
                                    }
                                }).collect::<Html>() }
                            </div>
                        </div>
                    </div>
                    <div class="hero-image">
                        <img src="/static/profile.jpg" alt={data.name.to_string()} class="profile-image"/>
                    </div>
                </div>
            </div>
        </section>
    }
}