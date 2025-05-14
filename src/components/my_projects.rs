// src/components/my_projects.rs
use wasm_bindgen::{prelude::Closure, JsCast};
use yew::prelude::*;
use web_sys::window;

#[function_component(Projects)]
pub fn projects() -> Html {
    let current_slide = use_state(|| 0);
    let is_paused = use_state(|| false);
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
            "./static/falseplayer.jpg",
            "PJ-Grep",
            "A fast, flexible pattern search tool for files and directories. Easily search for patterns in filenames and content, filter by file extensions, and get color-coded results.",
            vec![("View Project →", "https://github.com/rezkhaleghi/pj-grep")],
            "Rust"
        ),
        (
            "./static/falseplayer.jpg",
            "THIS WEBSITE :)",
            "I built this website using Yew, a Rust framework for creating web applications. It showcases my projects and skills, and serves as a portfolio to demonstrate my work.",
            vec![("View Website →", "https://reza.false.foundation")],
            "Rust, Yew, WebAssembly"
        ),
    ];

    let total_projects = projects.len();
    let cards_per_slide = 3;
    let need_slider = total_projects > cards_per_slide;
    
    let total_slides = if need_slider {
        (total_projects as f32 / cards_per_slide as f32).ceil() as usize
    } else {
        1
    };
    
    let current = *current_slide;

    let next_slide = {
        let current_slide = current_slide.clone();
        Callback::from(move |_| {
            let next = (current + 1) % total_slides;
            current_slide.set(next);
        })
    };

    let prev_slide = {
        let current_slide = current_slide.clone();
        Callback::from(move |_| {
            let prev = (current + total_slides - 1) % total_slides;
            current_slide.set(prev);
        })
    };

    // Auto-slide effect
    {
        let current_slide = current_slide.clone();
        let is_paused = is_paused.clone();
        use_effect_with((), move |_| {
            let closure = Closure::wrap(Box::new(move || {
                if !*is_paused {
                    let next = (*current_slide + 1) % total_slides;
                    current_slide.set(next);
                }
            }) as Box<dyn Fn()>);

            let interval = window()
                .unwrap()
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    5000,
                )
                .unwrap();

            closure.forget();
            move || {
                window().unwrap().clear_interval_with_handle(interval);
            }
        });
    }

    let on_mouse_enter = {
        let is_paused = is_paused.clone();
        Callback::from(move |_| {
            is_paused.set(true);
        })
    };

    let on_mouse_leave = {
        let is_paused = is_paused.clone();
        Callback::from(move |_| {
            is_paused.set(false);
        })
    };

    html! {
        <section id="projects" class="section">
            <div class="container">
                <h2 class="section-title">{ "Personal Projects (FalseFoundation)" }</h2>
                <p class="section-description">{ "A collection of personal projects and projects built under my company, FalseFoundation Co." }</p>
                {
                    if !need_slider {
                        html! {
                            <div class="cards">
                                {
                                    projects.iter().map(|(img, title, desc, links, tech)| {
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
                        }
                    } else {
                        html! {
                            <div class="slider-container-wrapper" onmouseenter={on_mouse_enter} onmouseleave={on_mouse_leave}>
                                <button 
                                    class="slider-nav slider-prev" 
                                    onclick={prev_slide} 
                                    aria-label="Previous project"
                                    disabled={total_slides <= 1}
                                >{ "←" }</button>
                                
                                <div class="projects-slider">
                                    <div class="slider-track" style={format!("transform: translateX({}%)", -100 * (current as isize))}>
                                        {
                                            (0..total_slides).map(|slide_index| {
                                                let start_idx = slide_index * cards_per_slide;
                                                let end_idx = (start_idx + cards_per_slide).min(total_projects);
                                                let slide_projects = &projects[start_idx..end_idx];
                                                
                                                html! {
                                                    <div class="slider-page">
                                                        {
                                                            slide_projects.iter().map(|(img, title, desc, links, tech)| {
                                                                html! {
                                                                    <div class="slider-card">
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
                                                                }
                                                            }).collect::<Html>()
                                                        }
                                                    </div>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </div>
                                </div>
                                
                                <button 
                                    class="slider-nav slider-next" 
                                    onclick={next_slide} 
                                    aria-label="Next project"
                                    disabled={total_slides <= 1}
                                >{ "→" }</button>
                                
                                <div class="slider-indicators">
                                    {
                                        (0..total_slides).map(|i| {
                                            let is_current = i == current;
                                            let indicator_class = if is_current { "slider-indicator active" } else { "slider-indicator" };
                                            let index = i;
                                            let current_slide_c = current_slide.clone();
                                            
                                            let on_click = Callback::from(move |_| {
                                                current_slide_c.set(index);
                                            });
                                            
                                            html! {
                                                <button 
                                                    class={indicator_class} 
                                                    onclick={on_click} 
                                                    aria-label={format!("Go to slide {}", i+1)}
                                                ></button>
                                            }
                                        }).collect::<Html>()
                                    }
                                </div>
                            </div>
                        }
                    }
                }
            </div>
        </section>
    }
}