// src/pages/home.rs
// Home page file that contains the main application page component
use yew::prelude::*;
use crate::components::{
    // header::Header,
    about::About, client_project::ClientProjects, contact::Contact, experience::Experience, footer::Footer, hero::Hero, lang::Languages, music::Music, my_projects::Projects, skills::Skills
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
                // <Beats />
                <Experience />
                <Languages />
                <Contact />
            </main>
            <Footer />
        </>
    }
}