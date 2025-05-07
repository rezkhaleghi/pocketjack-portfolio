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
                            <h3 style="font-size: 1.25rem;">{ "Software Engineer" }</h3>
                            <span style="color: var(--text-secondary);">{ "2024 - Present" }</span>
                        </div>
                        <div style="color: var(--accent); margin-bottom: 1rem;">{ "Unicorn Forex Broker" }</div>
                        <p style="color: var(--text-secondary);">
                            { "Assisted in developing secure wallet custody systems and independently created an air-gapped encryption solution for private key management. Designed and implemented a multi-chain blockchain explorer to streamline transaction tracking across networks. Focused on optimizing workflows and enhancing security in blockchain-based financial services." }
                        </p>
                    </div>
                    <div style="margin-bottom: 3rem;">
                        <div style="display: flex; justify-content: space-between; margin-bottom: 0.5rem;">
                            <h3 style="font-size: 1.25rem;">{ "Lead BackEnd Developer" }</h3>
                            <span style="color: var(--text-secondary);">{ "2022 - Present" }</span>
                        </div>
                        <div style="color: var(--accent); margin-bottom: 1rem;">{ "TMAR" }</div>
                        <p style="color: var(--text-secondary);">
                            { "Pioneered the development of TMAR Travel from scratch, architecting the entire backend, design patterns, and structures.
Continuously enhanced and maintained the platform, enabling users to request off-road vehicles with professional drivers for their trips.
Managed a small group of developers and designers, ensuring effective collaboration and project delivery." }
                        </p>
                    </div>
                    <div style="margin-bottom: 3rem;">
                        <div style="display: flex; justify-content: space-between; margin-bottom: 0.5rem;">
                            <h3 style="font-size: 1.25rem;">{ "BackEnd Developer" }</h3>
                            <span style="color: var(--text-secondary);">{ "2020 - 2022" }</span>
                        </div>
                        <div style="color: var(--accent); margin-bottom: 1rem;">{ "DigiAlpha Agency" }</div>
                        <p style="color: var(--text-secondary);">
                            { "Played a pivotal role in developing several blockChain projects for DigiAlpha, contributing to the company's web development portfolio and success as a business landing page." }
                        </p>
                    </div>
                    <div style="margin-bottom: 3rem;">
                        <div style="display: flex; justify-content: space-between; margin-bottom: 0.5rem;">
                            <h3 style="font-size: 1.25rem;">{ "Web Developer" }</h3>
                            <span style="color: var(--text-secondary);">{ "2016 - Present" }</span>
                        </div>
                        <div style="color: var(--accent); margin-bottom: 1rem;">{ "Freelance" }</div>
                        <p style="color: var(--text-secondary);">
                            { "As a freelance developer, I’ve delivered a wide range of projects—from simple e-commerce websites to fully customized business solutions. My work spans web applications, Telegram bots, API integrations, and custom backend systems, tailored to meet unique client needs." }
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}