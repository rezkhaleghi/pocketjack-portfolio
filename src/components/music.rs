use yew::prelude::*;

#[function_component(Music)]
pub fn music() -> Html {
    html! {
        <section id="music" class="section">
            <div class="container">
                <h2 class="section-title">{ "Music" }</h2>
                <div class="cards">
                    <div class="card music-card">
                        <img src="/static/music1.jpg" alt="Album/Track One" class="music-cover"/>
                        <h3 class="card-title">{ "Album/Track One" }</h3>
                        <p class="card-content">
                            { "Description of your first musical work. Talk about the inspiration and style." }
                        </p>
                        <div class="music-platforms">
                            <a href="https://soundcloud.com/pocketjack/track-one" title="Listen on SoundCloud">
                                <svg class="platform-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
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
                            <a href="https://open.spotify.com/artist/pocketjack" title="Listen on Spotify">
                                <svg class="platform-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <circle cx="12" cy="12" r="10"></circle>
                                    <path d="M8 11.2A5.6 5.6 0 0 1 16 8"></path>
                                    <path d="M8 14a3 3 0 0 1 4.8-2.4"></path>
                                    <path d="M8 16.8A5.6 5.6 0 0 1 16 14"></path>
                                </svg>
                            </a>
                            <a href="https://youtube.com/pocketjack" title="Listen on YouTube">
                                <svg class="platform-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M22.54 6.42a2.78 2.78 0 0 0-1.94-2C18.88 4 12 4 12 4s-6.88 0-8.6.46a2.78 2.78 0 0 0-1.94 2A29 29 0 0 0 1 11.75a29 29 0 0 0 .46 5.33A2.78 2.78 0 0 0 3.4 19c1.72.46 8.6.46 8.6.46s6.88 0 8.6-.46a2.78 2.78 0 0 0 1.94-2 29 29 0 0 0 .46-5.25 29 29 0 0 0-.46-5.33z"></path>
                                    <polygon points="9.75 15.02 15.5 11.75 9.75 8.48 9.75 15.02"></polygon>
                                </svg>
                            </a>
                        </div>
                    </div>
                    <div class="card music-card">
                        <img src="/static/music1.jpg" alt="Album/Track Two" class="music-cover"/>
                        <h3 class="card-title">{ "Album/Track Two" }</h3>
                        <p class="card-content">
                            { "Description of your second musical work. Mention any collaborations or special techniques." }
                        </p>
                        <div class="music-platforms">
                            <a href="https://soundcloud.com/pocketjack/track-two" title="Listen on SoundCloud">
                                <svg class="platform-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
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
                            <a href="https://open.spotify.com/artist/pocketjack" title="Listen on Spotify">
                                <svg class="platform-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <circle cx="12" cy="12" r="10"></circle>
                                    <path d="M8 11.2A5.6 5.6 0 0 1 16 8"></path>
                                    <path d="M8 14a3 3 0 0 1 4.8-2.4"></path>
                                    <path d="M8 16.8A5.6 5.6 0 0 1 16 14"></path>
                                </svg>
                            </a>
                            <a href="https://youtube.com/pocketjack" title="Listen on YouTube">
                                <svg class="platform-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M22.54 6.42a2.78 2.78 0 0 0-1.94-2C18.88 4 12 4 12 4s-6.88 0-8.6.46a2.78 2.78 0 0 0-1.94 2A29 29 0 0 0 1 11.75a29 29 0 0 0 .46 5.33A2.78 2.78 0 0 0 3.4 19c1.72.46 8.6.46 8.6.46s6.88 0 8.6-.46a2.78 2.78 0 0 0 1.94-2 29 29 0 0 0 .46-5.25 29 29 0 0 0-.46-5.33z"></path>
                                    <polygon points="9.75 15.02 15.5 11.75 9.75 8.48 9.75 15.02"></polygon>
                                </svg>
                            </a>
                        </div>
                    </div>
                    <div class="card music-card">
                        <img src="/static/music1.jpg" alt="Album/Track Three" class="music-cover"/>
                        <h3 class="card-title">{ "Album/Track Three" }</h3>
                        <p class="card-content">
                            { "Description of your third musical work. Share what makes this piece unique." }
                        </p>
                        <div class="music-platforms">
                            <a href="https://soundcloud.com/pocketjack/track-three" title="Listen on SoundCloud">
                                <svg class="platform-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
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
                            <a href="https://open.spotify.com/artist/pocketjack" title="Listen on Spotify">
                                <svg class="platform-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <circle cx="12" cy="12" r="10"></circle>
                                    <path d="M8 11.2A5.6 5.6 0 0 1 16 8"></path>
                                    <path d="M8 14a3 3 0 0 1 4.8-2.4"></path>
                                    <path d="M8 16.8A5.6 5.6 0 0 1 16 14"></path>
                                </svg>
                            </a>
                            <a href="https://youtube.com/pocketjack" title="Listen on YouTube">
                                <svg class="platform-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M22.54 6.42a2.78 2.78 0 0 0-1.94-2C18.88 4 12 4 12 4s-6.88 0-8.6.46a2.78 2.78 0 0 0-1.94 2A29 29 0 0 0 1 11.75a29 29 0 0 0 .46 5.33A2.78 2.78 0 0 0 3.4 19c1.72.46 8.6.46 8.6.46s6.88 0 8.6-.46a2.78 2.78 0 0 0 1.94-2 29 29 0 0 0 .46-5.25 29 29 0 0 0-.46-5.33z"></path>
                                    <polygon points="9.75 15.02 15.5 11.75 9.75 8.48 9.75 15.02"></polygon>
                                </svg>
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}