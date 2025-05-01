use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <section id="home" class="py-24">
            <div class="max-w-7xl mx-auto px-6">
                <div class="flex flex-col md:flex-row items-center justify-between gap-16">
                    <div class="flex-1 text-center md:text-left">
                        <h1 class="text-5xl font-bold mb-4 bg-gradient-to-r from-white to-blue-600 bg-clip-text text-transparent">
                            {"Reza \"PocketJack\""}
                        </h1>
                        <p class="text-xl text-gray-400 mb-6">
                            {"Software Engineer & Musician"}
                        </p>
                        <p class="text-lg text-gray-300 mb-8">
                            {"Building innovative software solutions and creating music that resonates. I blend technical expertise with creative expression to craft unique digital experiences."}
                        </p>
                        <div class="mb-10">
                            <a
                                href="#contact"
                                class="inline-block bg-blue-600 text-white py-3 px-6 rounded-md font-semibold hover:bg-blue-700 hover:-translate-y-0.5 transition-all duration-300"
                            >
                                {"Get in Touch"}
                            </a>
                        </div>
                        <div>
                            <div class="text-sm text-gray-400 uppercase font-semibold mb-4">
                                {"Connect With Me"}
                            </div>
                            <div class="flex gap-4 flex-wrap justify-center md:justify-start">
                                <a
                                    href="https://linkedin.com/in/pocketjack"
                                    target="_blank"
                                    class="flex items-center justify-center w-12 h-12 rounded-xl bg-gray-900 text-gray-400 hover:text-[#0077b5] hover:-translate-y-2 hover:shadow-lg transition-all duration-300"
                                    title="LinkedIn"
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z"></path>
                                        <rect x="2" y="9" width="4" height="12"></rect>
                                        <circle cx="4" cy="4" r="2"></circle>
                                    </svg>
                                </a>
                                <a
                                    href="https://github.com/pocketjack"
                                    target="_blank"
                                    class="flex items-center justify-center w-12 h-12 rounded-xl bg-gray-900 text-gray-400 hover:text-[#6e5494] hover:-translate-y-2 hover:shadow-lg transition-all duration-300"
                                    title="GitHub"
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
                                    </svg>
                                </a>
                                <a
                                    href="https://youtube.com/pocketjack"
                                    target="_blank"
                                    class="flex items-center justify-center w-12 h-12 rounded-xl bg-gray-900 text-gray-400 hover:text-[#ff0000] hover:-translate-y-2 hover:shadow-lg transition-all duration-300"
                                    title="YouTube"
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M22.54 6.42a2.78 2.78 0 0 0-1.94-2C18.88 4 12 4 12 4s-6.88 0-8.6.46a2.78 2.78 0 0 0-1.94 2A29 29 0 0 0 1 11.75a29 29 0 0 0 .46 5.33A2.78 2.78 0 0 0 3.4 19c1.72.46 8.6.46 8.6.46s6.88 0 8.6-.46a2.78 2.78 0 0 0 1.94-2 29 29 0 0 0 .46-5.25 29 29 0 0 0-.46-5.33z"></path>
                                        <polygon points="9.75 15.02 15.5 11.75 9.75 8.48 9.75 15.02"></polygon>
                                    </svg>
                                </a>
                                <a
                                    href="https://instagram.com/pocketjack"
                                    target="_blank"
                                    class="flex items-center justify-center w-12 h-12 rounded-xl bg-gray-900 text-gray-400 hover:text-[#e1306c] hover:-translate-y-2 hover:shadow-lg transition-all duration-300"
                                    title="Instagram"
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <rect x="2" y="2" width="20" height="20" rx="5" ry="5"></rect>
                                        <path d="M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z"></path>
                                        <line x1="17.5" y1="6.5" x2="17.51" y2="6.5"></line>
                                    </svg>
                                </a>
                                <a
                                    href="https://patreon.com/pocketjack"
                                    target="_blank"
                                    class="flex items-center justify-center w-12 h-12 rounded-xl bg-gray-900 text-gray-400 hover:text-[#f96854] hover:-translate-y-2 hover:shadow-lg transition-all duration-300"
                                    title="Patreon"
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <circle cx="12" cy="12" r="10"></circle>
                                        <circle cx="12" cy="12" r="4"></circle>
                                    </svg>
                                </a>
                                <a
                                    href="https://soundcloud.com/pocketjack"
                                    target="_blank"
                                    class="flex items-center justify-center w-12 h-12 rounded-xl bg-gray-900 text-gray-400 hover:text-[#ff7700] hover:-translate-y-2 hover:shadow-lg transition-all duration-300"
                                    title="SoundCloud"
                                >
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
                    <div class="flex-1 flex justify-center">
                        <img
                            src="/placeholder.svg?height=300&width=300"
                            alt="Reza (PocketJack)"
                            class="w-72 h-72 rounded-full object-cover border-4 border-blue-600"
                        />
                    </div>
                </div>
            </div>
        </section>
    }
}