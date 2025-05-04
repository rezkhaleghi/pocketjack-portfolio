use yew::prelude::*;
use crate::components::{
    header::Header,
    hero::Hero,
    about::About,
    projects::Projects,
    client_project::ClientProjects,
    music::Music,
    experience::Experience, 
    contact::Contact,
    footer::Footer,
};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Header />
            <main>
                <Hero />
                <About />
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