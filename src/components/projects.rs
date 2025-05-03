use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    let projects = vec![
        (
            "Quantum Pulse",
            "A real-time audio visualization tool that transforms music into stunning visual experiences using WebGL and advanced audio analysis algorithms.",
            vec!["Rust", "WebGL", "Web Audio API"],
            "/static/project1.jpg",
            "Project One",
        ),
        (
            "Neural Synth",
            "An AI-powered music generation tool that uses machine learning to create unique compositions based on user input and preferences.",
            vec!["TypeScript", "TensorFlow.js", "Web MIDI API"],
            "/static/project2.jpg",
            "Project Two",
        ),
        (
            "Cyber Nexus",
            "A decentralized social platform built on blockchain technology that gives creators full ownership of their content and fair compensation.",
            vec!["Rust", "WebAssembly", "Blockchain"],
            "/static/project3.jpg",
            "Project Three",
        ),
    ];

    html! {
        <section id="projects" class="section">
            <div class="container">
                <h2 class="section-title" data-text="Projects">{ "Projects" }</h2>
                <div class="projects-grid">
                    { for projects.into_iter().map(|(title, desc, tags, img, alt)| html! {
                        <div class="project-card">
                            <img src={img} alt={alt} class="project-image" />
                            <div class="project-content">
                                <h3 class="project-title">{ title }</h3>
                                <p class="project-description">{ desc }</p>
                                <div class="project-tags">
                                    { for tags.into_iter().map(|tag| html! {
                                        <span class="project-tag">{ tag }</span>
                                    })}
                                </div>
                                <div class="project-links">
                                    <a href="#" class="project-link">
                                        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                            <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
                                            <polyline points="15 3 21 3 21 9"></polyline>
                                            <line x1="10" y1="14" x2="21" y2="3"></line>
                                        </svg>
                                        { "Live Demo" }
                                    </a>
                                    <a href="#" class="project-link">
                                        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                            <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
                                        </svg>
                                        { "Source Code" }
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