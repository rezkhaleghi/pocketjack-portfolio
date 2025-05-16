// src/pages/home.rs
// Home page file that contains the main application page component
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

#[function_component(Home)]
pub fn home() -> Html {
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