use yew::prelude::*;

#[function_component(Music)]
pub fn music() -> Html {
    html! {
        <section id="music" class="section">
            <div class="container">
                <h2 class="section-title">{ "Music" }</h2>
                <div class="cards">
                    <div class="card music-card">
                        <iframe
                            class="music-cover"
                            width="100%"
                            height="200"
                            src="https://www.youtube.com/embed/yWnvb65k6_E"
                            title="Farigo - Dark Alley"
                            frameborder="0"
                            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                            allowfullscreen= true
                        ></iframe>
                        <h3 class="card-title">{ "Farigo - Dark Alley" }</h3>
                        <p class="card-content">
                            { "Produced by PocketJack x Moeeney" }
                        </p>
                        <div class="music-platforms">
                            <a href="https://soundcloud.com/farigo-music/dark-alley" title="Listen on SoundCloud">
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
                            <a href="https://open.spotify.com/track/0bs3pzYBAjC03KrvkSREiu?si=c858fdd71ca54790" title="Listen on Spotify">
                                <svg class="platform-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <circle cx="12" cy="12" r="10"></circle>
                                    <path d="M8 11.2A5.6 5.6 0 0 1 16 8"></path>
                                    <path d="M8 14a3 3 0 0 1 4.8-2.4"></path>
                                    <path d="M8 16.8A5.6 5.6 0 0 1 16 14"></path>
                                </svg>
                            </a>
                            <a href="https://www.youtube.com/watch?v=yWnvb65k6_E" title="Listen on YouTube">
                                <svg class="platform-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M22.54 6.42a2.78 2.78 0 0 0-1.94-2C18.88 4 12 4 12 4s-6.88 0-8.6.46a2.78 2.78 0 0 0-1.94 2A29 29 0 0 0 1 11.75a29 29 0 0 0 .46 5.33A2.78 2.78 0 0 0 3.4 19c1.72.46 8.6.46 8.6.46s6.88 0 8.6-.46a2.78 2.78 0 0 0 1.94-2 29 29 0 0 0 .46-5.25 29 29 0 0 0-.46-5.33z"></path>
                                    <polygon points="9.75 15.02 15.5 11.75 9.75 8.48 9.75 15.02"></polygon>
                                </svg>
                            </a>
                        </div>
                    </div>
                    <div class="card music-card">
                        <iframe
                            class="music-cover"
                            width="100%"
                            height="200"
                            scrolling="no"
                            frameborder="no"
                            allow="autoplay"
                            src="https://w.soundcloud.com/player/?url=https%3A//soundcloud.com/farigo-music/perfection&auto_play=false&hide_related=true&show_comments=false&show_user=true&show_reposts=false&visual=false"
                        ></iframe>
                        <h3 class="card-title">{ "Farigo - Perfection" }</h3>
                        <p class="card-content">
                            { "Produced by PocketJack x Moeeney x 808K x 808Cash x Farigo" }
                        </p>
                        <div class="music-platforms">
                            <a href="https://soundcloud.com/farigo-music/perfection" title="Listen on SoundCloud">
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
                            <a href="https://open.spotify.com/track/6hSHnuPlqLhkpGCbpYvVXn?si=1a002fbf245e411a" title="Listen on Spotify">
                                <svg class="platform-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <circle cx="12" cy="12" r="10"></circle>
                                    <path d="M8 11.2A5.6 5.6 0 0 1 16 8"></path>
                                    <path d="M8 14a3 3 0 0 1 4.8-2.4"></path>
                                    <path d="M8 16.8A5.6 5.6 0 0 1 16 14"></path>
                                </svg>
                            </a>
                            <a href="https://www.youtube.com/watch?v=Iov95aS-O7U" title="Listen on YouTube">
                                <svg class="platform-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M22.54 6.42a2.78 2.78 0 0 0-1.94-2C18.88 4 12 4 12 4s-6.88 0-8.6.46a2.78 2.78 0 0 0-1.94 2A29 29 0 0 0 1 11.75a29 29 0 0 0 .46 5.33A2.78 2.78 0 0 0 3.4 19c1.72.46 8.6.46 8.6.46s6.88 0 8.6-.46a2.78 2.78 0 0 0 1.94-2 29 29 0 0 0 .46-5.25 29 29 0 0 0-.46-5.33z"></path>
                                    <polygon points="9.75 15.02 15.5 11.75 9.75 8.48 9.75 15.02"></polygon>
                                </svg>
                            </a>
                        </div>
                    </div>
                    <div class="card music-card">
                        <iframe
                            class="music-cover"
                            width="100%"
                            height="200"
                            src="https://www.youtube.com/embed/7dofpxeOHCw"
                            title="Fargio - Freestyle"
                            frameborder="0"
                            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                            allowfullscreen= true
                        ></iframe>
                        <h3 class="card-title">{ "Fargio - Freestyle" }</h3>
                        <p class="card-content">
                            { "Produced by PocketJack x Moeeney" }
                        </p>
                        <div class="music-platforms">
                            <a href="https://soundcloud.com/farigo-music/freestyle" title="Listen on SoundCloud">
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
                            <a href="https://www.youtube.com/watch?v=7dofpxeOHCw" title="Listen on YouTube">
                                <svg class="platform-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M22.54 6.42a2.78 2.78 0 0 0-1.94-2C18.88 4 12 4 12 4s-6.88 0-8.6.46a2.78 2.78 0 0 0-1.94 2A29 29 0 0 0 1 11.75a29 29 0 0 0 .46 5.33A2.78 2.78 0 0 0 3.4 19c1.72.46 8.6.46 8.6.46s6.88 0 8.6-.46a2.78 2.78 0 0 0 1.94-2 29 29 0 0 0 .46-5.25 29 29 0 0 0-.46-5.33z"></path>
                                    <polygon points="9.75 15.02 15.5 11.75 9.75 8.48 9.75 15.02"></polygon>
                                </svg>
                            </a>
                        </div>
                    </div>
                    <div class="card music-card">
                        <iframe
                            class="music-cover"
                            width="100%"
                            height="200"
                            src="https://www.youtube.com/embed/G6mtWxIcYHA"
                            title="Blue (demo)"
                            frameborder="0"
                            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                            allowfullscreen= true
                        ></iframe>
                        <h3 class="card-title">{ "Blue (demo)" }</h3>
                        <p class="card-content">
                            { "Produced by: PocketJack x NersiBeatz x 808Cash" }
                        </p>
                        <div class="music-platforms">
                            <a href="https://soundcloud.com/pocketjack/blue" title="Listen on SoundCloud">
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
                            <a href="https://www.youtube.com/watch?v=G6mtWxIcYHA" title="Listen on YouTube">
                                <svg class="platform-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M22.54 6.42a2.78 2.78 0 0 0-1.94-2C18.88 4 12 4 12 4s-6.88 0-8.6.46a2.78 2.78 0 0 0-1.94 2A29 29 0 0 0 1 11.75a29 29 0 0 0 .46 5.33A2.78 2.78 0 0 0 3.4 19c1.72.46 8.6.46 8.6.46s6.88 0 8.6-.46a2.78 2.78 0 0 0 1.94-2 29 29 0 0 0 .46-5.25 29 29 0 0 0-.46-5.33z"></path>
                                    <polygon points="9.75 15.02 15.5 11.75 9.75 8.48 9.75 15.02"></polygon>
                                </svg>
                            </a>
                        </div>
                    </div>
                    <div class="card music-card">
                        <iframe
                            class="music-cover"
                            width="100%"
                            height="200"
                            src="https://www.youtube.com/embed/FoUr4ZgbvQo"
                            title="Mayas - Marpich"
                            frameborder="0"
                            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                            allowfullscreen= true
                        ></iframe>
                        <h3 class="card-title">{ "Mayas - Marpich" }</h3>
                        <p class="card-content">
                            { "Its from My ex-band MAYAS formed in 2017, I Play the Bass guitar of it." }
                        </p>
                        <div class="music-platforms">
                            <a href="https://www.youtube.com/watch?v=FoUr4ZgbvQo" title="Listen on YouTube">
                                <svg class="platform-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M22.54 6.42a2.78 2.78 0 0 0-1.94-2C18.88 4 12 4 12 4s-6.88 0-8.6.46a2.78 2.78 0 0 0-1.94 2A29 29 0 0 0 1 11.75a29 29 0 0 0 .46 5.33A2.78 2.78 0 0 0 3.4 19c1.72.46 8.6.46 8.6.46s6.88 0 8.6-.46a2.78 2.78 0 0 0 1.94-2 29 29 0 0 0 .46-5.25 29 29 0 0 0-.46-5.33z"></path>
                                    <polygon points="9.75 15.02 15.5 11.75 9.75 8.48 9.75 15.02"></polygon>
                                </svg>
                            </a>
                        </div>
                    </div>

                    <div class="card music-card">
                        <iframe
                            class="music-cover"
                            width="100%"
                            height="200"
                            src="https://youtube.com/embed/oS6rGsPtze4?si=Oukuy7S3Uhf9IOdu"
                            title="Dayan - Shekaste | Live Performance"
                            frameborder="0"
                            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                            allowfullscreen= true
                        ></iframe>
                        <h3 class="card-title">{ "Dayan - Shekaste | Live Performance" }</h3>
                        <p class="card-content">
                            { "A live session featuring three demo tracks by Dayan. I performed as the acoustic guitarist for this set." }
                        </p>
                        <div class="music-platforms">
                            <a href="https://www.youtube.com/watch?v=oS6rGsPtze4" title="Listen on YouTube">
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