use crate::models::{HeroData, SocialLink, SvgData};

pub const HERO_DATA: HeroData = HeroData {
    name: "Reza Khaleghi \"PocketJack\"",
    subtitle: "Software Engineer & Musician",
    description: "Building innovative software solutions and creating music that resonates. I blend technical expertise with creative expression to craft unique digital experiences.",
    social_links: &[
        SocialLink {
            platform: "linkedin",
            url: "https://linkedin.com/in/rezaxkhaleghi",
            title: "LinkedIn",
            svg: SvgData {
                width: 24,
                height: 24,
                view_box: "0 0 24 24",
                paths: &[
                    "M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z",
                    "M2 9h4v12H2z",
                    "M4 2a2 2 0 1 1 0 4 2 2 0 0 1 0-4z",
                ],
            },
        },
        SocialLink {
            platform: "github",
            url: "https://github.com/rezkhaleghi",
            title: "GitHub",
            svg: SvgData {
                width: 24,
                height: 24,
                view_box: "0 0 24 24",
                paths: &[
                    "M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22",
                ],
            },
        },
        SocialLink {
            platform: "youtube",
            url: "https://youtube.com/@pocketJack1",
            title: "YouTube",
            svg: SvgData {
                width: 24,
                height: 24,
                view_box: "0 0 24 24",
                paths: &[
                    "M22.54 6.42a2.78 2.78 0 0 0-1.94-2C18.88 4 12 4 12 4s-6.88 0-8.6.46a2.78 2.78 0 0 0-1.94 2A29 29 0 0 0 1 11.75a29 29 0 0 0 .46 5.33A2.78 2.78 0 0 0 3.4 19c1.72.46 8.6.46 8.6.46s6.88 0 8.6-.46a2.78 2.78 0 0 0 1.94-2 29 29 0 0 0 .46-5.25 29 29 0 0 0-.46-5.33z",
                    "M9.75 15.02l5.75-3.27-5.75-3.27v6.54z",
                ],
            },
        },
        SocialLink {
            platform: "instagram",
            url: "https://instagram.com/pocketjack1",
            title: "Instagram",
            svg: SvgData {
                width: 24,
                height: 24,
                view_box: "0 0 24 24",
                paths: &[
                    "M2 2h20v20H2z",
                    "M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z",
                    "M17.5 6.5h.01",
                ],
            },
        },
        SocialLink {
            platform: "patreon",
            url: "https://patreon.com/pocketjack",
            title: "Patreon",
            svg: SvgData {
                width: 24,
                height: 24,
                view_box: "0 0 24 24",
                paths: &[
                    "M12 2a10 10 0 1 1 0 20 10 10 0 0 1 0-20z",
                    "M12 8a4 4 0 1 0 0 8 4 4 0 0 0 0-8z",
                ],
            },
        },
        SocialLink {
            platform: "soundcloud",
            url: "https://soundcloud.com/pocketjack",
            title: "SoundCloud",
            svg: SvgData {
                width: 24,
                height: 24,
                view_box: "0 0 24 24",
                paths: &[
                    "M2 12h1",
                    "M6 12h1",
                    "M10 12h1",
                    "M14 12h1",
                    "M18 12h1",
                    "M4 18V6",
                    "M8 18V6",
                    "M12 18V6",
                    "M16 18V6",
                    "M20 18V6",
                ],
            },
        },
    ],
};