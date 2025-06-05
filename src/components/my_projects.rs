use wasm_bindgen::{prelude::Closure, JsCast};
use yew::prelude::*;
use web_sys::window;
use crate::data::PERSONAL_PROJECTS_DATA;

#[function_component(Projects)]
pub fn projects() -> Html {
    let current_slide = use_state(|| 0);
    let is_paused = use_state(|| false);
    let window_width = use_state(|| {
        window()
            .map(|w| w.inner_width().unwrap().as_f64().unwrap())
            .unwrap_or(0.0)
    });

    let projects = PERSONAL_PROJECTS_DATA;

    let total_projects = projects.len();
    let cards_per_slide = if *window_width <= 768.0 { 1 } else { 3 };
    let need_slider = total_projects > cards_per_slide;

    let total_slides = if need_slider {
        (total_projects as f64 / cards_per_slide as f64).ceil() as usize
    } else {
        1
    };

    let current = *current_slide;

    let update_window_width = {
        let window_width = window_width.clone();
        Callback::from(move |_| {
            if let Some(w) = window() {
                window_width.set(w.inner_width().unwrap().as_f64().unwrap());
            }
        })
    };

    {
        use_effect(move || {
            let listener = Closure::wrap(Box::new({
                let update_window_width = update_window_width.clone();
                move || update_window_width.emit(())
            }) as Box<dyn Fn()>);
            if let Some(w) = window() {
                w.add_event_listener_with_callback("resize", listener.as_ref().unchecked_ref())
                    .unwrap();
            }
            move || {
                if let Some(w) = window() {
                    w.remove_event_listener_with_callback("resize", listener.as_ref().unchecked_ref())
                        .unwrap();
                }
            }
        });
    }

    let next_slide = {
        let current_slide = current_slide.clone();
        Callback::from(move |_| {
            let next = (*current_slide + 1) % total_slides;
            current_slide.set(next);
        })
    };

    let prev_slide = {
        let current_slide = current_slide.clone();
        Callback::from(move |_| {
            let prev = if *current_slide == 0 {
                total_slides - 1
            } else {
                *current_slide - 1
            };
            current_slide.set(prev);
        })
    };

    // Auto-slide effect
    {
        let current_slide = current_slide.clone();
        let is_paused = is_paused.clone();
        use_effect(move || {
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
            move || {
                window().unwrap().clear_interval_with_handle(interval);
            }
        });
    }

    let on_mouse_enter = {
        let is_paused = is_paused.clone();
        Callback::from(move |_| is_paused.set(true))
    };

    let on_mouse_leave = {
        let is_paused = is_paused.clone();
        Callback::from(move |_| is_paused.set(false))
    };

    let transform_style = format!("transform: translateX(-{}%)", current * 100);

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
                                    projects.iter().map(|project| {
                                        html! {
                                            <div class="card">
                                                <img src={project.image} alt={project.title} class="project-image"/>
                                                <h3 class="card-title">{ project.title }</h3>
                                                <p class="card-content">{ project.description }</p>
                                                {
                                                    project.links.iter().map(|link| {
                                                        html! {
                                                            <a href={link.url} class="card-link">{ link.text }</a>
                                                        }
                                                    }).collect::<Html>()
                                                }
                                                <div style="margin-top: 1rem;">
                                                    <span style="color: var(--accent); font-weight: 600; margin-right: 0.5rem;">{ "Technologies:" }</span>
                                                    <span>{ project.technologies }</span>
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
                                    <div class="slider-track" style={transform_style}>
                                        {
                                            projects.chunks(cards_per_slide).enumerate().map(|(slide_index, chunk)| {
                                                html! {
                                                    <div class="slider-page" key={slide_index}>
                                                        {
                                                            chunk.iter().map(|project| {
                                                                html! {
                                                                    <div class="slider-card">
                                                                        <div class="card">
                                                                            <img src={project.image} alt={project.title} class="project-image"/>
                                                                            <h3 class="card-title">{ project.title }</h3>
                                                                            <p class="card-content">{ project.description }</p>
                                                                            {
                                                                                project.links.iter().map(|link| {
                                                                                    html! {
                                                                                        <a href={link.url} class="card-link">{ link.text }</a>
                                                                                    }
                                                                                }).collect::<Html>()
                                                                            }
                                                                            <div style="margin-top: 1rem;">
                                                                                <span style="color: var(--accent); font-weight: 600; margin-right: 0.5rem;">{ "Technologies:" }</span>
                                                                                <span>{ project.technologies }</span>
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