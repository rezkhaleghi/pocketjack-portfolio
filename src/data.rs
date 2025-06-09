use crate::models::{AboutData, AboutSection, ContactData, ContactLink, Experience, ExperienceData, ExperienceLink, HeroData, MusicData, MusicTrack, Project, ProjectLink, SkillsData, SocialLink, SvgData};

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
                width: "24",
                height: "24",
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
                width: "24",
                height: "24",
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
                width: "24",
                height: "24",
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
                width: "24",
                height: "24",
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
                width: "24",
                height: "24",
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
                width: "24",
                height: "24",
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
        SocialLink {
            platform: "medium",
            url: "https://medium.com/@rezaxkhaleghi",
            title: "Medium",
            svg: SvgData {
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                paths: &[
                    "M2 3h20v18H2z",
                    "M6.5 7.5l4.5 4.5-4.5 4.5",
                    "M13 7.5h4.5",
                ],
            },
        },
    ],
};

pub const ABOUT_DATA: AboutData = AboutData {
    sections: &[
        AboutSection {
            emoji: "🗿",
            text: "Hello World, This is Reza Khaleghi AKA Pocketjack. Software engineer, Musician and co-founder of FALSE-FOUNDATION. Born on July 25, 2000, currently base in Turkey.",
        },
        AboutSection {
            emoji: "💻",
            text: "My journey into tech started back in 2011 when I began learning computer programming. I spent a couple of years dabbling in different areas like networking and security, but it wasn’t until I found backend development that everything clicked. I started out as a freelancer and eventually moved into leading backend teams, focusing on building scalable and efficient systems.",
        },
        AboutSection {
            emoji: "🎵",
            text: "Outside of tech, I’m all about music. I picked up the bass guitar in 2013, then moved on to the acoustic guitar, and got hooked on rock music. In 2020, I dove into trap and hip-hop by making loops and beats, and it's become a hobby I still enjoy today.",
        },
        AboutSection {
            emoji: "🦀🎷",
            text: "Right now, I’m totally fascinated by two things: jazz music and Rust. They’ve both completely changed how I see the world—jazz opened my ears to a whole new dimension of sound, while Rust has shifted how I think about programming, making it even more enjoyable and powerful.",
        },
        AboutSection {
            emoji: "🏕️",
            text: "When I'm not coding or making music, you'll probably find me out in nature—whether it’s camping, hiking, or traveling. These have been passions of mine since I was a kid. If I’m back in Iran, you’ll probably catch me offroading in my old Land Rover, chasing that next adventure.",
        },
    ],
};

