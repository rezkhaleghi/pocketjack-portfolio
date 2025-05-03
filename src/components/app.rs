use yew::prelude::*;
use crate::components::header::Header;
use crate::components::hero::Hero;
use crate::components::about::About;
use crate::components::projects::Projects;
use crate::components::music::Music;
use crate::components::contact::Contact;
use crate::components::footer::Footer;
// use crate::components::back_to_top::BackToTop; // Removed as the module does not exist

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Header />
            <main>
                <Hero />
                <About />
                <Projects />
                <Music />
                <Contact />
            </main>
            <Footer />
            //             <BackToTop /> // Removed as the component does not exist
        </>
    }
}