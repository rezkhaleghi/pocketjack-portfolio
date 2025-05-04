use yew::prelude::*;

#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <section class="hero">
            <div class="container">
                <div class="hero">
                    <div class="hero-content">
                        <h1>{ "Reza \"PocketJack\"" }</h1>
                        <p class="subtitle">{ "Software Engineer & Musician" }</p>
                        <p class="hero-description">
                            { "Building innovative software solutions and creating music that resonates. I blend technical expertise with creative expression to craft unique digital experiences." }
                        </p>
                        <div style="margin-top: 2rem;">
                            <a href="#contact" class="cta-button">{ "Get in Touch" }</a>
                        </div>
                        <div class="social-buttons-container">
                            <div class="social-buttons-title">{ "Connect With Me" }</div>
                            <div class="social-buttons">
                                <a href="https://linkedin.com/in/pocketjack" target="_blank" class="social-button social-linkedin" title="LinkedIn">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z"></path>
                                        <rect x="2" y="9" width="4" height="12"></rect>
                                        <circle cx="4" cy="4" r="2"></circle>
                                    </svg>
                                </a>
                                <a href="https://github.com/pocketjack" target="_blank" class="social-button social-github" title="GitHub">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
                                    </svg>
                                </a>
                                <a href="https://youtube.com/pocketjack" target="_blank" class="social-button social-youtube" title="YouTube">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M22.54 6.42a2.78 2.78 0 0 0-1.94-2C18.88 4 12 4 12 4s-6.88 0-8.6.46a2.78 2.78 0 0 0-1.94 2A29 29 0 0 0 1 11.75a29 29 0 0 0 .46 5.33A2.78 2.78 0 0 0 3.4 19c1.72.46 8.6.46 8.6.46s6.88 0 8.6-.46a2.78 2.78 0 0 0 1.94-2 29 29 0 0 0 .46-5.25 29 29 0 0 0-.46-5.33z"></path>
                                        <polygon points="9.75 15.02 15.5 11.75 9.75 8.48 9.75 15.02"></polygon>
                                    </svg>
                                </a>
                                <a href="https://instagram.com/pocketjack" target="_blank" class="social-button social-instagram" title="Instagram">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <rect x="2" y="2" width="20" height="20" rx="5" ry="5"></rect>
                                        <path d="M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z"></path>
                                        <line x1="17.5" y1="6.5" x2="17.51" y2="6.5"></line>
                                    </svg>
                                </a>
                                <a href="https://patreon.com/pocketjack" target="_blank" class="social-button social-patreon" title="Patreon">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <circle cx="12" cy="12" r="10"></circle>
                                        <circle cx="12" cy="12" r="4"></circle>
                                    </svg>
                                </a>
                                <a href="https://soundcloud.com/pocketjack" target="_blank" class="social-button social-soundcloud" title="SoundCloud">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M2 12h1"></path>
                                        <path d="M6 12h1"></path>
                                        <path d="M10 12h1"></path>
                                        <path d="M14 12h1"></path>
                                        <path d="M18 12h1"></path>
                                        <path d="M4 18V6"></path>
                                        <path d="M8 18V6"></path>
                                        <path d="M12 18V6"></path>
                                        <path d="M16 18V6"></path>
                                        <path d="M20 18V6"></path>
                                    </svg>
                                </a>
                            </div>
                        </div>
                    </div>
                    <div class="hero-image">
                        <img src="/static/project1.jpg" alt="Reza (PocketJack)" class="profile-image"/>
                    </div>
                </div>
            </div>
        </section>
    }
}