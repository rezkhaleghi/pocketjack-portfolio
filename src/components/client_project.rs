use yew::prelude::*;

#[function_component(ClientProjects)]
pub fn client_project() -> Html {
    let current_slide = use_state(|| 0);
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
            vec![
                ("View Project →", "https://poudiguitar.com"),
            ],
            "Node.Js, MongoDB"
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
                <h2 class="section-title">{ "Clients Projects" }</h2>
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