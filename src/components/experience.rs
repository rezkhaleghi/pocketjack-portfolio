use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct ExperienceEntry {
    title: &'static str,
    company: &'static str,
    date: &'static str,
    description: &'static str,
}

#[function_component(Experience)]
pub fn experience() -> Html {
    let experiences = vec![
        ExperienceEntry {
            title: "Software Engineer",
            company: "Unicorn Forex Broker",
            date: "2024 - Present",
            description: "Assisted in developing secure wallet custody systems and independently created an air-gapped encryption solution for private key management. Designed and implemented a multi-chain blockchain explorer to streamline transaction tracking across networks. Focused on optimizing workflows and enhancing security in blockchain-based financial services.",
        },
        ExperienceEntry {
            title: "Lead BackEnd Developer",
            company: "TMAR",
            date: "2022 - Present",
            description: "Pioneered the development of TMAR Travel from scratch, architecting the entire backend, design patterns, and structures. Continuously enhanced and maintained the platform, enabling users to request off-road vehicles with professional drivers for their trips. Managed a small group of developers and designers, ensuring effective collaboration and project delivery.",
        },
        ExperienceEntry {
            title: "BackEnd Developer",
            company: "DigiAlpha",
            date: "2020 - 2022",
            description: "Played a pivotal role in developing several blockchain projects for DigiAlpha, contributing to the company's web development portfolio and success as a business landing page.",
        },
        ExperienceEntry {
            title: "Web Developer",
            company: "Freelance",
            date: "2016 - Present",
            description: "As a freelance developer, I’ve delivered a wide range of projects—from simple e-commerce websites to fully customized business solutions. My work spans web applications, Telegram bots, API integrations, and custom backend systems, tailored to meet unique client needs.",
        },
    ];

    html! {
        <section id="experience" class="section">
            <div class="container">
                <h2 class="section-title">{ "Experience" }</h2>
                <div class="timeline" role="list">
                    { for experiences.iter().enumerate().map(|(index, exp)| {
                        html! {
                            <div class={classes!("timeline-entry", if index % 2 == 0 { "left" } else { "right" })} role="listitem" aria-label={format!("{} at {}, {}", exp.title, exp.company, exp.date)}>
                                <div class="timeline-marker"></div>
                                <div class="timeline-content">
                                    <div class="experience-header">
                                        <h3 class="job-title">{ exp.title }</h3>
                                        <span class="job-date">{ exp.date }</span>
                                    </div>
                                    <div class="company-name">{ exp.company }</div>
                                    <p class="job-description">{ exp.description }</p>
                                </div>
                            </div>
                        }
                    })}
                </div>
            </div>
        </section>
    }
}