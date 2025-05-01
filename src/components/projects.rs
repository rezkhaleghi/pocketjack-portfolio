use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Project {
    title: String,
    description: String,
    link: String,
}

#[function_component(Projects)]
pub fn projects() -> Html {
    let projects = vec![
        Project {
            title: "Project One".to_string(),
            description: "A cool web app built with Rust.".to_string(),
            link: "https://github.com/yourusername/project-one".to_string(),
        },
        Project {
            title: "Project Two".to_string(),
            description: "A machine learning dashboard.".to_string(),
            link: "https://github.com/yourusername/project-two".to_string(),
        },
    ];

    html! {
        <section id="projects" class="py-16">
            <h2 class="text-3xl font-bold text-center mb-8">{"My Projects"}</h2>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                { for projects.iter().map(|project| html! {
                    <div class="bg-white p-6 rounded-lg shadow-md">
                        <h3 class="text-xl font-semibold mb-2">{ &project.title }</h3>
                        <p class="text-gray-600 mb-4">{ &project.description }</p>
                        <a href={project.link.clone()} class="text-blue-600 hover:underline">{"View Project"}</a>
                    </div>
                })}
            </div>
        </section>
    }
}