use yew::prelude::*;

#[function_component(Experience)]
pub fn experience() -> Html {
    html! {
        <section id="experience" class="section">
            <div class="container">
                <h2 class="section-title">{ "Experience" }</h2>
                <div style="max-width: 800px; margin: 0 auto;">
                    <div style="margin-bottom: 3rem;">
                        <div style="display: flex; justify-content: space-between; margin-bottom: 0.5rem;">
                            <h3 style="font-size: 1.25rem;">{ "Senior Software Engineer" }</h3>
                            <span style="color: var(--text-secondary);">{ "2020 - Present" }</span>
                        </div>
                        <div style="color: var(--accent); margin-bottom: 1rem;">{ "Company Name" }</div>
                        <p style="color: var(--text-secondary);">
                            { "Description of your responsibilities and achievements in this role. Highlight key projects and technologies you worked with." }
                        </p>
                    </div>
                    <div style="margin-bottom: 3rem;">
                        <div style="display: flex; justify-content: space-between; margin-bottom: 0.5rem;">
                            <h3 style="font-size: 1.25rem;">{ "Software Developer" }</h3>
                            <span style="color: var(--text-secondary);">{ "2018 - 2020" }</span>
                        </div>
                        <div style="color: var(--accent); margin-bottom: 1rem;">{ "Previous Company" }</div>
                        <p style="color: var(--text-secondary);">
                            { "Description of your responsibilities and achievements in this role. Mention any promotions or special recognition you received." }
                        </p>
                    </div>
                    <div>
                        <div style="display: flex; justify-content: space-between; margin-bottom: 0.5rem;">
                            <h3 style="font-size: 1.25rem;">{ "Junior Developer" }</h3>
                            <span style="color: var(--text-secondary);">{ "2016 - 2018" }</span>
                        </div>
                        <div style="color: var(--accent); margin-bottom: 1rem;">{ "First Company" }</div>
                        <p style="color: var(--text-secondary);">
                            { "Description of your responsibilities and achievements in this role. Explain how this position helped shape your career path." }
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}