pub const CLIENT_PROJECTS_DATA: &[Project] = &[
    Project {
        image: "./static/tmar.png",
        title: "Tmar-Travel",
        description: "Developed TMAR Travel, a ride-hailing platform for eco-tourism and off-road trips, where users can request a ride by selecting pickup, destination, and vehicle type. Architected the entire backend from scratch using a microservices approach, ensuring scalability, real-time processing, and clean service communication. Led a small agile team, guiding development, design patterns, and on-time delivery.",
        links: &[ProjectLink {
            text: "View Website →",
            url: "https://tmar.app",
        }],
        technologies: "Node.Js, Nest.Js, MSSQL, MongoDB",
    },
    Project {
        image: "./static/test.jpg",
        title: "UNFXB-Explorer",
        description: "Developed a multi-chain blockchain explorer enabling users to search transactions, wallets, hashes, and blocks across various blockchain networks including Bitcoin, Ethereum, Tron, Bitcoin Cash, Dogecoin, and Binance Coin (BNB). Leveraged blockchain APIs and technologies to create a user-friendly interface for navigating and exploring blockchain data across multiple networks.",
        links: &[ProjectLink {
            text: "View Website →",
            url: "https://explorer.unfxbit.com",
        }],
        technologies: "Node.Js, MongoDB",
    },
    Project {
        image: "./static/poudi.png",
        title: "Poudi-Guitar",
        description: "Engineered a simple guitar/music e-learning platform with robust e-commerce features and protected access codes. Orchestrated backend development for smooth and reliable user interaction, ensuring a user-friendly experience for accessing tutorial videos.",
        links: &[ProjectLink {
            text: "View Website →",
            url: "https://poudiguitar.com",
        }],
        technologies: "Node.Js, MongoDB",
    },
    Project {
        image: "./static/test.jpg",
        title: "Knowledge Management System(KMS)",
        description: "I successfully developed the backend of a Knowledge Management System (KMS) that functions as a social network for knowledge sharing and collaboration, using the KeystoneJS framework, a simple and flexible Node.js framework. This project involved creating a robust and scalable platform that enables users to connect, share insights, and manage knowledge assets within an organization.",
        links: &[],
        technologies: "KeystoneJS, PostgreSQL, GraphQL, Node.Js",
    },
    Project {
        image: "./static/test.jpg",
        title: "VVC Exchange",
        description: "Constructed a decentralized cryptocurrency exchange platform on the Stellar blockchain, enabling global fiat/crypto transfers and offering a versatile payment gateway.",
        links: &[],
        technologies: "Node.Js, Stellar Blockchain, MongoDB",
    },
    Project {
        image: "./static/test.jpg",
        title: "Wallet Custody",
        description: "Wallet Custody is a high-performance backend system designed to interface directly with multiple blockchain nodes (e.g., BTC, ETH, TRX, DOGE). It connects to these nodes to read blocks, monitor transactions in real time, and extract on-chain data critical for exchange and broker operations. Serving as the backbone of trading platforms, Wallet Custody ensures reliable transaction tracking, wallet activity monitoring, and seamless multi-chain support—all built with scalability and security in mind.",
        links: &[],
        technologies: "Nest.Js, MongoDB, Redis, RabbitMQ, bitcoin-core, web3.js",
    },
    Project {
        image: "./static/airgap.png",
        title: "AIR GAP",
        description: "The AIR-GAP Solution is a secure, offline-capable application designed for generating, encrypting, and managing cryptocurrency wallet mnemonic phrases (seed phrases). The application supports mnemonic generation in multiple languages (e.g., English) using 128-bit entropy and SHA-256 checksums, adhering to BIP-39 standards. It employs RSA-4096 for public-key encryption of mnemonics and AES-256-CBC for encrypting private keys, with unique initialization vectors (IVs) per user. The application is packaged as a single executable binary file.",
        links: &[],
        technologies: "Rust, HTML, CSS",
    },
    Project {
        image: "./static/digi.png",
        title: "DG-CMS",
        description: "Built a lightweight content management tool for DIGIALPHA Agency to help manage and publish blog posts and updates. The app allows the team to easily create, edit, and organize content through a simple interface tailored to their needs.",
        links: &[ProjectLink {
            text: "View Website →",
            url: "https://digialpha.agency",
        }],
        technologies: "Node.js, Express.js, MongoDB",
    },
    Project {
        image: "./static/test.jpg",
        title: "Crypto Telegram Bot",
        description: "Developed a Telegram bot that provides users with real-time on-chain data and whale activity alerts by integrating with Glassnode and Whale Alert APIs. Users can send cryptocurrency symbols (like BTC) to the bot and receive key on-chain metrics—such as transaction volume, exchange flows, and active addresses—along with major whale transfers and market-moving events. This tool offers a fast and accessible way to monitor blockchain trends and large transactions right within Telegram.",
        links: &[],
        technologies: "Node.js",
    },
];

