use yew::prelude::*;

#[function_component(Music)]
pub fn music() -> Html {
    let music = vec![
        (
            "Neon Dreams",
            "Album • 2023",
            "A journey through cyberpunk landscapes and digital realms, blending electronic beats with atmospheric soundscapes.",
            "/static/music1.jpg",
            "Album One",
        ),
        (
            "Digital Horizon",
            "EP • 2022",
            "An exploration of the boundaries between organic and digital sounds, featuring collaborations with other electronic artists.",
            "/static/music2.jpg",
            "Album Two",
        ),
        (
            "Quantum Echoes",
            "Single • 2023",
            "A fusion of electronic and ambient elements that creates an immersive sonic experience inspired by quantum physics and parallel universes.",
            "/static/music3.jpg",
            "Album Three",
        ),
    ];

    html! {
        <section id="music" class="section">
            <div class="container">
                <h2 class="section-title" data-text="Music">{ "Music" }</h2>
                <div class="music-grid">
                    { for music.into_iter().map(|(title, subtitle, desc, img, alt)| html! {
                        <div class="music-card">
                            <img src={img} alt={alt} class="music-cover" />
                            <div class="music-content">
                                <h3 class="music-title">{ title }</h3>
                                <div class="music-subtitle">{ subtitle }</div>
                                <p class="music-description">{ desc }</p>
                                <div class="music-platforms">
                                    <a href="#" class="platform-link" title="Spotify">
                                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                            <circle cx="12" cy="12" r="10"></circle>
                                            <path d="M8 11.2A5.6 5.6 0 0 1 16 8"></path>
                                            <path d="M8 14a3 3 0 0 1 4.8-2.4"></path>
                                            <path d="M8 16.8A5.6 5.6 0 0 1 16 14"></path>
                                        </svg>
                                    </a>
                                    <a href="#" class="platform-link" title="SoundCloud">
                                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                            <path d="M2 12h1"></path>
                                            <path d="M6 12h1"></path>
                                            <path d="M10 12h1"></path>
                                            <path d="M14 12h1"></path>
                                            <path d="M18 12h1"></path>
                                            <path d="M4 18V6"></path>
                                            <path d="M8 18V6"></path>
                                            <path d="M12 18V6"></path>
                                            <path d="M16 18V6"></path>
                                            <path d="M20 18V6"></path>
                                        </svg>
                                    </a>
                                    <a href="#" class="platform-link" title="YouTube">
                                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                            <path d="M22.54 6.42a2.78 2.78 0 0 0-1.94-2C18.88 4 12 4 12 4s-6.88 0-8.6.46a2.78 2.78 0 0 0-1.94 2A29 29 0 0 0 1 11.75a29 29 0 0 0 .46 5.33A2.78 2.78 0 0 0 3.4 19c1.72.46 8.6.46 8.6.46s6.88 0 8.6-.46a2.78 2.78 0 0 0 1.94-2 29 29 0 0 0 .46-5.25 29 29 0 0 0-.46-5.33z"></path>
                                            <polygon points="9.75 15.02 15.5 11.75 9.75 8.48 9.75 15.02"></polygon>
                                        </svg>
                                    </a>
                                </div>
                            </div>
                        </div>
                    })}
                </div>
            </div>
        </section>
    }
}