use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <section id="projects" class="section">
            <div class="container">
                <h2 class="section-title">{ "Personal Projects" }</h2>
                <div class="cards">
                    <div class="card">
                        <img src="/static/project1.jpg" alt="Project One" class="project-image"/>
                        <h3 class="card-title">{ "Project One" }</h3>
                        <p class="card-content">
                            { "A description of your first project. Explain what technologies you used and what problems it solves." }
                        </p>
                        <a href="https://github.com/pocketjack/project-one" class="card-link">{ "View Project →" }</a>
                    </div>
                    <div class="card">
                        <img src="/static/project2.png" alt="Project Two" class="project-image"/>
                        <h3 class="card-title">{ "Project Two" }</h3>
                        <p class="card-content">
                            { "A description of your second project. Highlight your role and the impact of this project." }
                        </p>
                        <a href="https://github.com/pocketjack/project-two" class="card-link">{ "View Project →" }</a>
                    </div>
                    <div class="card">
                        <img src="/static/project2.png" alt="Project Three" class="project-image"/>
                        <h3 class="card-title">{ "Project Three" }</h3>
                        <p class="card-content">
                            { "A description of your third project. Mention any challenges you overcame during development." }
                        </p>
                        <a href="https://github.com/pocketjack/project-three" class="card-link">{ "View Project →" }</a>
                    </div>
                </div>
            </div>
        </section>
    }
}