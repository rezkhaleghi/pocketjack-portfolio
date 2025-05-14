// src/components/app.rs
// app file that contains the main application component
use yew::prelude::*;
use crate::components::{
    // header::Header,
    hero::Hero,
    about::About,
    my_projects::Projects,
    client_project::ClientProjects,
    music::Music,
    experience::Experience, 
    contact::Contact,
    footer::Footer,
    skills::Skills,
};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            // <Header />
            <main>
                <Hero />
                <About />
                <Skills />
                <Projects />
                <ClientProjects />
                <Music />
                <Experience />
                <Contact />
            </main>
            <Footer />
        </>
    }
}