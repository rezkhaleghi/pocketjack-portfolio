use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <section id="projects" class="section">
            <div class="container">
                <h2 class="section-title">{ "Personal Projects" }</h2>
                <div class="cards">
                    <div class="card">
                        <img src="./static/fairfly.jpg" alt="Fair-Fly" class="project-image"/>
                        <h3 class="card-title">{ "Fair-Fly" }</h3>
                        <p class="card-content">
                            { "Fair Fly is a web-based app that helps users find and compare affordable flight deals across multiple providers. Its core feature is a smart price alert system that notifies users when fares drop to their preferred price." }
                        </p>
                        <a href="https:/fair-fly.site" class="card-link">{ "View Website →" }</a>
                           <div style="margin-top: 1rem;">
                            <span style="color: var(--accent); font-weight: 600; margin-right: 0.5rem;">{ "Technologies:" }</span>
                            <span>{ "Rust, WebAssembly, MongoDB" }</span>
                        </div>
                    </div>
                    <div class="card">
                        <img src="./static/jorcolab.jpg" alt="Jorco-Lab" class="project-image"/>
                        <h3 class="card-title">{ "Jorco-Lab" }</h3>
                        <p class="card-content">
                            { "Jorcolab is a creative hub and digital marketplace for musicians, producers, and artists. Discover and purchase high-quality beats and samples, book studio time, or offer and hire music services like mixing, mastering, recording, and live instrumentation." }
                        </p>
                        <a href="https://jorcolab.com" class="card-link">{ "View Website →" }</a>
                           <div style="margin-top: 1rem;">
                            <span style="color: var(--accent); font-weight: 600; margin-right: 0.5rem;">{ "Technologies:" }</span>
                            <span>{ "TypeScript, Nest.js, MongoDB" }</span>
                        </div>
                    </div>
                    <div class="card">
                        <img src="./static/falseplayer.jpg" alt="False-Player" class="project-image"/>
                        <h3 class="card-title">{ "False-Player" }</h3>
                        <p class="card-content">
                            { "False Player is a web platform and Telegram mini app that lets users search for movies, music, videos, and books from across the internet. Stream movies and music online directly—fast, free, and without the hassle." }
                        </p>
                        <a href="https://player.false.foundation" class="card-link">{ "View Project →" }</a>
                        <a href="https://t.me/FalsePlayer_bot" class="card-link">{ "View Telegram App →" }</a>
                           <div style="margin-top: 1rem;">
                            <span style="color: var(--accent); font-weight: 600; margin-right: 0.5rem;">{ "Technologies:" }</span>
                            <span>{ "Rust, actix-web, teloxide, MongoDB" }</span>
                        </div>
                    </div>
                    <div class="card">
                        <img src="./static/pjplayer.gif" alt="PJ-Player" class="project-image"/>
                        <h3 class="card-title">{ "PJ-Player" }</h3>
                        <p class="card-content">
                            { "PJ-Player is a Rust-based CLI tool that allows you to search, download, and stream audio directly from your terminal." }
                        </p>
                        <a href="https://player.false.foundation" class="card-link">{ "View Project →" }</a>
                           <div style="margin-top: 1rem;">
                            <span style="color: var(--accent); font-weight: 600; margin-right: 0.5rem;">{ "Technologies:" }</span>
                            <span>{ "Rust, crossterm" }</span>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}