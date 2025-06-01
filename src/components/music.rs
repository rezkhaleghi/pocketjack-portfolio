// src/components/music.rs
use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct MusicTrack {
    title: &'static str,
    youtube_url: &'static str,
    description: &'static str,
    platforms: Vec<PlatformLink>,
}

#[derive(Clone, PartialEq)]
struct PlatformLink {
    url: &'static str,
    platform: &'static str,
    title: &'static str,
    icon: &'static str,
}

#[function_component(Music)]
pub fn music() -> Html {
    let tracks = vec![
        MusicTrack {
            title: "Farigo - Dark Alley",
            youtube_url: "https://www.youtube.com/embed/yWnvb65k6_E",
            description: "Produced by PocketJack x Moeeney",
            platforms: vec![
                PlatformLink {
                    url: "https://soundcloud.com/farigo-music/dark-alley",
                    platform: "soundcloud",
                    title: "Listen on SoundCloud",
                    icon: r#"<path d="M3 9v6h4l5 5V4L7 9H3z"></path><path d="M16 12h2"></path><path d="M20 12h2"></path>"#,
                },
                PlatformLink {
                    url: "https://open.spotify.com/track/0bs3pzYBAjC03KrvkSREiu?si=c858fdd71ca54790",
                    platform: "spotify",
                    title: "Listen on Spotify",
                    icon: r#"<circle cx="12" cy="12" r="10"></circle><path d="M15.09 6.55a6.7 6.7 0 0 0-7.55 2.1 6.7 6.7 0 0 0 0 8.7 6.7 6.7 0 0 0 8.7 0"></path><path d="M13.66 9.28a4.5 4.5 0 0 0-5.66 1.42 4.5 4.5 0 0 0 0 5.66"></path><path d="M12.22 12a2.3 2.3 0 0 0-2.83.94 2.3 2.3 0 0 0 0 2.83"></path>"#,
                },
                PlatformLink {
                    url: "https://www.youtube.com/watch?v=yWnvb65k6_E",
                    platform: "youtube",
                    title: "Listen on YouTube",
                    icon: r#"<path d="M21 4a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V4z"></path><path d="M10 8l6 4-6 4V8z"></path>"#,
                },
            ],
        },
        MusicTrack {
            title: "Farigo - Perfection",
            youtube_url: "https://www.youtube.com/embed/Iov95aS-O7U",
            description: "Produced by PocketJack x Moeeney x 808K x 808Cash x Farigo",
            platforms: vec![
                PlatformLink {
                    url: "https://soundcloud.com/farigo-music/perfection",
                    platform: "soundcloud",
                    title: "Listen on SoundCloud",
                    icon: r#"<path d="M3 9v6h4l5 5V4L7 9H3z"></path><path d="M16 12h2"></path><path d="M20 12h2"></path>"#,
                },
                PlatformLink {
                    url: "https://open.spotify.com/track/6hSHnuPlqLhkpGCbpYvVXn?si=1a002fbf245e411a",
                    platform: "spotify",
                    title: "Listen on Spotify",
                    icon: r#"<circle cx="12" cy="12" r="10"></circle><path d="M15.09 6.55a6.7 6.7 0 0 0-7.55 2.1 6.7 6.7 0 0 0 0 8.7 6.7 6.7 0 0 0 8.7 0"></path><path d="M13.66 9.28a4.5 4.5 0 0 0-5.66 1.42 4.5 4.5 0 0 0 0 5.66"></path><path d="M12.22 12a2.3 2.3 0 0 0-2.83.94 2.3 2.3 0 0 0 0 2.83"></path>"#,
                },
                PlatformLink {
                    url: "https://www.youtube.com/watch?v=Iov95aS-O7U",
                    platform: "youtube",
                    title: "Listen on YouTube",
                    icon: r#"<path d="M21 4a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V4z"></path><path d="M10 8l6 4-6 4V8z"></path>"#,
                },
            ],
        },
        MusicTrack {
            title: "Farigo - Freestyle",
            youtube_url: "https://www.youtube.com/embed/7dofpxeOHCw",
            description: "Produced by PocketJack x Moeeney",
            platforms: vec![
                PlatformLink {
                    url: "https://soundcloud.com/farigo-music/freestyle",
                    platform: "soundcloud",
                    title: "Listen on SoundCloud",
                    icon: r#"<path d="M3 9v6h4l5 5V4L7 9H3z"></path><path d="M16 12h2"></path><path d="M20 12h2"></path>"#,
                },
                PlatformLink {
                    url: "https://open.spotify.com/album/5hvV1cVf9EkJu9tvFHxE5L?si=1ysLkteqSpGv322yCOzctw",
                    platform: "spotify",
                    title: "Listen on Spotify",
                    icon: r#"<circle cx="12" cy="12" r="10"></circle><path d="M15.09 6.55a6.7 6.7 0 0 0-7.55 2.1 6.7 6.7 0 0 0 0 8.7 6.7 6.7 0 0 0 8.7 0"></path><path d="M13.66 9.28a4.5 4.5 0 0 0-5.66 1.42 4.5 4.5 0 0 0 0 5.66"></path><path d="M12.22 12a2.3 2.3 0 0 0-2.83.94 2.3 2.3 0 0 0 0 2.83"></path>"#,
                },
                PlatformLink {
                    url: "https://www.youtube.com/watch?v=7dofpxeOHCw",
                    platform: "youtube",
                    title: "Listen on YouTube",
                    icon: r#"<path d="M21 4a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V4z"></path><path d="M10 8l6 4-6 4V8z"></path>"#,
                },
            ],
        },
        MusicTrack {
            title: "Blue (demo)",
            youtube_url: "https://www.youtube.com/embed/G6mtWxIcYHA",
            description: "Produced by PocketJack x NersiBeats x 808Cash",
            platforms: vec![
                PlatformLink {
                    url: "https://soundcloud.com/pocketjack/blue",
                    platform: "soundcloud",
                    title: "Listen on SoundCloud",
                    icon: r#"<path d="M3 9v6h4l5 5V4L7 9H3z"></path><path d="M16 12h2"></path><path d="M20 12h2"></path>"#,
                },
                PlatformLink {
                    url: "https://www.youtube.com/watch?v=G6mtWxIcYHA",
                    platform: "youtube",
                    title: "Listen on YouTube",
                    icon: r#"<path d="M21 4a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V4z"></path><path d="M10 8l6 4-6 4V8z"></path>"#,
                },
            ],
        },
        MusicTrack {
            title: "Mayas - Marpich",
            youtube_url: "https://www.youtube.com/embed/FoUr4ZgbvQo",
            description: "From my ex-band MAYAS, formed in 2017. I played the bass guitar.",
            platforms: vec![
                PlatformLink {
                    url: "https://www.youtube.com/watch?v=FoUr4ZgbvQo",
                    platform: "youtube",
                    title: "Listen on YouTube",
                    icon: r#"<path d="M21 4a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V4z"></path><path d="M10 8l6 4-6 4V8z"></path>"#,
                },
            ],
        },
        MusicTrack {
            title: "Dayan - Shekaste | Live Performance",
            youtube_url: "https://youtube.com/embed/oS6rGsPtze4?si=Oukuy7S3Uhf9IOdu",
            description: "A live session featuring three demo tracks by Dayan. I performed as the acoustic guitarist.",
            platforms: vec![
                PlatformLink {
                    url: "https://www.youtube.com/watch?v=oS6rGsPtze4",
                    platform: "youtube",
                    title: "Listen on YouTube",
                    icon: r#"<path d="M21 4a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V4z"></path><path d="M10 8l6 4-6 4V8z"></path>"#,
                },
            ],
        },
        MusicTrack {
            title: "Trap Type Beat | FREE",
            youtube_url: "https://youtube.com/embed/5Wh_iyh2Ino",
            description: "Produced by PocketJack x NersiBeats",
            platforms: vec![
                PlatformLink {
                    url: "https://www.youtube.com/watch?v=5Wh_iyh2Ino",
                    platform: "youtube",
                    title: "Listen on YouTube",
                    icon: r#"<path d="M21 4a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V4z"></path><path d="M10 8l6 4-6 4V8z"></path>"#,
                },
            ],
        },
        MusicTrack {
            title: "Moeeney - Liar",
            youtube_url: "https://www.youtube.com/embed/bGGL5BsdGEY",
            description: "Produced by PocketJack x Moeeney x Rwayne",
            platforms: vec![
                PlatformLink {
                    url: "https://www.youtube.com/watch?v=bGGL5BsdGEY",
                    platform: "youtube",
                    title: "Listen on YouTube",
                    icon: r#"<path d="M21 4a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V4z"></path><path d="M10 8l6 4-6 4V8z"></path>"#,
                },
            ],
        },
    ];

    html! {
        <section id="music" class="section">
            <div class="container">
                <h2 class="section-title">{ "Music" }</h2>
                <div class="cards">
                    { for tracks.iter().map(|track| {
                        html! {
                            <div class="card music-card">
                                <iframe
                                    class="music-cover"
                                    width="100%"
                                    height="200"
                                    src={track.youtube_url}
                                    title={track.title}
                                    frameborder="0"
                                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                                    allowfullscreen=true
                                    loading="lazy"
                                    aria-label={format!("Video player for {}", track.title)}
                                ></iframe>
                                <h3 class="card-title">{ track.title }</h3>
                                <p class="card-content">{ track.description }</p>
                                <div class="music-platforms">
                                    { for track.platforms.iter().map(|link| {
                                        html! {
                                            <a
                                                href={link.url}
                                                title={link.title}
                                                class={format!("platform-link platform-{}", link.platform)}
                                                aria-label={link.title}
                                            >
                                                <svg
                                                    class="platform-icon"
                                                    xmlns="http://www.w3.org/2000/svg"
                                                    viewBox="0 0 24 24"
                                                    fill="none"
                                                    stroke="currentColor"
                                                    stroke-width="2"
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                >
                                                    { Html::from_html_unchecked(link.icon.into()) }
                                                </svg>
                                                <span class="platform-fallback">{ link.platform }</span>
                                            </a>
                                        }
                                    })}
                                </div>
                            </div>
                        }
                    })}
                </div>
            </div>
        </section>
    }
}