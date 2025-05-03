use yew::prelude::*;
use gloo::events::EventListener;
use web_sys::{Window, ScrollToOptions};
use wasm_bindgen::prelude::*;

#[function_component(BackToTop)]
pub fn back_to_top() -> Html {
    let is_visible = use_state(|| false);

    {
        let is_visible = is_visible.clone();
        use_effect(move || {
            let window = web_sys::window().expect("no global window exists");
            let listener = EventListener::new(&window, "scroll", move |_| {
                let window = web_sys::window().expect("no global window exists");
                let scroll_y = window.page_y_offset().unwrap_or(0.0);
                is_visible.set(scroll_y > 300.0);
            });
            || drop(listener)
        });
    }

    let onclick = Callback::from(|e: MouseEvent| {
        e.prevent_default();
        let window = web_sys::window().expect("no global window exists");
        let options = ScrollToOptions::new();
        // options.top(0.0);
        // options.set_behavior("smooth");
        window.scroll_to_with_scroll_to_options(&options);
    });

    html! {
        <a href="#" class={classes!("back-to-top", if *is_visible { "visible" } else { "" })} {onclick}>
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="18 15 12 9 6 15"></polyline>
            </svg>
        </a>
    }
}