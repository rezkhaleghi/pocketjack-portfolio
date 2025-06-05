use yew::prelude::*;
use crate::data::MUSIC_DATA;

#[function_component(Music)]
pub fn music() -> Html {
    let tracks = MUSIC_DATA.tracks;

    html! {
        <section id="music" class="section">
            <div class="container">
                <h2 class="section-title">{ "Music" }</h2>
                <div class="cards">
                    { for tracks.iter().map(|track| {
                        html! {
                            <div class="card music-card">
                                <iframe
                                    class="music-cover"
                                    width="100%"
                                    height="200"
                                    src={track.youtube_url}
                                    title={track.title}
                                    frameborder="0"
                                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                                    allowfullscreen=true
                                    loading="lazy"
                                    aria-label={format!("Video player for {}", track.title)}
                                ></iframe>
                                <h3 class="card-title">{ track.title }</h3>
                                <p class="card-content">{ track.description }</p>
                                <div class="music-platforms">
                                    { for track.platforms.iter().map(|link| {
                                        html! {
                                            <a
                                                href={link.url}
                                                title={link.title}
                                                class={format!("platform-link platform-{}", link.platform)}
                                                aria-label={link.title}
                                                target="_blank"
                                            >
                                                <svg
                                                    class="platform-icon"
                                                    width={link.svg.width.to_string()}
                                                    height={link.svg.height.to_string()}
                                                    viewBox={link.svg.view_box}
                                                    fill="none"
                                                    stroke="currentColor"
                                                    stroke-width="2"
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                >
                                                    { for link.svg.paths.iter().map(|path| {
                                                        html! { <path d={*path}></path> }
                                                    })}
                                                </svg>
                                                <span class="platform-fallback">{ link.platform }</span>
                                            </a>
                                        }
                                    })}
                                </div>
                            </div>
                        }
                    })}
                </div>
            </div>
        </section>
    }
}