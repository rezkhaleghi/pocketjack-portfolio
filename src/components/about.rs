use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <section id="about" class="section">
            <div class="container">
                <h2 class="section-title" data-text="About">{ "About Me" }</h2>
                <div class="about-grid">
                    <div class="about-image">
                        <img src="/static/about.jpg" alt="Reza working" />
                    </div>
                    <div class="about-content">
                        <p>
                            { "Hello! I'm " }<strong>{ "Reza" }</strong>{ ", also known as " }<strong>{ "PocketJack" }</strong>{ ". I exist at the intersection of technology and creativity, crafting digital experiences that push boundaries and challenge conventions." }
                        </p>
                        <p>
                            { "As a " }<strong>{ "software engineer" }</strong>{ ", I specialize in building innovative solutions using modern technologies. My passion for clean, efficient code drives me to create applications that not only function flawlessly but also provide exceptional user experiences." }
                        </p>
                        <p>
                            { "When I'm not coding, you'll find me creating " }<strong>{ "music" }</strong>{ " that blends electronic elements with organic sounds. This dual passion allows me to approach problems from unique perspectives, finding creative solutions that others might miss." }
                        </p>
                        <div class="skills-container">
                            <h3 class="skills-title">{ "Technical Skills" }</h3>
                            <div class="skills-grid">
                                { for vec!["Rust", "JavaScript", "TypeScript", "React", "WebAssembly", "Node.js"].into_iter().map(|skill| html! {
                                    <div class="skill-item">
                                        <div class="skill-icon">
                                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                                <polyline points="16 18 22 12 16 6"></polyline>
                                                <polyline points="8 6 2 12 8 18"></polyline>
                                            </svg>
                                        </div>
                                        <div class="skill-name">{ skill }</div>
                                    </div>
                                })}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}