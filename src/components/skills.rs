use yew::prelude::*;

#[function_component(Skills)]
pub fn skills() -> Html {
    // Define the skills data
    let technical_skills = vec![
        "Rust", "WASM", "JavaScript", "TypeScript", "Nest.Js", "SpringBoot", "Git", "Docker",
        "Redis", "ZeroMQ", "Socketio", "Mongo DB", "Postgres", "TypeORM", "Linux", "GraphQL",
        "REST-APIs", "BlockChain", "Web3", "Test", "HTML"
    ];
    let hobbies = vec!["Bass Guitar", "Acoustic Guitar", "LoopMaking", "Composing","Camping", "Tanbour"];
    let languages = vec!["English", "German", "Persian"];

    // Function to create a skill card with category-specific icon
    let create_skill_card = |skill: &str, category: &str| {
        let icon_class = match category {
            "hobby" => "skill-icon-hobby",
            "language" => "skill-icon-language",
            _ => "skill-icon-technical",
        };
        let svg = match category {
            "hobby" => html! {
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#00d4ff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M2 22h20"></path>
                    <path d="M6 2v16a2 2 0 0 1-2 2h0a2 2 0 0 1-2-2V8"></path>
                    <path d="M20 2v16a2 2 0 0 0 2 2h0a2 2 0 0 0 2-2V8"></path>
                    <path d="M10 6h4"></path>
                    <circle cx="8" cy="6" r="1"></circle>
                    <circle cx="16" cy="6" r="1"></circle>
                </svg>
            },
            "language" => html! {
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#00d4ff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <circle cx="12" cy="12" r="10"></circle>
                    <path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"></path>
                    <path d="M2 12h20"></path>
                </svg>
            },
            _ => html! {
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#00d4ff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="16 18 22 12 16 6"></polyline>
                    <polyline points="8 6 2 12 8 18"></polyline>
                </svg>
            },
        };

        html! {
            <div class="skill-card">
                <div class={classes!("skill-icon", icon_class)}>
                    { svg }
                </div>
                <div class="skill-name">{ skill }</div>
            </div>
        }
    };

    // Function to arrange skills into a reverse pyramid
    let arrange_reverse_pyramid = |items: Vec<&str>, category: &str| -> Html {
        let mut rows: Vec<Vec<&str>> = Vec::new();
        let mut remaining = items;
        let mut row_size = 6; // Start with 6 items in the first row

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
                            { for row.iter().map(|&skill| create_skill_card(skill, category)) }
                        </div>
                    }
                }) }
            </>
        }
    };

    // Function to arrange skills into a normal pyramid
    let arrange_normal_pyramid = |items: Vec<&str>, category: &str| -> Html {
        let mut rows: Vec<Vec<&str>> = Vec::new();
        let mut remaining = items;
        let mut row_size = 1; // Start with 1 item in the first row

        while !remaining.is_empty() {
            let current_row_size = row_size.min(remaining.len());
            let (row_items, rest) = remaining.split_at(current_row_size);
            rows.push(row_items.to_vec());
            remaining = rest.to_vec();
            row_size += 1; // Increase row size by 1
        }

        html! {
            <>
                { for rows.iter().map(|row| {
                    html! {
                        <div class="skill-row">
                            { for row.iter().map(|&skill| create_skill_card(skill, category)) }
                        </div>
                    }
                }) }
            </>
        }
    };

    // Calculate margin-top for left/right pyramids to align with bottom of center pyramid
    let center_rows = 6;
    let hobby_rows = 3;
    let language_rows = 2;
    let row_height = "135px"; // 120px (skill-card) + 1.5rem (gap, ~15px)
    let hobby_margin_top = format!("calc({} * ({} - {}))", row_height, center_rows, hobby_rows);
    let language_margin_top = format!("calc({} * ({} - {}))", row_height, center_rows, language_rows);

    html! {
        <section id="skills" class="section">
            <div class="container">
                <div class="pyramid-container">
                    <div class="pyramid-column" style={format!("margin-top: {};", hobby_margin_top)}>
                        <button class="pyramid-button" aria-label="Hobbies section">{ "Hobbies" }</button>
                        <div class="skills-list" aria-label="List of hobbies">
                            { arrange_normal_pyramid(hobbies, "hobby") }
                        </div>
                    </div>
                    <div class="pyramid-column">
                        <h2 class="section-title">{ "Technical Skills" }</h2>
                        <div class="skills-list" aria-label="List of technical skills">
                            { arrange_reverse_pyramid(technical_skills, "technical") }
                        </div>
                    </div>
                    <div class="pyramid-column" style={format!("margin-top: {};", language_margin_top)}>
                        <button class="pyramid-button" aria-label="Languages section">{ "Languages" }</button>
                        <div class="skills-list" aria-label="List of languages">
                            { arrange_normal_pyramid(languages, "language") }
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}