pub const PERSONAL_PROJECTS_DATA: &[Project] = &[
    Project {
        image: "./static/fairfly.jpg",
        title: "Fair-Fly",
        description: "Fair Fly is a web-based app that helps users find and compare affordable flight deals across multiple providers. Its core feature is a smart price alert system that notifies users when fares drop to their preferred price.",
        links: &[ProjectLink {
            text: "View Website →",
            url: "https://fairfly.site",
        }],
        technologies: "Rust, WebAssembly, actix-web, Yew, MongoDB",
    },
    Project {
        image: "./static/jorcolab.jpg",
        title: "Jorco-Lab",
        description: "Jorcolab is a creative hub and digital marketplace for musicians, producers, and artists. Discover and purchase high-quality beats and samples, book studio time, or offer and hire music services like mixing, mastering, recording, and live instrumentation.",
        links: &[ProjectLink {
            text: "View Website →",
            url: "https://jorcolab.com",
        }],
        technologies: "TypeScript, Nest.js, MongoDB",
    },
    Project {
        image: "./static/test.jpg",
        title: "False-Player",
        description: "False Player is a web platform and Telegram mini app that lets users search for movies, music, videos, and books from across the internet. Stream movies and music online directly—fast, free, and without the hassle.",
        links: &[
            ProjectLink {
                text: "View Website →",
                url: "https://player.false.foundation",
            },
            ProjectLink {
                text: "View Telegram App →",
                url: "https://t.me/FalsePlayer_bot",
            },
        ],
        technologies: "Rust, actix-web, teloxide, MongoDB",
    },
    Project {
        image: "./static/pjplayer.gif",
        title: "PJ-Player",
        description: "PJ-Player is a Rust-based CLI tool that allows you to search, download, and stream audio directly from your terminal.",
        links: &[ProjectLink {
            text: "View Source Code →",
            url: "https://github.com/rezkhaleghi/pj-player",
        }],
        technologies: "Rust, crossterm",
    },
    Project {
        image: "./static/pjgrep.png",
        title: "PJ-Grep",
        description: "A fast, flexible pattern search tool for files and directories. Easily search for patterns in filenames and content, filter by file extensions, and get color-coded results.",
        links: &[ProjectLink {
            text: "View Source Code →",
            url: "https://github.com/rezkhaleghi/pj-grep",
        }],
        technologies: "Rust",
    },
    Project {
        image: "./static/portfolio.png",
        title: "THIS WEBSITE :)",
        description: "I built this website using Yew, a Rust framework for creating web applications. It showcases my projects and skills, and serves as a portfolio to demonstrate my work.",
        links: &
        [ProjectLink {
            text: "View Website →",
            url: "https://pocketjack.vercel.app",
        },
        ProjectLink {
            text: "View Source Code →",
            url: "https://github.com/rezkhaleghi/pocketjack-portfolio"
        }],
        technologies: "Rust, Yew, WebAssembly",
    },
];

pub const CONTACT_DATA: ContactData = ContactData {
    description: "Interested in working together or have questions about my work? Feel free to reach out through any of the channels below.",
    links: &[
        ContactLink {
            platform: "Email",
            url: "mailto:rezaxkhaleghi@gmail.com",
            display_text: "rezaxkhaleghi@gmail.com",
        },
        ContactLink {
            platform: "GitHub",
            url: "https://github.com/rezkhaleghi",
            display_text: "github.com/rezkhaleghi",
        },
        ContactLink {
            platform: "LinkedIn",
            url: "https://linkedin.com/in/rezaxkhaleghi",
            display_text: "linkedin.com/in/rezaxkhaleghi",
        },
    ],
};

