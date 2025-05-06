use wasm_bindgen::{prelude::Closure, JsCast};
use yew::prelude::*;
use web_sys::window;
use std::rc::Rc;

#[function_component(ClientProjects)]
pub fn client_project() -> Html {
    let current_slide = use_state(|| 0);
    let is_paused = use_state(|| false);
    let projects = vec![
        (
            "./static/fairfly.jpg",
            "Tmar-Travel",
            "Developed TMAR Travel, a ride-hailing platform for eco-tourism and off-road trips, where users can request a ride by selecting pickup, destination, and vehicle type. Architected the entire backend from scratch using a microservices approach, ensuring scalability, real-time processing, and clean service communication. Led a small agile team, guiding development, design patterns, and on-time delivery.",
            vec![("View Website →", "https://tmar.app")],
            "Node.Js, Nest.Js, MSSQL, MongoDB"
        ),
        (
            "./static/jorcolab.jpg",
            "UNFXB-Explorer",
            "Developed a multi-chain blockchain explorer enabling users to search transactions, wallets, hashes, and blocks across various blockchain networks including Bitcoin, Ethereum, Tron, Bitcoin Cash, Dogecoin, and Binance Coin (BNB). Leveraged blockchain APIs and technologies to create a user-friendly interface for navigating and exploring blockchain data across multiple networks.",
            vec![("View Website →", "https://explorer.unfxbit.com")],
            "Node.Js, MongoDB"
        ),
        (
            "./static/falseplayer.jpg",
            "Poudi-Guitar",
            "Engineered a simple guitar/music e-learning platform with robust e-commerce features and protected access codes. Orchestrated backend development for smooth and reliable user interaction, ensuring a user-friendly experience for accessing tutorial videos.",
            vec![("View Project →", "https://poudiguitar.com")],
            "Node.Js, MongoDB"
        ),
        (
            "./static/falseplayer.jpg",
            "Knowledge Management System(KMS)",
            "I successfully developed the backend of a Knowledge Management System (KMS) that functions as a social network for knowledge sharing and collaboration, using the KeystoneJS framework, a simple and flexible Node.js framework. This project involved creating a robust and scalable platform that enables users to connect, share insights, and manage knowledge assets within an organization.",
            vec![],
            "KeystoneJS, PostgreSQL, GraphQL, Node.Js"
        ),
        (
            "./static/falseplayer.jpg",
            "VVC Exchange",
            "Constructed a decentralized cryptocurrency exchange platform on the Stellar blockchain, enabling global fiat/crypto transfers and offering a versatile payment gateway.",
            vec![],
            "Node.Js, Stellar Blockchain, MongoDB"
        ),
    ];

    let total_projects = projects.len();
    let cards_per_slide = 3; // Adjusted to 3 to ensure slider activates with 4 projects
    let total_slides = (total_projects as f32 / cards_per_slide as f32).ceil() as usize; // Always calculate slides
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
                    5000, // Slide every 5 seconds
                )
                .unwrap();

            closure.forget(); // Prevent the closure from being dropped
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
        <section id="client-projects" class="section">
            <div class="container">
                <h2 class="section-title">{ "Clients Projects" }</h2>
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
                    
                    // Add slide indicators
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
            </div>
        </section>
    }
}