use yew::prelude::*;
use crate::data::SKILLS_DATA;
use crate::models::SvgData;

#[function_component(Skills)]
pub fn skills() -> Html {
  let data = SKILLS_DATA;

  let create_skill_card = |skill: &str, category: &str, icon: SvgData| {
      let icon_class = match category {
          "hobby" => "skill-icon-hobby",
        //   "language" => "skill-icon-language",
          "technical" => "skill-icon-technical",
          _ => "skill-icon",
      };
      // Clone SvgData fields to owned Strings
      let width = icon.width.to_string();
      let height = icon.height.to_string();
      let view_box = icon.view_box.to_string();
      let paths = icon.paths.iter().map(|&p| p.to_string()).collect::<Vec<_>>();
      
      html! {
          <div class="skill-card">
              <div class={classes!("skill-icon", icon_class)}>
                  <svg
                      xmlns="http://www.w3.org/2000/svg"
                      width={width}
                      height={height}
                      viewBox={view_box}
                      fill="none"
                      stroke="#ffffff"
                      stroke-width="1"
                      stroke-linecap="square"
                      stroke-linejoin="miter"
                  >
                      { for paths.iter().map(|path| html! { <path d={path.clone()}></path> }) }
                  </svg>
              </div>
              <div class="skill-name">{ skill }</div>
          </div>
      }
  };

  html! {
      <section id="skills" class="section">
          <div class="container">
              <h2 class="section-title">{ "Skills" }</h2>
              <div class="skills-section">
                  <div class="skills-category">
                      <h3 class="skills-subtitle">{ "Technical Skills" }</h3>
                      <div class="skills-grid" aria-label="List of technical skills">
                          { for data.technical_skills.iter().map(|&skill| {
                              create_skill_card(skill, "technical", data.technical_icon.clone())
                          })}
                      </div>
                  </div>
                  <div class="skills-category">
                      <h3 class="skills-subtitle">{ "Music Skills" }</h3>
                      <div class="skills-grid" aria-label="List of hobbies">
                          { for data.hobbies.iter().map(|&skill| {
                              create_skill_card(skill, "hobby", data.music_icon.clone())
                          })}
                      </div>
                  </div>
                //   <div class="skills-category">
                //       <h3 class="skills-subtitle">{ "Languages" }</h3>
                //       <div class="skills-grid" aria-label="List of languages">
                //           { for data.languages.iter().map(|&skill| {
                //               create_skill_card(skill, "language", data.language_icon.clone())
                //           })}
                //       </div>
                //   </div>
              </div>
          </div>
      </section>
  }
}