pub const EXPERIENCE_DATA: ExperienceData = ExperienceData {
    experiences: &[
        Experience {
            title: "Software Engineer",
            company: "Unicorn Forex Broker",
            date: "2024 - Present",
            description: "Worked on a large Broker CRM microservice and helped develop secure wallet custody systems. Independently built an air-gapped encryption solution for private key management. Designed a multi-chain blockchain explorer to streamline transaction tracking. Focused on improving workflows and security in blockchain-based financial services.",
            image: "/static/we-un.jpg",
            technologies: "Java, Springboot, Nest.js, Rust, TypeScript, Docker, MongoDB, ZMQ, Redis, Nats",
            links: &[ExperienceLink {
                text: "View Website →",
                url: "https://unfxb.com/",
            }],
        },
        Experience {
            title: "Lead BackEnd Developer",
            company: "TMAR",
            date: "2022 - Present",
            description: "Pioneered the development of TMAR Travel from scratch, architecting the entire backend, design patterns, and structures. Continuously enhanced and maintained the platform, enabling users to request off-road vehicles with professional drivers for their trips. Managed a small group of developers and designers, ensuring effective collaboration and project delivery.",
            image: "/static/we-tmar.jpg",
            technologies: "TypeScript, Nest.js, Node.js, Docker, MongoDB, Redis, MSSQL",
            links: &[ExperienceLink {
                text: "View Website →",
                url: "https://tmar.app",
            }],
        },
        Experience {
            title: "BackEnd Developer",
            company: "DigiAlpha",
            date: "2020 - 2022",
            description: "Played a pivotal role in developing several blockchain projects for DigiAlpha, contributing to the company's web development portfolio and success as a business landing page.",
            image: "/static/we-dg.jpg",
            technologies: "JavaScript, Node.js, MongoDB, Blockchain",
            links: &[ExperienceLink {
                text: "View Website →",
                url: "https://digialpha.agency",
            }],     
           },
        Experience {
            title: "Web Developer",
            company: "Freelance",
            date: "2016 - Present",
            description: "As a freelance developer, I’ve delivered a wide range of projects—from simple e-commerce websites to fully customized business solutions. My work spans web applications, Telegram bots, API integrations, and custom backend systems, tailored to meet unique client needs.",
            image: "/static/we0.jpg",
            technologies: "Rust, TypeScript, Node.js, MongoDB",
            links: &[ExperienceLink {
                text: "View Portfolio →",
                url: "https://pocketjack.vercel.app",
            }],
        },
    ],
};

