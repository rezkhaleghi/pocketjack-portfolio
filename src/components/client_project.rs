use yew::prelude::*;

#[function_component(ClientProjects)]
pub fn client_project() -> Html {
    html! {
        <section id="client-projects" class="section">
            <div class="container">
                <h2 class="section-title">{ "Client Projects" }</h2>
                <div class="cards">
                    <div class="card client-card">
                        <img src="/placeholder.svg?height=80&width=80" alt="Client One Logo" class="client-logo"/>
                        <h3 class="card-title">{ "Client Project One" }</h3>
                        <p class="card-content">
                            { "Description of the project you developed for this client. Highlight the challenges, your approach, and the results achieved." }
                        </p>
                        <div style="margin-top: 1rem;">
                            <span style="color: var(--accent); font-weight: 600; margin-right: 0.5rem;">{ "Technologies:" }</span>
                            <span>{ "Rust, WebAssembly, React" }</span>
                        </div>
                    </div>
                    <div class="card client-card">
                        <img src="/placeholder.svg?height=80&width=80" alt="Client Two Logo" class="client-logo"/>
                        <h3 class="card-title">{ "Client Project Two" }</h3>
                        <p class="card-content">
                            { "Description of the project you developed for this client. Explain the business problem and how your solution addressed it." }
                        </p>
                        <div style="margin-top: 1rem;">
                            <span style="color: var(--accent); font-weight: 600; margin-right: 0.5rem;">{ "Technologies:" }</span>
                            <span>{ "TypeScript, Node.js, MongoDB" }</span>
                        </div>
                    </div>
                    <div class="card client-card">
                        <img src="/placeholder.svg?height=80&width=80" alt="Client Three Logo" class="client-logo"/>
                        <h3 class="card-title">{ "Client Project Three" }</h3>
                        <p class="card-content">
                            { "Description of the project you developed for this client. Mention any specific requirements and how you met or exceeded expectations." }
                        </p>
                        <div style="margin-top: 1rem;">
                            <span style="color: var(--accent); font-weight: 600; margin-right: 0.5rem;">{ "Technologies:" }</span>
                            <span>{ "Python, Django, PostgreSQL" }</span>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}