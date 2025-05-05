use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    let current_slide = use_state(|| 0);
    let projects = vec![
        (
            "./static/fairfly.jpg",
            "Fair-Fly",
            "Fair Fly is a web-based app that helps users find and compare affordable flight deals across multiple providers. Its core feature is a smart price alert system that notifies users when fares drop to their preferred price.",
            vec![("View Website →", "https://fair-fly.site")],
            "Rust, WebAssembly, actix-web, Yew, MongoDB"
        ),
        (
            "./static/jorcolab.jpg",
            "Jorco-Lab",
            "Jorcolab is a creative hub and digital marketplace for musicians, producers, and artists. Discover and purchase high-quality beats and samples, book studio time, or offer and hire music services like mixing, mastering, recording, and live instrumentation.",
            vec![("View Website →", "https://jorcolab.com")],
            "TypeScript, Nest.js, MongoDB"
        ),
        (
            "./static/falseplayer.jpg",
            "False-Player",
            "False Player is a web platform and Telegram mini app that lets users search for movies, music, videos, and books from across the internet. Stream movies and music online directly—fast, free, and without the hassle.",
            vec![
                ("View Website →", "https://player.false.foundation"),
                ("View Telegram App →", "https://t.me/FalsePlayer_bot")
            ],
            "Rust, actix-web, teloxide, MongoDB"
        ),
        (
            "./static/pjplayer.gif",
            "PJ-Player",
            "PJ-Player is a Rust-based CLI tool that allows you to search, download, and stream audio directly from your terminal.",
            vec![("View Project →", "https://player.false.foundation")],
            "Rust, crossterm"
        ),
        (
            "./static/newproject.jpg",
            "New Project",
            "A placeholder project to test the slider functionality.",
            vec![("View Project →", "https://newproject.com")],
            "Rust, Yew"
        ),
    ];

    let (grid_projects, slider_projects) = projects.split_at(3);
    let total_slides = slider_projects.len();
    let current = *current_slide;

    let next_slide = {
        let current_slide = current_slide.clone();
        Callback::from(move |_| {
            current_slide.set((current + 1) % total_slides);
        })
    };

    let prev_slide = {
        let current_slide = current_slide.clone();
        Callback::from(move |_| {
            current_slide.set((current + total_slides - 1) % total_slides);
        })
    };

    html! {
        <section id="projects" class="section">
            <div class="container">
                <h2 class="section-title">{ "Personal Projects (FalseFoundation)" }</h2>
                <div class="cards">
                    {
                        grid_projects.iter().map(|(img, title, desc, links, tech)| {
                            html! {
                                <div class="card">
                                    <img src={*img} alt={*title} class="project-image"/>
                                    <h3 class="card-title">{ *title }</h3>
                                    <p class="card-content">{ *desc }</p>
                                    {
                                        links.iter().map(|(text, url)| {
                                            html! {
                                                <a href={*url} class="card-link">{ *text }</a>
                                            }
                                        }).collect::<Html>()
                                    }
                                    <div style="margin-top: 1rem;">
                                        <span style="color: var(--accent); font-weight: 600; margin-right: 0.5rem;">{ "Technologies:" }</span>
                                        <span>{ *tech }</span>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
                {
                    if !slider_projects.is_empty() {
                        let (img, title, desc, links, tech) = &slider_projects[current];
                        html! {
                            <div class="slider">
                                <button class="slider-nav slider-prev" onclick={prev_slide} disabled={total_slides <= 1}>{ "←" }</button>
                                <div class="slider-container" style={format!("transform: translateX(-{}%);", current * 100)}>
                                    <div class="slider-wrapper">
                                        <div class="card">
                                            <img src={*img} alt={*title} class="project-image"/>
                                            <h3 class="card-title">{ *title }</h3>
                                            <p class="card-content">{ *desc }</p>
                                            {
                                                links.iter().map(|(text, url)| {
                                                    html! {
                                                        <a href={*url} class="card-link">{ *text }</a>
                                                    }
                                                }).collect::<Html>()
                                            }
                                            <div style="margin-top: 1rem;">
                                                <span style="color: var(--accent); font-weight: 600; margin-right: 0.5rem;">{ "Technologies:" }</span>
                                                <span>{ *tech }</span>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                <button class="slider-nav slider-next" onclick={next_slide} disabled={total_slides <= 1}>{ "→" }</button>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        </section>
    }
}