pub const MUSIC_DATA: MusicData = MusicData {
    tracks: &[
        MusicTrack {
            title: "Farigo - Dark Alley",
            youtube_url: "https://www.youtube.com/embed/yWnvb65k6_E",
            description: "Produced by PocketJack x Moeeney",
            platforms: &[
                SocialLink {
                    platform: "soundcloud",
                    url: "https://soundcloud.com/farigo-music/dark-alley",
                    title: "Listen on SoundCloud",
                    svg: SvgData {
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        paths: &[
                            "M3 9v6h4l5 5V4L7 9H3z",
                            "M16 12h2",
                            "M20 12h2",
                        ],
                    },
                },
                SocialLink {
                    platform: "spotify",
                    url: "https://open.spotify.com/track/0bs3pzYBAjC03KrvkSREiu?si=c858fdd71ca54790",
                    title: "Listen on Spotify",
                    svg: SvgData {
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        paths: &[
                            "M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20z",
                            "M15.09 6.55a6.7 6.7 0 0 0-7.55 2.1 6.7 6.7 0 0 0 0 8.7 6.7 6.7 0 0 0 8.7 0",
                            "M13.66 9.28a4.5 4.5 0 0 0-5.66 1.42 4.5 4.5 0 0 0 0 5.66",
                            "M12.22 12a2.3 2.3 0 0 0-2.83.94 2.3 2.3 0 0 0 0 2.83",
                        ],
                    },
                },
                SocialLink {
                    platform: "youtube",
                    url: "https://www.youtube.com/watch?v=yWnvb65k6_E",
                    title: "Listen on YouTube",
                    svg: SvgData {
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        paths: &[
                            "M21 4a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V4z",
                            "M10 8l6 4-6 4V8z",
                        ],
                    },
                },
            ],
        },
        MusicTrack {
            title: "Farigo - Perfection",
            youtube_url: "https://www.youtube.com/embed/Iov95aS-O7U",
            description: "Produced by PocketJack x Moeeney x 808K x 808Cash x Farigo",
            platforms: &[
                SocialLink {
                    platform: "soundcloud",
                    url: "https://soundcloud.com/farigo-music/perfection",
                    title: "Listen on SoundCloud",
                    svg: SvgData {
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        paths: &[
                            "M3 9v6h4l5 5V4L7 9H3z",
                            "M16 12h2",
                            "M20 12h2",
                        ],
                    },
                },
                SocialLink {
                    platform: "spotify",
                    url: "https://open.spotify.com/track/6hSHnuPlqLhkpGCbpYvVXn?si=1a002fbf245e411a",
                    title: "Listen on Spotify",
                    svg: SvgData {
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        paths: &[
                            "M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20z",
                            "M15.09 6.55a6.7 6.7 0 0 0-7.55 2.1 6.7 6.7 0 0 0 0 8.7 6.7 6.7 0 0 0 8.7 0",
                            "M13.66 9.28a4.5 4.5 0 0 0-5.66 1.42 4.5 4.5 0 0 0 0 5.66",
                            "M12.22 12a2.3 2.3 0 0 0-2.83.94 2.3 2.3 0 0 0 0 2.83",
                        ],
                    },
                },
                SocialLink {
                    platform: "youtube",
                    url: "https://www.youtube.com/watch?v=Iov95aS-O7U",
                    title: "Listen on YouTube",
                    svg: SvgData {
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        paths: &[
                            "M21 4a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V4z",
                            "M10 8l6 4-6 4V8z",
                        ],
                    },
                },
            ],
        },
        MusicTrack {
            title: "Farigo - Freestyle",
            youtube_url: "https://www.youtube.com/embed/7dofpxeOHCw",
            description: "Produced by PocketJack x Moeeney",
            platforms: &[
                SocialLink {
                    platform: "soundcloud",
                    url: "https://soundcloud.com/farigo-music/freestyle",
                    title: "Listen on SoundCloud",
                    svg: SvgData {
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        paths: &[
                            "M3 9v6h4l5 5V4L7 9H3z",
                            "M16 12h2",
                            "M20 12h2",
                        ],
                    },
                },
                SocialLink {
                    platform: "spotify",
                    url: "https://open.spotify.com/album/5hvV1cVf9EkJu9tvFHxE5L?si=1ysLkteqSpGv322yCOzctw",
                    title: "Listen on Spotify",
                    svg: SvgData {
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        paths: &[
                            "M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20z",
                            "M15.09 6.55a6.7 6.7 0 0 0-7.55 2.1 6.7 6.7 0 0 0 0 8.7 6.7 6.7 0 0 0 8.7 0",
                            "M13.66 9.28a4.5 4.5 0 0 0-5.66 1.42 4.5 4.5 0 0 0 0 5.66",
                            "M12.22 12a2.3 2.3 0 0 0-2.83.94 2.3 2.3 0 0 0 0 2.83",
                        ],
                    },
                },
                SocialLink {
                    platform: "youtube",
                    url: "https://www.youtube.com/watch?v=7dofpxeOHCw",
                    title: "Listen on YouTube",
                    svg: SvgData {
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        paths: &[
                            "M21 4a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V4z",
                            "M10 8l6 4-6 4V8z",
                        ],
                    },
                },
            ],
        },
        MusicTrack {
            title: "Blue (demo)",
            youtube_url: "https://www.youtube.com/embed/G6mtWxIcYHA",
            description: "Produced by PocketJack x NersiBeats x 808Cash",
            platforms: &[
                SocialLink {
                    platform: "soundcloud",
                    url: "https://soundcloud.com/pocketjack/blue",
                    title: "Listen on SoundCloud",
                    svg: SvgData {
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        paths: &[
                            "M3 9v6h4l5 5V4L7 9H3z",
                            "M16 12h2",
                            "M20 12h2",
                        ],
                    },
                },
                SocialLink {
                    platform: "youtube",
                    url: "https://www.youtube.com/watch?v=G6mtWxIcYHA",
                    title: "Listen on YouTube",
                    svg: SvgData {
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        paths: &[
                            "M21 4a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V4z",
                            "M10 8l6 4-6 4V8z",
                        ],
                    },
                },
            ],
        },
        MusicTrack {
            title: "Mayas - Marpich",
            youtube_url: "https://www.youtube.com/embed/FoUr4ZgbvQo",
            description: "From my ex-band MAYAS, formed in 2017. I played the bass guitar.",
            platforms: &[
                SocialLink {
                    platform: "youtube",
                    url: "https://www.youtube.com/watch?v=FoUr4ZgbvQo",
                    title: "Listen on YouTube",
                    svg: SvgData {
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        paths: &[
                            "M21 4a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V4z",
                            "M10 8l6 4-6 4V8z",
                        ],
                    },
                },
            ],
        },
        MusicTrack {
            title: "Dayan - Shekaste | Live Performance",
            youtube_url: "https://youtube.com/embed/oS6rGsPtze4?si=Oukuy7S3Uhf9IOdu",
            description: "A live session featuring three demo tracks by Dayan. I performed as the acoustic guitarist.",
            platforms: &[
                SocialLink {
                    platform: "youtube",
                    url: "https://www.youtube.com/watch?v=oS6rGsPtze4",
                    title: "Listen on YouTube",
                    svg: SvgData {
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        paths: &[
                            "M21 4a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V4z",
                            "M10 8l6 4-6 4V8z",
                        ],
                    },
                },
            ],
        },
        MusicTrack {
            title: "Trap Type Beat | FREE",
            youtube_url: "https://youtube.com/embed/5Wh_iyh2Ino",
            description: "Produced by PocketJack x NersiBeats",
            platforms: &[
                SocialLink {
                    platform: "youtube",
                    url: "https://www.youtube.com/watch?v=5Wh_iyh2Ino",
                    title: "Listen on YouTube",
                    svg: SvgData {
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        paths: &[
                            "M21 4a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V4z",
                            "M10 8l6 4-6 4V8z",
                        ],
                    },
                },
            ],
        },
        MusicTrack {
            title: "Moeeney - Liar",
            youtube_url: "https://www.youtube.com/embed/bGGL5BsdGEY",
            description: "Produced by PocketJack x Moeeney x Rwayne",
            platforms: &[
                SocialLink {
                    platform: "youtube",
                    url: "https://www.youtube.com/watch?v=bGGL5BsdGEY",
                    title: "Listen on YouTube",
                    svg: SvgData {
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        paths: &[
                            "M21 4a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V4z",
                            "M10 8l6 4-6 4V8z",
                        ],
                    },
                },
            ],
        },
    ],
};

