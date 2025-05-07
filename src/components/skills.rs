use yew::prelude::*;

#[function_component(Skills)]
pub fn skills() -> Html {
    // Define the skills data
    let technical_skills = vec![
        "Rust", "WASM", "JavaScript", "TypeScript","Nest.Js","SpringBoot", "Git", "Docker",
        "Redis", "ZeroMQ","Socketio", "Mongo DB","Postgres", "TypeORM", "GraphQL", "REST-APIs", "BlockChain", "Web3","HTML"
    ];
    // let hobbies = vec!["Bass Guitar", "Acoustic Guitar", "LoopMaking", "Camping", "Tanbour"];
    // let languages = vec!["English", "German", "Persian"];

    // Function to create a skill card
    let create_skill_card = |skill: &str| {
        html! {
            <div class="skill-card">
                <div class="skill-icon">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#00d4ff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <polyline points="16 18 22 12 16 6"></polyline>
                        <polyline points="8 6 2 12 8 18"></polyline>
                    </svg>
                </div>
                <div class="skill-name">{ skill }</div>
            </div>
        }
    };

    // Function to arrange skills into a reverse pyramid
    let arrange_reverse_pyramid = |items: Vec<&str>| -> Html {
        let mut rows: Vec<Vec<&str>> = Vec::new();
        let mut remaining = items;
        let mut row_size = 6; // Start with 7 items in the first row

        while !remaining.is_empty() {
            let current_row_size = row_size.min(remaining.len());
            let (row_items, rest) = remaining.split_at(current_row_size);
            rows.push(row_items.to_vec());
            remaining = rest.to_vec();
            row_size = if row_size > 1 { row_size - 1 } else { 1 }; // Decrease row size by 1, minimum 1
        }

        html! {
            <>
                { for rows.iter().map(|row| {
                    html! {
                        <div class="skill-row">
                            { for row.iter().map(|&skill| create_skill_card(skill)) }
                        </div>
                    }
                }) }
            </>
        }
    };

    html! {
        <section id="skills" class="section">
            <div class="container">
                <h2 class="section-title">{ "Technical Skills" }</h2>
                <div class="skills-list">
                    { arrange_reverse_pyramid(technical_skills) }
                </div>

                // <h2 class="section-title" style="margin-top:2rem;">{ "Hobbies" }</h2>
                // <div class="skills-list">
                //     { arrange_reverse_pyramid(hobbies) }
                // </div>

                // <h2 class="section-title" style="margin-top:2rem;">{ "Languages" }</h2>
                // <div class="skills-list">
                //     { arrange_reverse_pyramid(languages) }
                // </div>
            </div>
        </section>
    }
}