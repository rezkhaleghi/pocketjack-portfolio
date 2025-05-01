use yew::prelude::*;
use chrono::{Datelike, Local};

#[function_component(Footer)]
pub fn footer() -> Html {
    let current_year = Local::now().year();
    html! {
        <footer style="padding: 1rem 0; text-align: center; border-top: 1px solid var(--border); display: flex; flex-direction: column; align-items: center; gap: 0.75rem;">
            <div style="display: flex; align-items: center; gap: 1rem;">
                <span style="font-weight: 500;">{"by PocketJack"}</span>
                <a href="https://github.com/rezkhaleghi" target="_blank" rel="noopener noreferrer" style="color: var(--text); display: flex; align-items: center;">
                    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
                    </svg>
                </a>
                <a href="mailto:rezaxkhaleghi@gmail.com" style="color: var(--text); display: flex; align-items: center;">
                    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"></path>
                        <polyline points="22,6 12,13 2,6"></polyline>
                    </svg>
                </a>
            </div>
            <div style="font-size: 0.9rem; color: var(--muted-text, #777);">
                <span>{format!("© {} False Foundation. All rights reserved.", current_year)}</span>
            </div>
        </footer>
    }
}