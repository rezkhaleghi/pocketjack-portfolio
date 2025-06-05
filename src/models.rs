#[derive(Debug, Clone)]
pub struct HeroData<'a> {
    pub name: &'a str,
    pub subtitle: &'a str,
    pub description: &'a str,
    pub social_links: &'a [SocialLink<'a>],
}

#[derive(Debug, Clone)]
pub struct SocialLink<'a> {
    pub platform: &'a str,
    pub url: &'a str,
    pub title: &'a str,
    pub svg: SvgData<'a>,
}


#[derive(Debug, Clone)]
pub struct AboutData<'a> {
    pub sections: &'a [AboutSection<'a>],
}

#[derive(Debug, Clone)]
pub struct AboutSection<'a> {
    pub emoji: &'a str,
    pub text: &'a str,
}


#[derive(Debug, Clone)]
pub struct Project<'a> {
    pub image: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub links: &'a [ProjectLink<'a>],
    pub technologies: &'a str,
}

#[derive(Debug, Clone)]
pub struct ProjectLink<'a> {
    pub text: &'a str,
    pub url: &'a str,
}



#[derive(Debug, Clone)]
pub struct ContactData<'a> {
    pub description: &'a str,
    pub links: &'a [ContactLink<'a>],
}

#[derive(Debug, Clone)]
pub struct ContactLink<'a> {
    pub platform: &'a str,
    pub url: &'a str,
    pub display_text: &'a str,
}



#[derive(Debug, Clone)]
pub struct ExperienceData<'a> {
    pub experiences: &'a [Experience<'a>],
}

#[derive(Debug, Clone)]
pub struct Experience<'a> {
    pub title: &'a str,
    pub company: &'a str,
    pub date: &'a str,
    pub description: &'a str,
    pub image: &'a str,
    pub technologies: &'a str,
    pub links: &'a [ExperienceLink<'a>],
}

#[derive(Debug, Clone)]
pub struct ExperienceLink<'a> {
    pub text: &'a str,
    pub url: &'a str,
}




#[derive(Debug, Clone)]
pub struct MusicData<'a> {
    pub tracks: &'a [MusicTrack<'a>],
}

#[derive(Debug, Clone)]
pub struct MusicTrack<'a> {
    pub title: &'a str,
    pub youtube_url: &'a str,
    pub description: &'a str,
    pub platforms: &'a [SocialLink<'a>],
}



#[derive(Debug, Clone)]
pub struct SvgData<'a> {
    pub width: &'a str,
    pub height: &'a str,
    pub view_box: &'a str,
    pub paths: &'a [&'a str],
}

// #[derive(Debug, Clone)]
// pub struct SkillsData<'a> {
//     pub technical_skills: &'a [&'a str],
//     pub hobbies: &'a [&'a str],
//     pub languages: &'a [&'a str],
//     pub technical_icon: SvgData<'a>,
//     pub hobby_icon: SvgData<'a>,
//     pub language_icon: SvgData<'a>,
// }