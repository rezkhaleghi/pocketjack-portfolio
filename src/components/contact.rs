use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <section id="contact" class="section">
            <div class="container">
                <h2 class="section-title" data-text="Contact">{ "Contact" }</h2>
                <div class="contact-grid">
                    <div class="contact-info">
                        <p class="contact-text">
                            { "Interested in working together or have questions about my work? I'm always open to new opportunities and collaborations. Feel free to reach out through any of the channels below." }
                        </p>
                        <div class="contact-methods">
                            <div class="contact-method">
                                <div class="contact-icon">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"></path>
                                        <polyline points="22,6 12,13 2,6"></polyline>
                                    </svg>
                                </div>
                                <div class="contact-details">
                                    <div class="contact-label">{ "Email" }</div>
                                    <div class="contact-value">
                                        <a href="mailto:contact@pocketjack.dev">{ "contact@pocketjack.dev" }</a>
                                    </div>
                                </div>
                            </div>
                            <div class="contact-method">
                                <div class="contact-icon">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
                                    </svg>
                                </div>
                                <div class="contact-details">
                                    <div class="contact-label">{ "GitHub" }</div>
                                    <div class="contact-value">
                                        <a href="https://github.com/pocketjack" target="_blank">{ "github.com/pocketjack" }</a>
                                    </div>
                                </div>
                            </div>
                            <div class="contact-method">
                                <div class="contact-icon">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z"></path>
                                        <rect x="2" y="9" width="4" height="12"></rect>
                                        <circle cx="4" cy="4" r="2"></circle>
                                    </svg>
                                </div>
                                <div class="contact-details">
                                    <div class="contact-label">{ "LinkedIn" }</div>
                                    <div class="contact-value">
                                        <a href="https://linkedin.com/in/pocketjack" target="_blank">{ "linkedin.com/in/pocketjack" }</a>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="contact-form">
                        <form>
                            <div class="form-group">
                                <label for="name" class="form-label">{ "Name" }</label>
                                <input type="text" id="name" class="form-control" placeholder="Your name" />
                            </div>
                            <div class="form-group">
                                <label for="email" class="form-label">{ "Email" }</label>
                                <input type="email" id="email" class="form-control" placeholder="Your email" />
                            </div>
                            <div class="form-group">
                                <label for="subject" class="form-label">{ "Subject" }</label>
                                <input type="text" id="subject" class="form-control" placeholder="Subject" />
                            </div>
                            <div class="form-group">
                                <label for="message" class="form-label">{ "Message" }</label>
                                <textarea id="message" class="form-control" placeholder="Your message"></textarea>
                            </div>
                            <button type="submit" class="form-submit">{ "Send Message" }</button>
                        </form>
                    </div>
                </div>
            </div>
        </section>
    }
}