pub const SKILLS_DATA: SkillsData = SkillsData {
    technical_skills: &[
        "Rust", "WASM", "JavaScript", "TypeScript", "Nest.Js", "SpringBoot", "Git", "Docker",
        "Redis", "ZeroMQ", "Socketio", "Mongo DB", "Postgres", "TypeORM", "Linux", "GraphQL",
        "REST-APIs", "BlockChain", "Web3", "Test", "HTML","Microservices",
    ],
    hobbies: &["Bass Guitar", "Acoustic Guitar", "LoopMaking", "Composing", "Camping", "Tanbour"],
    languages: &["English", "German", "Persian"],
    technical_icon: SvgData {
        width: "20",
        height: "20",
        view_box: "0 0 24 24",
        paths: &[
            "M16 18l6-6-6-6",
            "M8 6l-6 6 6 6",
        ],
    },
    hobby_icon: SvgData {
        width: "20",
        height: "20",
        view_box: "0 0 24 24",
        paths: &[
            "M2 22h20",
            "M6 2v16a2 2 0 0 1-2 2h0a2 2 0 0 1-2-2V8",
            "M20 2v16a2 2 0 0 0 2 2h0a2 2 0 0 0 2-2V8",
            "M10 6h4",
            "M8 6a1 1 0 1 0 0-2 1 1 0 0 0 0 2",
            "M16 6a1 1 0 1 0 0-2 1 1 0 0 0 0 2",
        ],
    },
    language_icon: SvgData {
        width: "20",
        height: "20",
        view_box: "0 0 24 24",
        paths: &[
            "M12 22a10 10 0 1 0 0-20 10 10 0 0 0 0 20z",
            "M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20",
            "M2 12h20",
        ],
    },
};