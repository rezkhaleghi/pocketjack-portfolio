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
pub struct SvgData<'a> {
    pub width: i32,
    pub height: i32,
    pub view_box: &'a str,
    pub paths: &'a [&'a str],
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